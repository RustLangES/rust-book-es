## Sintaxis de Métodos

Los métodos son similares a las funciones: los declaramos con la palabra clave
`fn` y un nombre, pueden tener parámetros y un valor de retorno, y contienen
alguno código que se ejecuta cuando el método es llamado desde otro lugar.
A diferencia de las funciones, los métodos se definen dentro del contexto de
una estructura (o un enum o un objeto de tipo trait, que cubriremos en el
[Capítulo 6][enums]<!-- ignore --> y el [Capítulo 17][trait-objects]<!-- ignore
-->, respectivamente), y su primer parámetro siempre es `self`, que representa
la instancia de la estructura en la que se está llamando al método.

### Definiendo Métodos

Vamos a cambiar la función `area` que tiene una instancia de `Rectangle` como
parámetro y en vez de eso definamos un método `area` en el struct `Rectangle`,
como se muestra en el Listado 5-13.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-13/src/main.rs}}
```

<span class="caption">Listing 5-13: Definición de un método `area` en el struct
`Rectangle` </span>

Para definir la función dentro del contexto de `Rectangle`, iniciamos un bloque
`impl` (implementación). Todo lo que esté dentro de este bloque `impl` estará
asociado al tipo `Rectangle`. Luego movemos la función `area` dentro de las
llaves del `impl` y cambiamos el primer (y en este caso, único) parámetro para
ser `self` en la firma y en todas partes dentro del cuerpo. En `main`, donde
llamamos a la función `area` y pasamos `rect1` como argumento, podemos en vez
de eso usar la *sintaxis de método* para llamar al método `area` en nuestra
instancia de `Rectangle`. La sintaxis de método va después de una instancia: se
agrega un punto seguido del nombre del método, paréntesis y cualquier argumento.

En la firma para `area`, usamos `&self` en vez de `rectangle: &Rectangle`. El
`&self` es en realidad una abreviatura para `self: &Self`. Dentro de un bloque
`impl`, el tipo `Self` es un alias para el tipo al que pertenece el bloque
`impl`. Los métodos deben tener un parámetro llamado `self` de tipo `Self` para
su primer parámetro, por lo que Rust nos permite abreviar esto con solo el
nombre `self` en el primer parámetro. Ten en cuenta que aún necesitamos usar el
`&` antes de la abreviatura `self` para indicar que este método presta la
instancia `Self`, al igual que hicimos en `rectangle: &Rectangle`. Los métodos
pueden tomar la propiedad de `self`, prestar `self` inmutablemente, como lo
hemos hecho aquí, o prestar `self` mutably, al igual que pueden hacerlo con
cualquier otro parámetro.

Elegimos `&self` aquí por la misma razón que usamos `&Rectangle` en la versión
de la función: no queremos tomar la propiedad, y solo queremos leer los datos
en la estructura, no escribir en ella. Si quisiéramos cambiar la instancia en
la que hemos llamado al método como parte de lo que el método hace, usaríamos
`&mut self` como primer parámetro. Tener un método que tome la propiedad de la
instancia usando solo `self` como primer parámetro es raro; esta técnica se
usa normalmente cuando el método transforma `self` en otra cosa y quieres
evitar que el que llama al método use la instancia original después de la
transformación.

La razón principal para usar métodos en vez de funciones, además de proveer la
sintaxis de método y no tener que repetir el tipo de `self` en cada firma de
método, es para la organización. Hemos puesto todas las cosas que podemos hacer
con una instancia de un tipo en un bloque `impl` en vez de hacer que los
usuarios futuros de nuestro código busquen las capacidades de `Rectangle` en
varios lugares en la biblioteca que proveemos.

Nota que podemos elegir darle al método el mismo nombre que uno de los campos
del struct. Por ejemplo, podemos definir un método en `Rectangle` que se llame
`width`:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-06-method-field-interaction/src/main.rs:here}}
```

Aquí, estamos eligiendo que el método `width` retorne `true` si el valor en el
campo `width` de la instancia es mayor que `0` y `false` si el valor es `0`:
podemos usar un campo dentro de un método del mismo nombre para cualquier
propósito. En `main`, cuando seguimos `rect1.width` con paréntesis, Rust sabe
que queremos decir el método `width`. Cuando no usamos paréntesis, Rust sabe
que queremos decir el campo `width`.

