## Trabajando con cualquier número de futures

En la sección anterior, cuando pasamos de usar dos futures a tres, tuvimos que cambiar
`join` por `join3`. Tener que usar una función diferente cada vez que cambiamos la
cantidad de futures que queremos combinar sería poco práctico. Afortunadamente, existe
una versión en macro de `join` que nos permite pasar un número arbitrario de argumentos
y, además, se encarga de esperar (await) cada future automáticamente.
Así, podríamos reescribir el código del Listado 17-13 para usar `join!` en lugar de
`join3`, como se muestra en Listado 17-14:

<Listing number="17-14" caption="Usando `join!` para esperar múltiples futures" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-14/src/main.rs:here}}
```

</Listing>

Esto sin duda es una gran mejora en comparación con tener que alternar entre `join`,
`join3` y `join4` y así sucesivamente! Sin embargo, incluso esta versión con macro solo
funciona cuando conocemos de antemano el número de futures que queremos combinar.
En Rust del mundo real, es muy común agregar futures a una colección y luego esperar
a que algunos o todos ellos se completen.

Para manejar todos los futures dentro de una colección, necesitamos iterar sobre ellos y
*unirlos* (join). La función `trpl::join_all` acepta cualquier tipo que implemente el rasgo
(trait) `Iterator`,  que aprendimos en el Capítulo 13, por lo que parece la solución
ideal. Probemos colocando nuestros futures en un vector y reemplazando
`join!` con `join_all`.

<Listing number="17-15" caption="Almacenando futures anónimos en un vector y llamando a `join_all`" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-15/src/main.rs:here}}
```

</Listing>

Desafortunadamente, esto no compila. En su lugar, obtenemos este error:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-15/
cargo build
copy just the compiler error
-->


```text
error[E0308]: mismatched types
  --> src/main.rs:45:37
   |
10 |         let tx1_fut = async move {
   |                       ---------- the expected `async` block
...
24 |         let rx_fut = async {
   |                      ----- the found `async` block
...
45 |         let futures = vec![tx1_fut, rx_fut, tx_fut];
   |                                     ^^^^^^ expected `async` block, found a different `async` block
   |
   = note: expected `async` block `{async block@src/main.rs:10:23: 10:33}`
              found `async` block `{async block@src/main.rs:24:22: 24:27}`
   = note: no two async blocks, even if identical, have the same type
   = help: consider pinning your async block and casting it to a trait object
```

Esto puede resultar sorprendente. Después de todo, ninguno de los futures devuelve un
valor, por lo que cada bloque produce un `Future<Output = ()>`. Sin embargo, `Future` es un trait, no
tipo concreto. Los tipos concretos (concrete types) son las estructuras de datos individuales que
el compilador genera para los bloques async. No se pueden colocar dos estructuras diferentes escritas
a mano dentro de un `Vec`, y lo mismo ocurre con las diferentes
estructuras generadas por el compilador.

Para solucionar esto, necesitamos usar *trait objects*, al igual que hicimos en la sección [“Refactorizando
para mejorar la modularidad y el manejo de errores”][dyn] en el Capítulo 12. (Cubriremos los trait objects
en detalle en el Capítulo 18.) Usar objetos de trait nos permite tratar cada
uno de los futures anónimos producidos por estos tipos como si fueran del mismo tipo,
ya que todos implementan el trait `Future`.

> Nota: En el capitulo 8, discutimos otra forma de incluir múltiples tipos en un
> `Vec`: usar un enum para representar cada uno de los diferentes tipos que pueden
> aparecer en el vector. Sin embargo, en este caso no podemos hacer eso. Pues,
> no tenemos forma de nombrar los diferentes tipos, ya que son anónimos. Además,
> la razón por la que recurrimos a un vector y `join_all` en primer lugar es porque
> queremos trabajar con una colección dinámica de futures, donde no sabemos cuáles
> serán hasta el tiempo de ejecución.

Empezaremos envolviendo cada uno de los futuros de `vec!` en una `Box::new`, como se
muestra en el Listado 17-16.

