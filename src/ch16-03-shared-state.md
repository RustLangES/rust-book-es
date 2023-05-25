## Concurrencia de estado compartido

El paso de mensajes es una buena manera de manejar la concurrencia, pero no es
la única. Otro método sería que varios hilos accedan a los mismos datos
compartidos. Considere esta parte del eslogan de la documentación del lenguaje
Go nuevamente: "no se comunique compartiendo memoria".

¿Qué significaría comunicarse compartiendo memoria? Además, ¿por qué los
entusiastas del paso de mensajes advierten que no se debe usar el intercambio de
memoria?

En cierto modo, los canales en cualquier lenguaje de programación son similares
al ownership único, porque una vez que transfieres un valor por un canal, ya
no debes usar ese valor. La concurrencia de memoria compartida es como el
ownership múltiple: varios hilos pueden acceder a la misma ubicación de memoria
al mismo tiempo. Como viste en el Capítulo 15, donde los punteros inteligentes
hicieron posible el ownership múltiple, el ownership múltiple puede agregar
complejidad porque estos propietarios diferentes necesitan administración. El
sistema de tipos y las reglas de ownership de Rust ayudan mucho a obtener esta
administración correcta. Para un ejemplo, veamos los mutex, uno de los
primitivos de concurrencia más comunes para la memoria compartida.

### Usando Mutexes para permitir el acceso a los datos de un hilo a la vez

*Mutex* es una abreviatura de *exclusión mutua*, como en, un mutex permite que
solo un hilo acceda a algunos datos en un momento dado. Para acceder a los
datos en un mutex, un hilo primero debe señalar que desea acceso solicitando
adquirir el *lock* del mutex. El lock es una estructura de datos que forma
parte del mutex que realiza un seguimiento de quién tiene actualmente acceso
exclusivo a los datos. Por lo tanto, el mutex se describe como *guardando* los
datos que contiene a través del sistema de bloqueo.

Los Mutexes tienen la reputación de ser difíciles de usar porque debes
recordar dos reglas:

* Debes intentar adquirir el bloqueo antes de utilizar los datos.
* Cuando hayas terminado con los datos que protege el mutex, debes desbloquear
  los datos para que otros hilos puedan adquirir el bloqueo.

Para una metáfora del mundo real para un mutex, imagina un panel de discusión
en una conferencia con un solo micrófono. Antes de que un panelista pueda
hablar, debe preguntar o señalar que desea usar el micrófono. Cuando obtienen
el micrófono, pueden hablar todo el tiempo que quieran y luego entregar el
micrófono al siguiente panelista que solicite hablar. Si un panelista olvida
entregar el micrófono cuando haya terminado con él, nadie más puede hablar. Si
la administración del micrófono compartido sale mal, ¡el panel no funcionará
como estaba previsto!

La gestión de mutexes puede ser increíblemente difícil de hacer bien, razón por
la cual tanta gente está entusiasmada con los canales. Sin embargo, gracias al
sistema de tipos y las reglas de ownership de Rust, no puedes bloquear y
desbloquear incorrectamente.

#### La API de `Mutex<T>`

Como un ejemplo de como usar un mutex, comencemos usando un mutex en un
contexto de un solo hilo, como se muestra en el Listado 16-12:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-12/src/main.rs}}
```

<span class="caption">Listing 16-12: Explorando la API de `Mutex<T>` en un
contexto de un solo hilo para simplificar</span>

Como con muchos tipos, creamos un `Mutex<T>` usando la función asociada `new`.
Para acceder a los datos dentro del mutex, usamos el método `lock` para
adquirir el bloqueo. Esta llamada bloqueará el hilo actual para que no pueda
hacer ningún trabajo hasta que sea nuestro turno de tener el bloqueo.

La llamada a `lock` fallaría si otro hilo que tiene el bloqueo se bloquea. En
ese caso, nadie nunca podría obtener el bloqueo, por lo que hemos elegido
`unwrap` y hacer que este hilo se bloquee si estamos en esa situación.

Después de que hayamos adquirido el bloqueo, podemos tratar el valor de
retorno llamado `num` en este caso, como una referencia mutable a los datos
internos. El sistema de tipos garantiza que adquirimos un bloqueo antes de
usar el valor en `m`. El tipo de `m` es `Mutex<i32>`, no `i32`, por lo que
*debemos* llamar a `lock` para poder usar el valor `i32` interno. No podemos
olvidar; el sistema de tipos no nos permitirá acceder al `i32` interno de
otra manera.

Como puedes sospechar, `Mutex<T>` es un smart pointer. Más precisamente, la
llamada a `lock` *devuelve* un smart pointer llamado `MutexGuard`, envuelto en
un `LockResult` que manejamos con la llamada a `unwrap`. El smart pointer
`MutexGuard` implementa `Deref` para apuntar a nuestros datos internos; el
smart pointer también tiene una implementación de `Drop` que libera el bloqueo
automáticamente cuando un `MutexGuard` sale del scope, lo que sucede al final
del scope interno. Como resultado, no corremos el riesgo de olvidar liberar
el bloqueo y bloquear el mutex para que otros hilos no puedan usarlo, porque
la liberación del bloqueo ocurre automáticamente.

Después de eliminar el bloqueo, podemos imprimir el valor mutex y ver que
pudimos cambiar el valor interno `i32` a 6.

#### Compartir un `Mutex<T>` entre varios hilos

Ahora, intentemos compartir un valor entre múltiples hilos usando `Mutex<T>`.
Activaremos 10 hilos y haremos que cada uno incremente un valor de contador en
1, por lo que el contador va de 0 a 10. El siguiente ejemplo en el Listado
16-13 tendrá un error del compilador, y usaremos ese error para aprender más
sobre el uso de `Mutex<T>` y cómo Rust nos ayuda a usarlo correctamente.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-13/src/main.rs}}
```

