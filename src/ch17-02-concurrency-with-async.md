## Concurrencia con Async

En esta sección, aplicaremos async a algunos de los mismos desafíos
de concurrencia que abordamos con threads en el capítulo 16. Hemos explorado muchas de
las ideas claves en esa sección, aquí se profundizara en las diferencias
entre threads and futures.

En muchos casos, las APIs para trabajar con concurrencia usando async son muy
similares a las que se usan con threads. En otros casos, terminan teniendo formas
bastante diferentes. Incluso cuando las APIs parecen similares entre threads y async,
a menudo presentan comportamientos distintos y casi siempre tienen diferentes
características de rendimiento.

### Conteo

La primera tarea que abordamos en el capítulo 16 fue contar en dos threads separados.
Hagamos lo mismo usando async. El crate `trpl`  proporciona una función `spawn_task`,
que se comporta de forma muy similar a la API de `thread::spawn`, y una función `sleep`,
que es una versión async de `thread::sleep`. Podemos usarlas juntas para implementar
el mismo ejemplo de conteo que con threads, como se muestra en el Listado 17-6.

<Listing number="17-6" caption="Using `spawn_task` to count with two" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-06/src/main.rs:all}}
```

</Listing>

Como punto de partida, configuramos nuestra función `main` con `trpl::run`, de
de modo que nuestra función de nivel superior pueda ser async.

> Nota: A partir de este punto en el capítulo, cada ejemplo incluirá
> este mismo código de envoltura con  `trpl::run` en `main`, así que a menudo lo omitiremos,
> igual que hacemos con `main`. ¡No olvides incluirlo en tu código!

Luego, dentro de ese bloque, escribimos dos bucles, cada uno con una llamada a `trpl::sleep`,
que espera medio segundo (500 milisegundos) antes de enviar el siguiente
mensaje. Uno de los bucles va dentro de `trpl::spawn_task`, mientras que el otro se ejecuta
en un bucle `for` de nivel superior. También añadimos un `await` después de cada llamada a `sleep`.

El resultado es similar a la versión basada en threads, incluyendo el detalle
de que los mensajes podrían aparecer en un orden distinto cada vez
que lo ejecutes en tu terminal.

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the second task!
hi number 1 from the first task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
```

Esta versión se detiene tan pronto como el bucle for dentro del bloque async principal
termina, porque la tarea creada con `spawn_task` se cierra cuando finaliza la función
main. Si quieres que el programa siga ejecutándose hasta que la tarea termine por completo,
necesitas usar un join handle para esperar a que la primera tarea finalice. Con
threads, utilizamos el método `join` para “bloquear”, la ejecución hasta que el hilo terminara.
En el Listado 17-7, podemos hacer lo mismo con `await`, ya que el handle de la tarea
en sí es un future. Su tipo de `Output` es un `Result`, por lo que también usamos unwrap
después de esperarlo con await.

<Listing number="17-7" caption="Using `await` with a join handle to run a task to completion" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-07/src/main.rs:handle}}
```

</Listing>

Esta versión actualizada se ejecuta hasta que *ambos* bucles terminan.

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the second task!
hi number 1 from the first task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!
```

Hasta ahora, parece que async y threads nos dan los mismos resultados básicos, solo
que con una sintaxis diferente: usamos `await` en lugar de llamar a `join` en el join
handle, y también esperamos las llamadas a `sleep`.

La mayor diferencia aquí es que no necesitamos crear otro hilo del sistema operativo
para lograrlo. De hecho, ni siquiera es necesario generar una tarea separada. Pues
los bloques async se compilan en futures anónimos, podemos colocar cada bucle dentro de un
bloque async y dejar que el runtime los ejecute hasta su finalización usando la
función `trpl::join`.

En el capítulo 16, mostramos cómo usar el método `join` en el tipo `JoinHandle`
que se obtiene al llamar `std::thread::spawn`. La función `trpl::join` es
similar, pero para futures. Cuando le pasas dos futures, genera un nuevo
future cuyo resultado es una tupla con los valores de salida de los futures originales,
pero solo cuando *ambos* han finalizado. Es decir, en Listado 17-8, usamos `trpl::join` para esperar a
que tanto `fut1` como `fut2` finalicen. En lugar de hacer await sobre `fut1` y `fut2` por separado, esperamos
el nuevo futuro producido por `trpl::join`. Ignoramos su salida, debido a que
solo contiene una tupla con dos valores unitarios.

