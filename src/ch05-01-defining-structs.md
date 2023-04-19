## Definiendo e Instanciando Structs

Los Structs son similares a las tuplas, discutido en la sección 
[“The Tuple Type”][tuples]<!--ignore --> en ambos casos mantenemos multiples 
valores relativos. Como en las tuplas, las partes de un struct pueden ser de 
diferentes tipos. A diferencia de las tuplas, en un struct tú nombras a cada 
pieza de datos para que quede claro, que significan estos valores. 
Agregando estos nombres significa que los structs son más flexibles 
que las tuplas: no tienes que confiar en el orden de los datos para especificar o 
acceder a los valores de una instancia.

Para definir un struct, debemos usar la palabra clave `struct` y el nombre del struct completo. 
El nombre del struct debe describir el significado de los datos que se agrupan.
Entonces, entre llaves, definimos los nombres y tipos de datos, que llamaremos
*campos*. Por ejemplo, en el Listing 5-1 mostramos una definición de un struct
que almacena información sobre una cuenta de usuario.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-01/src/main.rs:here}}
```

<span class="caption">Listing 5-1: Una definición de struct `User`</span>

Para usar un struct después de haberlo definido, creamos una *instancia* de ese
struct especificando valores concretos para cada uno de los campos. Creamos una
instancia al declarar el nombre del struct y luego agregar llaves que contienen
*clave: valor* pares, donde las claves son los nombres de los campos y los
valores son los datos que queremos almacenar en esos campos. No tenemos que
especificar los campos en el mismo orden en el que los declaramos en el struct.
En otras palabras, la definición del struct es como una plantilla general para
el tipo, y las instancias llenan esa plantilla con datos particulares para
crear valores del tipo. Por ejemplo, podemos declarar un usuario en particular
como se muestra en el Listing 5-2.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-02/src/main.rs:here}}
```

<span class="caption">Listing 5-2: Creando una instancia del struct `User`
</span>

Para acceder a un valor específico de un struct, usamos la notación de punto.
Por ejemplo, para acceder a la dirección de correo electrónico de este usuario,
usamos `user1.email`. Si la instancia es mutable, podemos cambiar un valor
asignando en un campo particular. El Listing 5-3 muestra cómo cambiar el valor
en el campo `email` de una instancia mutable de `User`.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-03/src/main.rs:here}}
```

<span class="caption">Listing 5-3: Cambiando el valor en el campo `email` de una
instancia `User`</span>

Nota que toda la instancia debe ser mutable; Rust no nos permite marcar solo
ciertos campos como mutables. Como cualquier expresión, podemos construir una
nueva instancia del struct como la última expresión en el cuerpo de la función
para devolver implícitamente esa nueva instancia.

Listing 5-4 muestra una función `build_user` que devuelve una instancia de
`User` con el correo electrónico y el nombre de usuario dados. El campo
`active` obtiene el valor de `true`, y el campo `sign_in_count` obtiene el
valor de `1`.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-04/src/main.rs:here}}
```

<span class="caption">Listing 5-4: Una función `build_user` que toma un email
y username y devuelve una instancia `User`</span>

Tiene sentido nombrar los parámetros de la función con el mismo nombre que los
campos del struct, pero tener que repetir los nombres de los campos y
variables `email` y `username` es un poco tedioso. Si el struct tuviera más
campos, repetir cada nombre sería aún más molesto. Afortunadamente, hay una
conveniente forma abreviada.

<!-- Old heading. Do not remove or links may break. -->
<a id="using-the-field-init-shorthand-when-variables-and-fields-have-the-same-name"></a>

### Usando la abreviatura Field Init