<Listing number="17-16" caption="Intentando usar `Box::new` para alinear los tipos de los futures en un `Vec`" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-16/src/main.rs:here}}
```

</Listing>

Desafortunadamente, esto aún no compila. De hecho, seguimos teniendo el mismo error
básico que antes, pero ahora aparece tanto en la segunda como en la tercera llamada a `Box::new`.
Además, también vemos nuevos errores relacionados con el trait `Unpin`. Volveremos a los errores
de `Unpin` en un momento. Primero, solucionemos los errores de tipo en las llamadas a
`Box::new`, anotando explícitamente el tipo de la variable  `futures`:

<Listing number="17-17" caption="Corrigiendo el resto de los errores de tipo usando una declaración de tipo explícita" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-17/src/main.rs:here}}
```

</Listing>

El tipo que tuvimos que escribir aquí es un poco complejo, así que desglosémoslo paso a paso:

* El tipo más interno es el futuro en sí. Indicamos explícitamente que su valor de salida
 es el tipo unitario `()` escribiendo `Future<Output = ()>`.
* Luego, usamos `dyn` para marcar el trait como dinámico.
* Toda la referencia al trait se envuelve dentro de un `Box`.
* Finalmente, especificamos explícitamente que `futures` es un `Vec` contiene estos elementos.

Esto ya supone una gran diferencia. Ahora, al ejecutar el compilador, solo vemos los
errores relacionados con `Unpin`. Aunque hay tres de ellos, todos son bastante
similares en su contenido.

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-16
cargo build
# copy *only* the errors
# fix the paths
-->

