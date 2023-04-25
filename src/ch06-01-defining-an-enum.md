## Definiendo un Enum

Los structs te permiten agrupar campos relacionados y datos, como un `Rectangulo`
con su `ancho` y `largo`. Por otro lado, las enumeraciones te permiten decir que
un valor es uno de un conjunto de posibles valores. Por ejemplo, podríamos querer
decir que `Rectangulo` es uno de un conjunto de posibles formas que también
incluye `Circulo` y `Triangulo`. Para hacer esto, Rust nos permite codificar estas
posibilidades como una enumeración.

Vamos a ver una situación que podemos expresar en código y veremos por qué
los enums son útiles y más apropiados que los structs en este caso. Digamos
que tenemos que trabajar con direcciones IP. Actualmente, dos estándares
son los que se usan para direcciones IP: la versión cuatro y la versión seis.
Como estos son los únicos posibles tipos de direcciones IP que nuestro
programa encontrará, podemos *enumerar* todas las variantes posibles, de
donde viene el nombre de enumeración.

Cualquier dirección IP puede ser una dirección de la versión cuatro o la versión
seis, pero no ambas al mismo tiempo. Esa propiedad de las direcciones IP hace
que la estructura de datos enum sea apropiada porque un valor enum puede ser
sólo una de sus variantes. Tanto la versión cuatro como la versión seis
direcciones son todavía fundamentalmente direcciones IP, por lo que deben ser
tratadas como el mismo tipo cuando el código está manejando situaciones que se
aplican a cualquier tipo de dirección IP.

Podemos expresar este concepto en código definiendo una enumeración `IpAddrKind`
y enumerando los posibles tipos de direcciones IP, `V4` y `V6`. Estas son las
variantes del enum:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:def}}
```

`IpAddrKind` ahora es un tipo de datos personalizado que podemos usar en otras
partes de nuestro código.

### Valores Enum

Podemos crear instancias de cada una de las dos variantes de `IpAddrKind` de
esta manera:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:instance}}
```

Nota que las variantes del enum están en el mismo espacio de nombres bajo su
identificador, y usamos dos puntos para separar los dos. Esto es útil porque
ahora ambos valores `IpAddrKind::V4` e `IpAddrKind::V6` son del mismo tipo:
`IpAddrKind`. Podemos entonces, por ejemplo, definir una función que tome
cualquier `IpAddrKind`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn}}
```

Y podemos llamar a esta función con cualquiera de las variantes:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn_call}}
```

Usando enums tiene aún más ventajas. Pensando más en nuestro tipo de dirección
IP, en este momento no tenemos una forma de almacenar los datos reales de la
dirección IP; solo sabemos qué tipo es. Dado que acabas de aprender sobre los
structs en el Capítulo 5, podrías estar tentado a abordar este problema con
structs como se muestra en el Listing 6-1.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-01/src/main.rs:here}}
```

<span class="caption">Listing 6-1: Almacenando los datos y la variante 
`IpAddrKind` de una dirección IP usando un `struct`</span>

Aquí, hemos definido un struct `IpAddr` que tiene dos campos: un campo `kind`
que es de tipo `IpAddrKind` (el enum que definimos anteriormente) y un campo
`address` de tipo `String`. Tenemos dos instancias de este struct. La primera
es `home`, y tiene el valor `IpAddrKind::V4` como su `kind` como datos de
dirección asociados de `127.0.0.1`. La segunda instancia es `loopback`. Tiene
la otra variante de `IpAddrKind` como su valor `kind`, `V6`, y tiene la
dirección `::1` asociada con ella. Hemos usado un struct para agrupar los
valores `kind` y `address` juntos, así que ahora la variante está asociada con
el valor.

Sin embargo, representar el mismo concepto usando sólo un enum es más conciso:
en lugar de un enum dentro de un struct, podemos poner datos directamente en
cada variante de enum. Esta nueva definición del enum `IpAddr` dice que tanto
las variantes `V4` como `V6` tendrán valores `String` asociados:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-02-enum-with-data/src/main.rs:here}}
```

