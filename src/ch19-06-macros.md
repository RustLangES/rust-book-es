## Macros

Hemos utilizado macros como `println!` a lo largo de este libro, pero no hemos
explorado completamente qué es una macro y cómo funciona. El término *macro* se
refiere a una familia de características en Rust: macros *declarativas* con
`macro_rules!` y tres tipos de macros *procedurales*:

* Macros Personalizadas `#[derive]` que especifican código agregado con el 
* atributo `derive` usado en structs y enums
* Macros similares a atributos que definen atributos personalizados utilizables
  en cualquier item
* Macros similares a funciones que se ven como llamadas a funciones, pero
  operan en los tokens especificados como argumento

Hablaremos de cada uno de estos a su vez, pero primero, veamos por qué
necesitamos macros cuando ya tenemos funciones.

### La Diferencia Entre Macros y Funciones

Fundamentalmente, las macros son una forma de escribir código que escribe otro
código, lo que se conoce como *metaprogramación*. En el Apéndice C, discutimos
el atributo `derive`, que genera una implementación de varios traits para ti.
También hemos usado las macros `println!` y `vec!` a lo largo del libro. Todas
estas macros *se expanden* para producir más código que el código que has
escrito manualmente.

La metaprogramación es útil para reducir la cantidad de código que tienes que
escribir y mantener, que también es uno de los roles de las funciones. Sin
embargo, las macros tienen algunos poderes adicionales que las funciones no
tienen.

Una función debe declarar el número y el tipo de parámetros que tiene la
función. Las macros, por otro lado, pueden tomar un número variable de
parámetros: podemos llamar a `println!("hello")` con un argumento o
`println!("hello {}", name)` con dos argumentos. Además, las macros se
expanden antes de que el compilador interprete el significado del código, por
lo que una macro puede, por ejemplo, implementar un trait en un tipo dado. Una
función no puede, porque se llama en tiempo de ejecución y un trait debe
implementarse en tiempo de compilación.

La desventaja de implementar una macro en lugar de una función es que las
definiciones de macros son más complejas que las definiciones de funciones
porque estás escribiendo código Rust que escribe código Rust. Debido a esta
indirección, las definiciones de macros generalmente son más difíciles de leer,
entender y mantener que las definiciones de funciones.

Otra diferencia importante entre las macros y las funciones es que debes
definir macros o traerlas al scope *antes* de llamarlas en un archivo, a
diferencia de las funciones que puedes definir en cualquier lugar y llamar en
cualquier lugar.

### Macros Declarativas con `macro_rules!` para Metaprogramación General

La forma más utilizada de macros en Rust es la *macro declarativa*. A veces
también se denominan “macros por ejemplo”, “`macro_rules!` macros” o simplemente
“macros”. En su núcleo, las macros declarativas te permiten escribir algo
similar a una expresión `match` de Rust. Como se discutió en el Capítulo 6, las
expresiones `match` son estructuras de control que toman una expresión,
comparan el valor resultante de la expresión con patrones y luego ejecutan el
código asociado con el patrón coincidente. Las macros también comparan un valor
con patrones que están asociados con un código particular: en esta situación,
el valor es el código fuente literal de Rust que se pasa a la macro; los
patrones se comparan con la estructura de ese código fuente; y el código
asociado con cada patrón, cuando coincide, reemplaza el código pasado a la
macro. Todo esto sucede durante la compilación.

Para definir una macro, usas el constructor `macro_rules!`. Exploremos cómo
usar `macro_rules!` mirando cómo se define la macro `vec!`. El Capítulo 8
cubrió cómo podemos usar la macro `vec!` para crear un nuevo vector con valores
particulares. Por ejemplo, la siguiente macro crea un nuevo vector que contiene
tres enteros:

```rust
let v: Vec<u32> = vec![1, 2, 3];
```

También podemos usar la macro `vec!` para crear un vector de dos enteros o un
vector de cinco string slices. No podríamos usar una función porque no 
conoceríamos el número o el tipo de valores.