<Listing number="17-8" caption="Using `trpl::join` to await two anonymous futures" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-08/src/main.rs:join}}
```

</Listing>

Cuando ejecutamos esto, vemos que ambos futuros se ejecutan hasta completarse:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the first task!
hi number 1 from the second task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!
```

Aquí verás exactamente el mismo orden en cada ejecución, lo cual es muy diferente
de lo que ocurría con threads. Esto se debe a que la función `trpl::join`  es *justa*,
lo que significa que revisa cada future con la misma frecuencia, alternando entre ellos y evitando
que uno avance más rápido que el otro si ambos están listos. Con threads, el sistema operativo
decide qué hilo revisar y cuánto tiempo permitirle ejecutarse. Con async Rust, es
el runtime el que decide que tarea revisar. (En la práctica, esto se vuelve más complejo
porque un runtime async puede usar threads del sistema operativo en segundo
plano para gestionar la concurrencia, lo que hace que garantizar la equidad requiera más trabajo
—¡pero sigue siendo posible!). Los runtimes no están obligados
a garantizar equidad en todas las operaciones, y muchas veces ofrecen diferentes APIs
para que elijas si quieres equidad o no.

Prueba algunas variaciones en la forma de esperar los futures y observa qué
sucede:

* Elimina el bloque async alrededor de uno o ambos bucles.
* Espera (`await`) cada bloque async inmediatamente después de definirlo.
* Envuelve solo el primer bucle en un bloque async y espera el future resultante
  después del cuerpo del segundo bucle.

Para un desafío extra, intenta predecir la salida en
cada caso *antes* de ejecutar el código.

### Paso de Mensajes

Compartir datos entre futures también te resultará familiar: volveremos a usar el paso de
mensajes, pero esta vez con versiones async de los tipos y funciones. Seguiremos un
enfoque ligeramente diferente al del capítulo 16 para destacar algunas diferencias clave
entre la concurrencia basada en threads y la basada en futures. En el Listado 17-9,
comenzaremos con un solo bloque async, sin generar una tarea
separada como lo hicimos al crear un thread independiente.

<Listing number="17-9" caption="Creating an async channel and assigning the two halves to `tx` and `rx`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-09/src/main.rs:channel}}
```

</Listing>

Aquí usamos `trpl::channel`, una version de async de la API multiple-producer y
single-consumer que usamos con threads en el capítulo 16. La version async
de esta API es solo un poco diferente de la versión basada en threads: en lugar
de un receptor inmutable, usa un receptor `rx` mutable, y su método `recv` produce
un future debemos esperar con `await` en lugar de devolver el valor directamente. Ahora
podemos enviar mensajes desde el sender al receiver. Fíjate en que no necesitamos crear un
thread separado ni siquiera una tarea; simplemente esperamos la
llamada `rx.recv`.

El método síncrono `Receiver::recv` en `std::mpsc::channel` bloquea la ejecución hasta
recibir un mensaje.  En cambio, el método `trpl::Receiver::recv` no, porque
es async. En lugar de bloquear, devuelve el control al runtime hasta que se recibe
un mensaje o se cierra el lado del envío del canal. Por otro lado, no
esperamos en la llamada a `send`,  porque esta no bloquea. No es necesario,
ya que el canal al que estamos enviando los mensajes es ilimitado.

> Nota: Todo este código async se ejecuta dentro de un bloque async dentro de una llamada a `trpl::run`,
> lo que permite evitar bloqueos dentro de él. Sin embargo, el código *fuera* de
> este bloque sí se bloqueará hasta que `run` termine. Esa es precisamente la función de
> `trpl::run`: te permite *eligir* en qué parte del código async quieres bloquear la ejecución,
> definiendo así la transición entre código síncrono y asíncrono. En la mayoría de
> runtimes async, la función `run` suele llamarse `block_on` por esta misma razón.

Hay dos cosas a notar en este ejemplo: Primero, ¡El mensaje llegará de inmediato!
Segundo, aunque estamos usando un future, todavía no hay concurrencia.
Todo sucede en secuencia, igual que si no hubiera futures
involucrados.

Para abordar esto, enviaremos una serie de mensajes con pausas entre ellos, como se
muestra en el Listado 17-10:

<!-- We cannot test this one because it never stops! -->

<Listing number="17-10" caption="Sending and receiving multiple messages over the async channel and sleeping with an `await` between each message" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-10/src/main.rs:many-messages}}
```