A veces, pero no siempre, cuando damos un método el mismo nombre que un campo
queremos que solo retorne el valor en el campo y no haga nada más. Los métodos
como este se llaman *getters*, y Rust no los implementa automáticamente para
los campos de un struct como lo hacen otros lenguajes. Los getters son útiles
porque puedes hacer que el campo sea privado, pero el método sea público, y así
permitir acceso de solo lectura a ese campo como parte de la API pública del
tipo. Hablaremos de qué es público y privado y cómo designar un campo o método
como público o privado en el [Capítulo 7][public]<!-- ignore -->.

> ### ¿Dónde está el Operador `->`?
>
> En C y C++, se usan dos operadores diferentes para llamar a métodos: se usa
> `.` si se está llamando a un método en el objeto directamente y `->` si se
> está llamando al método en un puntero al objeto y se necesita desreferenciar 
> el puntero primero. En otras palabras, si `object` es un puntero,
> `object->something()` es similar a `(*object).something()`.
>
> Rust no tiene un equivalente al operador `->`; en su lugar, Rust tiene una
> característica llamada *referenciación y desreferenciación automáticas*.
> Llamar a métodos es uno de los pocos lugares en Rust donde se tiene este
> comportamiento.
>
> Así es como funciona: cuando llamas a un método con `object.something()`,
> Rust automáticamente agrega `&`, `&mut`, o `*` para que `object` coincida
> con la firma del método. En otras palabras, lo siguiente es lo mismo:
>
> <!-- CAN'T EXTRACT SEE BUG https://github.com/rust-lang/mdBook/issues/1127 -->
> ```rust
> # #[derive(Debug,Copy,Clone)]
> # struct Point {
> #     x: f64,
> #     y: f64,
> # }
> #
> # impl Point {
> #    fn distance(&self, other: &Point) -> f64 {
> #        let x_squared = f64::powi(other.x - self.x, 2);
> #        let y_squared = f64::powi(other.y - self.y, 2);
> #
> #        f64::sqrt(x_squared + y_squared)
> #    }
> # }
> # let p1 = Point { x: 0.0, y: 0.0 };
> # let p2 = Point { x: 5.0, y: 6.5 };
> p1.distance(&p2);
> (&p1).distance(&p2);
> ```
>
> El primer ejemplo es más limpio. Este comportamiento de referencia y
> desreferenciación automática funciona porque los métodos tienen un receptor
> claro: el tipo de `self`. Dado el receptor y el nombre de un método, Rust
> puede determinar con certeza si el método está leyendo (`&self`), mutando
> (`&mut self`), o consumiendo (`self`). El hecho de que Rust haga que el
> préstamo sea implícito para los receptores de método es una gran parte de
> hacer que la propiedad sea ergonómica en la práctica.

### Métodos con más parámetros

Practiquemos usando métodos implementando un segundo método en la estructura
`Rectangle`. Esta vez queremos que una instancia de `Rectangle` tome otra
instancia de `Rectangle` y retorne `true` si el segundo `Rectangle` puede
completamente caber dentro de `self` (el primer `Rectangle`); de lo
contrario, debería retornar `false`. Es decir, una vez que hayamos definido el
método `can_hold`, queremos poder escribir el programa mostrado en el 
Listing 5-14.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-14/src/main.rs}}
```

<span class="caption">Listing 5-14: Uso del método `can_hold` aún no escrito
</span>

La salida esperada se vería como la siguiente porque ambas dimensiones de
`rect2` son más pequeñas que las dimensiones de `rect1`, pero `rect3` es más
ancha que `rect1`:

```text
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

