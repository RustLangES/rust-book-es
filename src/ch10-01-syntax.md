## Tipos de Datos Genéricos

Utilizamos genéricos para crear definiciones para elementos como firmas de
funciones o structs, que luego podemos usar con muchos tipos de datos
concretos diferentes. Primero veamos cómo definir funciones, structs,
enums y métodos usando genéricos. Luego discutiremos cómo los genéricos
afectan el rendimiento del código.

### Definiciones En Function

Al definir una función que usa genéricos, colocamos los genéricos en la firma de
la función donde normalmente especificaríamos los tipos de datos de los
parámetros y el valor de retorno. Hacerlo hace que nuestro código sea más
flexible y brinda más funcionalidad a los llamadores de nuestra función al
tiempo que evita la duplicación de código.

Continuando con nuestra función `largest`, el listado 10-4 muestra dos
funciones que encuentran el valor más grande en un slice. Luego
combinaremos estos en una sola función que usa genéricos.

<Listing number="10-4" file-name="src/main.rs" caption="Dos funciones que difieren solo en sus nombres y los tipos en sus firmas">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-04/src/main.rs:here}}
```

</Listing>

La función `largest_i32` es la que extrajimos en el listado 10-3 que encuentra
el `i32` más grande en un slice. La función `largest_char` encuentra el
`char` más grande en un slice. Los cuerpos de las funciones tienen el mismo
código, así que eliminemos la duplicación introduciendo un parámetro de tipo
generic en una sola función.

Para parametrizar los tipos en una nueva función única, necesitamos nombrar el
parámetro de tipo, tal como lo hacemos para los parámetros de valor de una
función. Pero usaremos `T` porque, por convención, los nombres de los
parámetros de tipo en Rust son cortos, a menudo solo una letra, y la
convención de nomenclatura de tipo de Rust es UpperCamelCase. Abreviatura de
"tipo", `T` es la opción predeterminada de la mayoría de los programadores de
Rust.

Cuando usamos un parámetro en el cuerpo de la función, tenemos que declarar el
nombre del parámetro en la firma para que el compilador sepa qué significa ese
nombre. De manera similar, cuando usamos un nombre de parámetro de tipo en la
firma de una función, tenemos que declarar el nombre del parámetro de tipo
antes de usarlo. Para definir un genérico en la función `largest`, coloque las
declaraciones de nombre de tipo dentro de corchetes angulares, `<>`, entre el
nombre de la función y la lista de parámetros, así:

```rust,ignore
fn largest<T>(list: &[T]) -> &T {
```

Leemos esta definición como: la función `largest` es genérico sobre algún tipo
`T`. Esta función tiene un parámetro llamado `list`, que es un slice de valores
de tipo `T`. La función `largest` devolverá una referencia a un valor del mismo
tipo `T`.

El listado 10-5 muestra la definición de la función `largest` combinada usando
el tipo de datos genéricos en su firma. La lista también muestra cómo podemos
llamar a la función con un slice de valores `i32` o valores `char`. Tenga en
cuenta que este código aún no se compilará, pero lo arreglaremos más adelante
en este capítulo.

<Listing number="10-5" file-name="src/main.rs" caption="La función `largest` está usando parámetros de tipo genérico; esto aún no se compila">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/src/main.rs}}
```

</Listing>

Si compilamos este código ahora, obtendremos este error:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/output.txt}}
```

El texto de ayuda menciona `std::cmp::PartialOrd`, que es un _trait_, y vamos a
hablar de traits en la siguiente sección. Por ahora, sepa que este error
indica que el cuerpo de `largest` no funcionará para todos los tipos posibles
que podría ser `T`. Debido a que queremos comparar valores de tipo `T` en el
cuerpo, solo podemos usar tipos cuyos valores se pueden ordenar. Para habilitar
las comparaciones, la biblioteca estándar tiene el trait `std::cmp::PartialOrd`
que puede implementar en tipos (consulte el Apéndice C para obtener más
información sobre este trait). Para corregir el código de ejemplo anterior, 
necesitaríamos seguir las sugerencia del texto de ayuda,
restringir los tipos válidos para `T` solo a aquellos que implementan
`PartialOrd`. El ejemplo entonces compilara, porque la biblioteca estándar
implementa `PartialOrd` tanto en `i32` como en `char`.

### Definiciones En Struct

También podemos definir structs para usar tipos genéricos en uno o más campos
usando la sintaxis `<>`. El listado 10-6 define un struct `Point<T>` para
contener valores `x` e `y` de cualquier tipo `T`.

<Listing number="10-6" file-name="src/main.rs" caption="Un struct `Point<T>` que contiene valores `x` and `y` de tipo `T`">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-06/src/main.rs}}
```

</Listing>

La sintaxis para usar genéricos en las definiciones de structs es similar a la
que se usa en las definiciones de funciones. Primero, declaramos el nombre del
parámetro de tipo dentro de corchetes angulares, justo después del nombre del
struct. Luego, usamos el tipo genérico en la definición del struct donde
especificaríamos tipos de datos concretos.

