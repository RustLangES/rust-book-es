## Tipos Avanzados

El sistema de tipos de Rust tiene algunas características que hemos mencionado
hasta ahora, pero que aún no hemos discutido. Comenzaremos discutiendo los
newtypes en general mientras examinamos por qué los newtypes son útiles
como tipos. Luego pasaremos a los type aliases, una característica similar a
los newtypes pero con semántica ligeramente diferente. También discutiremos
el tipo `!` y los tipos de tamaño dinámico.

### Usando el Newtype Pattern para Seguridad de Tipos y Abstracción

> Nota: Esta sección asume que has leído la sección anterior [“Usando el
> Pattern Newtype para Implementar Traits Externos en Tipos
> Externos.”][using-the-newtype-pattern]<!-- ignore -->

El newtype pattern también es útil para tareas más allá de las que hemos
discutido hasta ahora, incluyendo hacer cumplir estáticamente que los valores
nunca se confundan e indicar las unidades de un valor. Viste un ejemplo de
usar newtypes para indicar unidades en el Listado 20-16: recuerda que los
structs `Millimeters` y `Meters` envolvieron valores `u32` en un newtype. Si
escribiéramos una función con un parámetro de tipo `Millimeters`, no podríamos
compilar un programa que accidentalmente intentara llamar a esa función con
un valor de tipo `Meters` o un `u32` simple.

También podemos usar el pattern newtype para abstraer algunos detalles de
implementación de un tipo: el nuevo tipo puede exponer una API pública que es
diferente de la API del tipo interno privado.

Los newtypes también pueden ocultar la implementación interna. Por ejemplo,
podríamos proporcionar un tipo `People` para envolver un `HashMap<i32, String>`
que almacena el ID de una persona asociado con su nombre. El código que usa
`People` solo interactuaría con la API pública que proporcionamos, como un
método para agregar un string de nombre a la colección `People`; ese código
no necesitaría saber que asignamos un ID `i32` a los nombres internamente. El
newtype pattern es una forma ligera de lograr la encapsulación para ocultar
los detalles de implementación, que discutimos en la sección [“Encapsulación
que Oculta los Detalles de
Implementación”][encapsulacion-que-oculta-los-detalles-de-implementacion]<!-- ignore -->
del Capítulo 18.

### Creando Type Synonyms con Type Aliases

Rust proporciona la capacidad de declarar un _type alias_ para darle a un
tipo existente otro nombre. Para esto usamos la palabra clave `type`. Por
ejemplo, podemos crear el alias `Kilometers` a `i32` de la siguiente manera:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-04-kilometers-alias/src/main.rs:here}}
```

Ahora, el alias `Kilometers` es un _sinónimo_ para `i32`; a diferencia de los
tipos `Millimeters` y `Meters` que creamos en el Listado 20-16, `Kilometers`
no es un tipo nuevo y separado. Los valores que tienen el tipo `Kilometers`
se tratarán de la misma manera que los valores del tipo `i32`:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-04-kilometers-alias/src/main.rs:there}}
```

Debido a que `Kilometers` e `i32` son el mismo tipo, podemos agregar valores
de ambos tipos y podemos pasar valores `Kilometers` a funciones que toman
parámetros `i32`. Sin embargo, usando este método, no obtenemos los beneficios
de verificación de tipos que obtenemos del newtype pattern discutido
anteriormente. En otras palabras, si mezclamos valores `Kilometers` e `i32`
en algún lugar, el compilador no nos dará un error.

El caso de uso principal para los sinónimos de tipo es reducir la repetición.
Por ejemplo, podríamos tener un tipo largo como este:

```rust,ignore
Box<dyn Fn() + Send + 'static>
```

Escribir este tipo extenso en firmas de función y como anotaciones de tipo
en todo el código puede ser tedioso y propenso a errores. Imagina tener un
proyecto lleno de código como el del Listado 20-25.

<Listing number="20-25" caption="Usando un tipo largo en muchos lugares">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-25/src/main.rs:here}}
```

</Listing>

Un type alias hace que este código sea más manejable al reducir la repetición.
En el Listado 20-26, hemos introducido un alias llamado `Thunk` para el tipo
extenso y podemos reemplazar todos los usos del tipo con el alias más corto
`Thunk`.

<Listing number="20-26" caption="Introduciendo un type alias `Thunk` para reducir la repetición">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-26/src/main.rs:here}}
```

</Listing>

¡Este código es mucho más fácil de leer y escribir! Elegir un nombre
significativo para un alias de tipo también puede ayudar a comunicar tu
intención (_thunk_ es una palabra para código que se evaluará en un momento
posterior, por lo que es un nombre apropiado para un cierre que se almacena).

