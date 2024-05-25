## ¡Hola, Cargo!

Cargo es el sistema de compilación y administrador de paquetes de Rust. La
mayoría de los Rustaceans usan esta herramienta para administrar sus proyectos
Rust porque Cargo maneja muchas tareas para ti, como compilar tu código,
descargar las bibliotecas de las que depende tu código y compilar esas
bibliotecas. (Llamamos *dependencias* a las bibliotecas de las que depende tu
código).

Los programas Rust más simples, como el que hemos escrito hasta ahora, no
tienen dependencias. Si hubiéramos construido el proyecto “¡Hola, mundo!” con
Cargo, sólo usaría la parte de Cargo que maneja la compilación de tu código. A
medida que escribas programas Rust más complejos, agregarás dependencias, y si
comienzas un proyecto usando Cargo, agregar dependencias será mucho más fácil
de hacer.

Debido a que la gran mayoría de los proyectos Rust usan Cargo, el resto de este
libro asume que también estás usando Cargo. Cargo viene instalado con Rust si
usaste los instaladores oficiales que se discuten en la sección
[“Installation”][installation]<!-- ignore -->. Si instalaste Rust a través de
algunos otros medios, verifica si Cargo está instalado ingresando lo siguiente
en tu terminal:

```console
$ cargo --version
```

Si ves un número de versión, ¡lo tienes! Si ves un error, como `command not found`,
consulta la documentación de tu método de instalación para determinar cómo
instalar Cargo por separado.

### Creación de un proyecto con Cargo

Vamos a crear un nuevo proyecto usando Cargo y ver cómo difiere de nuestro
proyecto original “¡Hola, mundo!”. Navega de vuelta a tu directorio
*proyectos* (o dondequiera que hayas decidido almacenar tu código). Luego, en
cualquier sistema operativo, ejecuta lo siguiente:

```console
$ cargo new hello_cargo
$ cd hello_cargo
```

El primer comando crea un nuevo directorio y proyecto llamado *hello_cargo*.
Hemos nombrado a nuestro proyecto *hello_cargo*, y Cargo crea sus archivos en
un directorio con el mismo nombre.

Ve al directorio *hello_cargo* y lista los archivos. Verás que Cargo ha
generado dos archivos y un directorio para nosotros: un archivo *Cargo.toml* y
un directorio *src* con un archivo *main.rs* dentro.

También ha inicializado un nuevo repositorio Git junto con un archivo
*.gitignore*. Los archivos Git no se generarán si ejecutas `cargo new` dentro
de un repositorio Git existente; puedes anular este comportamiento usando
`cargo new --vcs=git`.

> Nota: Git es un sistema de control de versiones común. Puedes cambiar `cargo
> new` para usar un sistema de control de versiones diferente o ningún sistema
> de control de versiones usando la bandera `--vcs`. Ejecuta `cargo new --help`
> para ver las opciones disponibles.

Abre *Cargo.toml* en tu editor de texto de elección. Debería verse similar al
código del Listado 1-2.

<Listing number="1-2" file-name="Cargo.toml" caption="Contenido de *Cargo.toml* generado por `cargo new`">

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

</Listing>

Este archivo está en el formato [*TOML*][toml]<!-- ignore --> (*Tom’s Obvious,
Minimal Language*), que es el formato de configuración de Cargo.

La primera línea, `[package]`, es un encabezado de sección que indica que las
siguientes declaraciones están configurando un paquete. A medida que agreguemos
más información a este archivo, agregaremos otras secciones.

Las próximas tres líneas establecen la información de configuración que Cargo
necesita para compilar tu programa: el nombre, la versión y la edición de Rust
que se usará. Hablaremos sobre la entrada `edition` en [Apéndice E][appendix-e]
<!-- ignore -->.

La última línea, `[dependencies]`, es el comienzo de una sección para que
enumere cualquier dependencia de tu proyecto. En Rust, los paquetes de código
se denominan *crates*. No necesitaremos otros crates para este proyecto, pero
lo haremos en el primer proyecto del Capítulo 2, por lo que usaremos esta
sección de dependencias hasta entonces.

Ahora abre *src/main.rs* y echa un vistazo:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

¡Cargo ha generado un programa “Hello, world!”/“¡Hola, mundo!” para ti, 
¡igual que el que escribimos enl Listado 1-1! Hasta ahora, las diferencias 
entre nuestro proyecto y el proyecto generado por Cargo son que Cargo 
colocó el código en el directorio *src* y tenemos un archivo de 
configuración *Cargo.toml* en el directorio superior.

Cargo espera que tus archivos de origen vivan dentro del directorio *src*. El
directorio del proyecto de nivel superior es solo para archivos README, 
información de licencia, archivos de configuración y cualquier otra cosa 
que no esté relacionada con tu código. Usar Cargo te ayuda a organizar 
tus proyectos. Hay un lugar para todo, y todo está en su lugar.

