## Convirtiendo nuestro servidor de un solo hilo en un servidor multihilo

Actualmente, el servidor procesará cada solicitud de forma secuencial, lo que
significa que no procesará una segunda conexión hasta que se termine de
procesar la primera. Si el servidor recibe más y más solicitudes, esta
ejecución en serie será menos y menos óptima. Si el servidor recibe una
solicitud que tarda mucho tiempo en procesarse, las solicitudes posteriores
tendrán que esperar hasta que la solicitud larga haya terminado, incluso si las
nuevas solicitudes se pueden procesar rápidamente. Tendremos que solucionar
esto, pero primero, veremos el problema en acción.

### Simulando una solicitud lenta en la implementación actual del servidor

Para simular una solicitud lenta, podemos hacer que el servidor duerma durante
un tiempo antes de responder. Veremos cómo una solicitud de procesamiento
lento puede afectar a otras solicitudes realizadas a nuestra implementación
actual del servidor. El listado 21-10 implementa el manejo de una solicitud a
_/sleep_ con una respuesta lenta simulada que hará que el servidor duerma
durante 5 segundos antes de responder.

<Listing number="21-10" file-name="src/main.rs" caption="Simulando una solicitud lenta durmiendo durante 5 segundos">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-10/src/main.rs:here}}
```

</Listing>

Hemos cambiado de `if` a `match` ahora que tenemos tres casos. Necesitamos
hacer coincidir explícitamente con un slice de `request_line` para hacer
coincidir con los valores literales de string; `match` no hace referencia
automática y desreferenciación como el método de igualdad.

La primera opción es la misma que el bloque `if` del Listado 21-9. La segunda
opción coincide con una solicitud a _/sleep_. Cuando se recibe esa solicitud, el
servidor dormirá durante 5 segundos antes de representar la página HTML
correcta. La tercera opción es la misma que el bloque `else` del Listado 21-9.

Puedes ver cómo nuestro servidor es primitivo: ¡las bibliotecas reales
manejarían el reconocimiento de múltiples solicitudes de una manera mucho menos
verbosa!

Iniciamos el servidor con `cargo run`. Luego abrimos dos ventanas del navegador:
una para *http://127.0.0.1:7878/* y la otra para *http://127.0.0.1:7878/sleep*.
Si ingresas la URI _/_ varias veces, como antes, verás que responde rápidamente.
Pero si ingresas _/sleep_ y luego cargas _/_, verás que _/_ espera hasta que
`sleep` haya dormido durante sus 5 segundos completos antes de cargarse.

Existen varias técnicas que podríamos usar para evitar que las solicitudes se
acumulen detrás de una solicitud lenta, incluyendo usando async como vimos en el
Capitulo 17; la que implementaremos es un _pool de hilos_.

### Mejorando el rendimiento con un pool de hilos

Un _pool de hilos_ es un grupo de hilos generados que están esperando y listos
para manejar una tarea. Cuando el programa recibe una nueva tarea, asigna uno
de los hilos del grupo a la tarea, y ese hilo procesará la tarea. Los hilos
restantes en el grupo están disponibles para manejar cualquier otra tarea que
llegue mientras el primer hilo está procesando. Cuando el primer hilo termina
de procesar su tarea, se devuelve al grupo de hilos inactivos, listo para
manejar una nueva tarea. Un pool de hilos le permite procesar conexiones de
forma concurrente, aumentando el rendimiento de su servidor.

Limitaremos el número de hilos en el grupo a un número pequeño para protegernos
de los ataques de denegación de servicio (DoS); si nuestro programa creara un
nuevo hilo para cada solicitud que llegara, alguien que hiciera 10 millones de
solicitudes a nuestro servidor podría crear el caos al agotar todos los
recursos de nuestro servidor y detener el procesamiento de las solicitudes.

En lugar de crear un nuevo hilo para cada solicitud, crearemos un grupo de
hilos que actuarán como un pool de hilos. Cuando llega una solicitud, el
servidor enviará la solicitud al pool de hilos. El pool de hilos mantendrá una
cola de solicitudes entrantes. Cada uno de los hilos en el pool sacará una
solicitud de esta cola, manejará la solicitud y luego pedirá a la cola otra
solicitud. Con este diseño, podemos procesar hasta `N` solicitudes
simultáneamente, donde `N` es el número de hilos. Si cada hilo responde a una
solicitud de larga duración, las solicitudes posteriores aún pueden acumularse
en la cola, pero hemos aumentado el número de solicitudes de larga duración que
podemos manejar antes de llegar a ese punto.

Esta técnica es solo una de las muchas formas de mejorar el rendimiento de un
servidor web. Otras opciones que puede explorar son el modelo _fork / join_,
el modelo de _I / O asincrónico de un solo hilo_ o el _modelo de I / O
asincrónico de múltiples hilos_. Si está interesado en este tema, puedes leer
más sobre otras soluciones e intentar implementarlas; con un lenguaje de bajo
nivel como Rust, todas estas opciones son posibles.

Antes de comenzar a implementar un pool de hilos, hablemos sobre cómo debería
verse el uso del pool. Cuando intentas diseñar código, escribir la interfaz del
cliente primero puede ayudar a guiar tu diseño. Escribe la API del código para
que esté estructurado de la manera en que deseas llamarlo; luego implementa la
funcionalidad dentro de esa estructura en lugar de implementar la funcionalidad
y luego diseñar la API pública.

Similar a cómo usamos el desarrollo impulsado por pruebas en el proyecto en el
Capítulo 12, usaremos el desarrollo impulsado por el compilador aquí.
Escribiremos el código que llama a las funciones que queremos, y luego
analizaremos los errores del compilador para determinar qué debemos cambiar a
continuación para que el código funcione. Antes de hacer eso, sin embargo,
exploraremos la técnica que no vamos a usar como punto de partida.

<!-- Old headings. Do not remove or links may break. -->


<a id="code-structure-if-we-could-spawn-a-thread-for-each-request"></a>

#### Creando un hilo para cada solicitud

Primero, exploremos cómo podría lucir nuestro código si creáramos un nuevo hilo
para cada conexión. Como se mencionó anteriormente, este no es nuestro plan
final debido a los problemas con la posibilidad de generar un número ilimitado
de hilos, pero es un punto de partida para obtener un servidor web
multihilo. Luego agregaremos el pool de hilos como una mejora, y contrastar las
dos soluciones será más fácil. El Listado 21-11 muestra los cambios que debe
realizar en `main` para crear un nuevo hilo para manejar cada flujo dentro del
bucle `for`.

<Listing number="21-11" file-name="src/main.rs" caption="Creando un hilo para cada stream">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-11/src/main.rs:here}}
```