<span class="caption">Listing 16-13: Diez hilos cada uno incrementa un contador
custodiado por un `Mutex<T>`</span>

Creamos una variable `counter` para contener un `i32` dentro de un `Mutex<T>`,
como hicimos en el Listado 16-12. A continuación, creamos 10 hilos iterando
sobre un rango de números. Usamos `thread::spawn` y damos a todos los hilos el
mismo closure: uno que mueve el contador al hilo, adquiere un bloqueo en el
`Mutex<T>` llamando al método `lock`, y luego agrega 1 al valor en el mutex.
Cuando un hilo termina de ejecutar su closure, `num` saldrá del scope y
liberará el bloqueo para que otro hilo pueda adquirirlo.

En el hilo principal, recopilamos todos los identificadores de unión. Luego,
como hicimos en el Listado 16-2, llamamos a `join` en cada identificador para
asegurarnos de que todos los hilos terminen. En ese momento, el hilo principal
adquirirá el bloqueo e imprimirá el resultado de este programa.

Sugerimos que este ejemplo no se compilaría ¡Ahora descubramos por qué!

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-13/output.txt}}
```

El mensaje de error indica que el valor de `counter` se movió en la anterior
iteración del bucle. El compilador nos está diciendo que no podemos mover la
propiedad de `counter` a múltiples hilos. Arreglemos el error del compilador
con un método de múltiples propietarios que discutimos en el Capítulo 15.

#### Ownership Multiple con múltiples hilos

En el capítulo 15, le dimos a un valor múltiples dueños al usar el
smart pointer `Rc<T>` para crear un valor de recuento de referencia.
Hagamos lo mismo aquí y veamos qué sucede. Envolveremos el `Mutex<T>` en
`Rc<T>` en el Listado 16-14 y clonaremos el `Rc<T>` antes de mover el
ownership al hilo.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-14/src/main.rs}}
```

<span class="caption">Listing 16-14: Intentando usar `Rc<T>` para permitir
múltiples hilos para poseer `Mutex<T>`</span>