```text
error[E0308]: mismatched types
   --> src/main.rs:46:46
    |
10  |         let tx1_fut = async move {
    |                       ---------- the expected `async` block
...
24  |         let rx_fut = async {
    |                      ----- the found `async` block
...
46  |             vec![Box::new(tx1_fut), Box::new(rx_fut), Box::new(tx_fut)];
    |                                     -------- ^^^^^^ expected `async` block, found a different `async` block
    |                                     |
    |                                     arguments to this function are incorrect
    |
    = note: expected `async` block `{async block@src/main.rs:10:23: 10:33}`
               found `async` block `{async block@src/main.rs:24:22: 24:27}`
    = note: no two async blocks, even if identical, have the same type
    = help: consider pinning your async block and casting it to a trait object
note: associated function defined here
   --> file:///home/.rustup/toolchains/1.82/lib/rustlib/src/rust/library/alloc/src/boxed.rs:255:12
    |
255 |     pub fn new(x: T) -> Self {
    |            ^^^

error[E0308]: mismatched types
   --> src/main.rs:46:64
    |
10  |         let tx1_fut = async move {
    |                       ---------- the expected `async` block
...
30  |         let tx_fut = async move {
    |                      ---------- the found `async` block
...
46  |             vec![Box::new(tx1_fut), Box::new(rx_fut), Box::new(tx_fut)];
    |                                                       -------- ^^^^^^ expected `async` block, found a different `async` block
    |                                                       |
    |                                                       arguments to this function are incorrect
    |
    = note: expected `async` block `{async block@src/main.rs:10:23: 10:33}`
               found `async` block `{async block@src/main.rs:30:22: 30:32}`
    = note: no two async blocks, even if identical, have the same type
    = help: consider pinning your async block and casting it to a trait object
note: associated function defined here
   --> file:///home/.rustup/toolchains/1.82/lib/rustlib/src/rust/library/alloc/src/boxed.rs:255:12
    |
255 |     pub fn new(x: T) -> Self {
    |            ^^^

error[E0277]: `{async block@src/main.rs:10:23: 10:33}` cannot be unpinned
   --> src/main.rs:48:24
    |
48  |         trpl::join_all(futures).await;
    |         -------------- ^^^^^^^ the trait `Unpin` is not implemented for `{async block@src/main.rs:10:23: 10:33}`, which is required by `Box<{async block@src/main.rs:10:23: 10:33}>: Future`
    |         |
    |         required by a bound introduced by this call
    |
    = note: consider using the `pin!` macro
            consider using `Box::pin` if you need to access the pinned value outside of the current scope
    = note: required for `Box<{async block@src/main.rs:10:23: 10:33}>` to implement `Future`
note: required by a bound in `join_all`
   --> file:///home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/futures-util-0.3.30/src/future/join_all.rs:105:14
    |
102 | pub fn join_all<I>(iter: I) -> JoinAll<I::Item>
    |        -------- required by a bound in this function
...
105 |     I::Item: Future,
    |              ^^^^^^ required by this bound in `join_all`

error[E0277]: `{async block@src/main.rs:10:23: 10:33}` cannot be unpinned
  --> src/main.rs:48:9
   |
48 |         trpl::join_all(futures).await;
   |         ^^^^^^^^^^^^^^^^^^^^^^^ the trait `Unpin` is not implemented for `{async block@src/main.rs:10:23: 10:33}`, which is required by `Box<{async block@src/main.rs:10:23: 10:33}>: Future`
   |
   = note: consider using the `pin!` macro
           consider using `Box::pin` if you need to access the pinned value outside of the current scope
   = note: required for `Box<{async block@src/main.rs:10:23: 10:33}>` to implement `Future`
note: required by a bound in `futures_util::future::join_all::JoinAll`
  --> file:///home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/futures-util-0.3.30/src/future/join_all.rs:29:8
   |
27 | pub struct JoinAll<F>
   |            ------- required by a bound in this struct
28 | where
29 |     F: Future,
   |        ^^^^^^ required by this bound in `JoinAll`

error[E0277]: `{async block@src/main.rs:10:23: 10:33}` cannot be unpinned
  --> src/main.rs:48:33
   |
48 |         trpl::join_all(futures).await;
   |                                 ^^^^^ the trait `Unpin` is not implemented for `{async block@src/main.rs:10:23: 10:33}`, which is required by `Box<{async block@src/main.rs:10:23: 10:33}>: Future`
   |
   = note: consider using the `pin!` macro
           consider using `Box::pin` if you need to access the pinned value outside of the current scope
   = note: required for `Box<{async block@src/main.rs:10:23: 10:33}>` to implement `Future`
note: required by a bound in `futures_util::future::join_all::JoinAll`
  --> file:///home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/futures-util-0.3.30/src/future/join_all.rs:29:8
   |
27 | pub struct JoinAll<F>
   |            ------- required by a bound in this struct
28 | where
29 |     F: Future,
   |        ^^^^^^ required by this bound in `JoinAll`
```

Eso es mucho para asimilar, así que desglosémoslo. La primera parte del mensaje nos
indica que el primer bloque async (`src/main.rs:8:23: 20:10`) no implementa
el trait `Unpin`, y sugiere usar `pin!` o `Box::pin` para solucionarlo.
Más adelante en el capítulo, profundizaremos en más detalles sobre `Pin` y
`Unpin`. Por ahora, sin embargo, podemos simplemente seguir el consejo del compilador para salir
del apuro! En el Listado 17-18,  comenzamos actualizando la anotación de tipo para
`futures`, agregamos un `Pin` que envuelva cada `Box`. Luego, utilizamos `Box::pin` para fijar (pin)
los futures en sí mismos.

<Listing number="17-18" caption="Usando `Pin` y `Box::pin` para hacer que el tipo de `Vec` verifique" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-18/src/main.rs:here}}
```

</Listing>

Si compilamos y ejecutamos esto, finalmente obtenemos la salida que esperábamos:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
received 'hi'
received 'more'
received 'from'
received 'messages'
received 'the'
received 'for'
received 'future'
received 'you'
```

¡Uf!

Todavía hay más que podemos explorar aquí. Por un lado, usar `Pin<Box<T>>`
introduce un pequeño costo adicional debido a la asignación en el heap con
`Box`—y en realidad solo lo estamos haciendo para alinear los tipos. No necesitamos *realmente*
la asignación en el heap allocation, después de todo: estos futures son locales a esta función
en particular. Como mencionamos antes, `Pin`  en sí mismo es un tipo contenedor, por lo que podemos
obtener el beneficio de tener un solo tipo en el `Vec`—la razón original por la que usamos
`Box`—sin necesidad de una asignación en el heap allocation. En su lugar, podemos usar `Pin`
directamente con cada future, utilizando el macro `std::pin::pin`.

