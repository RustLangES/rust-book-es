## Cargo Workspaces

En el Capítulo 12, construimos un paquete que incluía un *crate* binario y un
*crate* de biblioteca. A medida que su proyecto se desarrolle, es posible que
encuentre que el *crate* de biblioteca continúa creciendo y que desea dividir
su paquete aún más en múltiples *crate* de biblioteca. Cargo ofrece una
característica llamada *workspaces* que puede ayudar a administrar múltiples
paquetes relacionados que se desarrollan en tándem.

### Creando un Workspace

Un *workspace* es un conjunto de paquetes que comparten el mismo *Cargo.lock* y
el directorio de salida. Hagamos un proyecto usando un *workspace* - usaremos
código trivial para que podamos concentrarnos en la estructura del
*workspace*. Hay varias formas de estructurar un *workspace*, así que solo
mostraremos una forma común. Tendremos un *workspace* que contiene un binario y
dos bibliotecas. El binario, que proporcionará la funcionalidad principal,
dependerá de las dos bibliotecas. Una biblioteca proporcionará una función
`add_one`, y una segunda biblioteca una función `add_two`. Estas tres cajas
serán parte del mismo *workspace*. Comenzaremos creando un nuevo directorio
para el *workspace*:

```console
$ mkdir add
$ cd add
```

Luego, en el directorio *add*, crearemos el archivo *Cargo.toml* que
configurará todo el *workspace*. Este archivo no tendrá una sección `[package]`.
En su lugar, comenzará con una sección `[workspace]` que nos permitirá agregar
miembros al *workspace*. También nos esforzamos en utilizar las últimas y mejores
versión del algoritmo de resolución de Cargo en nuestro workspace configurando el
`resolver` en `"2"`. Especificando la ruta al paquete con nuestro *crate*
binario; en este caso, esa ruta es *adder*:

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-01-workspace/add/Cargo.toml}}
```

A continuación, crearemos el crate binario `adder` ejecutando `cargo new` 
dentro del directorio *add*:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-01-adder-crate/add
remove `members = ["adder"]` from Cargo.toml
rm -rf adder
cargo new adder
copy output below
-->

```console
$ cargo new adder
    Creating binary (application) `adder` package
      Adding `adder` as member of workspace at `file:///projects/add`
```

Ejecutando `cargo new` dentro de un espacio de trabajo también agrega 
automáticamente el recién creado paquete a la clave `members` en la definición 
`[workspace]` en el `Cargo.toml`, así:

```toml
{{#include ../listings/ch14-more-about-cargo/output-only-01-adder-crate/add/Cargo.toml}}
```

En este punto, podemos construir el *workspace* ejecutando `cargo build`. Los
archivos en su directorio *add* deberían verse así:

```text
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

El *workspace* tiene un directorio *target* en el nivel superior que contendrá
los artefactos compilados. El paquete `adder` no tiene su propio directorio
*target*. Incluso si ejecutáramos `cargo build` desde dentro del directorio
*adder*, los artefactos compilados aún terminarían en *add/target* en lugar de
*add/adder/target*. Cargo estructura el directorio *target* en un *workspace*
de esta manera porque los *crate* en un *workspace* están destinados a
dependerse entre sí. Si cada *crate* tuviera su propio directorio *target*,
cada *crate* tendría que volver a compilar cada uno de los otros *crate* en el
*workspace* para colocar los artefactos en su propio directorio *target*. Al
compartir un directorio *target*, los *crate* pueden evitar la reconstrucción
innecesaria.

### Creando el Segundo Paquete en el Workspace

A continuación crearemos otro paquete miembro en el *workspace* y lo llamaremos
`add_one`. Cambie el *Cargo.toml* de nivel superior para especificar la ruta
*add_one* en la lista de `members`:

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/Cargo.toml}}
```

Luego generaremos un nuevo *crate* de biblioteca llamado `add_one`:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-02-add-one/add
remove `"add_one"` from `members` list in Cargo.toml
rm -rf add_one
cargo new add_one --lib
copy output below
-->

```console
$ cargo new add_one --lib
    Creating library `add_one` package
      Adding `add_one` as member of workspace at `file:///projects/add`
```

Tu directorio *add* debería tener estos directorios y archivos:

```text
├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

En el archivo *add_one/lib.rs*, agreguemos una función `add_one`:

<span class="filename">Filename: add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/add_one/src/lib.rs}}
```

Ahora podemos hacer que el paquete `adder` con nuestro binario dependa del 
paquete `add_one` con nuestra biblioteca. Primero, necesitaremos agregar una
dependencia de ruta en *adder/Cargo.toml*.

<span class="filename">Filename: adder/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/adder/Cargo.toml:6:7}}
```

Cargo no asume que los crates en un *workspace* dependerán entre sí, por lo que
necesitamos ser explícitos sobre las relaciones de dependencia.

A continuación, usaremos la función `add_one` (del *crate* `add_one`) en el
crate `adder`. Abra el archivo *adder/src/main.rs* y agregue una línea `use`
en la parte superior para traer el nuevo *crate* de biblioteca `add_one` al
alcance. Luego cambie la función `main` para llamar a la función `add_one`, como
en el Listado 14-7.