Adjuntamos datos a cada variante del enum directamente, por lo que no hay
necesidad de un struct extra. Aquí, también es más fácil ver otro detalle
de cómo funcionan los enums: el nombre de cada variante de enum que definimos
también se convierte en una función que construye una instancia del tipo enum.
Es decir, `IpAddr::V4()` es una llamada a función que toma un argumento
`String` y devuelve una instancia del tipo `IpAddr`. Obtenemos automáticamente
esta función constructora definida como resultado de definir el enum.

Hay otra ventaja de usar un enum en lugar de un struct: cada variante puede
tener diferentes tipos y cantidades de datos asociados con ella. La versión
cuatro de las direcciones IP siempre tendrá cuatro componentes numéricos que
tendrán valores entre 0 y 255. Si quisiéramos almacenar las direcciones `V4`
como cuatro valores `u8` pero aun así expresar las direcciones `V6` como un
valor `String`, no podríamos hacerlo con un struct. Los enums manejan este caso
con facilidad:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-03-variants-with-different-data/src/main.rs:here}}
```

Hemos mostrado varias formas diferentes de definir estructuras de datos para
almacenar direcciones IP de la versión cuatro y de la versión seis. Sin embargo,
resulta que querer almacenar direcciones IP y codificar cuál es tan común
que [la biblioteca estándar tiene una definición que podemos usar!][IpAddr]<!-- ignore -->
Veamos cómo define la biblioteca estándar `IpAddr`: tiene el enum exacto y las
variantes que hemos definido y usado, pero incrusta los datos de dirección
dentro de las variantes en forma de dos structs diferentes, que se definen de
manera diferente para cada variante:

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

Este código ilustra que puedes poner cualquier tipo de datos dentro de una
variante de enum: strings, tipos numéricos o structs, por ejemplo. ¡Incluso
puedes incluir otro enum! Además, los tipos de biblioteca estándar a menudo no
son mucho más complicados de lo que podrías idear.

Ten en cuenta que aunque la biblioteca estándar contiene una definición para
`IpAddr`, aún podemos crear y usar nuestra propia definición sin conflicto
porque no hemos traído la definición de la biblioteca estándar a nuestro
ámbito. Hablaremos más sobre cómo traer tipos al ámbito en el Capítulo 7.

Veamos otro ejemplo de una enumeración en el Listing 6-2: este tiene una amplia
variedad de tipos incrustados en sus variantes.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

<span class="caption">Listing 6-2: Un enum `Message` cuyas variantes almacenan 
diferentes cantidades y tipos de valores</span>

Este enum tiene cuatro variantes con diferentes tipos:

* `Quit` no tiene ningún dato asociado.
* `Move` tiene campos nombrados, como lo haría un struct.
* `Write` incluye un solo `String`.
* `ChangeColor` incluye tres valores `i32`.

Definiendo un enum con variantes como las del Listing 6-2 es similar a
definir diferentes tipos de definiciones de struct, excepto que el enum no
use la palabra clave `struct` y todas las variantes están agrupadas juntas
bajo el tipo `Message`. Los siguientes structs podrían contener los mismos
datos que las variantes de enum anteriores:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-04-structs-similar-to-message-enum/src/main.rs:here}}
```

Pero si usamos los diferentes structs, cada uno de los cuales tiene su propio
tipo, no podríamos definir tan fácilmente una función para tomar cualquiera
de estos tipos de mensajes como podríamos con el enum `Message` definido en
el Listing 6-2, que es un solo tipo.