Sin embargo, aún debemos ser explícitos sobre el tipo de la referencia fijada (pinned reference);
de lo contrario, Rust no sabrá interpretarlos como objetos de trait dinámicos,
que es lo que necesitamos en el `Vec`. Por lo tanto, usamos `pin!` en cada future
al definirlo y definimos `futures` como un `Vec`que contiene referencias mutables fijadas
a `Future`, como se muestra en el Listado 17-19.

<Listing number="17-19" caption="Usando `Pin` directamente con el macro `pin!` para evitar asignaciones innecesarias en el heap" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-19/src/main.rs:here}}
```

</Listing>

Hemos llegado hasta aquí ignorando el hecho de que podríamos tener diferentes tipos de `Output`.
Por ejemplo, en el Listado 17-20,  el future anónimo para `a` implementa
`Future<Output = u32>`, el de `b` implementa `Future<Output = &str>`, y el de `c`
implementa `Future<Output = bool>`.

<Listing number="17-20" caption="Tres futures con tipos distintos" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-20/src/main.rs:here}}
```

</Listing>

Podemos usar `trpl::join!` para esperarlos (await), ya que permite pasar múltiples futures
de diferentes tipos y produce una tupla con esos tipos. Lo que *no* podemos hacer es usar
`trpl::join_all`, porque requiere que todos los futures tengan el mismo tipo. Recordemos
que ese error fue el que nos llevó a esta aventura con `Pin`!

Este es un compromiso fundamental: podemos manejar un número dinámico de futures
con `join_all`, siempre que todos tengan el mismo tipo, o podemos manejar un número
fijo de futures con las funciones `join` o el macro `join!`,
incluso si tienen tipos diferentes. Sin embargo, esto es lo mismo que ocurre con cualquier
otro tipo en Rust. Los futures no son especiales en este sentido, aunque tengamos una
sintaxis conveniente para trabajar con ellos, ¡y eso es algo bueno!

### Carrera de futuros

Cuando usamos `join` y sus variantes, esperamos que *todas* las futures terminen
antes de continuar. Sin embargo,  a veces solo necesitamos que *alguna* de ellas
termine antes de seguir adelante—algo así como hacer
que compitan entre sí.

En el Listado 17-21, usamos `trpl::race` para hacer competir dos futuros, `slow` y
`fast`. Cada una imprime un mensaje al iniciar, espera un tiempo determinado
con `sleep`, e imprime otro mensaje al terminar. Luego, pasamos ambas a  `trpl::race`
y esperamos a que una de ellas termine.
(El resultado no será muy sorprendente: `fast` gana)
A diferencia de cuando usamos `race` en [Futures y la sintaxis async][async-program], aquí
simplemente ignoramos la instancia de `Either` que devuelve, porque todo lo interesante
sucede dentro de los bloques async.

