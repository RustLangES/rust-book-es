## Usando Threads para Ejecutar Código Simultáneamente

En la mayoría de los sistemas operativos actuales, el código de un programa
ejecutado se ejecuta en un _proceso_, y el sistema operativo administrará
múltiples procesos a la vez. Dentro de un programa, también puede tener partes
independientes que se ejecutan simultáneamente. Las características que ejecutan
estas partes independientes se llaman _threads_. Por ejemplo, un servidor web
podría tener múltiples hilos para que pudiera responder a más de una solicitud
al mismo tiempo.

Dividir la computación en su programa en múltiples hilos para ejecutar múltiples
tareas al mismo tiempo puede mejorar el rendimiento, pero también agrega
complejidad. Debido a que los hilos pueden ejecutarse simultáneamente, no hay
ninguna garantía inherente sobre el orden en que las partes de su código en
diferentes hilos se ejecutarán. Esto puede conducir a problemas, como:

- Race conditions, donde los hilos están accediendo a datos o recursos en
  un orden inconsistente
- Deadlocks, donde dos hilos están esperando el uno al otro, evitando que ambos
  hilos continúen
- Bugs que ocurren solo en ciertas situaciones y son difíciles de reproducir
  y arreglar de manera confiable

Rust intenta mitigar los efectos negativos de usar hilos, pero la programación
en un contexto multihilo aún requiere un pensamiento cuidadoso y requiere una
estructura de código que sea diferente de la de los programas que se ejecutan en
un solo hilo.

Los lenguajes de programación implementan hilos de varias maneras diferentes, y
muchos sistemas operativos proporcionan una API que el lenguaje puede llamar
para crear nuevos hilos. La biblioteca estándar de Rust utiliza un modelo _1:1_
de implementación de hilos, mediante el cual un programa utiliza un hilo del
sistema operativo por un hilo de lenguaje. Hay crates que implementan otros
modelos de enhebrado que hacen diferentes compensaciones al modelo 1:1. 
(El sistema async de Rust, que veremos en el próximo capítulo, también 
proporciona otro enfoque para la concurrencia).

### Creando un Nuevo Hilo con `spawn`

Para crear un nuevo hilo, llamamos a la función `thread::spawn` y pasamos un
closure (hablamos sobre closures en el Capítulo 13) que contiene el código que
queremos ejecutar en el nuevo hilo. El ejemplo en el Listado 16-1 imprime
algunos textos desde un hilo principal y otros textos desde un nuevo hilo:

<Listing number="16-1" file-name="src/main.rs" caption="Creando un nuevo hilo para imprimir una cosa mientras el hilo principal imprime algo más">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-01/src/main.rs}}
```

</Listing>

Nota que cuando el hilo principal de un programa Rust se completa, todos los
hilos creados se apagan, independientemente de si han terminado de ejecutarse o
no. La salida de este programa podría ser un poco diferente cada vez, pero se
verá similar a lo siguiente:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
```

Las llamadas a `thread::sleep` fuerzan a un hilo a detener su ejecución durante
una corta duración, permitiendo que se ejecute un hilo diferente. Los hilos
probablemente se turnarán, pero eso no está garantizado: depende de cómo su
sistema operativo programe los hilos. En esta ejecución, el hilo principal
imprimió primero, a pesar de que la instrucción de impresión del hilo creado
aparece primero en el código. Y aunque le dijimos al hilo creado que imprimiera
hasta que `i` sea `9`, solo llegó a `5` antes de que el hilo principal se 
apagara.

Si ejecutas este código y solo ves el output del hilo principal, o no ves
ninguna superposición, intenta aumentar los números en los rangos para crear
más oportunidades para que el sistema operativo cambie entre los hilos.

### Esperando a que todos los hilos terminen usando `join` Handles

El código en el Listado 16-1 no solo detiene el hilo creado prematuramente la
mayoría de las veces debido a que el hilo principal termina, sino que debido a
que no hay garantía sobre el orden en que se ejecutan los hilos, ¡tampoco
podemos garantizar que el hilo creado se ejecute en absoluto!

Podemos solucionar el problema de que el hilo creado no se ejecute o termine
prematuramente guardando el valor de retorno de `thread::spawn` en una variable.
El tipo de retorno de `thread::spawn` es `JoinHandle<T>`. Un `JoinHandle<T>` es 
un valor de propiedad que, cuando llamamos al método `join` en él, esperará a 
que su hilo termine. El Listado 16-2 muestra cómo usar el `JoinHandle<T>` del 
hilo que creamos en el Listado 16-1 y llamar a `join` para asegurarnos de que el 
hilo creado termine antes de que `main` salga:

<Listing number="16-2" file-name="src/main.rs" caption="Guardando un `JoinHandle<T>` devuelto por `thread::spawn` para garantizar que el hilo se ejecute hasta completarse">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-02/src/main.rs}}
```

</Listing>

Llamar a `join` en el handle bloquea el hilo que está actualmente en ejecución
hasta que el hilo representado por el handle termine. Bloquear un hilo significa
que ese hilo se impide realizar un trabajo o salir. Debido a que hemos puesto la
llamada a `join` después del bucle `for` del hilo principal, ejecutar el Listado
16-2 debería producir una salida similar a esta:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 1 from the spawned thread!
hi number 3 from the main thread!
hi number 2 from the spawned thread!
hi number 4 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
```

Los dos hilos continúan alternándose, pero el hilo principal espera debido a la
llamada a `handle.join()` y no termina hasta que el hilo creado haya terminado.

