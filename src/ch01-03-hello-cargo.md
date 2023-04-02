## ¡Hola, Cargo!

Cargo es el sistema de compilación y administrador de paquetes de Rust. La
mayoría de los Rustaceans usan esta herramienta para administrar sus proyectos
Rust porque Cargo maneja muchas tareas para usted, como compilar su código,
descargar las bibliotecas de las que depende su código y compilar esas
bibliotecas. (Llamamos a las bibliotecas de las que su código depende
*dependencias*).

Los programas Rust más simples, como el que hemos escrito hasta ahora, no
tienen dependencias. Si hubiéramos construido el proyecto “¡Hola, mundo!” con
Cargo, solo usaría la parte de Cargo que maneja la compilación de su código. A
medida que escriba programas Rust más complejos, agregará dependencias, y si
comienza un proyecto usando Cargo, agregar dependencias será mucho más fácil
de hacer.

Debido a que la gran mayoría de los proyectos Rust usan Cargo, el resto de este
libro asume que también está usando Cargo. Cargo viene instalado con Rust si
usó los instaladores oficiales que se discuten en la sección
[“Installation”][installation]<!-- ignore -->. Si instaló Rust a través de
algunos otros medios, verifique si Cargo está instalado ingresando lo siguiente
en su terminal:

```console
$ cargo --version
```

Si ve un número de versión, ¡lo tiene! Si ve un error, como `command not found`,
consulte la documentación de su método de instalación para determinar cómo
instalar Cargo por separado.

### Creación de un proyecto con Cargo

Vamos a crear un nuevo proyecto usando Cargo y ver cómo difiere de nuestro
original proyecto “¡Hola, mundo!”. Navegue de vuelta a su directorio
*proyectos* (o dondequiera que haya decidido almacenar su código). Luego, en
cualquier sistema operativo, ejecute lo siguiente:

```console
$ cargo new hello_cargo
$ cd hello_cargo
```

El primer comando crea un nuevo directorio y proyecto llamado *hello_cargo*.
Hemos nombrado a nuestro proyecto *hello_cargo*, y Cargo crea sus archivos en
un directorio con el mismo nombre.

Vaya al directorio *hello_cargo* y liste los archivos. Verá que Cargo ha
generado dos archivos y un directorio para nosotros: un archivo *Cargo.toml* y
un directorio *src* con un archivo *main.rs* dentro.

También ha inicializado un nuevo repositorio Git junto con un archivo
*.gitignore*. Los archivos Git no se generarán si ejecuta `cargo new` dentro
de un repositorio Git existente; puede anular este comportamiento usando
`cargo new --vcs=git`.

> Nota: Git es un sistema de control de versiones común. Puede cambiar `cargo
> new` para usar un sistema de control de versiones diferente o ningún sistema
> de control de versiones usando la bandera `--vcs`. Ejecute `cargo new --help`
> para ver las opciones disponibles.

Abra *Cargo.toml* en su editor de texto de elección. Debería verse similar al
código de la Lista 1-2.

<span class="filename">Nombre de archivo: Cargo.toml</span>

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

<span class="caption">Lista 1-2: Contenido de *Cargo.toml* generado por `cargo
new`</span>

Este archivo está en el formato [*TOML*][toml]<!-- ignore --> (*Tom’s Obvious,
Minimal Language*), que es el formato de configuración de Cargo.

La primera línea, `[package]`, es un encabezado de sección que indica que las
siguientes declaraciones están configurando un paquete. A medida que agreguemos
más información a este archivo, agregaremos otras secciones.

Las próximas tres líneas establecen la información de configuración que Cargo
necesita para compilar su programa: el nombre, la versión y la edición de Rust
que se usará. Hablaremos sobre la clave `edition` en [Apéndice E][appendix-e]
<!-- ignore -->.

La última línea, `[dependencies]`, es el comienzo de una sección para que
enumere cualquier dependencia de su proyecto. En Rust, los paquetes de código
se denominan *crates*. No necesitaremos otros crates para este proyecto, pero
lo haremos en el primer proyecto del Capítulo 2, por lo que usaremos esta
sección de dependencias entonces.

Ahora abra *src/main.rs* y eche un vistazo:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

¡Cargo ha generado un programa “Hello, world!”/“¡Hola, mundo!” para usted, 
¡igual que el que escribimos en la Lista 1-1! Hasta ahora, las diferencias 
entre nuestro proyecto y el proyecto generado por Cargo son que Cargo 
colocó el código en el directorio *src* y tenemos un archivo de 
configuración *Cargo.toml* en el directorio superior.

Cargo espera que sus archivos de origen vivan dentro del directorio *src*. El
directorio del proyecto de nivel superior es solo para archivos README, 
información de licencia, archivos de configuración y cualquier otra cosa 
que no esté relacionada con su código. Usar Cargo le ayuda a organizar 
sus proyectos. Hay un lugar para todo, y todo está en su lugar.

Si comenzó un proyecto que no usa Cargo, como hicimos con el proyecto
“¡Hola, mundo!”, puede convertirlo en un proyecto que sí use Cargo. Mueva el
código del proyecto al directorio *src* y cree un archivo *Cargo.toml*
adecuado.

### Construir y ejecutar un proyecto de Cargo

Ahora veamos qué es diferente cuando construimos y ejecutamos el programa
“¡Hola, mundo!” con Cargo. ¡Desde su directorio *hello_cargo*, construya su
proyecto ingresando el siguiente comando:

