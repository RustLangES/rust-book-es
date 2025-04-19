## Futures y la sintaxis `async`

Los elementos clave de la programación asíncrona en Rust son los *futures* y las
palabras clave `async` y `await`.

Un *future* es un valor que puede no estar listo ahora, pero que estará listo en
algún momento en el futuro. (Este mismo concepto aparece en muchos lenguajes,
a veces bajo otros nombres como “tarea” o “promesa”.) Rust proporciona un trait
`Future` como un bloque de construcción para que diferentes operaciones 
asíncronas puedan implementarse con diferentes estructuras de datos, pero con
una interfaz común. En Rust, decimos que los tipos que implementan el trait
`Future` son futuros. Cada tipo que implementa `Future` contiene su propia
información sobre el progreso que se ha hecho y lo que significa estar "listo".

La palabra clave `async` se puede aplicar a bloques y funciones para especificar
que pueden ser interrumpidos y reanudados. Dentro de un bloque asíncrono o una
función asíncrona, puedes usar la palabra clave `await` para esperar a que un
futuro esté listo, pudiendo *esperar un futuro*. Cada lugar donde esperas un
futuro dentro de un bloque o función asíncrona es un lugar donde ese bloque o
función asíncrona puede ser pausado y reanudado. El proceso de comprobar con un
futuro para ver si su valor está disponible se llama *polling*.

Algunos otros lenguajes también utilizan las palabras clave `async` y `await`
para la programación asíncrona. Si estás familiarizado con esos lenguajes, puede
que notes algunas diferencias significativas en cómo Rust hace las cosas,
incluyendo cómo maneja la sintaxis. ¡Por una buena razón, como veremos!

La mayor parte del tiempo al escribir Rust asíncrono, usamos las palabras clave
`async` y `await`. Rust las compila en código equivalente utilizando el trait
`Future`, al igual que compila los bucles `for` en código equivalente utilizando
el trait `Iterator`. Sin embargo, como Rust proporciona el trait `Future`, puedes
implementarlo para tus propios tipos de datos cuando sea necesario. Muchas de las
funciones que veremos a lo largo de este capítulo devuelven tipos con sus propias
implementaciones de `Future`. Volveremos a la definición del trait al final del
capítulo y profundizaremos más en cómo funciona, pero este es suficiente detalle
para seguir avanzando.

Todo esto puede parecer un poco abstracto. Escribamos nuestro primer programa
asíncrono: un pequeño web scraper. Pasaremos dos URLs desde la línea de 
comandos, obtendremos ambos de forma concurrente y devolveremos el resultado
de aquel que termine primero. Este ejemplo tendrá un poco de nueva sintaxis,
pero no te preocupes. Explicaremos todo lo que necesitas saber a medida que
avanzamos.

### Nuestro primer programa asíncrono

Para mantener este capítulo centrado en aprender lo asíncrono, en lugar de
manejar partes del ecosistema, hemos creado el crate `trpl` (`trpl` es la
abreviatura de “The Rust Programming Language”). Re-exporta todos los tipos,
traits y funciones que necesitarás, principalmente de los crates 
[`futures`][futures-crate] y [`tokio`][tokio].

- El crate `futures` es un hogar oficial para la experimentación de Rust para
  el código asíncrono, y es en realidad donde el tipo `Future` fue diseñado
  originalmente.

- Tokio es el runtime asíncrono más utilizado en Rust hoy en día, especialmente
  (¡pero no solo!) para aplicaciones web. Hay otros runtimes geniales por ahí,
  y pueden ser más adecuados para tus propósitos. Usamos Tokio bajo el capó
  para `trpl` porque está bien probado y ampliamente utilizado.

En algunos casos, `trpl` también renombra o envuelve las APIs originales para
permitirnos mantenernos enfocados en los detalles relevantes para este capítulo.
Si quieres entender qué hace el crate, te animamos a que eches un vistazo a
[su código fuente][crate-source]. Podrás ver de qué crate proviene cada
re-exportación, y hemos dejado extensos comentarios explicando qué hace el 
crate.

Crea un nuevo proyecto binario llamado `hello-async` y añade el crate `trpl`
como dependencia:

```console
$ cargo new hello-async
$ cd hello-async
$ cargo add trpl
```