</Listing>

Como aprendiste en el Capítulo 16, `thread::spawn` creará un nuevo hilo y luego
ejecutará el código en el cierre en el nuevo hilo. Si ejecutas este código y
cargas _/sleep_ en tu navegador, luego _/_ en otras dos pestañas del navegador,
verás que las solicitudes a _/_ no tienen que esperar a que _/sleep_ termine.
Sin embargo, como mencionamos, esto eventualmente abrumará el sistema porque
estarías creando nuevos hilos sin ningún límite.

También recordarás del Capítulo 17 que este es precisamente el tipo de situación 
donde `async` y `await` realmente brillan. Ten esto en mente mientras 
construimos el *thread pool* y reflexionamos sobre cómo se verían las cosas de 
manera diferente o similar usando `async`.

<!-- Old headings. Do not remove or links may break. -->


<a id="creating-a-similar-interface-for-a-finite-number-of-threads"></a>

#### Creando un número finito de hilos

Queremos que nuestro pool de hilos funcione de manera similar y familiar, de
modo que cambiar de hilos a un pool de hilos no requiera grandes cambios en el
código que usa nuestra API. El Listado 21-12 muestra la interfaz hipotética
para un struct `ThreadPool` que queremos usar en lugar de `thread::spawn`.

<Listing number="21-12" file-name="src/main.rs" caption="Nuestra interfaz ideal de `ThreadPool`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-12/src/main.rs:here}}
```

</Listing>

Utilizamos `ThreadPool::new` para crear un nuevo pool de hilos con un número
configurable de hilos, en este caso cuatro. Luego, en el bucle `for`,
`pool.execute` tiene una interfaz similar a `thread::spawn` en que toma un
cierre que el pool debe ejecutar para cada flujo. Necesitamos implementar
`pool.execute` para que tome el cierre y se lo dé a un hilo en el pool para que
lo ejecute. Este código aún no se compilará, pero lo intentaremos para que el
compilador pueda guiarnos en cómo solucionarlo.

<!-- Old headings. Do not remove or links may break. -->


<a id="building-the-threadpool-struct-using-compiler-driven-development"></a>

#### Construyendo `ThreadPool` usando el desarrollo impulsado por el compilador

Realiza los cambios en el Listado 21-12 a _src/main.rs_, y luego usemos los
errores del compilador de `cargo check` para impulsar nuestro desarrollo. Aquí
está el primer error que obtenemos:

```console
{{#include ../listings/ch21-web-server/listing-21-12/output.txt}}
```

¡Eso es genial! Este error nos dice que necesitamos un tipo o módulo
`ThreadPool`, así que lo construiremos ahora. Nuestra implementación de
`ThreadPool` será independiente del tipo de trabajo que nuestro servidor web
está haciendo. Entonces, cambiemos el crate de `hello` de un crate binario a un
crate de biblioteca para contener nuestra implementación de `ThreadPool`.
Después de cambiar a un crate de biblioteca, también podríamos usar la
biblioteca de pool de hilos separada para cualquier trabajo que queramos hacer
usando un pool de hilos y no solo para servir solicitudes web.

Crea un _src/lib.rs_ que contenga lo siguiente, que es la definición más simple
de un struct `ThreadPool` que podemos tener por ahora:

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-01-define-threadpool-struct/src/lib.rs}}
```

</Listing>

Luego edita el archivo _main.rs_ para traer `ThreadPool` al scope del crate
desde el crate de la biblioteca agregando el siguiente código en la parte
superior de _src/main.rs_:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/no-listing-01-define-threadpool-struct/src/main.rs:here}}
```

</Listing>

Este código aún no funcionará, pero verifiquémoslo nuevamente para obtener el
siguiente error que debemos abordar:

```console
{{#include ../listings/ch21-web-server/no-listing-01-define-threadpool-struct/output.txt}}
```

Este error indica que a continuación debemos crear una función asociada
llamada `new` para `ThreadPool`. También sabemos que `new` debe tener un
parámetro que pueda aceptar `4` como argumento y debe devolver una instancia de
`ThreadPool`. Implementemos la función `new` más simple que tendrá esas
características:

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-02-impl-threadpool-new/src/lib.rs}}
```

</Listing>

Elegimos `usize` como el tipo del parámetro `size`, porque sabemos que un número
negativo de hilos no tiene sentido. También sabemos que usaremos este `4` como
el número de elementos en una colección de hilos, que es para lo que se usa el
tipo `usize`, como se discutió en la sección [“Tipos de enteros”][integer-types]
del Capítulo 3.

Let’s check the code again:

```console
{{#include ../listings/ch21-web-server/no-listing-02-impl-threadpool-new/output.txt}}
```

Ahora ocurre un error porque no tenemos un método `execute` en `ThreadPool`.
Recordemos de la sección [“Creando un número finito de
hilos”](#creating-a-similar-interface-for-a-finite-number-of-threads)<!--
ignore --> que decidimos que nuestro pool de hilos debería tener una interfaz
similar a `thread::spawn`. Además, implementaremos la función `execute` para
que tome el cierre que se le da y se lo dé a un hilo inactivo en el pool para
que lo ejecute.

Definiremos el método `execute` en `ThreadPool` para tomar un closure como
parámetro. Recordemos de la sección [“Mover valores capturados fuera del
closure y los traits `Fn`”][fn-traits]<!-- ignore --> en el Capítulo 13 que
podemos tomar cierres como parámetros con tres traits diferentes: `Fn`,
`FnMut` y `FnOnce`. Necesitamos decidir qué tipo de cierre usar aquí. Sabemos
que terminaremos haciendo algo similar a la implementación de la biblioteca
estándar `thread::spawn`, por lo que podemos ver qué límites tiene la firma de
`thread::spawn` en su parámetro. La documentación nos muestra lo siguiente:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

El parámetro de tipo `F` es el que nos preocupa aquí; el parámetro de tipo `T`
está relacionado con el valor de retorno, y no nos preocupa eso. Podemos ver
que `spawn` usa `FnOnce` como límite de trait en `F`. Esto es probablemente lo
que queremos también, porque eventualmente pasaremos el argumento que obtenemos
en `execute` a `spawn`. Podemos estar más seguros de que `FnOnce` es el trait
que queremos usar porque el hilo para ejecutar una solicitud solo ejecutará el
closure de esa solicitud una vez, lo que coincide con el `Once` en `FnOnce`.

El trait `FnOnce` también tiene un trait bound `Send` y un lifetime bound
`'static`, que son útiles en nuestra situación: necesitamos `Send` para
transferir el closure de un hilo a otro y `'static` porque no sabemos cuánto
tiempo tomará el hilo para ejecutarse. Creemos un método `execute` en
`ThreadPool` que tomará un parámetro genérico de tipo `F` con estos bounds:

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-03-define-execute/src/lib.rs:here}}
```

</Listing>

Aún usamos `()` después de `FnOnce` porque este `FnOnce` representa un closure
que no toma parámetros y devuelve el tipo de unidad `()`. Al igual que las
definiciones de funciones, el tipo de retorno se puede omitir de la firma, pero
incluso si no tenemos parámetros, todavía necesitamos los paréntesis.

Una vez más, esta es la implementación más simple del método `execute`: no hace
nada, pero estamos tratando de que nuestro código compile. Verifiquemos
nuevamente:

```console
{{#include ../listings/ch21-web-server/no-listing-03-define-execute/output.txt}}
```

¡Compila! Pero ten en cuenta que si intentas `cargo run` y haces una solicitud
en el navegador, verás los errores en el navegador que vimos al comienzo del
capítulo. ¡Nuestra biblioteca aún no está llamando al closure pasado a
`execute`!

> Nota: Una frase que podrías escuchar sobre lenguajes con compiladores
> estrictos, como Haskell y Rust, es “si el código se compila, funciona”. Pero
> esta frase no es universalmente cierta. Nuestro proyecto se compila, ¡pero no
> hace absolutamente nada! Si estuviéramos construyendo un proyecto real y
> completo, este sería un buen momento para comenzar a escribir pruebas
> unitarias para verificar que el código se compile _y_ tenga el comportamiento
> que queremos.

Considera: ¿qué sería diferente aquí si fuéramos a ejecutar un *future* en lugar 
de un *closure*?

#### Validando el número de hilos en `new`

No estamos haciendo nada con los parámetros a `new` y `execute`. Implementemos
los cuerpos de estas funciones con el comportamiento que queremos. Para
comenzar, pensemos en `new`. Anteriormente, elegimos un tipo sin signo para el
parámetro `size`, porque un pool con un número negativo de hilos no tiene
sentido. Sin embargo, un pool con cero hilos tampoco tiene sentido, pero cero
es un `usize` perfectamente válido. Agregaremos código para verificar que
`size` es mayor que cero antes de devolver una instancia de `ThreadPool` y
hacer que el programa se bloquee si recibe un cero usando el macro `assert!`,
como se muestra en el Listado 21-13.

<Listing number="21-13" file-name="src/lib.rs" caption="Implementando `ThreadPool::new` para generar un panic si `size` es cero">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-13/src/lib.rs:here}}
```

</Listing>

Hemos agregado documentación para nuestro `ThreadPool` con comentarios de
documentación. Ten en cuenta que seguimos las buenas prácticas de documentación
agregando una sección que llama a las situaciones en las que nuestra función
puede entrar en panic, como se discutió en el Capítulo 14. ¡Intenta ejecutar
`cargo doc --open` y hacer clic en la estructura `ThreadPool` para ver cómo se
ven los documentos generados para `new`!

En lugar de agregar la macro `assert!` como lo hicimos aquí, podríamos cambiar
`new` a `build` y devolver un `Result` como lo hicimos con `Config::build` en
el proyecto I/O en el Listado 12-9. Pero hemos decidido en este caso que
intentar crear un pool de hilos sin ningún hilo debería ser un error
irrecuperable. Si te sientes ambicioso, intenta escribir una función llamada
`build` con la siguiente firma para comparar con la función `new`:

```rust,ignore
pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
```

#### Creando espacio para almacenar los hilos

Ahora que tenemos una forma de saber que tenemos un número válido de hilos para
almacenar en el pool, podemos crear esos hilos y almacenarlos en el struct
`ThreadPool` antes de devolver el struct. Pero, ¿cómo “almacenamos” un hilo?
Echemos otro vistazo a la firma de `thread::spawn`:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

La función `spawn` devuelve un `JoinHandle<T>`, donde `T` es el tipo que el
closure devuelve. Intentemos usar `JoinHandle` también y veamos qué sucede. En
nuestro caso, los closures que estamos pasando al pool de hilos manejarán la
conexión y no devolverán nada, por lo que `T` será el tipo de unidad `()`.

El código en el Listado 21-14 se compilará, pero aún no creará ningún hilo.
Hemos cambiado la definición de `ThreadPool` para contener un vector de
instancias de `thread::JoinHandle<()>`, inicializado el vector con una
capacidad de `size`, configurado un bucle `for` que ejecutará algún código para
crear los hilos y devuelto una instancia de `ThreadPool` que los contiene.

<Listing number="21-14" file-name="src/lib.rs" caption="Creando un vector para que `ThreadPool` contenga los hilos">

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-14/src/lib.rs:here}}
```

</Listing>

Hemos llevado `std::thread` al scope en la biblioteca, porque estamos usando
`thread::JoinHandle` como el tipo de los elementos en el vector en
`ThreadPool`.

Una vez que se recibe un tamaño válido, nuestro `ThreadPool` crea un nuevo
vector que puede contener `size` elementos. La función `with_capacity`
realiza la misma tarea que `Vec::new`, pero con una diferencia importante: se
asigna espacio en el vector. Debido a que sabemos que necesitamos almacenar
`size` elementos en el vector, hacer esta asignación por adelantado es
ligeramente más eficiente que usar `Vec::new`, que se redimensiona a sí mismo a
medida que se insertan elementos.

Cuando ejecutes `cargo check` nuevamente, debería tener éxito:

#### Un struct `Worker` responsable de enviar código desde el `ThreadPool` a un hilo

Dejamos un comentario en el bucle `for` en el Listado 21-14 con respecto a la
creación de hilos. Aquí, veremos cómo creamos hilos. La biblioteca estándar
proporciona `thread::spawn` como una forma de crear hilos, y `thread::spawn`
espera obtener algún código que el hilo debe ejecutar tan pronto como se cree
el hilo. Sin embargo, en nuestro caso, queremos crear los hilos y hacer que
_esperen_ el código que enviaremos más tarde. La implementación de la biblioteca
estándar de hilos no incluye ninguna forma de hacer eso; tenemos que
implementarlo manualmente.

Implementaremos este comportamiento introduciendo una nueva estructura de datos
entre `ThreadPool` y los hilos que administrarán este nuevo comportamiento.
Llamaremos a esta estructura de datos _"Worker"_, que es un término común en las
implementaciones de pooling. El Worker recoge el código que debe ejecutarse y
ejecuta el código en el hilo del Worker. Piensa en las personas que trabajan
en la cocina de un restaurante: los trabajadores esperan hasta que lleguen los
pedidos de los clientes, y luego son responsables de tomar esos pedidos y
cumplirlos.

En lugar de almacenar un vector de instancias `JoinHandle<()>` en el pool de
hilos, almacenaremos instancias del struct `Worker`. Cada `Worker` contendrá
una instancia `JoinHandle<()>`. Luego, implementaremos un método en `Worker`
que tomará un closure de código para ejecutar y lo enviará al hilo en ejecución
para su ejecución. También daremos a cada trabajador un `id` para que podamos
distinguir entre los diferentes trabajadores en el pool al registrar o depurar.

Aquí está el nuevo proceso que ocurrirá cuando creemos un `ThreadPool`.
Implementaremos el código que envía el closure al hilo después de que tengamos
`Worker` configurado de esta manera:

1. Definimos un struct `Worker` que contiene un `id` y un `JoinHandle<()>`.
2. Cambiamos `ThreadPool` para contener un vector de instancias `Worker`.
3. Definimos una función `Worker::new` que toma un número `id` y devuelve una
   instancia `Worker` que contiene un `id` y un hilo creado con un closure
   vacío.
4. En `ThreadPool::new`, usamos el contador del bucle `for` para generar un
   `id`, creamos un nuevo `Worker` con ese `id` y almacenamos el trabajador en
   el vector.

Si estás listo para un desafío, intenta implementar estos cambios por ti mismo
antes de ver el código en el Listado 21-15.

¿Listo? Aquí está el Listado 21-15 con una forma de hacer las modificaciones

<Listing number="21-15" file-name="src/lib.rs" caption="Modificando `ThreadPool` para contener instancias de `Worker` en lugar de contener hilos directamente">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-15/src/lib.rs:here}}
```

</Listing>

Hemos cambiado el nombre del campo en `ThreadPool` de `threads` a `workers`
porque ahora contiene instancias de `Worker` en lugar de instancias de
`JoinHandle<()>`. Usamos el contador en el bucle `for` como argumento para
`Worker::new`, y almacenamos cada nuevo `Worker` en el vector llamado
`workers`.

El código externo (como nuestro servidor en _src/main.rs_) no necesita conocer
los detalles de implementación con respecto al uso de un struct `Worker` dentro
de `ThreadPool`, por lo que hacemos que el struct `Worker` y su función `new`
sean privadas. La función `Worker::new` utiliza el `id` que le damos y almacena
una instancia `JoinHandle<()>` que se crea al generar un nuevo hilo usando un
closure vacío.

> Nota: Si el sistema operativo no puede crear un hilo porque no hay suficientes
> recursos del sistema, `thread::spawn` entrará en panic. Eso hará que todo
> nuestro servidor entre en panic, incluso si la creación de algunos hilos
> tiene éxito. Por simplicidad, este comportamiento está bien, pero en una
> implementación de grupo de hilos de producción, es probable que desee usar
> [`std::thread::Builder`][builder]<!-- ignore --> y su método
> [`spawn`][builder-spawn]<!-- ignore --> que devuelve `Result` en su lugar.

Este código se compilará y almacenará el número de instancias `Worker` que
especificamos como argumento para `ThreadPool::new`. Pero todavía no estamos
procesando el closure que obtenemos en `execute`. Veamos cómo hacer eso a
continuación.

#### Enviando solicitudes a hilos a través de canales

El siguiente problema que abordaremos es que los closures que se pasan a
`tread::spawn` no hacen absolutamente nada. Actualmente, obtenemos el closure
que queremos ejecutar en el método `execute`. Pero necesitamos darle a
`thread::spawn` un closure para ejecutar cuando creamos cada `Worker` durante
la creación del `ThreadPool`.

Queremos que los structs `Worker` que acabamos de crear obtengan el código a
ejecutar desde una cola mantenida en `ThreadPool` y envíen ese código a su
hilo para su ejecución.

Los canales que aprendimos en el Capítulo 16, una forma simple de comunicarse
entre dos hilos, serían perfectos para este caso de uso. Usaremos un canal para
funcionar como la cola de trabajos, y `execute` enviará un trabajo desde el
`ThreadPool` a las instancias `Worker`, que enviarán el trabajo a su hilo. Aquí
está el plan:

1. El `ThreadPool` creará un canal y mantendrá el emisor.
2. Cada `Worker` mantendrá el receptor.
3. Crearemos un nuevo struct `Job` que contendrá los closures que queremos
   enviar a través del canal.
4. El método `execute` enviará el trabajo que desea ejecutar a través del
   emisor.
5. En su hilo, el `Worker` recorrerá su receptor y ejecutará los closures de
   cualquier trabajo que reciba.

Empecemos por crear un canal en `ThreadPool::new` y mantener el emisor en la
instancia `ThreadPool`, como se muestra en el Listado 21-16. El struct `Job`
no contiene nada por ahora, pero será el tipo de elemento que enviaremos por el
canal.

<Listing number="21-16" file-name="src/lib.rs" caption="Modificando `ThreadPool` para almacenar el emisor de un canal que transmite instancias `Job`">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-16/src/lib.rs:here}}
```

</Listing>

En `ThreadPool::new`, creamos nuestro nuevo canal y hacemos que el pool
mantenga el emisor. Esto se compilará correctamente.

Intentemos pasar un receptor del canal a cada trabajador mientras el pool de
hilos crea el canal. Sabemos que queremos usar el receptor en el hilo que los
trabajadores generan, por lo que haremos referencia al parámetro `receiver` en
el closure. El código en el Listado 21-17 aún no se compilará.

<Listing number="21-17" file-name="src/lib.rs" caption="Pasando el receptor a los trabajadores">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-17/src/lib.rs:here}}
```

