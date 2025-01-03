## Apagado y limpieza eficientes

El código del Listing 21-20 está respondiendo requests de forma asíncrona 
mediante el uso de un pool de threads, como pretendíamos, Recibimos 
algunas advertencias sobre los campos `workers`, `id` y `thread` que no
estamos usando de forma directa que nos recuerda que no estamos limpiando
nada. Cuando usamos el método menos elegante <kbd>ctrl</kbd>-<kbd>c</kbd>
para detener el thread principal, todos los demás threads se detienen 
inmediatamente también, incluso si están en medio de servir una request.

A continuación, implementaremos el trait `Drop` para llamar a `join` en cada uno
de los threads del pool para que puedan terminar las requests en las que están
trabajando antes de cerrar. Luego implementaremos una forma de decirle a los 
threads que deben dejar de aceptar nuevas requests y cerrarse. Para ver este
código en acción, modificaremos nuestro servidor para que acepte solo dos
requests antes de cerrar el pool de threads correctamente.

Algo importante a tener en cuenta mientras avanzamos: nada de esto afecta las 
partes del código que manejan la ejecución de los closures, por lo que todo 
aquí sería exactamente igual si estuviéramos usando un thread pool para un 
runtime asincrónico.

### Implementando el Trait `Drop` en `ThreadPool`

Comencemos implementando `Drop` en nuestro pool de threads. Cuando el pool se
destruye, nuestros threads deberían unirse para asegurarse de que terminan su
trabajo. El Listing 21-22 muestra un primer intento de implementación de `Drop`;
este código aún no funcionará.

<Listing number="21-22" file-name="src/lib.rs" caption="Uniendo cada thread cuando el thread pool se sale del scope">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-22/src/lib.rs:here}}
```

</Listing>

Primero, iteramos a través de cada uno de los `workers` del pool de threads.
Usamos `&mut` para esto porque `self` es una referencia mutable, y también
necesitamos poder mutar `worker`. Para cada worker, imprimimos un mensaje
diciendo que este worker en particular se está cerrando, y luego llamamos a
`join` en el thread de ese worker. Si la llamada a `join` falla, usamos
`unwrap` para que Rust entre en pánico y haga una salida poco elegante.

Aquí está el error que obtenemos cuando compilamos este código:

```console
{{#include ../listings/ch21-web-server/listing-21-22/output.txt}}
```

El error nos dice que no podemos llamar a `join` porque solo tenemos un
mutable borrow de cada `worker` y `join` toma el ownership de su argumento. 
Para solucionar este problema, necesitamos mover el thread fuera de la
instancia de `Worker` que posee `thread` para que `join` pueda consumir el
thread. Hicimos esto en el Listing 17-15: si `Worker` tiene un
`Option<thread::JoinHandle<()>>` en su lugar, podemos llamar al método
`take` en el `Option` para mover el valor fuera de la variante `Some` y
dejar una variante `None` en su lugar. En otras palabras, un `Worker` que
se está ejecutando tendrá una variante `Some` en `thread`, y cuando
queramos limpiar un `Worker`, reemplazaremos `Some` con `None` para que el
`Worker` no tenga un thread para ejecutar.

Entonces sabemos que queremos actualizar la definición de `Worker` de esta
manera:

<Listing file-name="src/lib.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/no-listing-04-update-worker-definition/src/lib.rs:here}}
```

</Listing>

Ahora usemos el compilador para encontrar los otros lugares que necesitan
cambiar. Al verificar este código, obtenemos dos errores:

```console
{{#include ../listings/ch21-web-server/no-listing-04-update-worker-definition/output.txt}}
```

Abordemos el segundo error, que apunta al código al final de `Worker::new`;
necesitamos envolver el valor `thread` en `Some` cuando creamos un nuevo
`Worker`. Haga los siguientes cambios para corregir este error:

<Listing file-name="src/lib.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/no-listing-05-fix-worker-new/src/lib.rs:here}}
```

</Listing>

El primer error está en nuestra implementación de `Drop`. Mencionamos
anteriormente que pretendíamos llamar a `take` en el valor `Option` para mover
`thread` fuera de `worker`. Los siguientes cambios lo harán:

<Listing file-name="src/lib.rs">

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/no-listing-06-fix-threadpool-drop/src/lib.rs:here}}
```

</Listing>

Como discutimos en el Capítulo 18, el método `take` en `Option` toma la variante
`Some` y deja `None` en su lugar. Estamos usando `if let` para deconstruir el
`Some` y obtener el thread; luego llamamos a `join` en el thread. Si el thread
de un worker ya es `None`, sabemos que ese worker ya ha tenido su thread
limpiado, por lo que en ese caso no sucede nada.

### Señalando a los threads que dejen de escuchar por jobs

Con todos los cambios que hemos hecho, nuestro código se compila sin advertencias.
Sin embargo, las malas noticias son que este código aún no funciona de la manera
que queremos. La clave es la lógica en los closures ejecutados por los threads
de las instancias de `Worker`: en este momento, llamamos a `join`, pero eso no
detendrá los threads porque se ejecutan en un `loop` para siempre buscando jobs.
Si intentamos dejar caer nuestro `ThreadPool` con nuestra implementación actual
de `drop`, el thread principal se bloqueará para siempre esperando a que el
primer thread termine.

Para solucionar este problema, necesitamos un cambio en la implementación de
`drop` de `ThreadPool` y luego un cambio en el loop de `Worker`.

En primer lugar, cambiemos la implementación de `drop` de `ThreadPool` para
soltar explícitamente el `sender` antes de esperar a que los threads terminen.
El Listing 21-23 muestra los cambios en `ThreadPool` para soltar explícitamente
`sender`. Usamos la misma técnica `Option` y `take` que hicimos con el thread
para poder mover `sender` fuera de `ThreadPool`:

<Listing number="21-23" file-name="src/lib.rs" caption="Libera explicitamente `sender` antes de unirse a los threads del worker">

```rust,noplayground,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-23/src/lib.rs:here}}
```

</Listing>

Soltar `sender` cierra el canal, lo que indica que no se enviarán más mensajes.
Cuando eso sucede, todas las llamadas a `recv` que los workers hacen en el loop
infinito devolverán un error. En el Listing 21-24, cambiamos el loop de `Worker`
para salir del loop con gracia en ese caso, lo que significa que los hreads
terminarán cuando la implementación de `drop` de `ThreadPool` llame a `join`
en ellos.

<Listing number="21-24" file-name="src/lib.rs" caption="Saliendo explícitamente del loop cuando `recv` devuelve un error">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-24/src/lib.rs:here}}
```

</Listing>

Para ver este código en acción, modifiquemos `main` para aceptar solo dos
requests antes de cerrar el servidor con gracia, como se muestra en el
Listing 21-25.

<Listing number="21-25" file-name="src/main.rs" caption="Shut down the server after serving two requests by exiting the loop">

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/listing-21-25/src/main.rs:here}}
```

