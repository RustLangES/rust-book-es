## Incluyendo rutas al ámbito con la palabra clave `use`

Tener que escribir las rutas para llamar a las funciones puede sentirse
inconveniente y repetitivo. En el Listado 7-7, si elegimos la ruta absoluta o
relativa para la función `add_to_waitlist`, cada vez que queríamos llamar a
`add_to_waitlist` teníamos que especificar `front_of_house` y `hosting` también.
Afortunadamente, hay una manera de simplificar este proceso: podemos crear un
atajo a una ruta con la palabra clave `use` una vez, y luego usar el nombre
más corto en todas partes en el ámbito.

En el listado 7-11, traemos el módulo `crate::front_of_house::hosting` al ámbito
de la función `eat_at_restaurant` para que solo tengamos que especificar
`hosting::add_to_waitlist` para llamar a la función `add_to_waitlist` en
`eat_at_restaurant`.

<Listing number="7-11" file-name="src/lib.rs" caption="Introducir un módulo en el ámbito de aplicación con `use`">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-11/src/lib.rs}}
```

</Listing>

Agregar `use` y una ruta en un ámbito es similar a crear un enlace simbólico
en el sistema de archivos. Al agregar `use crate::front_of_house::hosting` en
la raíz del crate, hace que `hosting` sea ahora un nombre válido en ese ámbito, 
como si el módulo `hosting` hubiera sido definido en la raíz del crate. Las 
rutas traídas al ámbito con `use` también verifican la privacidad, como 
cualquier otra ruta.

Ten en cuenta que `use` solo crea el atajo para el ámbito particular en el que
ocurre él `use`. El Listado 7-12 mueve la función `eat_at_restaurant` a un
nuevo módulo hijo llamado `customer`, que es entonces un ámbito diferente al de 
la sentencia `use`, por lo que el cuerpo de la función no compilará.

<Listing number="7-12" file-name="src/lib.rs" caption="La sentencia `use` solo aplica en el ámbito donde se encuentra declarado">

```rust,noplayground,test_harness,does_not_compile,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-12/src/lib.rs}}
```

</Listing>

El error del compilador muestra que el acceso directo ya no se aplica dentro del módulo del
`customer`:

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-12/output.txt}}
```

Observe que también hay una advertencia de que él `use` ya no se utiliza en su
ámbito. Para solucionar este problema, mueva también él `use` dentro del módulo
`customer`, o haga referencia al acceso directo en el módulo padre con
`super::hosting` dentro del módulo hijo `customer`.

### Creando rutas de `use` idiomaticas

En el Listado 7-11, podrías haberte preguntado por qué especificamos
`use crate::front_of_house::hosting` y luego llamamos a `hosting::add_to_waitlist`
en `eat_at_restaurant`, en lugar de especificar toda la ruta
hasta la función `add_to_waitlist` para lograr el mismo resultado, como en el
Listado 7-13.

<Listing number="7-13" file-name="src/lib.rs" caption="Incorporando la función `add_to_waitlist` en el ámbito con `use`, que no es idiomático">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-13/src/lib.rs}}
```

</Listing>

Aunque el Listado 7-11 y 7-13 logran la misma tarea, el Listado 7-11 es la
forma idiomática de traer una función al ámbito con `use`. Traer el módulo
padre de la función al ámbito con `use` significa que tenemos que especificar
el módulo padre cuando llamamos a la función. Especificar el módulo padre
cuando llamamos a la función hace que quede claro que la función no está
definida localmente, al tiempo que minimiza la repetición de la ruta completa.
El código en el Listado 7-13 no es claro en cuanto a dónde se define
`add_to_waitlist`.

Por otro lado, cuando traemos structs, enums y otros items con `use`, es
idiomático especificar la ruta completa. El Listado 7-14 muestra la forma
idiomática de traer la struct `HashMap` de la biblioteca estándar al ámbito de
un crate binario.

<Listing number="7-14" file-name="src/main.rs" caption="Trayendo `HashMap` al ámbito de una manera idiomática">

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-14/src/main.rs}}
```

</Listing>

No hay una razón fuerte detrás de este idioma: es solo la convención que ha
surgido, y la gente se ha acostumbrado a leer y escribir código Rust de esta
manera.

La excepción a este idioma es si estamos trayendo dos elementos con el mismo 
nombre al ámbito con declaraciones `use`, porque Rust no lo permite. El Listado
7-15 muestra cómo traer dos tipos `Result` al ámbito que tienen el mismo nombre
pero módulos padres diferentes, y cómo referirse a ellos.

