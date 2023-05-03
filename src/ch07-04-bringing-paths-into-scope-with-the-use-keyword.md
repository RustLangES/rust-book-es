## Llevando rutas al Scope con la palabra clave `use`

Tener que escribir las rutas para llamar a las funciones puede sentirse
inconveniente y repetitivo. En el Listado 7-7, si elegimos la ruta absoluta o
relativa para la función `add_to_waitlist`, cada vez que queríamos llamar a
`add_to_waitlist` teníamos que especificar `front_of_house` y `hosting` también.
Afortunadamente, hay una manera de simplificar este proceso: podemos crear un
atajo a una ruta con la palabra clave `use` una vez, y luego usar el nombre
más corto en todas partes en el ámbito.

En el Listing 7-11, traemos el módulo `crate::front_of_house::hosting` al ámbito
de la función `eat_at_restaurant` para que solo tengamos que especificar
`hosting::add_to_waitlist` para llamar a la función `add_to_waitlist` en
`eat_at_restaurant`.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-11/src/lib.rs}}
```

<span class="caption">Listing 7-11: Colocando un módulo en el scope con
`use`</span>

Agregar `use` y un path en un ámbito es similar a crear un enlace simbólico
en el sistema de archivos. Al agregar `use crate::front_of_house::hosting` en
la raíz del crate, `hosting` es ahora un nombre válido en ese ámbito, como si
el módulo `hosting` hubiera sido definido en la raíz del crate. Los paths
traídas al ámbito con `use` también verifican la privacidad, como cualquier
otro path.

Ten en cuenta que `use` solo crea el atajo para el ámbito particular en el que
ocurre él `use`. El Listado 7-12 mueve la función `eat_at_restaurant` a un
nuevo módulo hijo llamado `customer`, que es entonces un ámbito diferente que
el `use` declaración, por lo que el cuerpo de la función no se compilará:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,test_harness,does_not_compile,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-12/src/lib.rs}}
```

<span class="caption">Listing 7-12: Una declaración de `use` solo aplica en el 
scope</span>

El compilador muestra un error que el atajo ya no se aplica dentro del módulo
`customer`:

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-12/output.txt}}
```

Observa que también hay una advertencia de que él `use` ya no se usa en su
ámbito. Para solucionar este problema, mueve él `use` dentro del módulo
`customer` también, o referencia el atajo en el módulo padre con
`super::hosting` dentro del módulo hijo `customer`.

### Creando Paths `use` Idiomáticos

En el Listado 7-11, podrías haberte preguntado por qué especificamos
`use crate::front_of_house::hosting` y luego llamamos a `hosting::add_to_waitlist`
en `eat_at_restaurant` en lugar de especificar la ruta `use` todo el camino
hasta la función `add_to_waitlist` para lograr el mismo resultado, como en el
Listado 7-13.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-13/src/lib.rs}}
```

<span class="caption">Listing 7-13: Incorporando la función `add_to_waitlist`
en el scope con `use`, que es unidiomático</span>

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

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-14/src/main.rs}}
```

<span class="caption">Listing 7-14: Trayendo `HashMap` al scope de una
manera idiomática</span>

No hay una razón fuerte detrás de este idiom: es solo la convención que ha
surgido, y la gente se ha acostumbrado a leer y escribir código Rust de esta
manera.

La excepción a este idiom es si estamos trayendo dos items con el mismo nombre
en el ámbito con declaraciones `use`, porque Rust no lo permite. El Listado
7-15 muestra cómo traer dos tipos `Result` al ámbito que tienen el mismo nombre
pero módulos padres diferentes y cómo referirse a ellos.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-15/src/lib.rs:here}}
```

<span class="caption">Listing 7-15: Incorporando dos tipos con el mismo nombre 
en el mismo scope requiere el uso de sus módulos principales.</span>

Como puedes ver, usar los módulos padres distingue los dos tipos `Result`. Sí,
en cambio, especificamos `use std::fmt::Result` y `use std::io::Result`, 
tendríamos dos tipos `Result` en el mismo ámbito y Rust no sabría a cuál nos 
referimos cuando usamos `Result`.

### Proporcionando nuevos nombres con el Keyword `as`

Hay otra solución a este problema de traer dos tipos con el mismo nombre en el
ámbito con `use`: después de la ruta, podemos especificar `as` y un nuevo
nombre local, o *alias*, para el tipo. El Listado 7-16 muestra otra forma de
escribir el código en el Listado 7-15 renombrando uno de los dos tipos
`Result` usando `as`.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-16/src/lib.rs:here}}
```

<span class="caption">Listing 7-16: Cambiando el nombre de un tipo cuando se
introduce en el scope con la keyword `as`</span>

En la segunda declaración `use`, elegimos el nuevo nombre `IoResult` para el
tipo `std::io::Result`, que no entrará en conflicto con el `Result` de
`std::fmt` que también hemos traído al ámbito. El Listado 7-15 y 7-16 se
consideran idiomáticos, ¡así que la elección depende de ti!

### Re-exportando nombres con `pub use`

Cuando traemos un nombre al Scope con la keyword `use`, el nombre está
disponible en ese ámbito de forma privada. Si queremos que el nombre esté
disponible para que el código que llama a nuestro código lo use, podemos
combinar `pub` y `use`. Esta técnica se llama *re-exporting* porque estamos
trayendo un item al ámbito, pero también haciendo que ese item esté disponible
para que otros lo traigan a su Scope.

Listing 7-11 muestra el código en Listing 7-10 con `use` en el módulo
`front_of_house` cambiado a `pub use`.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-17/src/lib.rs}}
```