Ahora podemos usar las diversas piezas proporcionadas por `trpl` para escribir
nuestro primer programa asíncrono. Construiremos una pequeña herramienta de
línea de comandos que obtiene dos páginas web, extrae el elemento `<title>` de
cada una e imprime el título de aquella que termine todo el proceso primero.

Empecemos escribiendo una función que toma una URL de página como parámetro,
hace una petición a ella y devuelve el texto del elemento título:

<Listing number="17-1" file-name="src/main.rs" caption="Definiendo una función asíncrona para obtener el elemento título de una página HTML">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-01/src/main.rs:all}}
```

</Listing>

En el Listado 17-1, definimos una función llamada `page_title`, y la marcamos
con la palabra clave `async`. Luego usamos la función `trpl::get` para obtener
cualquier URL que se pase, y esperamos la respuesta usando la palabra clave
`await`. Luego obtenemos el texto de la respuesta llamando a su método `text`,
y una vez más lo esperamos con la palabra clave `await`. Ambos pasos son
asíncronos. Para `get`, necesitamos esperar a que el servidor envíe la primera
parte de su respuesta, que incluirá cabeceras HTTP, cookies, etc. Esa parte de
la respuesta puede entregarse por separado del cuerpo de la petición. 
Especialmente si el cuerpo es muy grande, puede llevar algo de tiempo que todo
llegue. Por lo tanto, tenemos que esperar a que *toda* la respuesta llegue, por
lo que el método `text` también es asíncrono.

Tenemos que esperar explícitamente ambos de estos futuros, porque los futuros
en Rust son *perezosos*: no hacen nada hasta que les pides con `await`. (De
hecho, Rust mostrará una advertencia del compilador si no usas un futuro.) Esto
debería recordarte nuestra discusión de los iteradores 
[en el Capítulo 13][iterators-lazy].
Los iteradores no hacen nada a menos que llames a su método `next`—ya sea
directamente, o usando bucles `for` o métodos como `map` que usan `next` bajo
el capó. Con los futuros, se aplica la misma idea básica: no hacen nada a menos
que les pidas explícitamente. Esta estrategia de ejecución perezosa permite a 
Rust evitar ejecutar código asíncrono hasta que realmente sea necesario.

> Nota: Esto es diferente del comportamiento que vimos al usar `thread::spawn` 
> en el capítulo anterior, donde la closure que pasamos a otro hilo comenzó a
> ejecutarse inmediatamente. ¡También es diferente de cómo muchos otros 
> lenguajes abordan lo asíncrono! Pero es importante para Rust. Veremos por qué
> es así más adelante.

Una vez que tenemos `response_text`, podemos analizarlo en una instancia del
tipo `Html` usando `Html::parse`. En lugar de una cadena en bruto, ahora tenemos
un tipo de datos con el que podemos trabajar con el HTML como una estructura de
datos más rica. En particular, podemos usar el método `select_first` para
encontrar la primera instancia de un selector CSS dado. Pasando la cadena
`"title"`, obtendremos el primer elemento `<title>` en el documento, si lo hay.
Dado que puede que no haya ningún elemento coincidente, `select_first` devuelve
un `Option<ElementRef>`. Finalmente, usamos el método `Option::map`, que nos
permite trabajar con el elemento en el `Option` si está presente, y no hacer
nada si no lo está. (También podríamos usar una expresión `match` aquí, pero
`map` es más idiomático.) En el cuerpo de la función que proporcionamos a `map`,
llamamos a `inner_html` en el `title_element` para obtener su contenido, que es
un `String`. Cuando todo está dicho y hecho, tenemos un `Option<String>`.

Observa que la palabra clave `await` de Rust va después de la expresión que
estás esperando, no antes. Es decir, es una *palabra clave posfija*. Esto puede
ser diferente de lo que estás acostumbrado si has usado asíncrono en otros
lenguajes. Rust eligió esto porque hace que las cadenas de métodos sean mucho
más agradables de trabajar. Como resultado, podemos cambiar el cuerpo de
`page_url_for` para encadenar las llamadas a las funciones `trpl::get` y `text`
juntas con `await` entre ellas, como se muestra en el Listado 17-2:

<Listing number="17-2" file-name="src/main.rs" caption="Encadenando con la palabra clave `await`">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-02/src/main.rs:chaining}}
```