</Listing>

Hemos hecho algunos cambios pequeños y sencillos: pasamos el receptor al
constructor `Worker::new`, y luego lo usamos dentro del closure.

Cuando intentamos compilar este código, obtenemos este error:

```console
{{#include ../listings/ch21-web-server/listing-21-17/output.txt}}
```

El código está intentando pasar `receiver` a múltiples instancias de `Worker`.
Esto no funcionará, como recordará del Capítulo 16: la implementación de canal
que Rust proporciona es de múltiples _productores_, un solo _consumidor_. Esto
significa que no podemos simplemente clonar el extremo consumidor del canal
para solucionar este código. Tampoco queremos enviar un mensaje varias veces a
múltiples consumidores; queremos una lista de mensajes con múltiples
trabajadores de modo que cada mensaje se procese una vez.

Además, quitar un trabajo de la cola del canal implica modificar el `receiver`,
por lo que los hilos necesitan una forma segura de compartir y modificar el
`receiver`; de lo contrario, podríamos obtener condiciones de carrera (como se
explicó en el Capítulo 16).

Recuerda los smart pointers thread-safe discutidos en el Capítulo 16: para
compartir la propiedad entre varios hilos y permitir que los hilos muten el
valor, necesitamos usar `Arc<Mutex<T>>`. El tipo `Arc` permitirá que varios
trabajadores sean propietarios del receptor, y `Mutex` garantizará que solo un
trabajador obtenga un trabajo del receptor a la vez. El Listado 21-18 muestra
los cambios que debemos hacer.

