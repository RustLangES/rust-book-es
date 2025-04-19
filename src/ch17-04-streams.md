## Streams

Hasta ahora en este capítulo, hemos estado trabajando principalmente con futuros
individuales. La única gran excepción fue el receiver de canal asíncrono que
utilizamos. Recuerda cómo usamos el receiver para nuestro canal asíncrono en el
[“Paso de Mensajes”][17-02-messages] al principio del capítulo. El método
`recv` asíncrono produce una secuencia de elementos a lo largo del tiempo. Esta
es una instancia de un patrón mucho más general, a menudo llamado *stream*.

Una secuencia de elementos es algo que ya hemos visto antes, cuando miramos el
trait `Iterator` en el capítulo 13. Sin embargo, hay dos diferencias entre los
iteradores y el receptor de canal asíncrono. La primera es el elemento del
tiempo: los iteradores son sincrónicos, mientras que el receptor de canal es
asíncrono. La segunda es la API. Cuando trabajamos directamente con un
`Iterator`, llamamos a su método sincrónico `next`. Con el stream
`trpl::Receiver`, en particular, llamamos a un método asíncrono `recv`
en su lugar. Estas APIs, de otro modo, se sienten muy similares.

Un stream es similar a una forma asíncrona de iteración. Mientras que el
`trpl::Receiver` espera específicamente recibir mensajes, la API de stream
general es mucho más general: proporciona el siguiente elemento de la misma
manera que `Iterator`, pero de forma asíncrona. La similitud entre los
iteradores y los streams en Rust significa que en realidad podemos crear un
stream a partir de cualquier iterador. Al igual que con un iterador, podemos
trabajar con un stream llamando a su método `next` y luego esperando la
salida, como en el listado 17-30.

<Listing number="17-30" caption="Creando un stream a partir de un iterador y mostrando sus valores" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-30/src/main.rs:stream}}
```

</Listing>

Comenzamos con un array de números, que convertimos en un iterador y luego
llamamos a `map` para duplicar todos los valores. Luego convertimos el
iterador en un stream usando la función `trpl::stream_from_iter`. Luego
recorremos los elementos en el stream a medida que llegan con el bucle `while
let`.

Desafortunadamente, cuando intentamos ejecutar el código, no compila. En su
lugar, como podemos ver en la salida, informa que no hay un método `next`
disponible.

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-30
cargo build
copy only the error output
-->

```console
error[E0599]: no method named `next` found for struct `Iter` in the current scope
  --> src/main.rs:10:40
   |