</Listing>

¡Con eso, hemos escrito con éxito nuestra primera función asíncrona! Antes de
añadir algo de código en `main` para llamarla, hablemos un poco más sobre lo
que hemos escrito y lo que significa.

Cuando Rust ve un bloque marcado con la palabra clave `async`, lo compila en un
tipo de datos único y anónimo que implementa el trait `Future`. Cuando Rust ve
una función marcada con `async`, la compila en una función no asíncrona cuyo
cuerpo es un bloque asíncrono. El tipo de retorno de una función asíncrona es
el tipo del tipo de datos anónimo que el compilador crea para ese bloque 
asíncrono.

Por lo tanto, escribir `async fn` es equivalente a escribir una función que
devuelve un *futuro* del tipo de retorno. Cuando el compilador ve una definición
de función como la `async fn page_title` en el Listado 17-1, es equivalente a
una función no asíncrona definida de la siguiente manera:

```rust
# extern crate trpl; // requerido para mdbook test
use std::future::Future;
use trpl::Html;

fn page_title(url: &str) -> impl Future<Output = Option<String>> + '_ {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}
```

Veamos cada parte de la versión transformada:

* Utiliza la sintaxis `impl Trait` que discutimos en la sección [“Traits como
  parámetros”][impl-trait] en el Capítulo 10.
* El trait devuelto es un `Future`, con un tipo asociado de `Output`. Observa
  que el tipo `Output` es `Option<String>`, que es el mismo que el tipo de
  retorno original de la versión `async fn` de `page_title`.
* Todo el código llamado en el cuerpo de la función original está envuelto en
  un bloque `async move`. Recuerda que los bloques son expresiones. Todo este
  bloque es la expresión devuelta por la función.
* Este bloque asíncrono produce un valor con el tipo `Option<String>`, como se
  describió anteriormente. Ese valor coincide con el tipo `Output` en el tipo de
  retorno. Esto es igual que otros bloques que has visto.
* El nuevo cuerpo de la función es un bloque `async move` debido a cómo usa el
  parámetro `url`. (Hablaremos mucho más sobre `async` vs. `async move` más
  adelante en el capítulo.)
* La nueva versión de la función tiene un tipo de duración que no hemos visto
  antes en el tipo de salida: `'_`. Debido a que la función devuelve un `Future`
  que se refiere a una referencia —en este caso, la referencia del parámetro
  `url`— necesitamos decirle a Rust que queremos que esa referencia esté
  incluida. No tenemos que nombrar la duración aquí, porque Rust es lo
  suficientemente inteligente como para saber que solo hay una referencia que
  podría estar involucrada, pero *sí* tenemos que ser explícitos en que el
  `Future` resultante está vinculado por esa duración.

Ahora podemos llamar a `page_title` en `main`. Para empezar, solo obtendremos
el título de una sola página. En el Listado 17-3, seguimos el mismo patrón que
usamos para obtener los argumentos de la línea de comandos en el Capítulo 12.
Luego pasamos la primera URL a `page_title`, y esperamos el resultado. Dado que
el valor producido por el futuro es un `Option<String>`, usamos una expresión
`match` para imprimir diferentes mensajes para tener en cuenta si la página
tenía un `<title>`.

<Listing number="17-3" file-name="src/main.rs" caption="Llamando a la función `page_title` desde `main` con un argumento proporcionado por el usuario">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-03/src/main.rs:main}}
```

</Listing>

Desafortunadamente, esto no compila. El único lugar donde podemos usar la
palabra clave `await` es en funciones o bloques asíncronos, y Rust no nos
permitirá marcar la función especial `main` como `async`.

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-03
cargo build
copy just the compiler error
-->

```text
error[E0752]: `main` function is not allowed to be `async`
 --> src/main.rs:6:1
  |