<Listing number="21-18" file-name="src/lib.rs" caption="Compartiendo el receptor entre los trabajadores usando `Arc` y `Mutex`">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-18/src/lib.rs:here}}
```

</Listing>

En `ThreadPool::new`, ponemos el receptor en un `Arc` y un `Mutex`. Para cada
nuevo trabajador, clonamos el `Arc` para aumentar el recuento de referencias
para que los trabajadores puedan compartir la propiedad del receptor.

Con estos cambios, ¡el código se compila! ¡Estamos llegando!

#### Implementando el método `execute`

En este punto, finalmente implementaremos el método `execute` en `ThreadPool`.
También cambiaremos `Job` de un struct a un alias de tipo para un objeto de
trait que contiene el tipo de cierre que recibe `execute`. Como se discutió en
la sección [“Creación de sinónimos de tipo con alias de
tipo”][creating-type-synonyms-with-type-aliases]<!-- ignore -->
del Capítulo 20, los alias de tipo nos permiten hacer tipos largos más cortos
para facilitar su uso. Mira el Listado 21-19.

<Listing number="21-19" file-name="src/lib.rs" caption="Creando un alias de tipo `Job` para un `Box` que contenga cada closure y luego enviamos el trabajo por el canal">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-19/src/lib.rs:here}}
```

