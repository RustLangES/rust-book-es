## Extendiendo Cargo con comandos personalizados

Cargo está diseñado para que puedas extenderlo con nuevos subcomandos sin tener
que modificar Cargo. Si un binario en tu `$PATH` se llama `cargo-something`, lo
puedes ejecutar como si fuera un subcomando de Cargo ejecutando `cargo
something`. Los comandos personalizados como este también se enumeran cuando
ejecutas `cargo --list`. ¡Poder usar `cargo install` para instalar extensiones y
luego ejecutarlas como las herramientas integradas de Cargo es un beneficio
súper conveniente del diseño de Cargo!

## Resumen

Compartir código con Cargo y [crates.io](https://crates.io/)<!-- ignore --> es
parte de lo que hace que el ecosistema de Rust sea útil para muchas tareas
diferentes. La biblioteca estándar de Rust es pequeña y estable, pero los crates
son fáciles de compartir, usar y mejorar en una línea de tiempo diferente a la
del lenguaje. ¡No seas tímido al compartir código que te sea útil en
[crates.io](https://crates.io/)<!-- ignore -->; es probable que también sea útil
para otra persona!