</Listing>

Además de enviar los mensajes, también necesitamos recibirlos. En este caso, podríamos
hacerlo manualmente llamando a `rx.recv().await` cuatro veces, ya que sabemos cuántos
mensajes llegarán. Sin embargo, en la práctica, normalmente estaremos
esperando una cantidad *desconocida* de mensajes, por lo que
necesitamos seguir esperando hasta asegurarnos de que no quedan más.

En el Listado 16-10, usamos un bucle `for` para procesar todos los elementos recibidos de
un canal síncrono. Sin embargo, en Rust  aún no tiene una forma de escribir un bucle `for`
sobre una serie de elementos *asíncronos*. En su lugar, debemos usar un tipo de bucle que
aún no hemos visto: el bucle condicional `while let`. Este `while let` es la versión en bucle
de la construcción  `if let` que vimos en el capítulo 6. El bucle continuará
ejecutándose mientras el patrón que especifica coincida con
el valor recibido.

La llamada `rx.recv` produce un `Future`, que debemos esperar con `await`. El runtime pausará la ejecución
del `Future` hasta que esté listo. Cuando llegue un mensaje, el future se resolverá
en `Some(message)`, tantas veces como lleguen mensajes. Cuando el canal se cierre,
sin importar si llegaron mensajes o no, el future se resolverá en
en `None`, lo que indica que no hay más valores y debemos dejar de esperar
(es decir, dejar de hacer await).

El bucle `while let` combina todo esto. Si el resultado de
`rx.recv().await` es `Some(message)`, obtenemos acceso al mensaje y podemos usarlo dentro
del cuerpo del bucle, igual que con `if let`. Si el resultado es
`None`, el bucle termina. Cada vez que el bucle se completa, vuelve a alcanzar un punto de espera
(await), por lo que el runtime lo pausa nuevamente hasta que llegue otro mensaje.

Con esto, el código ahora envía y recibe todos los mensajes correctamente. Sin embargo,
todavía hay un par de problemas. Por un lado, los mensajes no llegan en intervalos
de medio segundo. En su lugar, todos llegan de golpe, dos segundos (2,000 milisegundos)
después de que el programa inicia. Además, el programa nunca finaliza: en lugar de cerrarse
cuando termina la recepción de mensajes, sigue esperando indefinidamente. Tendrás que cerrarlo manualmente con <span
class="keystroke">ctrl-c</span>.

Comencemos por entender por qué los mensajes llegan todos juntos después del retraso
total en lugar de llegar con pausas entre ellos. Dentro de un bloque async, el orden en el
que aparecen las palabras clave `await` en el código es el
mismo en el que ocurren cuando el programa se ejecuta.

En el Listado 17-10, solo hay un bloque async, por lo que todo se ejecuta de manera
lineal. Todavía no hay concurrencia. Primero se ejecutan todas las llamadas a `tx.send`,
intercaladas con las llamadas a `trpl::sleep`  y sus correspondientes awaits.
Solo después de eso, el bucle `while let`  puede comenzar a procesar los `await`
en las llamadas a `recv`.

Para obtener el comportamiento deseado, donde hay un retraso entre la recepción de cada
mensaje, necesitamos colocar las operaciones de `tx` y `rx` en bloques async separados.
Así, el runtime puede ejecutarlas de forma independiente usando `trpl::join`,
igual que en el ejemplo de conteo. Una vez más, esperamos el resultado de
`trpl::join`,  no los futures individuales. Si esperáramos los futures uno
tras otro, volveríamos a un flujo secuencial —exactamente
lo que queremos evitar.

<!-- We cannot test this one because it never stops! -->