Ten en cuenta que porque hemos usado un solo tipo genérico para definir
`Point<T>`, esta definición dice que el struct `Point<T>` es genérico sobre algún
tipo `T`, y los campos `x` e `y` son _ambos_ ese mismo tipo, sea cual sea ese
tipo. Si creamos una instancia de un `Point<T>` que tenga valores de diferentes
tipos, como en el listado 10-7, nuestro código no se compilará.

<Listing number="10-7" file-name="src/main.rs" caption="Los campos `x` e `y` deben ser del mismo tipo porque ambos tienen el mismo tipo de dato genérico `T`.">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/src/main.rs}}
```

</Listing>

En este ejemplo, cuando asignamos el valor entero `5` a `x`, le decimos al
compilador que el tipo genérico `T` será un entero para esta instancia de
`Point<T>`. Luego, cuando especificamos `4.0` para `y`, que hemos definido para
tener el mismo tipo que `x`, obtendremos un error de tipo incompatible como
este:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/output.txt}}
```

Para definir un struct `Point` donde `x` e `y` son ambos genéricos pero podrían
tener diferentes tipos, podemos usar múltiples parámetros de tipo genérico. Por
ejemplo, en el listado 10-8, cambiamos la definición de `Point` para que sea
generic sobre los tipos `T` y `U` donde `x` es de tipo `T` e `y` es de tipo
`U`.

<Listing number="10-8" file-name="src/main.rs" caption="Un `Point<T, U>` genérico sobre dos tipos para que `x` e `y` puedan ser valores de tipos diferentes">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-08/src/main.rs}}
```

</Listing>

¡Ahora todas las instancias de `Point` que se muestran se permiten! Puede usar
tantos parámetros de tipo genérico en una definición como desee, pero usar más
de unos pocos hace que su código sea difícil de leer. Si encuentra que necesita
muchos tipos genéricos en su código, podría indicar que su código necesita
reestructurarse en piezas más pequeñas.

### Definiciones En Enum

Como hicimos con structs, podemos definir enums para contener tipos genéricos en
sus variantes. Echemos otro vistazo al enum `Option<T>` que la biblioteca
estándar proporciona, que usamos en el Capítulo 6:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Esta definición debería tener más sentido para ti ahora. Como puede ver, el
enum `Option<T>` es genérico sobre el tipo `T` y tiene dos variantes: `Some`,
que contiene un valor de tipo `T`, y `None`, que no contiene ningún valor.
Al usar el enum `Option<T>`, podemos expresar el concepto abstracto de un valor
opcional, y porque `Option<T>` es genérico, podemos usar esta abstracción sin
importar el tipo del valor opcional.

Los enums también pueden usar múltiples tipos genéricos. La definición del enum
`Result` que usamos en el Capítulo 9 es un ejemplo:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

El enum `Result` es un genérico en dos tipos, `T` y `E`. Tiene dos variantes:
`Ok`, que contiene un valor de tipo `T`, y `Err`, que contiene un valor de tipo
`E`. Esta definición es apropiada porque el significado de `Result` es que uno
de estos dos tipos, `T` o `E`, será el tipo del valor que se devuelve cuando se
produce un error o cuando se tiene éxito (devolviendo un valor de tipo `T`) o
falla (devolviendo un valor de tipo `E`). De hecho, esta es la definición que
usamos para abrir un archivo en el listado 9-3, donde `T` se llenó con el tipo
`std::fs::File` cuando el archivo se abrió con éxito y `E` se llenó con el tipo
`std::io::Error` cuando hubo problemas para abrir el archivo.

Cuando reconoces situaciones en tu código con múltiples definiciones de struct
o enum que difieren solo en los tipos de los valores que contienen, puedes
evitar la duplicación usando tipos genéricos en su lugar.

### Definiciones En Method

Podemos implementar métodos en structs y enums y usar tipos genéricos en sus
definiciones también. El listado 10-9 muestra el struct `Point<T>` que
definimos en el listado 10-6 con un método llamado `x` implementado en él.

<Listing number="10-9" file-name="src/main.rs" caption="Implementando un método llamado `x` en el struct `Point<T>` que devolverá una referencia al campo `x` de tipo `T`">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-09/src/main.rs}}
```

</Listing>

Aquí, hemos definido un método llamado `x` en `Point<T>` que devuelve una
referencia a la data en el campo `x`.

Ten en cuenta que tenemos que declarar `T` justo después de `impl` para que
podamos usar `T` para especificar que estamos implementando métodos en el tipo
`Point<T>`. Al declarar `T` como un tipo genérico después de `impl`, Rust puede
identificar que el tipo en los corchetes angulares en `Point` es un tipo
generic en lugar de un tipo concreto. Podríamos haber elegido un nombre
diferente para este parámetro genérico que el parámetro genérico declarado en la
definición del struct, pero usar el mismo nombre es convencional. Los métodos
escritos dentro de un `impl` que declara el tipo genérico se definirán en
cualquier instancia del tipo, sin importar qué tipo concreto termine
sustituyendo al tipo genérico.

