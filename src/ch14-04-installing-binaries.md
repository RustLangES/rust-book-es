<!-- Old link, do not remove -->
<a id="installing-binaries-from-cratesio-with-cargo-install"></a>

## Instalando Binarios con `cargo install`

El comando `cargo install` te permite instalar y usar crates binarios localmente.
Esto no está destinado a reemplazar los paquetes del sistema; está destinado a 
ser una forma conveniente para que los desarrolladores de Rust instalen 
herramientas que otros han compartido en [crates.io](https://crates.io/)<!-- 
ignore -->. Tenga en cuenta que solo puede instalar paquetes que tengan 
objetivos binarios. Un *objetivo binario* es el programa ejecutable que se crea 
si el crate tiene un archivo *src/main.rs* u otro archivo especificado como un 
binario, en oposición a un objetivo de biblioteca que no se puede ejecutar por 
sí solo, pero que es adecuado para incluirlo en otros programas. Por lo general, 
las crates tienen información en el archivo *README* sobre si una crate es una 
biblioteca, tiene un objetivo binario, o ambos.

Todos los binarios instalados con `cargo install` se almacenan en la carpeta
raíz de instalación de *bin*. Si instalaste Rust usando *rustup.rs* y no tienes
configuraciones personalizadas, este directorio será *$HOME/.cargo/bin*. 
Asegúrese de que el directorio de instalación esté en su `$PATH` para poder 
ejecutar los programas que ha instalado con `cargo install`.

Por ejemplo, en el Capítulo 12, mencionamos que hay una implementación de Rust
de la herramienta `grep` llamada `ripgrep` para buscar archivos. Para instalar
`ripgrep`, podemos ejecutar lo siguiente:

<!-- manual-regeneration
cargo install something you don't have, copy relevant output below
-->

```console
$ cargo install ripgrep
    Updating crates.io index
  Downloaded ripgrep v13.0.0
  Downloaded 1 crate (243.3 KB) in 0.88s
  Installing ripgrep v13.0.0
--snip--
   Compiling ripgrep v13.0.0
    Finished `release` profile [optimized + debuginfo] target(s) in 10.64s
  Installing ~/.cargo/bin/rg
   Installed package `ripgrep v13.0.0` (executable `rg`)
```

La penúltima línea de la salida muestra la ubicación y el nombre del binario
instalado, que en el caso de `ripgrep` es `rg`. Mientras el directorio de
instalación esté en su `$PATH`, como se mencionó anteriormente, puede ejecutar
`rg --help` y comenzar a usar una herramienta más rápida y oxidada para buscar
archivos!