10 |         while let Some(value) = stream.next().await {
   |                                        ^^^^
   |
   = note: the full type name has been written to 'file:///projects/async_await/target/debug/deps/async_await-9de943556a6001b8.long-type-1281356139287206597.txt'
   = note: consider using `--verbose` to print the full type name to the console
   = help: items from traits can only be used if the trait is in scope
help: the following traits which provide `next` are implemented but not in scope; perhaps you want to import one of them
   |
1  + use crate::trpl::StreamExt;
   |
1  + use futures_util::stream::stream::StreamExt;
   |
1  + use std::iter::Iterator;
   |
1  + use std::str::pattern::Searcher;
   |
help: there is a method `try_next` with a similar name
   |
10 |         while let Some(value) = stream.try_next().await {
   |                                        ~~~~~~~~
```

Como sugiere la salida, la razón del error del compilador es que necesitamos el
método correcto en el ámbito para poder usar el método `next`. Dada nuestra
discusión hasta ahora, podrías esperar razonablemente que sea `Stream`, pero el
trait que necesitamos aquí es en realidad `StreamExt`. El `Ext` allí es por
“extensión”: este es un patrón común en la comunidad de Rust para extender un
trait con otro.

¿Por qué necesitamos `StreamExt` en lugar de `Stream`, y qué hace el trait
`Stream` en sí? Brevemente, la respuesta es que en todo el ecosistema de Rust,
el trait `Stream` define una interfaz de bajo nivel que combina
efectivamente los traits `Iterator` y `Future`. El trait `StreamExt`
suministra un conjunto de APIs de nivel superior sobre `Stream`, incluyendo
el método `next` así como otros métodos de utilidad similares a los
proporcionados por el trait `Iterator`. Volveremos a los traits
`Stream` y `StreamExt` con un poco más de detalle al final del capítulo.
Por ahora, esto es suficiente para dejarnos seguir avanzando.

La solución al error del compilador es agregar una declaración `use` para
`trpl::StreamExt`, como en el listado 17-31.

<Listing number="17-31" caption="Usando exitosamente un iterador como base para un stream" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-31/src/main.rs:all}}
```

</Listing>

Con todas esas piezas juntas, este código funciona como queremos. ¡Lo que es
más, ahora que tenemos `StreamExt` en el ámbito, podemos usar todos sus
métodos de utilidad, al igual que con los iteradores! Por ejemplo, en el
listado 17-32, usamos el método `filter` para filtrar todo menos los
múltiplos de tres y cinco.

<Listing number="17-32" caption="Filtrando un `Stream` con el método `StreamExt::filter`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-32/src/main.rs:all}}
```

</Listing>

Por supuesto, esto no es muy interesante. Podríamos hacer eso con
iteradores normales y sin nada asíncrono. Así que veamos algunas de las
otras cosas que podemos hacer que son únicas para los streams.

### Componiendo Streams

Muchos conceptos se representan naturalmente como streams: elementos que
se vuelven disponibles en una cola, o trabajar con más datos de los que
pueden caber en la memoria de una computadora al extraer (pull) solo 
fragmentos (chunks) de ellos del sistema de archivos a la vez, o datos que 
llegan a través de la red a lo largo del tiempo. Debido a que los streams son 
futuros, también podemos usarlos con cualquier otro tipo de futuro, y podemos
combinarlos de maneras interesantes. Por ejemplo, podemos agrupar
eventos para evitar activar demasiadas llamadas de red, establecer
tiempos de espera en secuencias de operaciones de larga duración, o
restringir eventos de la interfaz de usuario para evitar hacer
trabajo innecesario.

Empecemos creando un pequeño stream de mensajes, como un sustituto
para un stream de datos que podríamos ver desde un WebSocket u otro
protocolo de comunicación en tiempo real. En el listado 17-33, creamos
una función `get_messages` que devuelve `impl Stream<Item = String>`. Para
su implementación, creamos un canal asíncrono, recorremos las primeras
diez letras del alfabeto inglés y las enviamos a través del canal.

Nosotros también usamos un nuevo tipo: `ReceiverStream`, que convierte
el receptor `rx` del `trpl::channel` en un `Stream` con un método `next`.
De vuelta en `main`, usamos un bucle `while let` para imprimir todos los
mensajes del stream.

<Listing number="17-33" caption="Usando el receptor `rx` como un `ReceiverStream`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-33/src/main.rs:all}}
```

</Listing>

Cuando ejecutamos este código, obtenemos exactamente los resultados que
esperábamos:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Message: 'a'
Message: 'b'
Message: 'c'
Message: 'd'
Message: 'e'
Message: 'f'
Message: 'g'
Message: 'h'
Message: 'i'
Message: 'j'
```

Podríamos hacer esto con la API `Receiver` regular, o incluso la API
`Iterator` regular, sin embargo. Agreguemos algo que requiera streams:
agregando un tiempo de espera que se aplique a cada elemento en el stream, y
una demora en los elementos que emitimos.

En el listado 17-34, comenzamos agregando un tiempo de espera al stream
con el método `timeout`, que proviene del trait `StreamExt`. Luego
actualizamos el cuerpo del bucle `while let`, porque el stream ahora
devuelve un `Result`. La variante `Ok` indica que un mensaje llegó a
tiempo; la variante `Err` indica que el tiempo de espera se agotó
antes de que llegara algún mensaje. Hacemos un `match` en ese
resultado y ya sea imprimimos el mensaje cuando lo recibimos
exitosamente, o imprimimos un aviso sobre el tiempo de espera. Finalmente,
ten en cuenta que fijamos los mensajes después de aplicar el tiempo
de espera a ellos, porque el helper de tiempo de espera produce un
stream que necesita ser fijado para ser sondeado.

<Listing number="17-34" caption="Usando el método `StreamExt::timeout` para establecer un límite de tiempo en los elementos de un stream" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-34/src/main.rs:timeout}}
```

</Listing>

Sin embargo, como no hay demoras entre los mensajes, este tiempo de espera
no cambia el comportamiento del programa. Agreguemos una demora variable
a los mensajes que enviamos. En `get_messages`, usamos el método
`enumerate` del iterador con el array `messages` para que podamos
obtener el índice de cada elemento que estamos enviando junto con el
elemento en sí. Luego aplicamos una demora de 100 milisegundos
a los elementos de índice par y una demora de 300 milisegundos a los
elementos de índice impar, para simular las diferentes demoras que
podríamos ver de un stream de mensajes en el mundo real. Debido a que
nuestro tiempo de espera es de 200 milisegundos, esto debería afectar
la mitad de los mensajes.

<Listing number="17-35" caption="Enviando mensajes a través de `tx` con una demora asíncrona sin hacer de `get_messages` una función asíncrona" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-35/src/main.rs:messages}}
```

</Listing>

Para usar sleep entre mensajes en la función `get_messages` sin bloquear, 
necesitamos usar async. Sin embargo, no podemos hacer de `get_messages` una 
función asíncrona, porque entonces devolveríamos un `Future<Output = Stream<Item = String>>`
en lugar de un `Stream<Item = String>>`. El llamador tendría que esperar a 
`get_messages` para obtener acceso al stream. Pero recuerda:
todo en un futuro dado sucede de manera lineal; la concurrencia ocurre
*entre* futuros. Esperar a `get_messages` requeriría que enviara todos
los mensajes, incluyendo las esperas (sleep) entre el envío de
cada mensaje, antes de devolver el stream receptor. Como resultado,
el tiempo de espera terminaría siendo inútil. No habría demoras
en el stream en sí: todas las demoras ocurrirían antes de que el
stream estuviera disponible.

En su lugar, dejamos `get_messages` como una función regular que devuelve un
stream, y generamos una tarea para manejar las llamadas asíncronas
`sleep`.

> Nota: llamar a `spawn_task` de esta manera funciona porque ya configuramos
> nuestro runtime. Llamar a esta implementación particular de `spawn_task`
> *sin* configurar primero un runtime causará un pánico. Otras
> implementaciones eligen diferentes compensaciones: pueden generar un nuevo
> runtime y así evitar el pánico, pero terminar con un poco de sobrecarga
> adicional, o simplemente no proporcionar una forma independiente de generar
> tareas sin referencia a un runtime. Debes asegurarte de saber qué
> compensación ha elegido tu runtime y escribir tu código en consecuencia.

Ahora nuestro código tiene un resultado mucho más interesante. Entre
cada par de mensajes, vemos un error reportado: `Problem: Elapsed(())`.

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-35
cargo run
copy only the program output, *not* the compiler output
-->

```text
Message: 'a'
Problem: Elapsed(())
Message: 'b'
Message: 'c'
Problem: Elapsed(())
Message: 'd'
Message: 'e'
Problem: Elapsed(())
Message: 'f'
Message: 'g'
Problem: Elapsed(())
Message: 'h'
Message: 'i'
Problem: Elapsed(())
Message: 'j'
```

El tiempo de espera no previene que los mensajes lleguen al final: aún
obtenemos todos los mensajes originales. Esto se debe a que nuestro
canal es ilimitado: puede contener tantos mensajes como podamos
ajustar en memoria. Si el mensaje no llega antes de que se agote el
tiempo, nuestro manejador de stream lo tendrá en cuenta, pero cuando
vuelva a sondear el stream, el mensaje puede haber llegado
ahora.

Puedes obtener un comportamiento diferente si es necesario usando
otros tipos de canales, o otros tipos de streams más
generalmente. Veamos uno de esos en la práctica en nuestro
ejemplo final para esta sección, combinando un stream de intervalos
de tiempo con este stream de mensajes.

### Combinando Streams

Primero, creemos otro stream, que emitirá un elemento cada milisegundo si
lo dejamos ejecutarse directamente. Para simplificar, podemos usar la
función `sleep` para enviar un mensaje con un retraso, y combinarlo
con el mismo enfoque de crear un stream a partir de un canal que
usamos en `get_messages`. La diferencia es que esta vez, vamos a
enviar de vuelta el conteo de intervalos que ha transcurrido, así
que el tipo de retorno será `impl Stream<Item = u32>`, y podemos
llamar a la función `get_intervals`.

En el listado 17-36, comenzamos definiendo un `count` en la tarea. (También
podríamos definirlo fuera de la tarea, pero es más claro limitar el
alcance de cualquier variable dada). Luego creamos un bucle
infinito. Cada iteración del bucle duerme (sleep) de forma
asíncrona durante un milisegundo, incrementa el conteo y luego lo
envía a través del canal. Debido a que todo esto está envuelto
en la tarea creada por `spawn_task`, todo se limpiará junto con
el runtime, incluyendo el bucle infinito.

<Listing number="17-36" caption="Creando un stream con un contador que se emitirá una vez cada milisegundo" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-36/src/main.rs:intervals}}
```