</Listing>

Después de crear una nueva instancia de `Job` usando el closure que obtenemos
en `execute`, enviamos ese trabajo por el extremo de envío del canal. Estamos
llamando a `unwrap` en `send` para el caso de que el envío falle. Esto podría
suceder si, por ejemplo, detenemos todos nuestros hilos de ejecución, lo que
significa que el extremo receptor ha dejado de recibir nuevos mensajes. En este
momento, no podemos detener que nuestros hilos se ejecuten: nuestros hilos
continúan ejecutándose mientras exista el pool. La razón por la que usamos
`unwrap` es que sabemos que el caso de falla no sucederá, pero el compilador no
sabe eso.

¡Pero aún no hemos terminado! En el trabajador, nuestro cierre que se pasa a
`thread::spawn` todavía solo _hace referencia_ al extremo receptor del canal.
En su lugar, necesitamos que el cierre se repita para siempre, preguntando al
extremo receptor del canal por un trabajo y ejecutando el trabajo cuando lo
obtiene. Hagamos el cambio que se muestra en el Listado 21-20 a `Worker::new`.

<Listing number="21-20" file-name="src/lib.rs" caption="Recibiendo y ejecutando los trabajos en el hilo del worker">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-20/src/lib.rs:here}}
```

</Listing>

Aquí, primero llamamos a `lock` en el `receiver` para adquirir el mutex, y
luego llamamos a `unwrap` para que el hilo actual se bloquee en caso de que
ocurra algún error. Adquirir un bloqueo puede fallar si el mutex está en un
estado _envenenado_, lo que puede suceder si algún otro hilo se bloqueó mientras
sostenía el bloqueo en lugar de liberar el bloqueo. En esta situación, llamar a
`unwrap` para que este hilo se bloquee es la acción correcta a tomar. Siéntase
libre de cambiar este `unwrap` a un `expect` con un mensaje de error que sea
significativo para ti.

Si obtenemos el bloqueo en el mutex, llamamos a `recv` en el receptor para
recibir un `Job`. Un `unwrap` final mueve más allá de cualquier error aquí
también, que podría ocurrir si el hilo que tiene el extremo de envío se ha
apagado, similar a cómo el método `send` devuelve `Err` si el receptor se
apaga.

La llamada a `recv` bloquea, por lo que si aún no hay un trabajo, el hilo
actual esperará hasta que haya un trabajo disponible. El `Mutex<T>` garantiza
que solo un hilo `Worker` a la vez está tratando de solicitar un trabajo.

¡Nuestro pool de hilos ahora está en un estado funcional! Ejecuta `cargo run`
y haz algunas solicitudes:

<!-- manual-regeneration
cd listings/ch21-web-server/listing-21-20
cargo run
make some requests to 127.0.0.1:7878
Can't automate because the output depends on making requests
-->

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
warning: field `workers` is never read
 --> src/lib.rs:7:5
  |
6 | pub struct ThreadPool {
  |            ---------- field in this struct
7 |     workers: Vec<Worker>,
  |     ^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: fields `id` and `thread` are never read
  --> src/lib.rs:48:5
   |
47 | struct Worker {
   |        ------ fields in this struct
48 |     id: usize,
   |     ^^
49 |     thread: thread::JoinHandle<()>,
   |     ^^^^^^

warning: `hello` (lib) generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.91s
     Running `target/debug/hello`
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
```

