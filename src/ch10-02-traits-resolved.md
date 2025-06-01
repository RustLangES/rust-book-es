## Traits: Definiendo Comportamiento Compartido

Un _trait_ define funcionalidad que un tipo particular tiene y puede compartir
con otros tipos. Podemos usar traits para definir comportamiento compartido de
una manera abstracta. Podemos usar _trait bounds_ para especificar que un tipo
genérico puede ser cualquier tipo que tenga cierto comportamiento.

> Nota: Los traits son similares a una característica a menudo llamada
> _interfaces_ en otros lenguajes, aunque con algunas diferencias.
> En español también se les conoce como _rasgos_ pero en el libro intentaremos
> mantener la palabra clave sin traducir, no obstante creamos esta
> [encuesta](https://github.com/RustLangES/rust-book-es/discussions/29) para futuras
> revisiones.

### Definiendo un Trait

El comportamiento de un tipo consiste en los métodos que podemos llamar en ese
tipo. Diferentes tipos comparten el mismo comportamiento si podemos llamar los
mismos métodos en todos esos tipos. Las definiciones de traits son una manera
de agrupar firmas de métodos para definir un conjunto de comportamientos
necesarios para lograr algún propósito.

Por ejemplo, digamos que tenemos múltiples structs que contienen varios tipos y
cantidades de texto: un struct `NewsArticle` que contiene una historia de
noticias archivada en una ubicación particular y un `Tweet` que puede tener
como máximo 280 caracteres junto con metadatos que indican si es un nuevo
tweet, un retweet, o una respuesta a otro tweet.

Queremos hacer una biblioteca de agregación de medios llamada `aggregator` que
puede mostrar resúmenes de datos que podrían estar almacenados en una
instancia de `NewsArticle` o `Tweet`. Para hacer esto, necesitamos un resumen
de cada tipo, y solicitaremos ese resumen llamando un método `summarize` en
una instancia. El listado 10-12 muestra la definición de un trait `Summary`
público que expresa este comportamiento.

<Listing number="10-12" file-name="src/lib.rs" caption="Un trait `Summary` que consiste en el comportamiento proporcionado por un método `summarize`">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-12/src/lib.rs}}
```

</Listing>

Aquí, declaramos un trait usando la palabra clave `trait` y luego el nombre
del trait, que en este caso es `Summary`. También hemos declarado el trait
como `pub` para que los crates que dependen de este crate puedan hacer uso de
este trait también, como veremos en algunos ejemplos. Dentro de las llaves
curvas, declaramos las firmas de los métodos que describen los comportamientos
de los tipos que implementan este trait, que en este caso es `fn summarize
(&self) -> String`.

Después de la firma del método, en lugar de proporcionar una implementación
dentro de llaves curvas, usamos un punto y coma. Cada tipo que implementa este
trait debe proporcionar su propio comportamiento personalizado para el cuerpo
del método. El compilador hará cumplir que cualquier tipo que tenga el trait
`Summary` tendrá el método `summarize` definido con esta firma exactamente.

Un trait puede tener múltiples métodos en su cuerpo: las firmas de los métodos
se enumeran una por línea y cada línea termina en un punto y coma.

### Implementando un Trait en un Tipo

Ahora que hemos definido el trait `Summary`, podemos implementarlo en los
tipos en nuestro agregador de medios. El listado 10-13 muestra una
implementación del trait `Summary` en el struct `NewsArticle` que usa el
encabezado, el autor y la ubicación para crear el valor de retorno de
`summarize`. Para el struct `Tweet`, definimos `summarize` como el nombre de
usuario seguido del texto completo del tweet, asumiendo que el contenido del
tweet ya está limitado a 280 caracteres.

<Listing number="10-13" file-name="src/lib.rs" caption="Implementación del trait `Summary` en los tipos `NewsArticle` y `Tweet`">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-13/src/lib.rs:here}}
```

</Listing>

Implementar un trait en un tipo es similar a implementar métodos regulares.
La diferencia es que después de `impl`, ponemos el nombre del trait que
queremos implementar, luego usamos la palabra clave `for`, y luego
especificamos el nombre del tipo que queremos implementar el trait. Dentro del
bloque `impl`, ponemos las firmas de los métodos que la definición del trait ha
definido. En lugar de agregar un punto y coma después de cada firma, usamos
llaves y llenamos el cuerpo del método con el comportamiento específico que
queremos que los métodos del trait tengan para el tipo en particular.