```console
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

Este comando crea un archivo ejecutable en *target/debug/hello_cargo* (o
*target\debug\hello_cargo.exe* en Windows) en lugar de en su directorio
actual. Debido a que la compilación predeterminada es una compilación de
depuración, Cargo coloca el binario en un directorio llamado *debug*. Puede
ejecutar el ejecutable con este comando:

```console
$ ./target/debug/hello_cargo # o .\target\debug\hello_cargo.exe en Windows
Hello, world!
```

Si todo va bien, `Hello, world!` debería imprimirse en la terminal. Ejecutar
`cargo build` por primera vez también hace que Cargo cree un nuevo archivo en
el nivel superior: *Cargo.lock*. Este archivo rastrea las versiones exactas de
las dependencias de su proyecto. Este proyecto no tiene dependencias, por lo
que el archivo es un poco escaso. Nunca necesitará cambiar este archivo
manualmente; Cargo administra sus contenidos para usted.

Acabamos de construir un proyecto con `cargo build` y ejecutarlo con
`./target/debug/hello_cargo`, pero también podemos usar `cargo run` para
compilar el código y luego ejecutar el ejecutable resultante en un solo
comando:

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Usar `cargo run` es más conveniente que tener que recordar ejecutar `cargo
build` y luego usar la ruta completa al binario, por lo que la mayoría de los
desarrolladores usan `cargo run`.

Tenga en cuenta que esta vez no vimos salida que indicara que Cargo estaba
compilando `hello_cargo`. Cargo supo que los archivos no habían cambiado, por
lo que no volvió a construir, sino que solo ejecutó el binario. Si hubiera
modificado su código fuente, Cargo habría reconstruido el proyecto antes de
ejecutarlo, y habría visto esta salida:

```console
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Cargo también proporciona un comando llamado `cargo check`. Este comando
comprueba rápidamente su código para asegurarse de que compila, pero no
produce un ejecutable:

```console
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

¿Por qué no querría un ejecutable? A menudo, `cargo check` es mucho más rápido
que `cargo build` porque omite el paso de producir un ejecutable. Si está
verificando continuamente su trabajo mientras escribe el código, usar
`cargo check` acelerará el proceso de informarle si su proyecto todavía está
compilando. ¡Por lo tanto, muchos Rustaceans ejecutan `cargo check` de
periodicamente mientras escriben su programa para asegurarse de que compila.
Luego ejecutan `cargo build` cuando están listos para usar el ejecutable.

Resumamos lo que hemos aprendido hasta ahora sobre Cargo:

* Podemos crear un proyecto usando `cargo new`.
* Podemos construir un proyecto usando `cargo build`.
* Podemos construir y ejecutar un proyecto en un solo paso usando `cargo run`.
* Podemos construir un proyecto sin producir un binario para verificar errores
  usando `cargo check`.
* En lugar de guardar el resultado de la compilación en el mismo directorio que
  nuestro código, Cargo lo almacena en el directorio *target/debug*.

Una ventaja adicional de usar Cargo es que los comandos son los mismos sin
importar en qué sistema operativo esté trabajando. Por lo tanto, en este
punto, ya no proporcionaremos instrucciones específicas para Linux y macOS
versus Windows.

### Construyendo versión de lanzamiento

Cuando su proyecto finalmente esté listo para su lanzamiento, puede usar `cargo
build --release` para compilarlo con optimizaciones. Este comando creará un
ejecutable en *target/release* en lugar de *target/debug*. Las optimizaciones
hacen que su código Rust se ejecute más rápido, pero al activarlos se alarga el
tiempo que tarda su programa en compilarse. Es por eso que hay dos perfiles
diferentes: uno para el desarrollo, cuando desea reconstruir rápidamente y
con frecuencia, y otro para construir el programa final que le dará al usuario
que no se reconstruirá repetidamente y que se ejecutará lo más rápido posible.
Si está midiendo el tiempo de ejecución de su código, asegúrese de ejecutar
`cargo build --release` y realizar la prueba de rendimiento con el ejecutable
en *target/release*.

### Cargo como convención

Con proyectos simples, Cargo no proporciona mucho valor sobre solo usar
`rustc`, pero demostrará su valor a medida que sus programas se vuelvan más
intrincados. Una vez que los programas crecen a múltiples archivos o necesitan
una dependencia, es mucho más fácil dejar que Cargo coordine la construcción.

Aunque el proyecto `hello_cargo` es simple, ahora usa mucho de la herramienta
real que usará en el resto de su carrera de Rust. De hecho, para trabajar en
cualquier proyecto existente, puede usar los siguientes comandos para verificar
el código usando Git, cambiar al directorio del proyecto y construir:

```console
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```

Para obtener más información sobre Cargo, consulte [su documentación][cargo].

## Resumen

¡Ya estás en un gran comienzo en tu viaje de Rust! En este capítulo, has
aprendido cómo:

* Instalar la última versión estable de Rust usando `rustup`
* Actualizar a una versión más reciente de Rust
* Abrir documentación instalada localmente
* Escribir y ejecutar un programa "¡Hola, mundo!" usando `rustc` directamente
* Crear y ejecutar un nuevo proyecto usando las convenciones de Cargo

Es un buen momento para construir un programa más sustancial para acostumbrarse
a leer y escribir código Rust. Entonces, en el capítulo 2, construiremos un
programa de juego de adivinanzas. Si prefiere comenzar aprendiendo cómo
funcionan los conceptos de programación comunes en Rust, consulte el capítulo 3
y luego regrese al capítulo 2.

[installation]: ch01-01-installation.html#installation
[toml]: https://toml.io
[appendix-e]: appendix-05-editions.html
[cargo]: https://doc.rust-lang.org/cargo/