¡Éxito! Ahora tenemos un pool de hilos que ejecuta conexiones de forma
asincrónica. Nunca hay más de cuatro hilos creados, por lo que nuestro sistema
no se sobrecargará si el servidor recibe muchas solicitudes. Si hacemos una
solicitud a _/sleep_, el servidor podrá atender otras solicitudes haciendo que
otro hilo las ejecute.

> Nota: Si abres _/sleep_ en múltiples ventanas del navegador simultáneamente,
> podrían cargarse una a la vez en intervalos de 5 segundos. Algunos navegadores
> web ejecutan múltiples instancias de la misma solicitud secuencialmente por
> razones de almacenamiento en caché. Esta limitación no es causada por nuestro
> servidor web.

Este es un buen momento para pausar y considerar cómo sería diferente el código 
de los Listados 21-18, 21-19 y 21-20 si usáramos futures en lugar de un closure 
para el trabajo a realizar. ¿Qué tipos cambiarían? ¿Cómo serían diferentes las 
firmas de los métodos, si es que cambiarían? ¿Qué partes del código 
permanecerían iguales?

Después de aprender sobre el bucle `while let` en los Capítulos 17 y 18, es 
posible que te preguntes por qué no escribimos el código del hilo del trabajador 
como se muestra en el Listado 21-21.