El Listado 19-28 muestra una definición ligeramente simplificada de la macro
`vec!`.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-28/src/lib.rs}}
```

<span class="caption">Listing 19-28: Una versión simplificada de la definición 
de la macro `vec!`</span>

> Nota: La definición real de la macro `vec!` en la biblioteca estándar incluye
> código para preasignar la cantidad correcta de memoria por adelantado. Ese
> código es una optimización que no incluimos aquí para hacer el ejemplo más
> simple.

La anotación `#[macro_export]` indica que esta macro debe estar disponible
siempre que la biblioteca en la que se define la macro se traiga al scope. Sin
esta anotación, la macro no se puede traer al scope.

Luego comenzamos la definición de la macro con `macro_rules!` y el nombre de la
macro que estamos definiendo *sin* el signo de exclamación. El nombre, en este
caso `vec`, va seguido de llaves que denotan el cuerpo de la definición de la
macro.

La estructura en el cuerpo de `vec!` es similar a la estructura de una
expresión `match`. Aquí tenemos un brazo con el patrón `( $( $x:expr ),* )`,
seguido de `=>` y el bloque de código asociado con este patrón. Si el patrón
coincide, se emitirá el bloque de código asociado. Dado que este es el único
patrón en esta macro, solo hay una forma válida de coincidir; cualquier otro
patrón dará como resultado un error. Las macros más complejas tendrán más de una
opción.

La sintaxis válida del pattern en una macro es diferente de la sintaxis de los
patterns cubiertos en el Capítulo 18 porque los patterns de macro se comparan
con la estructura del código Rust en lugar de con valores. Recorramos lo que
significan las piezas del pattern en el Listado 19-28; para obtener la sintaxis
completa del pattern de macro, consulte la [Referencia de Rust][ref].

Primero, usamos un conjunto de paréntesis para englobar todo el patrón. Usamos
el signo de dólar (`$`) para declarar una variable en el sistema de macros que
contendrá el código Rust que coincida con el patrón. El signo de dólar hace que
quede claro que esta es una variable de macro en lugar de una variable regular 
de Rust. A continuación, viene un conjunto de paréntesis que captura los valores
que coinciden con el patrón dentro de los paréntesis para su uso en el código de
reemplazo. Dentro de `$()` está `$x:expr`, que coincide con cualquier expresión
de Rust y le da el nombre `$x`.

La coma que sigue a `$()` índica que opcionalmente podría aparecer un carácter
de coma separador literal después del código que coincide con el código en
`$()`. Él `*` especifica que el patrón coincide cero o más veces con lo que
precede al `*`.

Cuando llamamos a esta macro con `vec![1, 2, 3];`, el patrón `$x` coincide tres
veces con las tres expresiones `1`, `2` y `3`.

Ahora veamos el pattern en el cuerpo del código asociado con esta opción:
`temp_vec.push($x);`. Dentro de `$()*` se genera para cada parte que coincide
con `$()` en el patrón cero o más veces dependiendo de cuántas veces coincida
el patrón. Él `$x` se reemplaza con cada expresión que coincida. Cuando llamamos
a esta macro con `vec![1, 2, 3];`, el código generado que reemplaza esta
llamada a la macro será el siguiente:

```rust,ignore
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

Hemos definido una macro que puede tomar cualquier número de argumentos de
cualquier tipo y puede generar código para crear un vector que contenga los
elementos especificados.

Para obtener más información sobre cómo escribir macros, consulta la 
documentación en línea u otros recursos, como [“The Little Book of Rust
Macros”][tlborm] iniciado por Daniel Keep y continuado por Lukas Wirth.

### Macros Procedurales para Generar Código a partir de Atributos

Las macros procedurales, que es la segunda forma de macros, actúan más como
una función (y son un tipo de procedimiento). Las macros procedurales aceptan
código como entrada, operan en ese código y producen código como salida en
lugar de coincidir con patrones y reemplazar el código por otro código como lo
hacen las macros declarativas. Los tres tipos de macros procedurales son
derivaciones personalizadas, atributos y funciones, y todas funcionan de manera
similar.

Al crear macros procedurales, las definiciones deben residir en su propio
crate con un tipo de crate especial. Esto se debe a razones técnicas complejas
que esperamos eliminar en el futuro. En el Listado 19-29 se muestra cómo se 
define una macro procedural, donde `some_attribute` es un marcador de posición
para usar un tipo específico de macro.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```