Debido a que los nombres de los parámetros y los nombres de los campos del
struct son exactamente los mismos en el Listing 5-4, podemos usar la *abreviatura
Field Init* para reescribir `build_user` para que se comporte exactamente igual
pero no tenga la repetición de `username` y `email`, como se muestra en el
Listing 5-5.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-05/src/main.rs:here}}
```

<span class="caption">Listing 5-5: Una función `build_user` que usa field init
abreviado porque los parámetros `username` e `email` tienen el mismo nombre que
los campos del struct</span>

Aquí, estamos creando una nueva instancia del struct `User`, que tiene un
campo llamado `email`. Queremos establecer el valor del campo `email` en el
valor del parámetro `email` de la función `build_user`. Debido a que el campo
`email` y el parámetro `email` tienen el mismo nombre, solo necesitamos escribir
`email` en lugar de `email: email`.

### Creando Instancias de Otras Instancias con Sintaxis de Struct Update

Suele ser útil crear una nueva instancia de un struct que incluya la mayoría de
los valores de otra instancia, pero cambie algunos. Puede hacer esto usando la
*sintaxis de struct update*.

Primero, en el Listing 5-6 mostramos cómo crear una nueva instancia de `User`
regularmente, sin la sintaxis de actualización. Establecemos un nuevo valor para
`email`, pero de lo contrario usamos los mismos valores de `user1` que creamos
en el Listing 5-2.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-06/src/main.rs:here}}
```

<span class="caption">Listing 5-6: Creando una nueva instancia `User` usando uno
de los valores de `user1`</span>

Usando la sintaxis de struct update, podemos lograr el mismo efecto con menos
código, como se muestra en el Listing 5-7. La sintaxis `..` especifica que los
campos restantes que no se establecen explícitamente deben tener el mismo valor
que los campos en la instancia dada.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-07/src/main.rs:here}}
```

<span class="caption">Listing 5-7: Usando una sintaxis de struct update para 
introducir un nuevo valor `email` para una instancia `User` pero para usar el 
resto de los valores de `user1`</span>

El código en el Listing 5-7 también crea una instancia en `user2` que tiene un
valor diferente para `email` pero tiene los mismos valores para los campos
`username`, `active` y `sign_in_count` de `user1`. Él `..user1` debe ir al
final para especificar que cualquier campo restante debe obtener sus valores
del campo correspondiente en `user1`, pero podemos elegir especificar valores
para tantos campos como queramos en cualquier orden, independientemente del
orden de los campos en la definición del struct.

Nota que la sintaxis de update struct usa `=` como una asignación; esto es
porque mueve los datos, como vimos en la sección [“Variables y datos
interactuando con Move”][move]<!-- ignore -->. En este ejemplo, ya no podemos
usar `user1` como un todo después de crear `user2` porque el `String` en el
campo `username` de `user1` se movió a `user2`. Si hubiéramos dado a `user2`
nuevos valores `String` para `email` y `username`, y por lo tanto solo usamos
los valores de `active` y `sign_in_count` de `user1`, entonces `user1` todavía
sería válido después de crear `user2`. Tanto `active` como `sign_in_count` son
tipos que implementan la trait `Copy`, por lo que el comportamiento que
discutimos en la sección [“Datos de pila: Copy”][copy]<!-- ignore --> se
aplicaría.

### Usando Structs de Tuplas sin Campos Nombrados para Crear Diferentes Tipos

Rust también admite structs que se parecen a tuplas, llamados *structs de
tuplas*. Los structs de tuplas tienen el significado adicional que proporciona
el nombre del struct, pero no tienen nombres asociados a sus campos; en su
lugar, solo tienen los tipos de los campos. Los structs de tuplas son útiles
cuando desea darle un nombre al conjunto completo y hacer que el conjunto sea
un tipo diferente de otros conjuntos, y cuando nombrar cada campo como en un
struct regular sería verboso o redundante.

Para definir un struct de tupla, comience con la palabra clave `struct` y el
nombre del struct seguido por los tipos en la tupla. Por ejemplo, aquí
definimos y usamos dos structs de tupla llamados `Color` y `Point`:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-01-tuple-structs/src/main.rs}}
```