Sabemos que queremos definir un método, por lo que estará dentro del bloque 
`impl Rectangle`. El nombre del método será `can_hold`, y tomará un préstamo
inmutable de otro `Rectangle` como parámetro. Podemos decir cuál será el tipo
del parámetro mirando el código que llama al método: `rect1.can_hold(&rect2)`
pasa `&rect2`, que es un préstamo inmutable a `rect2`, una instancia de
`Rectangle`. Esto tiene sentido porque solo necesitamos leer `rect2` (en lugar
de escribir, lo que significaría que necesitaríamos un préstamo mutable), y
queremos que `main` conserve la propiedad de `rect2` para que podamos usarlo
nuevamente después de llamar al método `can_hold`. El valor de retorno de
`can_hold` será un Booleano, y la implementación verificará si el ancho y
alto de `self` son mayores que el ancho y alto del otro `Rectangle`,
respectivamente. Agreguemos el nuevo método `can_hold` al bloque `impl` del 
Listing 5-13 que se muestra en el Listing 5-15.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-15/src/main.rs:here}}
```

<span class="caption">Listing 5-15: Implementando el método `can_hold` en
`Rectangle` que toma otra instancia de `Rectangle` como un parámetro</span>

Cuando ejecutamos este código con la función `main` en el Listing 5-14, 
obtendremos el resultado deseado. Los métodos pueden tomar múltiples parámetros 
que agregamos a la firma después del parámetro `self`, y esos parámetros 
funcionan igual que los parámetros en las funciones.

### Funciones asociadas

Todas las funciones definidas dentro de un bloque `impl` se llaman *funciones
asociadas* porque están asociadas con el tipo nombrado después del `impl`.
Podemos definir funciones asociadas que no tengan `self` como su primer
parámetro (y, por lo tanto, no sean métodos) porque no necesitan una instancia del
tipo con el que trabajar. Ya hemos usado una función como esta: la función
`String::from` que está definida en el tipo `String`.

Las funciones asociadas que no son métodos son a menudo utilizadas para
constructores que devolverán una nueva instancia de la estructura. Estás
a menudo se llaman `new`, pero `new` no es un nombre especial y no está
incorporado en el lenguaje. Por ejemplo, podríamos elegir proporcionar una
función asociada llamada `square` que tendría un parámetro de dimensión y lo
usaría como ancho y alto, de modo que sea más fácil crear un `Rectangle`
cuadrado en lugar de tener que especificar el mismo valor dos veces:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-03-associated-functions/src/main.rs:here}}
```

La palabra clave `Self` en el tipo de retorno y en el cuerpo de la función es
un alias para el tipo que aparece después de la palabra clave `impl`, que en
este caso es `Rectangle`.

Para llamar a esra función asociada, usamos la sintaxis `::` con el nombre de
la estructura; `let sq = Rectangle::square(3);` es un ejemplo. Esta función
está en el espacio de nombres de la estructura: la sintaxis `::` se usa tanto
para las funciones asociadas como para los espacios de nombres creados por los
módulos. Discutiremos los módulos en el [Capítulo 7][modules]<!-- ignore -->.

### Bloques `impl` múltiples

Cada struct es permitido tener múltiples bloques `impl`. Por ejemplo, el
Listing 5-15 es equivalente al código mostrado en el Listing 5-16, que tiene
cada método en su propio bloque `impl`.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-16/src/main.rs:here}}
```

<span class="caption">Listing 5-16: Rescribiendo Listing 5-15 usando multiples 
bloques `impl`</span>

No hay razón para separar estos métodos en múltiples bloques `impl` aquí, pero
esta es una sintaxis válida. Veremos un caso en el que los múltiples bloques
`impl` son útiles en el Capítulo 10, donde discutiremos los tipos genéricos y
los traits.

## Resumen

Los structs te permiten crear tipos personalizados que son significativos para
su dominio. Al usar structs, puede mantener piezas de datos asociadas entre sí
y nombrar cada pieza para hacer que su código sea claro. En los bloques `impl`,
puede definir funciones que están asociadas con su tipo, y los métodos son un
tipo de función asociada que le permite especificar el comportamiento que
tienen las instancias de sus structs.

Pero los structs no son la única forma de crear tipos personalizados: pasemos
a la función enum de Rust para agregar otra herramienta a su toolbox.

[enums]: ch06-00-enums.html
[trait-objects]: ch17-02-trait-objects.md
[public]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword
[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