</Listing>

Este tipo de bucle infinito, que solo termina cuando todo el runtime se
desmonta, es bastante común en Rust asíncrono: muchos programas
necesitan seguir ejecutándose indefinidamente. Con async, esto no bloquea nada
más, siempre que haya al menos un punto de espera (await) en cad
a iteración a través del bucle.

En el bloque async de la función principal, comenzamos llamando a
`get_intervals`. Luego combinamos los streams `messages` e
`intervals` con el método `merge`, que combina múltiples streams
en un solo stream que produce elementos de cualquiera de los
streams de origen tan pronto como los elementos están disponibles,
sin imponer ningún orden particular. Finalmente, recorremos
ese stream combinado en lugar de sobre `messages` (listado 17-37).

<Listing number="17-37" caption="Intentando combinar streams de mensajes e intervalos" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-37/src/main.rs:main}}
```

</Listing>

En este punto, ni `messages` ni `intervals` necesitan ser fijados o
mutables, porque ambos se combinarán en el único stream `merged`.
Sin embargo, esta llamada a `merge` no compila. (Tampoco lo hace la
llamada `next` en el bucle `while let`, pero volveremos a eso después de
arreglar esto). Los dos streams tienen diferentes tipos. El stream
`messages` tiene el tipo `Timeout<impl Stream<Item = String>>`,
donde `Timeout` es el tipo que implementa `Stream` para una
llamada a `timeout`. Mientras tanto, el stream `intervals`
tiene el tipo `impl Stream<Item = u32>`. Para combinar estos
dos streams, necesitamos transformar uno de ellos para que
coincida con el otro.

En el listado 17-38, volvemos a trabajar el stream `intervals`, porque 
`messages` ya está en el formato básico que queremos y tiene que manejar errores 
de tiempo de espera. Primero, podemos usar el método auxiliar `map` para 
transformar los `intervals` en una cadena. En segundo lugar, necesitamos hacer 
coincidir el `Timeout` de `messages`. Sin embargo, como en realidad no
*queremos* un tiempo de espera para `intervals`, podemos simplemente crear un 
tiempo de espera que sea más largo que los otros tiempos de 
espera que estamos usando. Aquí, creamos un tiempo de espera de 10 segundos con 
`Duration::from_secs(10)`. Finalmente, necesitamos hacer `stream` mutable, para 
que las llamadas `next` del bucle `while let` puedan iterar a través del stream, 
y fijarlo para que sea seguro hacerlo.

<!-- We cannot directly test this one, because it never stops. -->

<Listing number="17-38" caption="Alineando los tipos del stream `intervals` con el tipo del stream `messages`" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-38/src/main.rs:main}}
```