Ahora que la biblioteca ha implementado el trait `Summary` en `NewsArticle` y
`Tweet`, los usuarios de la biblioteca pueden llamar a los métodos de trait
en las instancias de `NewsArticle` y `Tweet` en la misma forma en que llamamos
a los métodos regulares. La única diferencia es que el usuario debe traer el
trait al scope, así como los tipos. Aquí hay un ejemplo de cómo un crate
binario podría usar nuestra biblioteca de `aggregator`:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-01-calling-trait-method/src/main.rs}}
```

Este código imprime `1 new tweet: horse_ebooks: of course, as you probably already
know, people`.

Otros crates que dependen de nuestro crate `aggregator` pueden usar el trait
`Summary` en el ámbito para implementar `Summary` en sus propios tipos. Una
restricción a tener en cuenta es que podemos implementar un trait en un tipo
solo si ambos el trait o el tipo, o ambos, son local a nuestro crate. Por
ejemplo, podemos implementar traits de la biblioteca estándar como `Display`
en un tipo personalizado como `Tweet` como parte de nuestra funcionalidad de
crate `aggregator`, porque el tipo `Tweet` es local a nuestro crate
`aggregator`. También podemos implementar `Summary` en `Vec<T>` en nuestro
crate `aggregator`, porque el trait `Summary` es local a nuestro crate
`aggregator`.

Pero no podemos implementar traits externos en tipos externos. Por ejemplo,
digamos que queremos implementar `Display` en `Vec<T>` como parte de nuestra
funcionalidad de crate `aggregator`. Esto no es posible porque tanto `Display`
como `Vec<T>` están definidos en la biblioteca estándar y no son locales a
nuestro crate `aggregator`. La restricción de implementar un trait en un tipo
solo si uno de ellos es local a nuestro crate es parte de una propiedad
llamada _coherencia_, y más específicamente la _regla huérfana_, así llamada
porque el tipo padre no está presente. Esta regla asegura que el código de
otras personas no pueda romper su código y viceversa. Sin la regla, dos crates
podrían implementar el mismo trait para el mismo tipo, y Rust no sabría qué
implementación usar.

### Implementaciones predeterminadas

A veces es útil tener un comportamiento predeterminado para algunos o todos
los métodos en un trait en lugar de requerir implementaciones para todos los
métodos en cada tipo. Luego, a medida que implementamos el trait en un tipo
particular, podemos mantener o anular el comportamiento predeterminado para
cada método.

En el listado 10-14, especificamos un string predeterminado para el método
`summarize` del trait `Summary` en lugar de solo definir la firma del método,
como hicimos en el listado 10-12.

<Listing number="10-14" file-name="src/lib.rs" caption="Definición de un trait `Summary` con un valor predeterminado implementado del método `summarize`">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-14/src/lib.rs:here}}
```

</Listing>

Para usar una implementación predeterminada para resumir instancias de
`NewsArticle`, especificamos un bloque `impl` vacío con `impl Summary for
NewsArticle {}`.

Aunque ya no estamos definiendo el método `summarize` en `NewsArticle`
directamente, hemos proporcionado una implementación predeterminada y
especificado que `NewsArticle` implementa el trait `Summary`. Como resultado,
todavía podemos llamar al método `summarize` en una instancia de `NewsArticle`,
como esto:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-02-calling-default-impl/src/main.rs:here}}
```

Este código imprime `New article available! (Read more...)`.

Crear una implementación predeterminada no requiere que cambiemos nada sobre
la implementación de `Summary` en `Tweet` en el listado 10-13. La razón es que
la sintaxis para anular una implementación predeterminada es la misma que la
sintaxis para implementar un método de trait que no tiene una implementación
predeterminada.

Las implementaciones predeterminadas pueden llamar otros métodos en el mismo
trait, incluso si esos métodos no tienen una implementación predeterminada.
De esta manera, un trait puede proporcionar una gran cantidad de
funcionalidad útil y solo requiere que los implementadores especifiquen una
pequeña parte de ella. Por ejemplo, podríamos definir el trait `Summary` para
tener un método `summarize_author` cuya implementación es requerida, y luego
definir un método `summarize` que tenga una implementación predeterminada que
llame al método `summarize_author`:

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:here}}
```

Para usar esta version de `Summary`, solo necesitamos definir `summarize_author`
cuando implementamos el trait en un tipo:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:impl}}
```

Después de definir `summarize_author`, podemos llamar a `summarize` en
instancias de la estructura `Tweet`, y la implementación predeterminada de
`summarize` llamará a la definición de `summarize_author` que hemos
proporcionado. Debido a que hemos implementado `summarize_author`, el trait
`Summary` nos ha dado el comportamiento del método `summarize` sin requerirnos
escribir más código.

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/main.rs:here}}
```