<Listing number="17-21" caption="Usando `race` para obtener el resultado de cualquier futuro que termine primero" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-21/src/main.rs:here}}
```

</Listing>

Observa que si inviertes el orden de los argumentos en `race`, el orden de los mensajes
“started” cambia, aunque el futuro `fast` siempre se completa
primero. Esto se debe que la implementación de la función `race` en particular
no es justa. Siempre ejecuta los futuros en el orden en que se pasan como
argumentos. Otras implementaciones *sí* son justas y eligen aleatoriamente qué
futuro evaluar primero. De todas maneras, independientemente de si la implementación de
`race` que estamos usando es justa o no, *uno* de los futuros se ejecutará hasta el primer
`await` antes de que otra tarea pueda comenzar.

Repasemos sobre [Futures y la sintaxis async][async-program] donde en cada `await`,
Rust le da la oportunidad al runtime de pausar la tarea y cambiar a otra si el futuro
que se está esperando aún no está listo. Lo contrario también es cierto: Rust *solo* pausa
los bloques asíncronos y devuelve el control al runtime en un punto de await. Todo lo que
ocurre entre puntos de *await* es síncrono.

Esto significa que si realizas una gran cantidad de trabajo dentro de un bloque asíncrono
sin un await, ese futuro bloqueará el progreso de otros futuros. A veces, esto se
conoce como un futuro dejando sin recursos a otros futuros. En algunos casos, esto puede
no ser un gran problema. Sin embargo, si estás realizando una configuración costosa o
una tarea de larga duración, o si tienes un futuro que seguirá ejecutando una
tarea indefinidamente, necesitarás pensar en cuándo
y dónde devolver el control al runtime.

Del mismo modo, si tienes operaciones bloqueantes de larga duración, async
puede ser una herramienta útil para permitir que diferentes partes del programa
se relacionen entre sí.

Pero ¿cómo *podrías* devolver el control al runtime en esos casos?

### Cediendo el Control

Simulemos una operación de larga duración. En el Listado 17-22 se introduce la función `slow`.
Esta usa `std::thread::sleep` en lugar de `trpl::sleep`, por lo que al llamar a
`slow` se bloqueará el hilo actual durante un cierto número de milisegundos. Podemos
usar `slow` como sustituto de operaciones reales que son tanto de larga duración
como bloqueante.

<Listing number="17-22" caption="Usando `thread::sleep` para simular operaciones lentas" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-22/src/main.rs:slow}}
```

</Listing>

En el Listado 17-23, usamos `slow` para emular ese tipo de trabajo con la CPU limitada trabajando en
un par de futuros. Para empezar, cada futuro en este código solo devuelve el control del runtime
*después* de terminar un motón de operaciones lentas.

<Listing number="17-23" caption="Usando `thread::sleep` para simular operaciones lentas" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-23/src/main.rs:slow-futures}}
```

</Listing>

Si ejecutas esto, verás esta salida:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-23/
cargo run
copy just the output
-->

```text
'a' started.
'a' ran for 30ms
'a' ran for 10ms
'a' ran for 20ms
'b' started.
'b' ran for 75ms
'b' ran for 10ms
'b' ran for 15ms
'b' ran for 350ms
'a' finished.
```

Al igual que con el ejemplo anterior, `race` finaliza tan pronto como `a` termina.
Sin embargo, no hay intercalación entre los dos futuros. El futuro `a` realiza todo su
trabajo hasta que se espera la llamada a `trpl::sleep`, es entonces donde el futuro `b` realiza todo su
trabajo hasta que se espera su propia llamada a `trpl::sleep`, y solo después el futuro `a`
termina. Para permitir que ambos futuros avancen entre sus tareas lentas, necesitamos
puntos de espera (con await) para poder devolver el control al runtime. ¡Eso
significa que necesitamos algo que podamos esperar!

Ya podemos ver este tipo de transferencia de control en el Listado 17-23: si elimináramos
la llamada a `trpl::sleep` al final del futuro `a`, este se completaría sin que el futuro
`b` se ejecute *en absoluto*.  Tal vez podríamos usar la función `sleep`
como punto de partida?

<Listing number="17-24" caption="Usando `sleep` para dejar que las operaciones cambien de progreso" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-24/src/main.rs:here}}
```

</Listing>

En el Listado 17-24, añadimos llamadas a `trpl::sleep` con `await` entre cada llamada a
`slow`. Ahora los dos futuros trabajan de forma intercalada:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-24
cargo run
copy just the output
-->

```text
'a' started.
'a' ran for 30ms
'b' started.
'b' ran for 75ms
'a' ran for 10ms
'b' ran for 10ms
'a' ran for 20ms
'b' ran for 15ms
'a' finished.
```

El futuro `a` sigue ejecutándose un poco antes de ceder el control a `b`, porque llama
a `slow` antes de hacer cualquier llamada a `trpl::sleep`, pero después de eso, los futuros se
intercambian cada vez que uno de ellos alcanza un punto de await. En este caso, hemos
hecho eso después de cada llamada a `slow`, pero podríamos dividir el trabajo de la
manera que tenga más sentido para nosotros.