Nota que los valores `black` y `origin` son diferentes tipos porque son
instancias de diferentes structs de tupla. Cada struct que defina es su propio
tipo, incluso si los campos dentro del struct tienen los mismos tipos. Por
ejemplo, una función que toma un parámetro de tipo `Color` no puede tomar un
`Point` como argumento, incluso si ambos tipos están compuestos por tres
valores `i32`. De lo contrario, las instancias de structs de tupla son
similares a las tuplas en que puede descomponerlas en sus piezas individuales,
y puede usar un `.` seguido por el índice para acceder a un valor individual.

### Structs de Unidad sin Campos

También puede definir structs que no tienen ningún campo. Estos se llaman
*structs de unidad* porque se comportan de manera similar a `()`, el tipo de
unidad que mencionamos en la sección [“El tipo de tupla”][tuples]<!-- ignore
-->. Los structs de unidad pueden ser útiles cuando necesita implementar un
trait en algún tipo, pero no tiene datos que desea almacenar en el tipo
propio. Discutiremos los traits en el Capítulo 10. Aquí hay un ejemplo de
declarar e instanciar un struct de unidad llamado `AlwaysEqual`:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-04-unit-like-structs/src/main.rs}}
```

Para definir `AlwaysEqual`, usamos la palabra clave `struct`, el nombre que
queremos y luego un punto y coma. ¡No se necesitan llaves ni paréntesis! Luego
podemos obtener una instancia de `AlwaysEqual` en la variable `subject` de la
misma manera: usando el nombre que definimos, sin llaves ni paréntesis.
Imagina que más tarde implementaremos un comportamiento para este tipo de tal
manera que cada instancia de `AlwaysEqual` siempre sea igual a cada instancia
de cualquier otro tipo, tal vez para tener un resultado conocido para fines de
prueba. No necesitaríamos ningún dato para implementar ese comportamiento.
Verás en el Capítulo 10 cómo definir traits e implementarlos en cualquier
tipo, incluidos los structs de unidad.

> ### Ownership de los datos de Struct
>
> En el struct `User` de la definición en el Listing 5-1, usamos el tipo
> `String` en lugar del tipo `&str` de la cadena de caracteres. Esta es una
> elección deliberada porque queremos que cada instancia de este struct tenga
> todos sus datos y que esos datos sean válidos durante todo el tiempo que el
> struct sea válido.
>
> También es posible para los structs almacenar referencias a datos que son
> propiedad de algo más, pero para hacerlo requiere el uso de *lifetimes*, una
> característica de Rust que discutiremos en el Capítulo 10. Los lifetimes
> garantizan que los datos referenciados por un struct sean válidos durante el
> tiempo que el struct sea válido. Digamos que intentas almacenar una
> referencia en un struct sin especificar lifetimes, como el siguiente; esto
> no funcionará:
>
> <span class="filename">Filename: src/main.rs</span>
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust,ignore,does_not_compile
> struct User {
>     active: bool,
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
> }
>
> fn main() {
>     let user1 = User {
>         active: true,
>         username: "someusername123",
>         email: "someone@example.com",
>         sign_in_count: 1,
>     };
> }
> ```
>
> El compilador se quejará de que necesita especificadores de lifetime:
>
> ```console
> $ cargo run
>    Compiling structs v0.1.0 (file:///projects/structs)
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:3:15
>   |
> 3 |     username: &str,
>   |               ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 ~     username: &'a str,
>   |
>
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:4:12
>   |
> 4 |     email: &str,
>   |            ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 |     username: &str,
> 4 ~     email: &'a str,
>   |
>
> For more information about this error, try `rustc --explain E0106`.
> error: could not compile `structs` due to 2 previous errors
> ```
>
> En el Capítulo 10, discutiremos como solucionar estos errores para que puedas
> almacenar referencias en structs, pero por ahora, solucionaremos los errores
> usando tipos propios como `String` en lugar de referencias como `&str`.

<!-- manual-regeneration
for the error above
after running update-rustc.sh:
pbcopy < listings/ch05-using-structs-to-structure-related-data/no-listing-02-reference-in-struct/output.txt
paste above
add `> ` before every line -->

[tuples]: ch03-02-data-types.html#the-tuple-type
[move]: ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move
[copy]: ch04-01-what-is-ownership.html#stack-only-data-copy