Una vez más, compilamos y obtenemos... ¡diferentes errores! El compilador nos
está enseñando mucho.

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-14/output.txt}}
```

Wow, ¡ese mensaje de error es muy extenso! Aquí está la parte importante en la
que debemos enfocarnos: `` `Rc<Mutex<i32>>` cannot be sent between threads
safely ``. El compilador también nos está diciendo la razón por la que:
``the trait `Send` is not implemented for `Rc<Mutex<i32>>` ``. Hablaremos de
`Send` en la siguiente sección: es uno de los traits que asegura que los tipos
que usamos con hilos están destinados a su uso en situaciones concurrentes.

Desafortunadamente, `Rc<T>` no es seguro para compartir entre hilos. Cuando
`Rc<T>` administra el recuento de referencia, agrega al recuento para cada
llamada a `clone` y resta del recuento cuando se descarta cada clon. Pero no
usa ningún primitivo de concurrencia para asegurarse de que los cambios en el
recuento no puedan ser interrumpidos por otro hilo. Esto podría conducir a
recuentos incorrectos: errores sutiles que podrían a su vez conducir a fugas
de memoria o que un valor se descarte antes de que hayamos terminado con él.
Lo que necesitamos es un tipo exactamente como `Rc<T>` pero que haga cambios en
el recuento de referencia de una manera segura para hilos.

#### Recuento de referencia atómico con `Arc<T>`

Afortunadamente, `Arc<T>` *es* un tipo como `Rc<T>` que es seguro de usar en
situaciones concurrentes. La *a* significa *atómico*, lo que significa que es
un tipo de recuento de referencia atómico. Los átomos son un tipo adicional de
primitiva de concurrencia que no cubriremos en detalle aquí: consulte la
documentación de la biblioteca estándar para [`std::sync::atomic`][atomic]
<!-- ignore --> para más detalles. En este punto, solo necesita saber que los
Átomos funcionan como tipos primitivos, pero son seguros para compartir entre
hilos.

Entonces podrías preguntarte por qué todos los tipos primitivos no son atómicos
y por qué los tipos de biblioteca estándar no se implementan para usar `Arc<T>`
de forma predeterminada. La razón es que la seguridad de los hilos conlleva una
penalización de rendimiento que solo desea pagar cuando realmente lo necesita.
Si solo está realizando operaciones en valores dentro de un solo hilo, su
código puede ejecutarse más rápido si no tiene que hacer cumplir las garantías
que proporcionan los átomos.

Volvamos a nuestro ejemplo: `Arc<T>` y `Rc<T>` tienen la misma API, por lo que
arreglamos nuestro programa cambiando la línea `use`, la llamada a `new` y la
llamada a `clone`. El código en el Listado 16-15 finalmente se compilará y
ejecutará:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-15/src/main.rs}}
```

<span class="caption">Listing 16-15: Usando un `Arc<T>` para envolver `Mutex<T>`
para poder compartir el ownership a través de múltiples hilos</span>

Este código imprimirá lo siguiente:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Result: 10
```

¡Lo hicimos! Contamos de 0 a 10, lo que puede no parecer muy impresionante,
pero nos enseñó mucho sobre `Mutex<T>` y la seguridad de los hilos. También
podría usar la estructura de este programa para realizar operaciones más
complicadas que simplemente incrementar un contador. Usando esta estrategia,
puede dividir un cálculo en partes independientes, dividir esas partes en
hilos y luego usar un `Mutex<T>` para que cada hilo actualice el resultado
final con su parte.

Nota que si estás haciendo operaciones numéricas simples, hay tipos más
simples que los tipos `Mutex<T>` proporcionados por el [`std::sync::atomic`
módulo de la biblioteca estándar][atomic]<!-- ignore -->. Estos tipos
proporcionan acceso seguro y concurrente a tipos primitivos. Elegimos usar
`Mutex<T>` con un tipo primitivo para este ejemplo para que pudiéramos
concentrarnos en cómo funciona `Mutex<T>`.

### Similitudes entre `RefCell<T>`/`Rc<T>` y `Mutex<T>`/`Arc<T>`

Es posible que hayas notado que `counter` es inmutable, pero podríamos obtener
una referencia mutable al valor dentro de él; esto significa que `Mutex<T>`
proporciona mutabilidad interior, como lo hace la familia `Cell`. De la misma
manera que usamos `RefCell<T>` en el Capítulo 15 para permitirnos mutar
contenidos dentro de un `Rc<T>`, usamos `Mutex<T>` para mutar contenidos dentro
de un `Arc<T>`.

Un detalle a tener en cuenta es que Rust no puede protegerte de todos los 
errores lógicos al usar `Mutex<T>`. Recuerda en el Capítulo 15 que usar `Rc<T>`
venía con el riesgo de crear ciclos de referencia, donde dos valores `Rc<T>`
se refieren entre sí, causando fugas de memoria. De manera similar, `Mutex<T>`
viene con el riesgo de crear *deadlocks*. Estos ocurren cuando una operación
necesita bloquear dos recursos y dos hilos han adquirido cada uno de los
bloqueos, lo que los hace esperar el uno al otro para siempre. Si está
interesado en los deadlocks, intente crear un programa Rust que tenga un
deadlock; luego investigue las estrategias de mitigación de deadlock para
mutexes en cualquier lenguaje y pruebe implementarlas en Rust. La documentación 
de la API de la biblioteca estándar para `Mutex<T>` y `MutexGuard` ofrece
información útil.

Terminaremos este capítulo hablando sobre los traits `Send` y `Sync` y cómo
podemos usarlos con tipos personalizados.

[atomic]: ../std/sync/atomic/index.html