<span class="caption">Listing 19-29: Un ejemplo de definición de una macro
procedural</span>

La función que define una macro procedural tome un `TokenStream` como entrada y
devuelve un `TokenStream` como salida. El `TokenStream` tipo es definido por el
`proc_macro` crate que se incluye con Rust y representa una secuencia de tokens.
Esta es la base de la macro: el código fuente en el que la macro está operando
constituye la entrada `TokenStream`, y el código que la macro produce es el
`TokenStream` de salida. La función también tiene un atributo adjunto que
especifica qué tipo de macro procedural estamos creando. Podemos tener varios
tipos de macros procedurales en el mismo crate.

Veamos los diferentes tipos de macros procedurales. Comenzaremos con una
derivación personalizada y luego explicaremos las pequeñas diferencias que
hacen que las otras formas sean diferentes.

### Cómo Escribir una Macro `derive` Personalizada

Creemos un crate llamado `hello_macro` que defina un trait llamado
`HelloMacro` con una función asociada llamada `hello_macro`. En lugar de hacer
que nuestros usuarios implementen el trait `HelloMacro` para cada uno de sus
tipos, proporcionaremos una macro procedural para que los usuarios puedan
anotar sus tipos con `#[derive(HelloMacro)]` para obtener una implementación
predeterminada de la función `hello_macro`. La implementación predeterminada
imprimirá `Hello, Macro! My name is TypeName!`, donde `TypeName` es el nombre
del tipo en el que se ha definido este trait. En otras palabras, escribiremos
un crate que permita a otro programador escribir código como el Listado 19-30
usando nuestro crate.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-30/src/main.rs}}
```

<span class="caption">Listing 19-30: El código que un usuario de nuestro crate
podrá escribir cunado se use nuestra macro procedural</span>

Este código imprimirá `Hello, Macro! My name is Pancakes!` cuando hayamos
terminado. El primer paso es hacer un nuevo crate de biblioteca, así:

```console
$ cargo new hello_macro --lib
```

A continuación, definiremos el trait `HelloMacro` y su función asociada:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-20-impl-hellomacro-for-pancakes/hello_macro/src/lib.rs}}
```