</Listing>

Esto nos lleva *casi* a donde necesitamos estar. Todo se verifica
el tipo. Sin embargo, si ejecutas esto, habrá dos problemas. Primero,
nunca se detendrá. ¡Tendrás que detenerlo con <span class="keystroke">ctrl-c</span>!
En segundo lugar, los mensajes del alfabeto inglés estarán enterrados en
medio de todos los mensajes del contador de intervalos:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the tasks running differently rather than
changes in the compiler -->

```text
--snip--
Interval: 38
Interval: 39
Interval: 40
Message: 'a'
Interval: 41
Interval: 42
Interval: 43
--snip--
```

En el listado 17-39 se muestra una forma de resolver estos últimos dos
problemas. Primero, usamos el método `throttle` en el stream `intervals`, para 
que no abrume al stream `messages`. La limitación (throttling) es una forma de 
limitar la tasa a la que una función será llamada—o, en este caso, con qué 
frecuencia se sondeará el stream. Una vez cada cien milisegundos debería 
bastante, porque eso está en el mismo rango de tiempo que la frecuencia con la 
que llegan nuestros mensajes.

Para limitar el número de elementos que aceptaremos de un stream, podemos
usar el método `take`. Lo aplicamos al *stream combinado*, porque
queremos limitar la salida final, no solo un stream u otro.