6 | async fn main() {
  | ^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`
```

La razón por la que `main` no puede ser marcada como `async` es que el código
asíncrono necesita un *runtime*: un crate de Rust que gestiona los detalles de
la ejecución de código asíncrono. La función `main` de un programa puede
*inicializar* un runtime, pero no es un runtime *en sí mismo*. (Veremos más
sobre por qué esto es un poco más adelante.) Cada programa de Rust que ejecuta
código asíncrono tiene al menos un lugar donde configura un runtime y ejecuta
los futuros.

La mayoría de los lenguajes que admiten asíncrono incluyen un runtime con el
lenguaje. Rust no lo hace. En su lugar, hay muchos runtimes asíncronos
disponibles, cada uno de los cuales hace diferentes compensaciones adecuadas
para el caso de uso al que se dirigen. Por ejemplo, un servidor web de alto
rendimiento con muchos núcleos de CPU y una gran cantidad de RAM tiene
necesidades muy diferentes a las de un microcontrolador con un solo núcleo, una
pequeña cantidad de RAM y sin capacidad para hacer asignaciones en el montón. 
Los crates que proporcionan esos runtimes también suelen suministrar versiones
asíncronas de funcionalidades comunes como la E/S de archivos o de red.

Aquí, y a lo largo del resto de este capítulo, usaremos la función `run` del
crate `trpl`, que toma un futuro como argumento y lo ejecuta hasta su
finalización. Detrás de escena, llamar a `run` configura un runtime para usarlo
para ejecutar el futuro pasado. Una vez que el futuro se completa, `run`
devuelve cualquier valor que el futuro haya producido.

Podríamos pasar el futuro devuelto por `page_title` directamente a `run`. Una
vez completado, podríamos hacer una coincidencia en el `Option<String>`
resultante, de la misma manera que intentamos hacer en el Listado 17-3. Sin
embargo, para la mayoría de los ejemplos en el capítulo (¡y la mayoría del 
código asíncrono en el mundo real!), haremos más que una sola llamada a función
asíncrona, por lo que en su lugar pasaremos un bloque `async` y esperaremos
explícitamente el resultado de llamar a `page_title`, como en el Listado 17-4.

<Listing number="17-4" caption="Esperando un bloque asíncrono con `trpl::run`" file-name="src/main.rs">

<!-- should_panic,noplayground because mdbook test does not pass args -->

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch17-async-await/listing-17-04/src/main.rs:run}}
```

</Listing>

Cuando ejecutamos esto, obtenemos el comportamiento que podríamos haber esperado
inicialmente:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-04
cargo build # skip all the build noise
cargo run https://www.rust-lang.org
# copy the output here
-->

```console
$ cargo run -- https://www.rust-lang.org
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/async_await 'https://www.rust-lang.org'`
The title for https://www.rust-lang.org was
            Rust Programming Language
```

¡Uf! ¡Finalmente tenemos algo de código asíncrono funcional! Ahora compila, y
podemos ejecutarlo. Antes de añadir código para competir dos sitios entre sí,
volvamos brevemente nuestra atención a cómo funcionan los futuros.

Cada *punto de espera* — es decir, cada lugar donde el código usa la palabra
clave `await` — representa un lugar donde el control se devuelve al runtime. 
Para que esto funcione, Rust necesita hacer un seguimiento del estado 
involucrado en el bloque asíncrono, para que el runtime pueda iniciar otro 
trabajo y luego volver cuando esté listo para intentar avanzar en este de nuevo. 
Esta es una máquina de estados invisible, como si escribieras un enum de esta 
manera para guardar el estado actual en cada punto de `await`:

```rust
{{#rustdoc_include ../listings/ch17-async-await/no-listing-state-machine/src/lib.rs:enum}}
```

Escribir el código para la transición entre cada estado a mano sería tedioso y
propenso a errores, especialmente al añadir más funcionalidad y más estados al
código más adelante. En su lugar, el compilador de Rust crea y gestiona las
estructuras de datos de la máquina de estados para el código asíncrono
automáticamente. Si te lo estás preguntando: sí, las reglas normales de
préstamo y propiedad en torno a las estructuras de datos se aplican. 
Afortunadamente, el compilador también se encarga de comprobarlas por nosotros, 
y tiene buenos mensajes de error. ¡Trabajaremos a través de algunos de esos más 
tarde en el capítulo!

En última instancia, algo tiene que ejecutar esa máquina de estados. Eso algo es
un runtime. (Es por eso que a veces puedes encontrarte con referencias a
*ejecutores* al investigar runtimes: un ejecutor es la parte de un runtime
responsable de ejecutar el código asíncrono.)