</Listing>

No querríamos que un servidor web del mundo real se apague después de servir
solo dos requests. Este código solo demuestra que el apagado y la limpieza
con gracia funcionan.

El método `take` es definido en el trait `Iterator` y limita la iteración
de los primeros dos items como máximo. El `ThreadPool` saldrá del scope 
al final de `main` y la implementación `drop` correrá.

Iniciamos el servidor con `cargo run` y hacemos tres requests. La tercera
request debería fallar, y en su terminal debería ver una salida similar a esta:

<!-- manual-regeneration
cd listings/ch21-web-server/listing-21-25
cargo run
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
third request will error because server will have shut down
copy output below
Can't automate because the output depends on making requests
-->

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/hello`
Worker 0 got a job; executing.
Shutting down.
Shutting down worker 0
Worker 3 got a job; executing.
Worker 1 disconnected; shutting down.
Worker 2 disconnected; shutting down.
Worker 3 disconnected; shutting down.
Worker 0 disconnected; shutting down.
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
```

Es posible que vea un orden diferente de workers y mensajes impresos. Podemos
ver cómo funciona este código a partir de los mensajes: los workers 0 y 3
obtuvieron las dos primeras requests. El servidor dejó de aceptar conexiones
después de la segunda conexión, y la implementación `Drop` en `ThreadPool`
comienza a ejecutarse antes de que el worker 3 comience su trabajo. Al soltar
`sender` desconecta a todos los workers y les dice que se apaguen. Los workers
imprimen un mensaje cuando se desconectan, y luego el pool de threads llama a
`join` para esperar a que cada thread worker termine.

Fijémonos en un aspecto interesante de esta ejecución en particular: el
`ThreadPool` soltó el `sender`, y antes de que cualquier worker recibiera un
error, intentamos unirnos al worker 0. El worker 0 aún no había recibido un
error de `recv`, por lo que el thread principal se bloqueó esperando a que el
worker 0 terminara. Mientras tanto, el worker 3 recibió un job y luego todos
los threads recibieron un error. Cuando el worker 0 terminó, el thread principal
esperó a que el resto de los workers terminaran. En ese momento, todos habían
salido de sus loops y se detuvieron.

¡Enhorabuena! Hemos completado nuestro proyecto; tenemos un servidor web básico
que usa un pool de threads para responder de forma asíncrona. Podemos realizar
un apagado con gracia del servidor, que limpia todos los threads del pool.

Aquí está el código completo como referencia:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/no-listing-07-final-code/src/main.rs}}
```

</Listing>

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-07-final-code/src/lib.rs}}
```

</Listing>

¡Podríamos hacer más! Si quieres seguir mejorando este proyecto, aquí hay algunas
ideas:

* Añadir más documentación a `ThreadPool` y sus métodos públicos.
* Añadir tests de la funcionalidad de la librería.
* Cambiar las llamadas a `unwrap` por un manejo de errores más robusto.
* Usar `ThreadPool` para realizar alguna tarea que no sea servir requests web.
* Encontrar una librería de pool de threads en [crates.io](https://crates.io/) e
  implementar un servidor web similar usando la librería en su lugar. Luego
  compara su API y robustez con el pool de threads que implementamos.

## Resumen

¡Bien hecho! ¡Has llegado al final del libro! Queremos agradecerte por unirte
a nosotros en este tour de Rust. Ahora estás listo para implementar tus propios
proyectos en Rust y ayudar con los proyectos de otras personas. Ten en cuenta
que hay una comunidad acogedora de otros Rustaceans que estarían encantados de
ayudarte con cualquier desafío que encuentres en tu viaje con Rust.