<Listing number="14-7" file-name="adder/src/main.rs" caption="Usando el crate de biblioteca `add_one` desde el crate `adder`">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-07/add/adder/src/main.rs}}
```

</Listing>

¡Construyamos el workspace ejecutando `cargo build` en el directorio superior
*add*!

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
```

Para ejecutar el crate binario desde el directorio *add*, podemos especificar
qué paquete en el *workspace* queremos ejecutar con el argumento `-p` y el
nombre del paquete con `cargo run`:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo run -p adder
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo run -p adder
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
```

Esto ejecuta el código en *adder/src/main.rs*, que depende del crate `add_one`.

#### Dependiendo de un Paquete Externo en un Workspace

Observa que el workspace tiene solo un archivo *Cargo.lock* en el nivel
superior, en lugar de tener un *Cargo.lock* en cada directorio de *crate*.
Esto asegura que todos los *crate* estén usando la misma versión de todas las
dependencias. Si agregamos el paquete `rand` al *Cargo.toml* de *adder* y
*add_one*, Cargo resolverá ambos a una versión de `rand` y lo registrará en el
único *Cargo.lock*. Hacer que todos los *crate* en el *workspace* usen las
mismas dependencias significa que los *crate* siempre serán compatibles entre
sí. Agreguemos el *crate* `rand` a la sección `[dependencies]` en el archivo
*add_one/Cargo.toml* para que podamos usar el *crate* `rand` en el *crate*
`add_one`:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
-->

<span class="filename">Filename: add_one/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add/add_one/Cargo.toml:6:7}}
```

Ahora podemos agregar `use rand;` al archivo *add_one/src/lib.rs*, y construir
todo el *workspace* ejecutando `cargo build` en el directorio *add* traerá e
compilará el *crate* `rand`. Obtendremos una advertencia porque no nos estamos
refiriendo al `rand` que trajimos al scope:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.5
   --snip--
   Compiling rand v0.8.5
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
warning: unused import: `rand`
 --> add_one/src/lib.rs:1:5
  |
1 | use rand;
  |     ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `add_one` (lib) generated 1 warning (run `cargo fix --lib -p add_one` to apply 1 suggestion)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.95s
```

El archivo *Cargo.lock* de nivel superior ahora contiene información sobre la
dependencia de `add_one` en `rand`. Sin embargo, aunque `rand` se usa en algún
lugar del *workspace*, no podemos usarlo en otros *crate* del *workspace* a
menos que agreguemos `rand` a sus archivos *Cargo.toml* también. Por ejemplo,
si agregamos `use rand;` al archivo *adder/src/main.rs* para el paquete
`adder`, obtendremos un error:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-03-use-rand/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
  --snip--
   Compiling adder v0.1.0 (file:///projects/add/adder)
error[E0432]: unresolved import `rand`
 --> adder/src/main.rs:2:5
  |
2 | use rand;
  |     ^^^^ no external crate `rand`
```

Para solucionar esto, edita el archivo *Cargo.toml* del paquete `adder` e
indica que `rand` es una dependencia para él también. Construir el paquete
`adder` agregará `rand` a la lista de dependencias para `adder` en
*Cargo.lock*, pero no se descargarán copias adicionales de `rand`. Cargo se 
asegurara de que cada *crate* en cada paquete en el *workspace* que usa el
paquete `rand` estará usando la misma versión siempre y cuando se especifiquen 
como versiones compatibles de `rand`, ahorrándonos espacio y asegurando que los 
*crate* en el *workspace* serán compatibles entre sí.

#### Agregando un Test a un Workspace

Para otra mejora, agreguemos una prueba de la función `add_one::add_one` dentro
del *crate* `add_one`:

<span class="filename">Filename: add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add/add_one/src/lib.rs}}
```

Ahora ejecutamos `cargo test` en el directorio superior *add* para ejecutar los 
tests una estructura de workspace como esta ejecutará los tests para todos los
crates en el workspace:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test
copy output below; the output updating script doesn't handle subdirectories in
paths properly
-->

```console
$ cargo test
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running unittests src/lib.rs (target/debug/deps/add_one-93c49ee75dc46543)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/adder-3a47283c568d2b6a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

La primera sección del output muestra que el test `it_works` en el crate
`add_one` pasó. La siguiente sección muestra que no se encontraron tests en el
crate `adder`, y luego la última sección muestra que no se encontraron tests de
documentación en el crate `add_one`.

También podemos ejecutar tests para un crate en particular en el workspace
desde el directorio superior usando la bandera `-p` y especificando el nombre
del crate que queremos testear:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test -p add_one
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo test -p add_one
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/add_one-93c49ee75dc46543)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Este output muestra que `cargo test` solo ejecutó los tests para el crate
`add_one` y no ejecutó los tests del crate `adder`.

Si tu publicas los crates en el workspace en [crates.io](https://crates.io/),
cada crate en el workspace necesitará ser publicado por separado. Como `cargo
test`, podemos publicar un crate en particular en nuestro workspace usando la
bandera `-p` y especificando el nombre del crate que queremos publicar.

Para practicar aún más, agrega un crate `add_two` a este workspace de manera
similar al crate `add_one`!

Conforme tu proyecto crece, considera usar un workspace: es más fácil de entender
componentes pequeños e individuales que un gran blob de código. Además,
mantener los crates en un workspace puede hacer que la coordinación entre
crates sea más fácil si se cambian a menudo al mismo tiempo.