Tenemos un trait y su función. En este punto, nuestro usuario de crate podría
implementar el trait para lograr la funcionalidad deseada, así:

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-20-impl-hellomacro-for-pancakes/pancakes/src/main.rs}}
```

Sin embargo, tendrían que escribir el bloque de implementación para cada tipo
que quisieran usar con `hello_macro`; queremos evitar que tengan que hacer este
trabajo.

Además, aún no podemos proporcionar una implementación predeterminada de
la función `hello_macro` que imprimirá el nombre del tipo en el que se
implementa el trait: rust no tiene capacidades de reflexión, por lo que no
puede buscar el nombre del tipo en tiempo de ejecución. Necesitamos una macro
para generar código en tiempo de compilación.

El siguiente paso es definir la macro procedural. En el momento de escribir
esto, las macros procedurales deben estar en su propio crate. Eventualmente,
esta restricción podría ser levantada. La convención para estructurar crates y
macro crates es la siguiente: para un crate llamado `foo`, un crate de macro
procedural de derivación personalizada se llama `foo_derive`. Creemos un nuevo
crate llamado `hello_macro_derive` dentro de nuestro proyecto `hello_macro`:

```console
$ cargo new hello_macro_derive --lib
```

Nuestros dos crates están estrechamente relacionados, por lo que creamos el
crate de macro procedural dentro del directorio de nuestro crate `hello_macro`.
Si cambiamos la definición del trait en `hello_macro`, también tendremos que
cambiar la implementación de la macro procedural en `hello_macro_derive`. Los
dos crates deberán publicarse por separado, y los programadores que usen estos
crates deberán agregar ambos como dependencias y traerlos a ambos al scope.
En su lugar, podríamos hacer que el crate `hello_macro` use `hello_macro_derive`
como una dependencia y vuelva a exportar el código de la macro procedural.
Sin embargo, la forma en que hemos estructurado el proyecto hace posible que
los programadores usen `hello_macro` incluso si no quieren la funcionalidad
`derive`.

Necesitamos declarar el crate `hello_macro_derive` como un crate de macro
procedural. También necesitaremos funcionalidad de los crates `syn` y `quote`,
como veremos en un momento, por lo que necesitamos agregarlos como dependencias.
Agrega lo siguiente al archivo *Cargo.toml* para `hello_macro_derive`:

<span class="filename">Filename: hello_macro_derive/Cargo.toml</span>

```toml
{{#include ../listings/ch19-advanced-features/listing-19-31/hello_macro/hello_macro_derive/Cargo.toml:6:12}}
```

Para comenzar a definir la macro procedural, coloca el código del Listado 19-31
en tu archivo *src/lib.rs* para el crate `hello_macro_derive`. Ten en cuenta que
este código no se compilará hasta que agreguemos una definición para la función
`impl_hello_macro`.

<span class="filename">Filename: hello_macro_derive/src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-31/hello_macro/hello_macro_derive/src/lib.rs}}
```

<span class="caption">Listing 19-31: Código que la mayoría de los crates de
macros procedurales requerirán para procesar código Rust</span>

Observa que hemos dividido el código en la función `hello_macro_derive`, que
es responsable de analizar el `TokenStream`, y la función `impl_hello_macro`,
que es responsable de transformar el árbol de sintaxis: esto hace que escribir
una macro procedural sea más conveniente. El código en la función externa
(`hello_macro_derive` en este caso) será el mismo para casi todos los crates de
macros procedurales que veas o crees. El código que especifiques en el cuerpo
de la función interna (`impl_hello_macro` en este caso) será diferente
dependiendo del propósito de tu macro procedural.

Hemos introducido tres nuevos crates: `proc_macro`, [`syn`], y [`quote`]. El
crate `proc_macro` viene con Rust, por lo que no necesitamos agregarlo a las
dependencias en *Cargo.toml*. El crate `proc_macro` es la API del compilador
que nos permite leer y manipular código Rust desde nuestro código.

El crate `syn` analiza el código Rust desde un string en una estructura de
datos en la que podemos realizar operaciones. El crate `quote` convierte las
estructuras de datos de `syn` nuevamente en código Rust. Estos crates hacen que
sea mucho más simple analizar cualquier tipo de código Rust que deseemos
manipular: escribir un analizador completo para el código Rust no es una tarea
sencilla.

La función `hello_macro_derive` se llamará cuando un usuario de nuestro crate
especifique `#[derive(HelloMacro)]` en un tipo. Esto es posible porque hemos
anotado la función `hello_macro_derive` aquí con `proc_macro_derive` y
especificado el nombre `HelloMacro`, que coincide con el nombre de nuestro
trait; esta es la convención que siguen la mayoría de las macros procedurales.

La función `hello_macro_derive` convierte primero el `input` de un
`TokenStream` a una estructura de datos que podemos interpretar y realizar
operaciones. Aquí es donde entra en juego `syn`. La función `parse` en `syn`
toma un `TokenStream` y devuelve un struct `DeriveInput` que representa el
código Rust analizado. El Listado 19-32 muestra las partes relevantes del
struct `DeriveInput` que obtenemos al analizar el string `struct Pancakes;`:

```rust,ignore
DeriveInput {
    // --snip--

    ident: Ident {
        ident: "Pancakes",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}
```

<span class="caption">Listing 19-32: La instancia `DeriveInput` que obtenemos
al analizar el código que tiene el atributo de la macro en el 
Listado 19-30</span>

Los campos de este struct muestran que el código Rust que hemos analizado es un
struct unitario con el `ident` (identificador, es decir, el nombre) de
`Pancakes`. Hay más campos en este struct para describir todo tipo de código
Rust; consulta la [documentación de `syn` para `DeriveInput`][syn-docs] para
obtener más información.

Pronto definiremos la función `impl_hello_macro`, que es donde construiremos el
código Rust que queremos incluir. Pero antes de hacerlo, ten en cuenta que la
salida de nuestra macro derive también es un `TokenStream`. El `TokenStream`
devuelto se agrega al código que escriben los usuarios de nuestro crate, por lo
que cuando compilan su crate, obtendrán la funcionalidad adicional que
proporcionamos en el `TokenStream` modificado.

Es posible que hayas notado que estamos usando `unwrap` para hacer que la 
función `hello_macro_derive` genere un panic si la llamada a la función 
`syn::parse` falla. Es necesario que nuestra macro procedural genere un panic
en caso de error porque las funciones `proc_macro_derive` deben devolver
`TokenStream` en lugar de `Result` para cumplir con la API de las macros
procedurales. Hemos simplificado este ejemplo usando `unwrap`; en código de
producción, debes proporcionar mensajes de error más específicos sobre lo que
salió mal usando `panic!` o `expect`.

Ahora que tenemos el código para convertir el código de Rust anotado de un
`TokenStream` a una instancia `DeriveInput`, generemos el código que implementa
el trait `HelloMacro` en el tipo anotado, como se muestra en el Listado 19-33.

<span class="filename">Filename: hello_macro_derive/src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-33/hello_macro/hello_macro_derive/src/lib.rs:here}}
```

<span class="caption">Listing 19-33: Implementando el trait `HelloMacro` usando
el código Rust analizado</span>

Obtenemos una instancia del struct `DeriveInput` que contiene el nombre
(identificador) del tipo anotado usando `ast.ident`. El struct en el Listado
19-32 muestra que cuando ejecutamos la función `impl_hello_macro` en el código
del Listado 19-30, el `ident` que obtenemos tendrá el campo `ident` con un
valor de `"Pancakes"`. Por lo tanto, la variable `name` en el Listado 19-33
contendrá una instancia del struct `Ident` que, cuando se imprima, será la
cadena `"Pancakes"`, el nombre del struct en el Listado 19-30.

La macro `quote!` nos permite construir el código que queremos devolver. El
compilador espera algo diferente al resultado directo de la ejecución de la 
macro `quote!`, por lo que debemos convertirlo a un `TokenStream`. Hacemos esto
llamando al método `into`, que consume esta representación intermedia y
devuelve un valor del tipo `TokenStream` requerido.

La macro `quote!` también proporciona algunas mecánicas de plantillas muy
interesantes: podemos ingresar `#name`, y `quote!` lo reemplazará con el valor
de la variable `name`. Incluso puedes hacer alguna repetición similar a la forma
en que funcionan las macros regulares. Consulta [la documentación del crate
`quote`][quote-docs] para obtener una introducción completa.