Sin embargo, realmente no queremos *esperar* aquí: queremos avanzar lo más rápido posible.
Solo necesitamos devolver el control al runtime. Podemos hacer eso directamente,
usando la función `yield_now`. En el Listado 17-25, reemplazamos todas esas llamadas
a `sleep` con `yield_now`.

<Listing number="17-25" caption="Usando `yield_now` para dejar que las operaciones cambien de progreso" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-25/src/main.rs:yields}}
```

</Listing>

Esto no solo deja más claro el propósito real, sino que también puede ser
significativamente más rápido que usar `sleep`, ya que los temporizadores como el
que usa `sleep` suelen tener limitaciones en su granularidad. La versión de `sleep`
que estamos usando, por ejemplo, siempre dormirá al menos un milisegundo, incluso si le pasamos un
`Duration` de un nanosegundo. Y de nuevo, las computadoras modernas son *rápidas*:
pueden hacer mucho en un solo milisegundo.

Puedes comprobarlo tú mismo configurando un pequeño benchmark, como el del Listado 17-26.
(No es una forma especialmente rigurosa de hacer pruebas de rendimiento, pero es
suficiente para mostrar la diferencia en este caso). Aquí omitimos toda la
impresión de estado, pasamos un `Duration`de un nanosegundo a `trpl::sleep`, y dejamos
que cada futuro se ejecute por sí solo, sin alternar entre ellos.  Luego,
ejecutamos 1,000 iteraciones y comparamos cuánto tiempo toma el futuro que usa
`trpl::sleep` en comparación con el que usa `trpl::yield_now`.

<Listing number="17-26" caption="Comparando el rendimiento de `sleep` y `yield_now`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-26/src/main.rs:here}}
```

</Listing>

Esta versión con `yield_now` es *mucho* más rápido!

Esto significa que async puede ser útil incluso para tareas que consumen
muchos recursos de cómputo, dependiendo de lo que haga el resto del programa.
Proporciona una herramienta útil para estructurar las relaciones entre distintas
partes del código. Esto es un tipo de *multitarea cooperativa*, donde cada futuro
decide cuándo ceder el control a través de puntos de await. Por lo tanto,
también es responsabilidad de cada futuro evitar bloquearse por demasiado tiempo. De hecho,
en algunos sistemas operativos embebidos basados en Rust, ¡esta es la *única* forma de realizar multitarea!

Por supuesto en código real, no estarás alternando llamadas a funciones con await
en cada línea. Aunque ceder el control de esta manera es relativamente barato,
no es gratuito. En muchos casos, intentar dividir una tarea intensiva
en cómputo podría hacerla significativamente más lenta, así que a veces es mejor,
en *términos de rendimiento general*, permitir que una operación bloquee brevemente.
Siempre es recomendable medir el rendimiento para identificar los cuellos de botella
reales en tu código. Sin embargo, si *notas* que muchas tareas están ejecutándose en
serie cuando esperabas que fueran concurrentes, este concepto es clave para entender
qué está ocurriendo.

### Construyendo Nuestras Propias Abstracciones Asíncronas

También podemos componer futuros para crear nuevos patrones. Por ejemplo, podemos construir
una función `timeout` utilizando los bloques asíncronos que ya tenemos. Al final,
obtendremos otro bloque de construcción que podremos reutilizar para crear
más abstracciones asíncronas aún más avanzadas.

En el Listado 17-27 muestra cómo esperamos que `timeout` funcione con un futuro
lento.