<Listing number="17-11" caption="Separating `send` and `recv` into their own `async` blocks and awaiting the futures for those blocks" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-11/src/main.rs:futures}}
```

</Listing>

Con el código actualizado en el Listado 17-11, los mensajes ahora se imprimen a intervalos
de 500 milisegundos en lugar de llegar todos de golpe después de dos segundos.

Sin embargo, el programa aún no finaliza debido a la forma en que el bucle `while let`
interactúa con `trpl::join`:

* El future devuelto por `trpl::join` solo se completa cuando *ambos* futures que
recibe han terminado.
* El future de `tx` se completa cuando termina el último sleep después de enviar el
  último mensaje en `vals`.
* El future de `rx` no se completará hasta que el bucle `while let` termine.
* El bucle `while let` no terminará hasta que esperar `rx.recv` devuelva `None`.
* `rx.recv().await` solo devolverá `None` cuando el otro extremo del canal
  se cierre.
* El canal solo se cerrará si llamamos a `rx.close` o cuando se elimine (drop)
  el lado del envío `tx`.
* No llamamos a `rx.close`  en ninguna parte, y `tx` no se eliminará hasta que finalice el
  bloque async externo pasado a `trpl::run`.
* El bloque no puede terminar porque está esperando que `trpl::join` se complete,
  lo que nos devuelve al inicio de esta lista.

Podríamos cerrar manualmente `rx` llamando a `rx.close` en algún punto, pero eso no tendría
mucho sentido. Detenernos después de manejar un número arbitrario de mensajes
haría que el programa se cerrara, pero podríamos perder mensajes. Necesitamos otra forma de
asegurarnos de que `tx` se elimine (drop) *antes* del final de la función.

En este momento, el bloque async donde enviamos los mensajes solo toma prestado `tx`
porque enviar un mensaje no requiere propiedad, pero si pudiéramos mover `tx` dentro
de ese bloque async, se eliminaría cuando el bloque terminara. En el capítulo 13,
aprendimos a usar la palabra clave `move` con closures, y en el capítulo 16, vimos
que a menudo necesitamos mover datos dentro de closures cuando trabajamos con hilos. La
misma lógica se aplica a los bloques async, por lo que `move`
funciona con ellos de la misma manera que con las closures.

En el Listado 17-12, cambiamos el bloque async que envía los mensajes de un simple bloque
`async` a un bloque `async move`. Cuando ejecutamos *esta* versión del código, el
programa se cierra correctamente después de que se envían y reciben todos los mensajes.

<Listing number="17-12" caption="A working example of sending and receiving messages between futures which correctly shuts down when complete" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-12/src/main.rs:with-move}}
```

</Listing>

Este canal asíncrono también admite multiple-producer, por lo que podemos llamar a  `clone`
en `tx` si queremos enviar mensajes desde varios futures. En el Listado 17-13,
clonamos `tx`, creando `tx1` fuera del primer bloque async. Luego movemos `tx1` dentro
de ese bloque, tal como hicimos antes con `tx`. Más adelante, movemos el
`tx` original a un *nuevo* bloque async, donde enviamos más mensajes con un pequeño retraso
adicional. Colocamos este nuevo bloque async después del bloque de recepción de mensajes,
pero podría ir antes sin problema. Lo importante no es el orden en que los futures se crean,
sino el orden en que los esperamos (await).

Ambos bloques async para enviar mensajes deben ser `async move`,
de modo que tanto `tx` y `tx1` se eliminen (drop) cuando esos bloques terminen. De lo contrario,
volveríamos al mismo bucle infinito del principio. Finalmente, cambiamos de
`trpl::join` a `trpl::join3` para manejar el future adicional.

<Listing number="17-13" caption="Using multiple producers with async blocks" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-13/src/main.rs:here}}
```

</Listing>

Ahora vemos todos los mensajes de ambos futures de envío. Como cada uno
usa un retraso ligeramente diferente después de enviar, los mensajes
también se reciben en esos intervalos distintos.

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
received 'hi'
received 'more'
received 'from'
received 'the'
received 'messages'
received 'future'
received 'for'
received 'you'
```

Este es un buen comienzo, pero nos limita a solo unos pocos futures: dos con `join`
o tres con `join3` . Veamos cómo podemos manejar una cantidad mayor de futures.

[streams]: ch17-05-streams.html