Este código imprime `1 new tweet: (Read more from @horse_ebooks...)`.

Ten en cuenta que no es posible llamar a la implementación predeterminada desde
una implementación primordial de ese mismo método.

### Traits como parametros

Ahora que sabes cómo definir y implementar traits, podemos explorar cómo usar
traits para definir funciones que aceptan muchos tipos diferentes. Usaremos el
trait `Summary` que implementamos en los tipos `NewsArticle` y `Tweet` en el
listado 10-13 para definir una función `notify` que llama al método `summarize`
en su parámetro `item`, que es de algún tipo que implementa el trait `Summary`.
Para hacer esto, usamos la sintaxis `impl Trait`, como esto:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-04-traits-as-parameters/src/lib.rs:here}}
```

En lugar de un tipo concreto para el parámetro `item`, especificamos el
parámetro `impl` y el nombre del trait. Cualquier tipo que implemente el trait
`Summary` puede ser pasado al parámetro `item` en la función `notify`. El
código que llama a la función `notify` con cualquier otro tipo, como un `String`
o un `i32`, no compilará porque esos tipos no implementan `Summary`.

<!-- Old headings. Do not remove or links may break. -->


<a id="fixing-the-largest-function-with-trait-bounds"></a>

#### Sintaxis de trait bound

La sintaxis `impl Trait` funciona para casos sencillos, pero en realidad es
azúcar sintáctico para una forma más larga conocida como _trait bound_; se ve
así:

```rust,ignore
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

Esta forma más larga es equivalente al ejemplo en la sección anterior pero más
detallado. Colocamos los límites de los traits con la declaración del parámetro
generic después de dos puntos y dentro de corchetes angulares.

La sintaxis `impl Trait` es conveniente y hace que el código sea más conciso en
casos simples, mientras que la sintaxis de trait bound más completa puede
expresar más complejidad en otros casos. Por ejemplo, podemos tener dos
parámetros que implementan `Summary`. Hacerlo con la sintaxis `impl Trait` se
ve así:

```rust,ignore
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

Usando `impl Trait` es apropiado si queremos que esta función permita que
`item1` y `item2` tengan tipos diferentes (siempre que ambos tipos implementen
`Summary`). Sin embargo, si queremos forzar que ambos parámetros tengan el
mismo tipo, debemos usar un trait bound, como esto:

```rust,ignore
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

El tipo generic `T` especificado como el tipo de los parámetros `item1` e
`item2` restringe la función de tal manera que el tipo concreto del valor
pasado como argumento para `item1` e `item2` debe ser el mismo.

#### Especificando múltiples trait bounds con la sintaxis `+`

También podemos especificar más de un trait bound. Digamos que queremos que
`notify` use la representación de cadena de un tipo que implementa `Summary`
en el cuerpo de la función. Para hacer esto, necesitamos que el parámetro
`item` implemente tanto `Display` como `Summary`. Podemos hacerlo usando la
sintaxis `+`:

```rust,ignore
pub fn notify(item: &(impl Summary + Display)) {
```

La sintaxis `+` también es válida con los trait bounds en tipos generics:

```rust,ignore
pub fn notify<T: Summary + Display>(item: &T) {
```

Con los dos trait bounds especificados, el cuerpo de `notify` puede llamar a
`summarize` y usar `{}` para formatear `item`.

#### Trait bounds más claros con cláusulas `where`

Usar demasiados trait bounds tiene sus inconvenientes. Cada generic tiene sus
propios trait bounds, por lo que las funciones con múltiples parámetros de tipo
generic pueden contener mucha información de trait bound entre el nombre de la
función y su lista de parámetros, lo que hace que la firma de la función sea
difícil de leer. Por esta razón, Rust tiene una sintaxis alternativa para
especificar los trait bounds dentro de una cláusula `where` después de la
firma de la función. Así que en lugar de escribir esto:

```rust,ignore
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

podemos usar una cláusula `where`, como esta:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-07-where-clause/src/lib.rs:here}}
```

La firma de esta función está menos desordenada: el nombre de la función, la
lista de parámetros y el tipo de retorno están muy juntos, similar a una función
sin muchos trait bounds.

### Devolviendo tipos que implementan traits

