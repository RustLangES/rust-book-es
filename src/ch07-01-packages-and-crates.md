## Paquetes y Crates

Las primeras partes del sistema de módulos que cubriremos son los paquetes y los
crates.

Un _crate_ es la cantidad más pequeña de código que el compilador Rust considera
a la vez. Incluso si ejecutas `rustc` en lugar de `cargo` y pasas un solo
archivo de código fuente (como lo hicimos en la sección “Escribir y Ejecutar un
Programa Rust” del Capítulo 1), el compilador considera que ese archivo es un
crate. Los crates pueden contener módulos, y los módulos pueden definirse en
otros archivos que se compilan con el crate, como veremos en las próximas
secciones.

Un crate puede venir en una de dos formas: un crate binario o un crate de
biblioteca. Los _crates binarios_ son programas que puedes compilar a un
ejecutable que puedes ejecutar, como un programa de línea de comandos o un
servidor. Cada uno debe tener una función llamada `main` que defina lo que
sucede cuando se ejecuta el ejecutable. Todos los crates que hemos creado hasta
ahora han sido crates binarios.

Los _crates de biblioteca_ no tienen una función `main`, y no se compilan a un
ejecutable. En su lugar, definen funcionalidad destinada a ser compartida con
múltiples proyectos. Por ejemplo, el crate `rand` que usamos en el [Capítulo
2][rand]<!-- ignore --> proporciona funcionalidad que genera números aleatorios.
La mayor parte del tiempo, cuando los Rustaceans dicen “crate”, se refieren a
crate de biblioteca, y usan “crate” indistintamente con el concepto general de
programación de una “biblioteca”.

El _crate root_ es un archivo fuente que el compilador Rust comienza y forma el
módulo raíz de tu crate (explicaremos los módulos en profundidad en la sección
[“Definir Módulos para Controlar el Alcance y la Privacidad”][modules]<!-- ignore
-->).

Un _paquete_ es un conjunto de uno o más crates que proporciona un conjunto de
funcionalidades. Un paquete contiene un archivo _Cargo.toml_ que describe cómo
compilar esos crates. Cargo es en realidad un paquete que contiene el crate
binario para la herramienta de línea de comandos que has estado usando para
compilar tu código. El paquete Cargo también contiene un crate de biblioteca en
el que el crate binario depende. Otros proyectos pueden depender del crate de
biblioteca Cargo para usar la misma lógica que la herramienta de línea de
comandos Cargo usa.

Un paquete puede venir en dos formas: un paquete binario o un paquete libreria.
Un paquete puede contener tantos crates binarios como desees, pero como máximo
solo un crate de biblioteca. Un paquete debe contener al menos un crate, ya sea
un crate de biblioteca o un crate binario.

Veamos qué sucede cuando creamos un paquete. Primero, ingresamos el comando
`cargo new`:

```console
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

Después de ejecutar `cargo new my-project`, usamos `ls` para ver lo que crea 
Cargo. En el directorio del proyecto, hay un archivo _Cargo.toml_, que nos da un
paquete. También hay un directorio _src_ que contiene _main.rs_. Abre 
_Cargo.toml_ en tu editor de texto, y observa que no hay mención de 
_src/main.rs_. Cargo sigue una convención de que _src/main.rs_ es la raíz del 
crate de un crate binario con el mismo nombre que el paquete. Del mismo modo, 
Cargo sabe que si el directorio del paquete contiene _src/lib.rs_, el paquete 
contiene un crate de biblioteca con el mismo nombre que el paquete, y 
_src/lib.rs_ es su raíz del crate. Cargo pasa los archivos raíz del crate a 
`rustc` para compilar la biblioteca o el binario.

Aquí, tenemos un paquete que solo contiene _src/main.rs_, lo que significa que
solo contiene un crate binario llamado `my-project`. Si un paquete contiene
_src/main.rs_ y _src/lib.rs_, tiene dos crates: un binario y una biblioteca,
ambos con el mismo nombre que el paquete. Un paquete puede tener múltiples
crates binarios colocando archivos en el directorio _src/bin_: cada archivo será
un crate binario separado.

[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
[rand]: ch02-00-guessing-game-tutorial.html#generar-un-numero-aleatorio