Los type alias también se utilizan con frecuencia con el tipo `Result<T, E>`
para reducir la repetición. Considera el módulo `std::io` en la biblioteca
estándar. Las operaciones de E/S a menudo devuelven un `Result<T, E>` para
manejar situaciones en las que las operaciones no funcionan. Esta biblioteca
tiene una estructura `std::io::Error` que representa todos los posibles errores
de E/S. Muchas de las funciones en `std::io` devolverán `Result<T, E>` donde
`E` es `std::io::Error`, como estas funciones en el trait `Write`:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-05-write-trait/src/lib.rs}}
```

Él `Result<..., Error>` se repite mucho. Como tal, `std::io` tiene esta
declaración de alias de tipo:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-06-result-alias/src/lib.rs:here}}
```

Dado que esta declaración está en el módulo `std::io`, podemos usar el alias
completamente calificado `std::io::Result<T>`; es decir, un `Result<T, E>`
con `E` lleno como `std::io::Error`. Las firmas de las funciones del trait
`Write` terminan viéndose así:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-06-result-alias/src/lib.rs:there}}
```

El type alias ayuda de dos maneras: hace que el código sea más fácil de
escribir _y_ nos da una interfaz consistente en todo `std::io`. Debido a que
es un alias, es solo otro `Result<T, E>`, lo que significa que podemos usar
cualquier método que funcione en `Result<T, E>` con él, así como la sintaxis
especial como el operador `?`.

### El tipo que nunca retorna

Rust tiene un tipo especial llamado `!`, conocido en la jerga de la teoría de
tipos como _tipo vacío_ porque no tiene valores. Preferimos llamarlo _tipo
never_ porque se encuentra en el lugar del tipo de retorno cuando una función
nunca retornará. Aquí hay un ejemplo:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-07-never-type/src/lib.rs:here}}
```

Este código se lee como “la función `bar` devuelve never”. Las funciones que
devuelven never se llaman _funciones divergentes_. No podemos crear valores
del tipo `!` por lo que `bar` nunca puede devolver.

Pero, ¿qué utilidad tiene un tipo del que nunca se pueden crear valores?
Recuerda el código del Juego de Adivinar el Número mostrado en el Listado
2-5; hemos reproducido parte de él aquí en el Listado 20-27.

<Listing number="20-27" caption="Un `match` con una opción que termina en `continue`">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:ch19}}
```

</Listing>

En ese momento, omitimos algunos detalles en este código. En el Capítulo 6 en
la sección [“El operador de control de flujo
`match`”][the-match-control-flow-operator]<!-- ignore -->
discutimos que las opciones de `match` deben devolver todos el mismo tipo. Por
lo tanto, por ejemplo, el siguiente código no funciona:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-08-match-arms-different-types/src/main.rs:here}}
```

El tipo de `guess` en este código tendría que ser un entero _y_ un string, y
Rust requiere que `guess` tenga solo un tipo. Entonces, ¿qué devuelve
`continue`? ¿Cómo se nos permitió devolver un `u32` de una opción y tener otra
opción que termina con `continue` en el Listado 20-27?

Como habrás adivinado, `continue` tiene un valor `!`. Es decir, cuando Rust
calcula el tipo de `guess`, mira ambas opciones de `match`, el primero con un
valor de `u32` y el segundo con un valor de `!`. Debido a que `!` nunca puede
tener un valor, Rust decide que el tipo de `guess` es `u32`.

La forma formal de describir este comportamiento es que las expresiones de tipo
`!` se pueden convertir en cualquier otro tipo. Se nos permite terminar esta
opción de `match` con `continue` porque `continue` no devuelve un valor; en
cambio, mueve el control de nuevo a la parte superior del bucle, por lo que en
el caso de `Err`, nunca asignamos un valor a `guess`.

El tipo never también es útil con la macro `panic!`. Recordemos la función
`unwrap` que llamamos en valores `Option<T>` para producir un valor o
generar un panic con esta definición:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-09-unwrap-definition/src/lib.rs:here}}
```

En este código, ocurre lo mismo que en el `match` del Listado 20-27: Rust ve
que `val` tiene el tipo `T` y `panic!` tiene el tipo `!`, por lo que el
resultado de la expresión `match` es `T`. Este código funciona porque `panic!`
no produce un valor; termina el programa. En el caso de `None`, no devolveremos
un valor de `unwrap`, por lo que este código es válido.

Una expresión final que tiene el tipo `!` es un `loop`:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-10-loop-returns-never/src/main.rs:here}}
```

Aquí, el bucle nunca termina, por lo que `!` es el valor de la expresión. Sin
embargo, esto no sería cierto si incluyéramos un `break`, porque el bucle
terminaría cuando llegara al `break`.

### Tipos de tamano dinamico y el trait `Sized`

Rust necesita conocer ciertos detalles sobre sus tipos, como la cantidad de
espacio para asignar a un valor de un tipo particular. Esto deja una esquina de
su sistema de tipos un poco confusa al principio: el concepto de _tipos de
tamaño dinámico_. A veces se refiere como _DST_ o _tipos no dimensionados_,
estos tipos nos permiten escribir código usando valores cuyo tamaño solo
podemos conocer en tiempo de ejecución.