Si comenzaste un proyecto que no usa Cargo, como hicimos con el proyecto
“¡Hola, mundo!”, puedes convertirlo en un proyecto que sí use Cargo. Mueve el
código del proyecto al directorio *src* y crea un archivo *Cargo.toml*
adecuado.

### Construir y ejecutar un proyecto de Cargo

Ahora veamos qué es diferente cuando construimos y ejecutamos el programa
“¡Hola, mundo!” con Cargo. ¡Desde tu directorio *hello_cargo*, construye tu
proyecto ingresando el siguiente comando:

```console
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

Este comando crea un archivo ejecutable en *target/debug/hello_cargo* (o
*target\debug\hello_cargo.exe* en Windows) en lugar de en tu directorio
actual. Debido a que la compilación predeterminada es una compilación de
depuración, Cargo coloca el binario en un directorio llamado *debug*. Puedes
llamar al ejecutable con este comando:

```console
$ ./target/debug/hello_cargo # o .\target\debug\hello_cargo.exe en Windows
Hello, world!
```

Si todo va bien, `Hello, world!` debería imprimirse en la terminal. Ejecutar
`cargo build` por primera vez también hace que Cargo cree un nuevo archivo en
el nivel superior: *Cargo.lock*. Este archivo rastrea las versiones exactas de
las dependencias de tu proyecto. Este proyecto no tiene dependencias, por lo
que el archivo es un poco escaso. Nunca necesitarás cambiar este archivo
manualmente; Cargo administra su contenido para ti.

Acabamos de construir un proyecto con `cargo build` y ejecutarlo con
`./target/debug/hello_cargo`, pero también podemos usar `cargo run` para
compilar el código y luego llamar al ejecutable resultante en un solo
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

Ten en cuenta que esta vez no vimos salida que indicara que Cargo estaba
compilando `hello_cargo`. Cargo supo que los archivos no habían cambiado, por
lo que no volvió a construir, sino que solo ejecutó el binario. Si hubieras
modificado tu código fuente, Cargo habría reconstruido el proyecto antes de
ejecutarlo, y habrías visto esta salida:

```console
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Cargo también proporciona un comando llamado `cargo check`. Este comando
comprueba rápidamente tu código para asegurarse de que compila, pero no
produce un ejecutable:

```console
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

¿Por qué no querrías un ejecutable? A menudo, `cargo check` es mucho más rápido
que `cargo build` porque omite el paso de producir un ejecutable. Si estás
verificando continuamente tu trabajo mientras escribes el código, usar
`cargo check` acelerará el proceso de informarte si tu proyecto todavía aún está
compilando. ¡Por lo tanto, muchos Rustaceans ejecutan `cargo check`
periódicamente mientras escriben su programa para asegurarse de que compila!
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
importar en qué sistema operativo estés trabajando. Por lo tanto, en este
punto, ya no proporcionaremos instrucciones específicas para Linux y macOS
versus Windows.

### Construyendo una versión de lanzamiento

Cuando tu proyecto finalmente esté listo para su lanzamiento, puedes usar `cargo
build --release` para compilarlo con optimizaciones. Este comando creará un
ejecutable en *target/release* en lugar de *target/debug*. Las optimizaciones
hacen que tu código Rust se ejecute más rápido, pero al activarlos se alarga el
tiempo que tarda tu programa en compilarse. Es por eso que hay dos perfiles
diferentes: uno para el desarrollo, cuando deseas reconstruir rápidamente y
con frecuencia, y otro para construir el programa final que le darás al usuario,
que no se reconstruirá repetidamente y que se ejecutará lo más rápido posible.
Si estás midiendo el tiempo de ejecución de tu código, asegúrate de ejecutar
`cargo build --release` y realizar la prueba de rendimiento con el ejecutable
en *target/release*.

### Cargo como convención

Con proyectos simples, Cargo no proporciona mucho valor por sobre sólo usar
`rustc`, pero demostrará su valor a medida que tus programas se vuelvan más
intrincados. Una vez que los programas crecen a múltiples archivos o necesitan
una dependencia, es mucho más fácil dejar que Cargo coordine la construcción.

Aunque el proyecto `hello_cargo` es simple, ahora usas muchas de las herramientas
reales que usarás en el resto de tu carrera en Rust. De hecho, para trabajar en
cualquier proyecto existente, puedes usar los siguientes comandos para verificar
el código usando Git, cambiar al directorio del proyecto y construir:

```console
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```

Para obtener más información sobre Cargo, consulta [su documentación][cargo].

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
programa de juego de adivinanzas. Si prefieres comenzar aprendiendo cómo
funcionan los conceptos de programación comunes en Rust, consulta el capítulo 3
y luego regresa al capítulo 2.

[installation]: ch01-01-installation.html#instalacion
[toml]: https://toml.io
[appendix-e]: appendix-05-editions.html
[cargo]: https://doc.rust-lang.org/cargo/