También podemos usar la sintaxis `impl Trait` en el tipo de retorno de una
función para devolver un valor de algún tipo que implementa un trait, como se
muestra aquí:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-05-returning-impl-trait/src/lib.rs:here}}
```

Al usar `impl Summary` para el tipo de retorno, especificamos que la función
`returns_summarizable` devuelve algún tipo que implementa el trait `Summary`
sin nombrar el tipo concreto. En este caso, `returns_summarizable` devuelve un
`Tweet`, pero el código que llama a esta función no necesita saber eso.

La capacidad de especificar un tipo que es una implementación de un trait
especialmente útil en el contexto de los closures y los iteradores, que
cubriremos en el Capítulo 13. Los closures y los iteradores crean tipos que
solo el compilador conoce o tipos que son muy largos de especificar. La sintaxis
`impl Trait` te permite especificar de manera concisa que una función devuelve
algún tipo que implementa el trait `Iterator` sin necesidad de escribir un tipo
muy largo.

Sin embargo, no puedes usar `impl Trait` si la función devuelve más de un tipo.
Por ejemplo, este código que devuelve un `NewsArticle` o un `Tweet` con el tipo
de retorno especificado como `impl Summary` no compilaría:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-06-impl-trait-returns-one-type/src/lib.rs:here}}
```

Devolver un `NewsArticle` o un `Tweet` no está permitido debido a las
restricciones en torno a cómo se implementa la sintaxis `impl Trait` en el
compilador. Cubriremos cómo escribir una función con este comportamiento en la
sección ["Usando objetos trait que permiten valores de diferentes
tipos"][usando-trait-objects-que-permiten-valores-de-diferentes-tipos]<!--
ignore --> del Capítulo 18.

### Usando trait bounds para implementar métodos condicionalmente

Al usar un trait bound con un bloque `impl` que usa parámetros de tipo generic,
podemos implementar métodos condicionalmente para tipos que implementan los
traits especificados. Por ejemplo, el tipo `Pair<T>` en el listado 10-15 siempre
implementa la función `new` para devolver una nueva instancia de `Pair<T>`
(recuerda de la sección ["Definiendo métodos"][methods]<!-- ignore --> del
Capítulo 5 que `Self` es un alias de tipo para el tipo del bloque `impl`, que en
este caso es `Pair<T>`). Pero en el siguiente bloque `impl`, `Pair<T>` solo
implementa el método `cmp_display` si su tipo interno `T` implementa el trait
`PartialOrd` que permite la comparación _y_ el trait `Display` que permite la
impresión.

<Listing number="10-15" file-name="src/lib.rs" caption="Implementación condicional de métodos en un tipo generic dependiendo de los trait bounds">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-15/src/lib.rs}}
```

</Listing>

También podemos implementar condicionalmente un trait para cualquier tipo que
implemente otro trait. Implementaciones de un trait en cualquier tipo que
satisfaga los trait bounds se llaman _implementaciones blanket_ y son usados
extensivamente en la biblioteca estándar de Rust. Por ejemplo, la biblioteca
estándar implementa el trait `ToString` en cualquier tipo que implemente el
trait `Display`. El bloque `impl` en la biblioteca estándar se ve similar a este
código:

```rust,ignore
impl<T: Display> ToString for T {
    // --snip--
}
```

Debido a que la biblioteca estándar tiene esta implementación, podemos llamar al
método `to_string` definido por el trait `ToString` en cualquier tipo que
implemente el trait `Display`. Por ejemplo, podemos convertir enteros en sus
valores `String` correspondientes de esta manera porque los enteros implementan
`Display`:

```rust
let s = 3.to_string();
```

Las implementaciones generales aparecen en la documentación del trait en la
sección "Implementors".

Traits y trait bounds nos permiten usar genéricos para reducir la duplicación de
código, pero también para especificar a el compilador que queremos que un tipo
generic tenga un comportamiento particular. El compilador puede usar la
información de los trait bounds para verificar que todos los tipos concretos que
usamos con nuestro código proporcionan el comportamiento correcto. En lenguajes
de tipado dinámico, obtendríamos un error en tiempo de ejecución si llamamos a
un método en un tipo que no define el método. Pero Rust mueve estos errores al
tiempo de compilación, por lo que estamos obligados a corregir los problemas
antes de que nuestro código pueda ejecutarse. Además, no tenemos que escribir
código que verifique el comportamiento en tiempo de ejecución porque ya hemos
verificado en tiempo de compilación. Hacerlo mejora el rendimiento sin tener que
renunciar a la flexibilidad de los generics.

[usando-trait-objects-que-permiten-valores-de-diferentes-tipos]: ch18-02-trait-objects.html#usando-trait-objects-que-permiten-valores-de-diferentes-tipos
[methods]: ch05-03-method-syntax.html#definiendo-metodos