Pero veamos que sucede cuando movemos la llamada a `handle.join()` antes del
bucle `for` en `main`, como esto:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/no-listing-01-join-too-early/src/main.rs}}
```

</Listing>

El hilo principal ahora espera a que el hilo creado termine antes de comenzar su
bucle `for`, para que el output no se intercale más. La salida ahora se verá
así:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 3 from the main thread!
hi number 4 from the main thread!
```

Pequeños detalles, como dónde se llama a `join`, pueden afectar si sus hilos se
ejecutan al mismo tiempo.

### Usando `move` Closures con Threads

A menudo usamos la keyword `move` con closures pasadas a `thread::spawn` porque
el closure tomará posesión de los valores que usa del entorno, transfiriendo así
el ownership de esos valores de un hilo a otro. En la sección ["Capturando
el Entorno con Closures"][capture]<!-- ignore --> del Capítulo 13,
discutimos `move` en el contexto de las closures. Ahora, nos concentraremos más
en la interacción entre `move` y `thread::spawn`.

Observa en el Listado 16-1 que el closure que pasamos a `thread::spawn` no tiene
argumentos: no estamos usando ningún dato del hilo principal en el código del
hilo creado. Para usar datos del hilo principal en el hilo creado, el closure
del hilo creado debe capturar los valores que necesita. El Listado 16-3 muestra
un intento de crear un vector en el hilo principal y usarlo en el hilo creado.
Sin embargo, esto aún no funcionará aún, como verás en un momento.

<Listing number="16-3" file-name="src/main.rs" caption="Intentando usar un vector creado por el hilo principal en otro hilo">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-03/src/main.rs}}
```

</Listing>

El closure usa `v`, por lo que capturará `v` y lo hará parte del entorno del
closure. Debido a que `thread::spawn` ejecuta este closure en un nuevo hilo,
deberíamos poder acceder a `v` dentro de ese nuevo hilo. Pero cuando compilamos
este ejemplo, obtenemos el siguiente error:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-03/output.txt}}
```

Rust _infiere_ cómo capturar `v`, y porque `println!` solo necesita una
referencia a `v`, el closure intenta pedir prestado `v`. Sin embargo, hay un
problema: Rust no puede decir cuánto tiempo se ejecutará el hilo creado, por lo
que no sabe si la referencia a `v` siempre será válida.

El Listado 16-4 proporciona un escenario que es más probable que tenga una
referencia a `v` que no sea válida:

<Listing number="16-4" file-name="src/main.rs" caption="Un hilo con un closure que intenta capturar una referencia a `v` desde un hilo principal que deja de tener `v`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-04/src/main.rs}}
```

</Listing>

Si Rust nos permitiera ejecutar este código, existe la posibilidad de que el
hilo creado se ponga inmediatamente en segundo plano sin ejecutarse en absoluto.
El hilo creado tiene una referencia a `v` dentro, pero el hilo principal
inmediatamente deja caer `v`, usando la función `drop` que discutimos en el
Capítulo 15. Luego, cuando el hilo creado comienza a ejecutarse, `v` ya no es
válido, por lo que una referencia a él también es inválida. ¡Oh no!

Para solucionar el error en el Listado 16-3, podemos seguir el consejo del mensaje
de error:

<!-- manual-regeneration
after automatic regeneration, look at listings/ch16-fearless-concurrency/listing-16-03/output.txt and copy the relevant part
-->

```text
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ++++
```

Al agregar la keyword `move` antes del closure, forzamos al closure a tomar
ownership de los valores que está usando en lugar de permitir que Rust infiera
que debería pedir prestado los valores. La modificación al Listado 16-3 que se
muestra en el Listado 16-5 se compilará y ejecutará como lo pretendemos.

<Listing number="16-5" file-name="src/main.rs" caption="Usando la keyword `move` para forzar a un closure a tomar ownership de los valores que utiliza">


```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-05/src/main.rs}}
```

</Listing>

Podríamos sentir la tentación de intentar lo mismo para arreglar el código en el
Listado 16-4 donde el hilo principal llamó a `drop` usando un closure `move`.
Sin embargo, esta solución no funcionará porque lo que el Listado 16-4 está
intentando hacer está prohibido por una razón diferente. Si agregáramos `move`
al closure, moveríamos `v` al entorno del closure, y ya no podríamos llamar a
`drop` en el hilo principal. En su lugar, obtendríamos este error del
compilador:

```console
{{#include ../listings/ch16-fearless-concurrency/output-only-01-move-drop/output.txt}}
```

Las reglas de ownership de Rust nos han salvado de nuevo! Obtenemos un error del
código en el Listado 16-3 porque Rust es conservador y solo pide prestado `v`
para el hilo, lo que significa que el hilo principal podría teóricamente
invalidar la referencia del hilo creado. Al decirle a Rust que mueva la
propiedad de `v` al hilo creado, le garantizamos a Rust que el hilo principal no
usará `v` nunca más. Si cambiamos el Listado 16-4 de la misma manera, entonces
estamos violando las reglas de ownership cuando intentamos usar `v` en el hilo
principal. La keyword `move` anula la conservadora predeterminada de Rust de
pedir prestado; no nos permite violar las reglas de ownership.

Con una comprensión básica de los hilos y la API de hilos, veamos qué podemos
_hacer_ con los hilos.

[capture]: ch13-01-closures.html#capturing-the-environment-with-closures