<span class="caption">Listing 7-17: Hacer que un nombre esté disponible para que 
lo use cualquier código desde un nuevo scope con `pub use`</span>

Antes de este cambio, el código externo tendría que llamar a la función
`add_to_waitlist` usando la ruta 
`restaurant::front_of_house::hosting::add_to_waitlist()`. Ahora que este 
`pub use` ha reexportado el módulo `hosting` desde el módulo raíz, el código 
externo puede usar la ruta `restaurant::hosting::add_to_waitlist()` en su lugar.

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
pública conveniente con `pub use`”][ch14-pub-use]<!-- ignore --> del Capítulo
14.

### Usando paquetes externos

En el Capítulo 2, programamos un proyecto de juego de adivinanzas que usaba un
paquete externo llamado `rand` para obtener números aleatorios. Para usar
`rand` en nuestro proyecto, agregamos esta línea a *Cargo.toml*:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

Añadir `rand` como dependencia en *Cargo.toml* le dice a Cargo que descargue el
paquete `rand` y cualquier dependencia de [crates.io](https://crates.io/) y
haga que `rand` esté disponible para nuestro proyecto.

Luego, para llevar las definiciones de `rand` al Scope de nuestro paquete,
agregamos una línea `use` que comienza con el nombre del paquete, `rand`, y
enumera los items que queremos traer al Scope. Recuerda que en la sección
[“Generando un número aleatorio”][rand]<!-- ignore --> del Capítulo 2,
traíamos el trait `Rng` al Scope y llamábamos a la función `rand::thread_rng`:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:ch07-04}}
```

Los miembros de la comunidad de Rust han puesto muchos paquetes disponibles en
[crates.io](https://crates.io/), y traer cualquiera de ellos a tu paquete
involucra estos mismos pasos: listarlos en el archivo *Cargo.toml* de tu
paquete y usar `use` para traer items de sus crates al Scope.

Ten en cuenta que la biblioteca estándar `std` también es una crate externa a
nuestro paquete. Debido a que la biblioteca estándar se envía con el lenguaje
Rust, no necesitamos cambiar *Cargo.toml* para incluir `std`. Pero sí
necesitamos referirnos a él con `use` para traer items de allí al Scope de
nuestro paquete. Por ejemplo, con `HashMap` usaríamos esta línea:

```rust
use std::collections::HashMap;
```

Esta es un path absoluto que comienza con `std`, el nombre del crate de la
biblioteca estándar. También podríamos escribir este `use` como:

### Usando Paths Anidados para Limpiar Listas `use` Grandes

Si estamos usando varios items definidos en el mismo crate o el mismo módulo,
enumerar cada item en su propia línea puede ocupar mucho espacio vertical en
nuestros archivos. Por ejemplo, estas dos declaraciones `use` que teníamos en
el juego de adivinanzas en el Listado 2-4 traen items de `std` al Scope:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-01-use-std-unnested/src/main.rs:here}}
```

En su lugar, podemos usar paths anidados para traer los mismos items al Scope
en una línea. Hacemos esto especificando la parte común del path, seguida de
dos puntos y luego llaves alrededor de una lista de las partes de los paths que
difieren, como se muestra en el Listado 7-18.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-18/src/main.rs:here}}
```

<span class="caption">Listing 7-18: Especificación de un path anidado 
path para incluir varios items con el mismo prefix en el scope</span>

En programas más grandes, traer muchos items al Scope desde el mismo crate o
módulo usando paths anidados puede reducir la cantidad de declaraciones `use`
separadas necesarias en gran medida.

Podemos usar un path anidado en cualquier nivel de un path, lo cual es útil
cuando combinamos dos declaraciones `use` que comparten un subpath. Por
ejemplo, el Listado 7-19 muestra dos declaraciones `use`: una que trae `std::io`
al Scope y otra que trae `std::io::Write` al Scope.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-19/src/lib.rs}}
```

<span class="caption">Listing 7-19: Dos sentencias `use` donde una es un 
sub-path del otro</span>

La parte común de estos dos paths es `std::io`, así que podemos usar un path
anidado para traer ambos al Scope en una línea, como se muestra en el Listado
7-20.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-20/src/lib.rs}}
```

<span class="caption">Listing 7-20: Combinando los paths del Listing 7-19 en
una instrucción `use`</span>

Esta línea trae `std::io` y `std::io::Write` al Scope.

### El Operador Glob

Si queremos incluir al Scope *todos* los items públicos definidos en un path,
podemos especificar ese path seguido del operador glob `*`:

```rust
use std::collections::*;
```

Este `use` trae todos los items públicos definidos en `std::collections` al
Scope actual. ¡Ten cuidado cuando uses el operador glob! Glob puede hacer más
difícil decir qué nombres están en el Scope y dónde se definió un nombre usado
en tu programa.

El operador glob es a veces usado cuando se prueba para traer todo bajo prueba
al módulo `tests`; hablaremos de eso en la sección [“Cómo Escribir
Pruebas”][writing-tests]<!-- ignore --> en el Capítulo 11. El operador glob
también es usado a veces como parte del patrón prelude: ve [la documentación de
la biblioteca estándar](../std/prelude/index.html#other-preludes)<!-- ignore -->
para más información sobre ese patrón.

[ch14-pub-use]: ch14-02-publishing-to-crates-io.html#exporting-a-convenient-public-api-with-pub-use
[rand]: ch02-00-guessing-game-tutorial.html#generating-a-random-number
[writing-tests]: ch11-01-writing-tests.html#how-to-write-tests