<Listing number="21-21" file-name="src/lib.rs" caption="Una implementación alternativa de `Worker::new` usando `while let`">

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-21/src/lib.rs:here}}
```

</Listing>

Este código se compila y se ejecuta, pero no produce el comportamiento de
sub procesamiento deseado: una solicitud lenta aún hará que otras solicitudes
esperen ser procesadas. La razón es algo sutil: el struct `Mutex` no tiene
un método público `unlock` porque el ownership del bloqueo se basa en la
duración del `MutexGuard<T>` dentro del `LockResult<MutexGuard<T>>` que el
método `lock` devuelve. En tiempo de compilación, el borrow checker puede hacer
cumplir la regla de que un recurso protegido por un `Mutex` no se puede acceder
a menos que tengamos el bloqueo. Sin embargo, esta implementación también puede
resultar en que el bloqueo se mantenga más tiempo de lo previsto si no somos
conscientes de la duración del `MutexGuard<T>`.

El código en el Listado 21-21 que usa `let job =
receiver.lock().unwrap().recv().unwrap();` funciona porque con `let`, los
valores temporales utilizados en la expresión del lado derecho del signo igual
se descartan inmediatamente cuando finaliza la declaración `let`. Sin embargo,
`while let` (y `if let` y `match`) no descarta los valores temporales hasta el
final del bloque asociado. En el Listado 21-21, el bloqueo permanece retenido
durante la duración de la llamada a `job()`, lo que significa que otros
trabajadores no pueden recibir trabajos.

[creating-type-synonyms-with-type-aliases]: ch20-04-advanced-types.html#creando-type-synonyms-con-type-aliases
[integer-types]: ch03-02-data-types.html#tipos-de-enteros
[fn-traits]: ch13-01-closures.html#moving-captured-values-out-of-the-closure-and-the-fn-traits
[builder]: https://doc.rust-lang.org/std/thread/struct.Builder.html
[builder-spawn]: https://doc.rust-lang.org/std/thread/struct.Builder.html#method.spawn