Queremos que nuestra macro procedural genere una implementación de nuestro 
trait `HelloMacro` para el tipo que el usuario ha anotado, lo cual podemos 
lograr utilizando `#name`. La implementación del trait tiene la función
`hello_macro`, cuyo cuerpo contiene la funcionalidad que queremos proporcionar:
imprimir `Hello, Macro! My name is` y luego el nombre del tipo anotado.

La macro `stringify!` utilizada aquí está incorporada en Rust. Toma una
expresión de Rust como `1 + 2` y en tiempo de compilación convierte la
expresión en un literal de string como `"1 + 2"`. Esto es diferente a `format!`
o `println!`, macros que evalúan la expresión y luego convierten el resultado en
un `String`. Existe la posibilidad de que la entrada `#name` sea una expresión
para imprimir literalmente, por lo que usamos `stringify!`. El uso de
`stringify!` también ahorra una asignación al convertir `#name` en un literal
de string en tiempo de compilación.

En este punto, `cargo build` debería completarse correctamente tanto en
`hello_macro` como en `hello_macro_derive`. ¡Conectemos estos crates al código
del Listado 19-30 para ver la macro procedural en acción! Crea un nuevo
proyecto binario en tu directorio *projects* usando `cargo new pancakes`.
Necesitamos agregar `hello_macro` y `hello_macro_derive` como dependencias en el
*Cargo.toml* de `pancakes`. Si estás publicando tus versiones de `hello_macro`
y `hello_macro_derive` en [crates.io](https://crates.io/), serían dependencias
regulares; si no, puedes especificarlas como dependencias `path` de la
siguiente manera:

```toml
{{#include ../listings/ch19-advanced-features/no-listing-21-pancakes/pancakes/Cargo.toml:7:9}}
```

Coloca el código del Listado 19-30 en *src/main.rs* y ejecuta `cargo run`:
debería imprimir `Hello, Macro! My name is Pancakes!` La implementación del
trait `HelloMacro` de la macro procedural se incluyó sin que el crate `pancakes`
tuviera que implementarlo; la macro `#[derive(HelloMacro)]` agregó la
implementación del trait.

A continuación, vamos a explorar cómo los otros tipos de macros procedurales
difieren de las macros derive personalizadas.

### Macros similares a atributos

Las macros similares a atributos son similares a las macros derivadas 
personalizadas, pero en lugar de generar código para el atributo `derive`,
permiten crear nuevos atributos. También son más flexibles: `derive` solo
funciona para structs y enums; los atributos se pueden aplicar a otros items
también, como funciones. Aquí hay un ejemplo de uso de una macro similar a un
atributo: digamos que tienes un atributo llamado `route` que anota funciones
cuando se usa un framework de aplicación web:

```rust,ignore
#[route(GET, "/")]
fn index() {
```

El atributo `#[route]` será definido por el framework como una macro procedural.
La firma de la función de definición de la macro se vería así:

```rust,ignore
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```

Aquí, tenemos dos parámetros de tipo `TokenStream`. El primero es para el
contenido del atributo: la parte `GET, "/"`. El segundo es el cuerpo del item
al que se adjunta el atributo: en este caso, `fn index() {}` y el resto del
cuerpo de la función.

Aparte de eso, las macros similares a atributos funcionan de la misma manera
que las macros derivadas personalizadas: creas un crate con el tipo `proc-macro`
y defines una función que genera el código que deseas.

### Macros similares a funciones

Las macros tipo función definen macros que se ven como llamadas a funciones.
De manera similar a las macros `macro_rules!`, son más flexibles que las
funciones; por ejemplo, pueden tomar un número desconocido de argumentos. Sin
embargo, las macros `macro_rules!` solo se pueden definir usando la sintaxis
similar a la de los patterns que discutimos en la sección [“Macros declarativas
con `macro_rules!` para metaprogramación general”][decl]<!-- ignore -->
anteriormente. Las macros tipo función toman un parámetro `TokenStream` y su
definición manipula ese `TokenStream` usando código Rust como los otros dos
tipos de macros procedurales. Un ejemplo de una macro tipo función es una macro
`sql!` que podría ser llamada así:

```rust,ignore
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

Esta macro analizaría la declaración SQL dentro de ella y verificaría que sea
sintácticamente correcta, lo cual es un procesamiento mucho más complejo de lo 
que una macro `macro_rules!` puede hacer. La macro `sql!` se definiría así:

```rust,ignore
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```

Esta definición es similar a la firma de la macro de derivación personalizada:
recibimos los tokens que están dentro de los paréntesis y devolvemos el código
que queremos generar.

## Resumen

¡Uf! Ahora que tienes algunas características de Rust en tu caja de herramientas
que probablemente no usarás a menudo, pero sabrás que están disponibles en
circunstancias muy particulares. Hemos introducido varios temas complejos para
que cuando los encuentres en sugerencias de mensajes de error o en el código de
otras personas, puedas reconocer estos conceptos y sintaxis. Usa este capítulo
como referencia para guiarte hacia soluciones.

¡A continuación, pondremos en práctica todo lo que hemos discutido a lo largo
del libro y haremos un proyecto más!

[ref]: https://doc.rust-lang.org/reference/macros-by-example.html
[tlborm]: https://veykril.github.io/tlborm/
[`syn`]: https://crates.io/crates/syn
[`quote`]: https://crates.io/crates/quote
[syn-docs]: https://docs.rs/syn/1.0/syn/struct.DeriveInput.html
[quote-docs]: https://docs.rs/quote
[decl]: #declarative-macros-with-macro_rules-for-general-metaprogramming