Ahora podemos entender por qué el compilador nos impidió hacer que `main` en sí
fuera una función asíncrona en el Listado 17-3. Si `main` fuera una función
asíncrona, algo más tendría que gestionar la máquina de estados para cualquier
futuro que `main` devolviera, ¡pero `main` es el punto de inicio del programa!
En su lugar, llamamos a la función `trpl::run` en `main`, que configura un
runtime y ejecuta el futuro devuelto por el bloque `async` hasta que devuelva
`Ready`.

> Nota: algunos runtimes proporcionan macros para que *puedas* escribir una
> función `main` asíncrona. Esos macros reescriben `async fn main() { ... }`
> para ser un `fn main` normal que hace lo mismo que hicimos a mano en el
> Listado 17-5: llamar a una función que ejecuta un futuro hasta su finalización
> de la misma manera que `trpl::run` hace.

Pongamos estas piezas juntas y veamos cómo podemos escribir código concurrente,
llamando a `page_title` con dos URLs diferentes pasadas desde la línea de
comandos y compitiéndolas.

<Listing number="17-5" caption="" file-name="src/main.rs">

<!-- should_panic,noplayground because mdbook does not pass args -->

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch17-async-await/listing-17-05/src/main.rs:all}}
```

</Listing>

En el Listado 17-5, comenzamos llamando a `page_title` para cada una de las
URLs proporcionadas por el usuario. Guardamos los futuros producidos al llamar a
`page_title` como `title_fut_1` y `title_fut_2`. Recuerda, estos todavía no
hacen nada, porque los futuros son perezosos, y aún no los hemos esperado. Luego
pasamos los futuros a `trpl::race`, que devuelve un valor para indicar cuál de
los futuros pasados a él termina primero.

> Nota: Bajo el capó, `race` está construido sobre una función más general,
> `select`, que encontrarás más a menudo en el código de Rust del mundo real.
> Una función `select` puede hacer muchas cosas que la función `trpl::race` no
> puede, pero también tiene cierta complejidad adicional que podemos omitir por
> ahora.

Cualquiera de los futuros puede “ganar” legítimamente, por lo que no tiene
sentido devolver un `Result`. En su lugar, `race` devuelve un tipo que no hemos
visto antes, `trpl::Either`. El tipo `Either` es algo similar a un `Result`, en
que tiene dos casos. A diferencia de `Result`, sin embargo, no hay noción de
éxito o fracaso integrada en `Either`. En su lugar, usa `Left` y `Right` para
indicar “uno u otro”.

```rust
enum Either<A, B> {
    Left(A),
    Right(B),
}
```

La función `race` devuelve `Left` si el primer argumento termina primero, con la
salida de ese futuro, y `Right` con la salida del segundo argumento futuro si
*ese* termina primero. Esto coincide con el orden en que aparecen los argumentos
cuando se llama a la función: el primer argumento está a la izquierda del
segundo argumento.

También actualizamos `page_title` para devolver la misma URL pasada. De esa
manera, si la página que se devuelve primero no tiene un `<title>` que podamos
resolver, aún podemos imprimir un mensaje significativo. Con esa información
disponible, terminamos actualizando nuestra salida de `println!` para indicar
tanto qué URL terminó primero como cuál fue el `<title>` de la página web en
esa URL, si lo hay.

¡Has construido un pequeño scraper web funcional ahora! Elige un par de URLs y
ejecuta la herramienta de línea de comandos. Puedes descubrir que algunos sitios
son confiablemente más rápidos que otros, mientras que en otros casos qué sitio
“gana” varía de una ejecución a otra. Más importante aún, has aprendido los
conceptos básicos de trabajar con futuros, por lo que ahora podemos profundizar
en aún más de las cosas que podemos hacer con asíncrono.

[impl-trait]: ch10-02-traits.html#traits-as-parameters
[iterators-lazy]: ch13-02-iterators.html

<!-- TODO: map source link version to version of Rust? -->

[crate-source]: https://github.com/rust-lang/book/tree/main/packages/trpl
[futures-crate]: https://crates.io/crates/futures
[tokio]: https://tokio.rs