Hay una similitud entre enums y structs que puede ser útil de recordar: al
igual que puedes definir métodos en structs usando `impl`, puedes definir
métodos en enums. Aquí hay un método llamado `call` que podemos definir en
nuestro enum `Message`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-05-methods-on-enums/src/main.rs:here}}
```

El cuerpo del método usaría `self` para obtener el valor en el que llamamos
el método. En este ejemplo, hemos creado una variable `m` que tiene el valor
`Message::Write(String::from("hello"))`, y eso es lo que será `self` en el
cuerpo del método `call` cuando se ejecute `m.call()`.

Veamos otro enum en la librería estándar que es muy común y útil: `Option`.

### El Enum `Option` y Sus Ventajas Sobre los Valores Null

Esta sección explora un caso de estudio de `Option`, que es otro enum definido
por la biblioteca estándar. El tipo `Option` codifica el escenario muy común en
el que un valor podría ser algo o podría ser nada.

Por ejemplo, si solicita el primer elemento de una lista no vacía, obtendría un
valor. Si solicita el primer elemento de una lista vacía, no obtendría nada.
Expresar este concepto en términos del sistema de tipos significa que el
compilador puede verificar si ha manejado todos los casos que debería estar
manejando; esta funcionalidad puede prevenir errores que son extremadamente
comunes en otros lenguajes de programación.

El diseño del lenguaje de programación a menudo se piensa en términos de qué
características se incluyen, pero las características que se excluyen son
importantes también. Rust no tiene la característica de null que muchos otros
lenguajes tienen. *Null* es un valor que significa que no hay ningún valor
allí. En los lenguajes con null, las variables siempre pueden estar en uno de
dos estados: null o no null.

En su presentación del 2009 “Null References: The Billion Dollar Mistake”,
Tony Hoare, el inventor de null, tiene esto que decir:

> Llámalo mi error de un billón de dólares. En ese momento, estaba diseñando el
> primer sistema de tipos completo para referencias en un lenguaje de
> programación orientado a objetos. Mi objetivo era asegurarme de que todo el
> uso de referencias fuera absolutamente seguro, con verificación realizada
> automáticamente por el compilador. Pero no pude resistir la tentación de
> poner un valor nulo, simplemente porque era tan fácil de implementar. Esto a
> dado lugar a innumerables errores, vulnerabilidades y bloqueos del sistema,
> que probablemente han causado un billón de dólares de dolor y daños en los
> últimos cuarenta años.

El problema con los valores nulos es que si intentas utilizar un valor nulo como
un valor no nulo, obtendrás un error de algún tipo. Debido a que esta propiedad
nula o no nula es pervasiva, es extremadamente fácil cometer este tipo de error.

Sin embargo, el concepto que null está tratando de expresar sigue siendo
útil: un null es un valor que es actualmente inválido o ausente por alguna
razón.

El problema no es realmente con el concepto, sino con la implementación
particular. Como tal, Rust no tiene null, pero tiene un enum que puede
codificar el concepto de un valor presente o ausente. Este enum es
`Option<T>`, y está [definido por la biblioteca estándar][option]<!-- ignore -->
de la siguiente manera:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

El enum `Option<T>` es tan útil que incluso está incluido en el prelude; no
necesitas traerlo a ámbito explícitamente. Sus variantes también están
incluidas en el prelude: puedes usar `Some` y `None` directamente sin el
prefijo `Option::`. El enum `Option<T>` es aún un enum regular, y `Some(T)`
y `None` son aún variantes de tipo `Option<T>`.

La sintaxis `<T>` es una característica de Rust que aún no hemos hablado. Es
un parámetro de tipo genérico, y cubriremos los genéricos en más detalle en
el Capítulo 10. Por ahora, todo lo que necesitas saber es que `<T>` significa
que la variante `Some` del enum `Option` puede contener una pieza de datos de
cualquier tipo, y que cada tipo concreto que se usa en lugar de `T` hace que
el tipo `Option<T>` general sea un tipo diferente. Aquí hay algunos ejemplos
de usar valores `Option` para contener tipos de números y tipos de strings:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-06-option-examples/src/main.rs:here}}
```