Profundicemos en los detalles de un tipo de tamaño dinámico llamado `str`, que
hemos estado usando a lo largo del libro. Así es, no `&str`, sino `str` por sí
solo, es un DST. No podemos saber cuánto tiempo es la cadena hasta el tiempo de
ejecución, lo que significa que no podemos crear una variable de tipo `str`, ni
podemos tomar un argumento de tipo `str`. Considera el siguiente código, que
no funciona:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-11-cant-create-str/src/main.rs:here}}
```

Rust necesita conocer cuánta memoria asignar para cualquier valor de un tipo
particular, y todos los valores de un tipo deben usar la misma cantidad de
memoria. Si Rust nos permitiera escribir este código, estos dos valores `str`
necesitarían ocupar el mismo espacio. Pero tienen longitudes diferentes: `s1`
necesita 12 bytes de almacenamiento y `s2` necesita 15. Es por eso que no es
posible crear una variable que contenga un tipo de tamaño dinámico.

Entonces, ¿qué hacemos en este caso? En este caso, como ya sabes, la solución
es hacer que los tipos de `s1` y `s2` sean `&str` en lugar de `str`. Recuerda
de la sección [“String Slices”][string-slices]<!-- ignore --> del
Capítulo 4 que la estructura de datos de slice solo almacena la posición de
inicio y la longitud del slice. Por lo tanto, aunque un `&T` es un solo
valor que almacena la dirección de memoria de donde se encuentra el `T`, un
`&str` son _dos_ valores: la dirección del `str` y su longitud. Como tal,
podemos conocer el tamaño de un valor `&str` en tiempo de compilación: es dos
veces la longitud de un `usize`. Es decir, siempre conocemos el tamaño de un
`&str`, sin importar cuán larga sea la cadena a la que se refiere. En general,
esta es la forma en que se usan los tipos de tamaño dinámico en Rust: tienen un
bit adicional de metadatos que almacena el tamaño de la información dinámica.
La regla de oro de los tipos de tamaño dinámico es que debemos envolverlos en
algún tipo de puntero.

Podemos combinar `str` con todo tipo de punteros: por ejemplo, `Box<str>` o
`Rc<str>`. De hecho, ya has visto esto antes, pero con un tipo de tamaño
dinámico diferente: los traits. Cada trait es un tipo de tamaño dinámico al que
podemos referirnos usando el nombre del trait. En el Capítulo 18 en la
sección [“Usando trait objects que permiten valores de diferentes
tipos”][usando-trait-objects-que-permiten-valores-de-diferentes-tipos]<!--
ignore -->, mencionamos que para usar traits como objetos de trait, debemos
ponerlos detrás de un puntero, como `&dyn Trait` o `Box<dyn Trait>` (`Rc<dyn
Trait>` también funcionaría).

Para trabajar con DST, Rust proporciona el trait `Sized` para determinar si el
tamaño de un tipo es conocido en tiempo de compilación o no. Este trait se
implementa automáticamente para todo lo cuyo tamaño es conocido en tiempo de
compilación. Además, Rust agrega implícitamente un límite en `Sized` a cada
función generic. Es decir, una definición de función generic como esta:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-12-generic-fn-definition/src/lib.rs}}
```

en realidad se trata como si hubiéramos escrito esto:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-13-generic-implicit-sized-bound/src/lib.rs}}
```

De forma predeterminada, las funciones generic solo funcionarán en tipos que
tienen un tamaño conocido en tiempo de compilación. Sin embargo, puede usar la
siguiente sintaxis especial para relajar esta restricción:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-14-generic-maybe-sized/src/lib.rs}}
```

Un trait bound en `?Sized` significa “`T` puede o no ser `Sized`” y esta
notación anula el valor predeterminado de que los tipos generic deben tener un
tamaño conocido en tiempo de compilación. La sintaxis `?Trait` con este
significado solo está disponible para `Sized`, no para ningún otro trait.

También debes tener en cuenta que hemos cambiado el tipo del parámetro `t` de
`T` a `&T`. Debido a que el tipo puede no ser `Sized`, necesitamos usarlo
detrás de algún tipo de puntero. En este caso, hemos elegido una referencia.

¡A continuación, hablaremos sobre funciones y closures!

[encapsulacion-que-oculta-los-detalles-de-implementacion]: ch18-01-what-is-oo.html#encapsulacion-que-oculta-los-detalles-de-implementacion
[string-slices]: ch04-03-slices.html#string-slices
[the-match-control-flow-operator]: ch06-02-match.html#the-match-control-flow-operator
[usando-trait-objects-que-permiten-valores-de-diferentes-tipos]: ch18-02-trait-objects.html#usando-trait-objects-que-permiten-valores-de-diferentes-tipos
[using-the-newtype-pattern]: ch20-03-advanced-traits.html#usando-el-pattern-newtype-para-implementar-traits-externos-en-tipos-externos