<Listing number="17-39" caption="Usando `throttle` y `take` para gestionar los streams combinados" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-39/src/main.rs:throttle}}
```

</Listing>

Ahora, cuando ejecutamos el programa, se detiene después de extraer veinte 
elementos del stream, y los intervalos no abruman a los mensajes. También
no obtenemos `Interval: 100` o `Interval: 200` o así, sino que
en su lugar obtenemos `Interval: 1`, `Interval: 2`, y así sucesivamente — 
¡incluso cuando tenemos un stream de origen que *puede* producir un evento
cada milisegundo! Eso se debe a que la llamada `throttle` produce un nuevo
stream, envolviendo el stream original, de modo que el stream original
solo se sondea a la tasa de limitación, no a su propia tasa “nativa”.
No tenemos un montón de mensajes de intervalo no manejados que
estamos eligiendo ignorar. En su lugar, ¡nunca producimos esos mensajes de
intervalo en primer lugar! Esta es la “pereza” inherente de los futuros
de Rust en acción nuevamente, lo que nos permite elegir nuestras
características de rendimiento.

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-39
cargo run
copy and paste only the program output
-->

```text
Interval: 1
Message: 'a'
Interval: 2
Interval: 3
Problem: Elapsed(())
Interval: 4
Message: 'b'
Interval: 5
Message: 'c'
Interval: 6
Interval: 7
Problem: Elapsed(())
Interval: 8
Message: 'd'
Interval: 9
Message: 'e'
Interval: 10
Interval: 11
Problem: Elapsed(())
Interval: 12
```

Hay una última cosa que necesitamos manejar: ¡errores! Con ambos
streams basados en canales, las llamadas `send` podrían fallar
cuando el otro lado del canal se cierra—y eso es solo una cuestión de
cómo el runtime ejecuta los futuros que componen el stream. Hasta
ahora hemos ignorado esto llamando a `unwrap`, pero en una
aplicación bien comportada, deberíamos manejar explícitamente el
error, al mínimo terminando el bucle para que no intentemos enviar
más mensajes. El listado 17-40 muestra una estrategia de error
simple: imprime el problema y luego `break` de los bucles. Como
siempre, la forma correcta de manejar un error de envío de
mensaje variará—¡solo asegúrate de tener una estrategia!

<Listing number="17-40" caption="Manejando errores y cerrando los bucles">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-40/src/main.rs:errors}}
```

</Listing>

Ahora que hemos visto un montón de async en la práctica, echemos un
paso atrás y profundicemos en algunos de los detalles de cómo
`Future`, `Stream`, y los otros traits clave que Rust usa para
hacer que async funcione.

[17-02-messages]: ch17-02-concurrency-with-async.html#paso-de-mensajes