<Listing number="7-15" file-name="src/lib.rs" caption="Incorporando dos tipos con el mismo nombre en el mismo ámbito requiere el uso de sus módulos principales.">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-15/src/lib.rs:here}}
```

</Listing>

Como puedes ver, usar los módulos padres distingue los dos tipos `Result`. Sí,
en cambio, especificamos `use std::fmt::Result` y `use std::io::Result`,
tendríamos dos tipos `Result` en el mismo ámbito, y Rust no sabría a cuál nos
referimos cuando usamos `Result`.

### Proporcionando nuevos nombres con el Keyword `as`

Hay otra solución a este problema de traer dos elementos con el mismo nombre al
ámbito con `use`: después de la ruta, podemos especificar `as` y un nuevo
nombre local, o _alias_, para el tipo. El Listado 7-16 muestra otra forma de
escribir el código en el Listado 7-15 renombrando uno de los dos tipos
`Result` usando `as`.

<Listing number="7-16" file-name="src/lib.rs" caption="Cambiando el nombre de un tipo cuando se introduce en el ámbito con la keyword `as`">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-16/src/lib.rs:here}}
```

</Listing>

En la segunda declaración `use`, elegimos el nuevo nombre `IoResult` para el
tipo `std::io::Result`, que no entrará en conflicto con el `Result` de
`std::fmt` que también hemos traído al ámbito. El Listado 7-15 y 7-16 se
consideran idiomáticos, ¡así que la elección depende de ti!

### Re-exportando nombres con `pub use`

Cuando traemos un nombre al ámbito con la keyword `use`, el nombre es privado 
para el ámbito en el que lo importamos. Si queremos que el nombre esté
disponible para que el código que llama a nuestro código lo use, podemos
combinar `pub` y `use`. Esta técnica se llama _re-exporting_ porque estamos
trayendo un elemento al ámbito, pero también haciendo que ese elemento esté 
disponible para que otros lo traigan a su ámbito.

El listado 7-17 muestra el código del listado 7-11 con `use` en el módulo
`front_of_house` cambiado a `pub use`.

<Listing number="7-17" file-name="src/lib.rs" caption="Hacer que un nombre esté disponible para que lo use cualquier código desde un nuevo ámbito con `pub use`">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-17/src/lib.rs}}
```

</Listing>

Antes de este cambio, el código externo tendría que llamar a la función
`add_to_waitlist` usando la ruta
`restaurant::front_of_house::hosting::add_to_waitlist()`, el cual también 
debería tener requerido el modulo `front_of_house` para ser marcado como `pub`. 
Ahora que este `pub use` ha reexportado el módulo `hosting` desde el módulo raíz,
el código externo puede usar la ruta `restaurant::hosting::add_to_waitlist()` en
su lugar.

Re-exportar es útil cuando la estructura interna de tu código es diferente de
cómo los programadores que llaman a tu código pensarían sobre el dominio. Por
ejemplo, en esta metáfora de un restaurante, la gente que dirige el restaurante
piensa en “front of house” y “back of house”. Pero los clientes que visitan un
restaurante probablemente no pensarán en las partes del restaurante en esos
términos. Con `pub use`, podemos escribir nuestro código con una estructura
pero exponer una estructura diferente. Hacerlo hace que nuestra biblioteca esté
bien organizada para los programadores que trabajan en la biblioteca y los
programadores que llaman a la biblioteca. Veremos otro ejemplo de `pub use` y
cómo afecta la documentación de tu crate en la sección [“Exportando una API
pública conveniente con `pub use`”][ch14-pub-use]<!-- ignore --> del Capítulo 14.

### Usando paquetes externos

En el Capítulo 2, programamos un proyecto de juego de adivinanzas que usaba un
paquete externo llamado `rand` para obtener números aleatorios. Para usar
`rand` en nuestro proyecto, agregamos esta línea a _Cargo.toml_:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch14-03-cargo-workspaces.md
-->

<Listing file-name="Cargo.toml">

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

</Listing>

Añadir `rand` como dependencia en _Cargo.toml_ le dice a Cargo que descargue el
paquete `rand` y cualquier dependencia de [crates.io](https://crates.io/) y
haga que `rand` esté disponible para nuestro proyecto.

Luego, para llevar las definiciones de `rand` al ámbito de nuestro paquete,
agregamos una línea `use` que comienza con el nombre del paquete, `rand`, y
enumera los items que queremos traer al ámbito. Recuerda que en la sección
[“Generando un número aleatorio”][rand]<!-- ignore --> del Capítulo 2,
traíamos el trait `Rng` al ámbito y llamábamos a la función `rand::thread_rng`:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:ch07-04}}
```