También podemos especificar restricciones en los tipos genéricos al definir
métodos en el tipo. Por ejemplo, podríamos implementar métodos solo en
instancias de `Point<T>` con un tipo `f32` concreto en lugar de en instancias
de `Point<T>` con cualquier tipo genérico. En el listado 10-10 usamos el tipo
concreto `f32`, lo que significa que no declaramos ningún tipo después de
`impl`.

<Listing number="10-10" file-name="src/main.rs" caption="Un bloque `impl` que solo aplica a un struct con un tipo concreto particular para el parámetro del tipo genérico `T`">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-10/src/main.rs:here}}
```

</Listing>

Este código significa que el tipo `Point<f32>` tendrá un método
`distance_from_origin` definido en él, y otros tipos de `Point<T>` que no sean
de tipo `f32` no tendrán este método definido. El método mide qué tan lejos
está nuestro punto del punto en las coordenadas (0.0, 0.0) y usa operaciones
matemáticas que solo están disponibles para tipos de punto flotante.

Los parámetros de tipo genérico en una definición de struct no siempre son los
mismos que los que usas en las firmas de métodos de ese mismo struct. El
listado 10-11 usa los tipos genéricos `X1` e `Y1` para el struct `Point` y `X2`
`Y2` para la firma del método `mixup` para hacer el ejemplo más claro. El
método crea una nueva instancia de `Point` con el valor `x` del `self` `Point`
(de tipo `X1`) y el valor `y` del `Point` pasado (de tipo `Y2`).

<Listing number="10-11" file-name="src/main.rs" caption="Un método que usa diferentes tipos genérico de la definición de su struct">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-11/src/main.rs}}
```

</Listing>

En `main`, hemos definido un `Point` que tiene un `i32` para `x` (con valor `5`)
y un `f64` para `y` (con valor `10.4`). La variable `p2` es un `Point` struct
que tiene un string slice para `x` (con valor `"Hello"`) y un `char` para `y`
(con valor `c`). Llamar a `mixup` en `p1` con el argumento `p2` nos da `p3`,
que tendrá un `i32` para `x`, porque `x` vino de `p1`. La variable `p3` tendrá
un `char` para `y`, porque `y` vino de `p2`. La llamada al macro `println!`
imprimirá `p3.x = 5, p3.y = c`.

El propósito de este ejemplo es demostrar una situación en la que algunos
parámetros genérico se declaran con `impl` y otros se declaran con la definición
del método. Aquí, los parámetros genérico `X1` e `Y1` se declaran después de
`impl` porque van con la definición del struct. Los parámetros genérico `X2` e
`Y2` se declaran después de `fn mixup`, porque solo son relevantes para el
método.

<a id="rendimiento-de-codigo-usando-genericos"></a>

### Rendimiento de código usando genéricos

Quizás te estés preguntando si hay un costo de rendimiento al usar parámetros
de tipo genérico. La buena noticia es que usar tipos genéricos no hará que tu
programa se ejecute más lento de lo que lo haría con tipos concretos.

Rust logra esto realizando _monomorfización_ del código usando genéricos en
tiempo de compilación. _Monomorfización_ es el proceso de convertir código
genérico en código específico llenando los tipos concretos que se usan cuando
se compila. En este proceso, el compilador hace lo contrario de los pasos que
usamos para crear la función genérica en el listado 10-5: el compilador mira
todos los lugares donde se llama el código genérico y genera código para los
tipos concretos con los que se llama el código genérico.

Veamos como funciona esto usando el enum genérico de la biblioteca estándar
`Option<T>`:

```rust
let integer = Some(5);
let float = Some(5.0);
```

Cuando Rust compila este código, realiza monomorfización. Durante ese
proceso, el compilador lee los valores que se han usado en las instancias de
`Option<T>` e identifica dos tipos de `Option<T>`: uno es `i32` y el otro es
`f64`. Como tal, expande la definición genérica de `Option<T>` en dos
definiciones especializadas a `i32` y `f64`, reemplazando así la definición
genérica con las específicas.

La versión monomorfizada del código se ve similar al siguiente (el compilador
usa nombres diferentes a los que estamos usando aquí para ilustración):

<Listing file-name="src/main.rs">

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

</Listing>

El genérico `Option<T>` se reemplaza con las definiciones específicas creadas por
el compilador. Debido a que Rust compila código genérico en código que
especifica el tipo en cada instancia, no pagamos ningún costo de rendimiento
por usar genéricos. Cuando el código se ejecuta, se comporta de la misma manera
que si hubiéramos duplicado cada definición a mano. El proceso de
monomorfización hace que los genéricos de Rust sean extremadamente eficientes
en tiempo de ejecución.