<Listing number="17-27" caption="Usando nuestro imaginario `timeout` para ejecutar una operación lenta con un límite de tiempo" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-27/src/main.rs:here}}
```

</Listing>

¡Vamos a implementarlo! Para empezar, pensemos en la API de `timeout`:

* Debe ser una función asíncrona para que podamos esperarla con `await`.
* Su primer parámetro debe ser un futuro a ejecutar. Podemos hacerlo genérico
  para que funcione con cualquier futuro.
* Su segundo parámetro será el tiempo máximo de espera. Si usamos un `Duration`,
  será fácil pasarlo a `trpl::sleep`.
* Debe devolver un `Result`. Si el futuro se completa exitosamente, el
  `Result` será `Ok` con el valor producido por el futuro. Si el tiempo de
  espera se agota primero, el `Result` será un `Err` con la duración que esperó
  antes de expirar.

El Listado 17-28 muestra esta declaración.

<!-- This is not tested because it intentionally does not compile. -->

<Listing number="17-28" caption="Definiendo la firma de `timeout`" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-28/src/main.rs:declaration}}
```

</Listing>

Eso satisface nuestros objetivos en cuanto a los tipos. Ahora pensemos en el *comportamiento*
que necesitamos: queremos hacer competir el futuro que recibimos con el tiempo límite.
Porque, podemos usar `trpl::sleep` para crear un futuro temporizador a partir de la duración otorgada,
y usar `trpl::race` para ejecutar ese temporizador junto con el futuro que nos pasaron.

Como sabemos `race` no es justo y evalúa los argumentos en el orden en que se
pasan. Por lo tanto, pasamos `future_to_try` primero a `race` para que tenga la oportunidad de
completarse incluso si `max_time` es un tiempo muy corto. Si `future_to_try` termina primero,
`race` devolverá `Left` con el resultado del `future`. Si
`timer` finaliza antes, `race` devolverá `Right` con la salida del temporizador, que es
`()`.

En el Listado 17-29, hacemos un match sobre el resultado que se espera en `trpl::race`. Si
`future_to_try` se completa con éxito y obtenemos un `Left(output)`, retornamos un `Ok(output)`.
Si en cambio el temporizador se agota y obtenemos un `Right(())`, ignoramos el `()`
con `_` y devolvemos `Err(max_time)`.

<Listing number="17-29" caption="Definiendo `timeout` con `race` y `sleep`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-29/src/main.rs:implementation}}
```

</Listing>

Con eso, tenemos un `timeout` que funciona, construido a partir de otros dos ayudantes asíncronos (async helpers). Si
ejecutamos nuestro código, imprimirá el modo de fallo después del tiempo de espera:

```text
Failed after 2 seconds
```

Dado que los futuros pueden componerse con otros futuros, es posible construir herramientas
muy poderosas utilizando pequeños bloques asíncronos. Por ejemplo, este mismo enfoque puede
usarse para combinar tiempos de espera con reintentos y, a su vez, aplicar esto a tareas
como llamadas de red, uno de los ejemplos del inicio del capítulo.

En la práctica, trabajarás directamente con `async` y `await`, y
en segundo lugar con funciones y macros como `join`, `join_all`, `race`, entre otras.
Solo necesitarás recurrir a pin ocasionalmente para utilizarlas con estas
APIs.

Hasta ahora, hemos visto varias formas de trabajar con múltiples futuros al mismo
tiempo. A continuación, exploraremos cómo manejar múltiples futuros en secuencia
a lo largo del tiempo usando *streams*. Antes de continuar, aquí hay un par de
cosas que podrías considerar:

* Usamos un `Vec` con `join_all`  para esperar a que todos los futuros
  en un grupo terminaran. ¿Cómo podrías usar un `Vec` para procesar un grupo de
  futuros en secuencia en su lugar? ¿Cuáles serían las ventajas y desventajas de hacerlo?

* Échale un vistazo al tipo `futures::stream::FuturesUnordered` del crate de `futures`.
  ¿En qué se diferenciaría su uso con respecto a un `Vec`? (No te preocupes por el hecho
  de que provenga de la parte de `stream` del crate; funciona perfectamente
  con cualquier colección de futuros).


[collections]: ch08-01-vectors.html#using-an-enum-to-store-multiple-types
[dyn]: ch12-03-improving-error-handling-and-modularity.html
[async-program]: ch17-01-futures-and-syntax.html#our-first-async-program