Los miembros de la comunidad de Rust han puesto muchos paquetes disponibles en
[crates.io](https://crates.io/), y traer cualquiera de ellos a tu paquete
involucra estos mismos pasos: listarlos en el archivo _Cargo.toml_ de tu
paquete y usar `use` para traer items de sus crates al ámbito.

Ten en cuenta que la biblioteca estándar `std` también es una crate externa a
nuestro paquete. Debido a que la biblioteca estándar se envía con el lenguaje
Rust, no necesitamos cambiar _Cargo.toml_ para incluir `std`. Pero sí
necesitamos referirnos a él con `use` para traer items de allí al ámbito de
nuestro paquete. Por ejemplo, con `HashMap` usaríamos esta línea:

```rust
use std::collections::HashMap;
```

Esta es una ruta absoluta que comienza con `std`, el nombre del crate de la
biblioteca estándar. También podríamos escribir este `use` como:

### Usando rutas anidadas para limpiar listas `use` grandes

Si estamos usando varios elementos definidos en el mismo crate o el mismo módulo,
enumerar cada elemento en su propia línea puede ocupar mucho espacio vertical en
nuestros archivos. Por ejemplo, estas dos declaraciones `use` que teníamos en
el juego de adivinanzas en el Listado 2-4 traen items de `std` al ámbito:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-01-use-std-unnested/src/main.rs:here}}
```

</Listing>

En su lugar, podemos utilizar rutas anidadas para incluir los mismos elementos 
en una sola línea. Hacemos esto especificando la parte común de la ruta, seguida 
de dos puntos y luego entre llaves los elementos, como se muestra en el 
Listado 7-18.

<Listing number="7-18" file-name="src/main.rs" caption="Especificación de una ruta anidada para incluir en el ámbito varios elementos con el mismo prefijo">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-18/src/main.rs:here}}
```

</Listing>

En programas más grandes, traer muchos items al ámbito desde el mismo crate o
módulo usando rutas anidadas puede reducir la cantidad de declaraciones `use`
necesarias en gran medida.

Podemos usar una ruta anidada en cualquier nivel de una ruta, lo que es útil 
cuando combinamos dos sentencias `use` que comparten una sub-ruta. Por ejemplo, 
el Listado 7-19 muestra dos sentencias use: una que trae `std::io` al ámbito y
otra que trae `std::io::Write` al ámbito.

<Listing number="7-19" file-name="src/lib.rs" caption="Dos sentencias `use` donde una es una sub-ruta de la otra">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-19/src/lib.rs}}
```

</Listing>

La parte común de estas dos rutas es `std::io`, así que podemos usar una ruta
anidada para traer ambos al ámbito en una línea, como se muestra en el Listado
7-20.

<Listing number="7-20" file-name="src/lib.rs" caption="Combinando las rutas del listado 7-19 en una sentencia `use`">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-20/src/lib.rs}}
```

</Listing>

Esta línea trae `std::io` y `std::io::Write` al ámbito.

### El Operador Asterisco (Glob)

Si queremos incluir al ámbito _todos_ los elementos públicos definidos en una 
ruta, podemos especificar esa ruta seguido del operador glob `*`:

```rust
use std::collections::*;
```

Esta sentencia `use` trae todos los elementos públicos definidos en 
`std::collections` al ámbito actual. Tenga cuidado al utilizar el operador 
`glob`. El operador `glob` puede hacer más difícil saber qué elementos están en 
el ámbito y dónde se definió un elemento que este siendo utilizado en su 
programa. Además, si la dependencia cambia sus definiciones, lo que has 
importado también cambia, lo que puede provocar errores de compilación cuando 
actualices la dependencia, por ejemplo, si la dependencia añade una definición 
con el mismo nombre que una definición tuya en el mismo ámbito.

El operador glob se utiliza a menudo cuando se realizan pruebas para llevar todo 
lo que se está probando al módulo de `pruebas`; hablaremos de ello en la sección 
["Cómo escribir pruebas"][writing-tests]<!-- ignore --> del capítulo 11. El 
operador glob también se utiliza a veces como parte del patrón prelude: consulte 
[la documentación de la biblioteca estándar](../std/prelude/index.html#other-preludes)<!-- ignore --> 
para obtener más información sobre ese patrón.

[ch14-pub-use]: ch14-02-publishing-to-crates-io.html#exportando-una-api-publica-conveniente-con-pub-use
[rand]: ch02-00-guessing-game-tutorial.html#generar-un-numero-aleatorio
[writing-tests]: ch11-01-writing-tests.html#como-escribir-tests