El tipo de `some_number` es `Option<i32>`. El tipo de `some_string` es
`Option<String>`, que es un tipo diferente. Rust puede inferir estos tipos
porque hemos especificado un valor dentro de la variante `Some`. Para
`absent_number`, Rust nos requiere anotar el tipo general de `Option`: el
compilador no puede inferir el tipo que la variante `Some` correspondiente
tendrá al mirar solo un valor `None`. Aquí, le decimos a Rust que queremos
que `absent_number` sea del tipo `Option<i32>`.

Cuando tenemos un valor `Some`, sabemos que un valor está presente y el valor
se mantiene dentro del `Some`. Cuando tenemos un valor `None`, en cierto
sentido significa lo mismo que null: no tenemos un valor válido. Entonces,
¿por qué tener `Option<T>` es mejor que tener null?

En resumen, porque `Option<T>` y `T` (donde `T` puede ser cualquier tipo) son
tipos diferentes, el compilador no nos permitirá usar un valor `Option<T>` como
si fuera definitivamente un valor válido. Por ejemplo, este código no se
compilará, porque está tratando de agregar un `i8` a un `Option<i8>`:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/src/main.rs:here}}
```

Si ejecutamos este código, obtenemos un mensaje de error como este:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/output.txt}}
```

¡Intenso! En efecto, este mensaje de error significa que Rust no entiende cómo
agregar un `i8` y un `Option<i8>`, porque son tipos diferentes. Cuando tenemos
un valor de un tipo como `i8` en Rust, el compilador se asegurará de que
siempre tengamos un valor válido. Podemos proceder con confianza sin tener que
comprobar si es nulo antes de usar ese valor. Solo cuando tenemos un
`Option<i8>` (o el tipo de valor que estemos trabajando) tenemos que preocuparnos
por posiblemente no tener un valor, y el compilador se asegurará de que
manejemos ese caso antes de usar el valor.

En otras palabras, tienes que convertir un `Option<T>` a un `T` antes de que
puedas realizar operaciones `T` con él. Generalmente, esto ayuda a detectar uno
de los errores más comunes con null: asumiendo que algo no es nulo cuando
realmente lo es.

La eliminación del riesgo de asumir incorrectamente un valor no null
ayuda a tener más confianza en su código. Para tener un valor que
puede ser null, debe optar explícitamente por hacer que el tipo de ese
valor sea `Option<T>`. Luego, cuando use ese valor, se le requerirá
expresar explícitamente el caso cuando el valor sea null. En todas
las partes en que un valor tiene un tipo que no es un `Option<T>`, *puede*
asegurarse de que el valor no sea null. Esta fue una decisión
deliberada del diseño de Rust para limitar la pervasión de nulos y
aumentar la seguridad del código de Rust.

Entonces ¿cómo obtienes el valor `T` de un `Some` cuando tienes un valor de
tipo `Option<T>` para que puedas usar ese valor? El enum `Option<T>` tiene una
gran cantidad de métodos que son útiles en una variedad de situaciones; puedes
verlos en [su documentación][docs]<!-- ignore -->. Familiarizarse con los
métodos en `Option<T>` será extremadamente útil en su viaje con Rust.

En general, para usar un valor `Option<T>`, querrás tener código que maneje
cada variante. Quieres tener algún código que se ejecute solo cuando tienes un
valor `Some(T)`, y este código está permitido de usar el `T` interno. Quieres
tener algún otro código que se ejecute solo si tienes un valor `None`, y ese
código no tiene un valor `T` disponible. La expresión `match` es un
constructo de flujo de control que hace exactamente esto cuando se usa con
enums: ejecutará diferente código dependiendo de la variante del enum que
tenga, y ese código puede usar los datos dentro del valor que coincida.

[IpAddr]: ../std/net/enum.IpAddr.html
[option]: ../std/option/enum.Option.html
[docs]: ../std/option/enum.Option.html
