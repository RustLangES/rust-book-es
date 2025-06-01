# El Lenguaje de Programación Rust

[El Lenguaje de Programación Rust](title-page.md)
[Prefacio](foreword.md)
[Introducción](ch00-00-introduction.md)

## Empezando

- [Empezando](ch01-00-getting-started.md)

  - [Instalación](ch01-01-installation.md)
  - [¡Hola, Mundo!](ch01-02-hello-world.md)
  - [¡Hola, Cargo!](ch01-03-hello-cargo.md)

- [Programando un juego de adivinanzas](ch02-00-guessing-game-tutorial.md)

- [Conceptos Comunes de Programación](ch03-00-common-programming-concepts.md)

  - [Variables y Mutabilidad](ch03-01-variables-and-mutability.md)
  - [Tipos de Datos](ch03-02-data-types.md)
  - [Funciones](ch03-03-how-functions-work.md)
  - [Comentarios](ch03-04-comments.md)
  - [Flujo de Control](ch03-05-control-flow.md)

- [Entendiendo el Ownership](ch04-00-understanding-ownership.md)

  - [¿Qué es el Ownership?](ch04-01-what-is-ownership.md)
  - [Referencias y Prestamos](ch04-02-references-and-borrowing.md)
  - [El Tipo Slice](ch04-03-slices.md)

- [Usando Structs para Estructurar Datos Relacionados](ch05-00-structs.md)

  - [Definiendo e Instanciando Structs](ch05-01-defining-structs.md)
  - [Un Programa de Ejemplo Usando Structs](ch05-02-example-structs.md)
  - [Sintaxis de Métodos](ch05-03-method-syntax.md)

- [Enums y Pattern Matching](ch06-00-enums.md)
  - [Definiendo un Enum](ch06-01-defining-an-enum.md)
  - [El operador de control de flujo `match`](ch06-02-match.md)
  - [Flujo de Control Conciso con `if let` y `let else`](ch06-03-if-let.md)

## Conocimientos básicos de Rust

- [Administrando Proyectos en Crecimiento con Paquetes, Crates y Módulos](ch07-00-managing-growing-projects-with-packages-crates-and-modules.md)

  - [Paquetes y Crates](ch07-01-packages-and-crates.md)
  - [Definiendo módulos para controlar el scope y la privacidad](ch07-02-defining-modules-to-control-scope-and-privacy.md)
  - [Paths para referirse a un item en el árbol de módulos](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md)
  - [Incluyendo rutas al Scope con la palabra clave `use`](ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)
  - [Separando Módulos en Diferentes Archivos](ch07-05-separating-modules-into-different-files.md)

- [Colecciones comunes](ch08-00-common-collections.md)

  - [Almacenando listas de valores con vectores](ch08-01-vectors.md)
  - [Almacenando texto codificado en UTF-8 con Strings](ch08-02-strings.md)
  - [Almacenar Claves con Valores Asociados en HashMaps](ch08-03-hash-maps.md)

- [Manejo de Errores](ch09-00-error-handling.md)

  - [Errores irrecuperables con `panic!`](ch09-01-unrecoverable-errors-with-panic.md)
  - [Errores recuperables con `Result`](ch09-02-recoverable-errors-with-result.md)
  - [`panic!` o no `panic!`](ch09-03-to-panic-or-not-to-panic.md)

- [Tipos Genéricos, Traits y Lifetimes](ch10-00-generics.md)

  - [Tipos de Datos Genéricos](ch10-01-syntax.md)
  - [Traits: Definiendo Comportamiento Compartido](ch10-02-traits.md)
  - [Validando Referencias con Lifetimes](ch10-03-lifetime-syntax.md)

- [Escribiendo Tests Automatizados](ch11-00-testing.md)

  - [Cómo Escribir Tests](ch11-01-writing-tests.md)
  - [Controlando Cómo Los Tests Son Ejecutados](ch11-02-running-tests.md)
  - [Organización De Los Tests](ch11-03-test-organization.md)

- [Un proyecto de I/O: Construyendo un programa de línea de comandos](ch12-00-an-io-project.md)
  - [Aceptando argumentos de línea de comandos](ch12-01-accepting-command-line-arguments.md)
  - [Leyendo un archivo](ch12-02-reading-a-file.md)
  - [Refactorizando para mejorar la modularidad y el manejo de errores](ch12-03-improving-error-handling-and-modularity.md)
  - [Desarrollando la funcionalidad de la biblioteca con T.D.D.](ch12-04-testing-the-librarys-functionality.md)
  - [Trabajando con Variables de Entorno](ch12-05-working-with-environment-variables.md)
  - [Escribiendo mensajes de error estándar en lugar del output estándar](ch12-06-writing-to-stderr-instead-of-stdout.md)

## Pensando en Rust

- [Características De Lenguajes Funcionales: Iteradores y Closures](ch13-00-functional-features.md)

  - [Closures: Funciones anónimas que capturan su entorno](ch13-01-closures.md)
  - [Procesando una serie de elementos con Iteradores](ch13-02-iterators.md)
  - [Mejorando nuestro proyecto I/O](ch13-03-improving-our-io-project.md)
  - [Comparando Performance: Bucles vs. Iteradores](ch13-04-performance.md)

- [Más sobre Cargo y Crates.io](ch14-00-more-about-cargo.md)

  - [Personalizando Compilaciones con Perfiles de Lanzamiento](ch14-01-release-profiles.md)
  - [Publicando un Crate a Crates.io](ch14-02-publishing-to-crates-io.md)
  - [Cargo Workspaces](ch14-03-cargo-workspaces.md)
  - [Instalando Binarios con `cargo install`](ch14-04-installing-binaries.md)
  - [Extendiendo Cargo con Comandos Personalizados](ch14-05-extending-cargo.md)

- [Smart Pointers](ch15-00-smart-pointers.md)
  - [Using `Box<T>` to Point to Data on the Heap](ch15-01-box.md)
  - [Treating Smart Pointers Like Regular References with `Deref`](ch15-02-deref.md)
  - [Running Code on Cleanup with the `Drop` Trait](ch15-03-drop.md)
  - [`Rc<T>`, the Reference Counted Smart Pointer](ch15-04-rc.md)
  - [`RefCell<T>` and the Interior Mutability Pattern](ch15-05-interior-mutability.md)
  - [Reference Cycles Can Leak Memory](ch15-06-reference-cycles.md)

- [Fearless Concurrency](ch16-00-concurrency.md)
  - [Using Threads to Run Code Simultaneously](ch16-01-threads.md)
  - [Using Message Passing to Transfer Data Between Threads](ch16-02-message-passing.md)
  - [Shared-State Concurrency](ch16-03-shared-state.md)
  - [Extensible Concurrency with the `Send` and `Sync` Traits](ch16-04-extensible-concurrency-sync-and-send.md)

- [Fundamentos de la Programación Asíncrona: Async, Await, Futures y Streams](ch17-00-async-await.md)
    - [Futures y la sintaxis Async](ch17-01-futures-and-syntax.md)
    - [Aplicando Concurrencia Con Async](ch17-02-concurrency-with-async.md)
    - [Trabajar con cualquier número de futuros](ch17-03-more-futures.md)
    - [Streams](ch17-04-streams.md)
    - [Profundizando en los Traits para Async](ch17-05-traits-for-async.md)
    - [Futuros, tareas e hilos](ch17-06-futures-tasks-threads.md)

- [Rust como un Lenguaje de Programación Orientado a Objetos](ch18-00-oop.md)
  - [Características de Lenguajes Orientados a Objetos](ch18-01-what-is-oo.md)
  - [Usando Trait Objects que Permiten Valores de Diferentes Tipos](ch18-02-trait-objects.md)
  - [Implementando un Patrón de Diseño Orientado a Objetos](ch18-03-oo-design-patterns.md)

## Temas Avanzados

- [Patterns and Matching](ch19-00-patterns.md)

  - [Todos los lugares donde se pueden usar Patterns](ch19-01-all-the-places-for-patterns.md)
  - [Refutabilidad: Si un Pattern Puede Fallar al Hacer Match](ch19-02-refutability.md)
  - [Sintaxis de los Patterns](ch19-03-pattern-syntax.md)

- [Características Avanzadas](ch20-00-advanced-features.md)

  - [Rust Inseguro](ch20-01-unsafe-rust.md)
  - [Advanced Traits](ch20-03-advanced-traits.md)
  - [Advanced Types](ch20-04-advanced-types.md)
  - [Advanced Functions and Closures](ch20-05-advanced-functions-and-closures.md)
  - [Macros](ch20-06-macros.md)

- [Final Project: Building a Multithreaded Web Server](ch21-00-final-project-a-web-server.md)

  - [Building a Single-Threaded Web Server](ch21-01-single-threaded.md)
  - [Turning Our Single-Threaded Server into a Multithreaded Server](ch21-02-multithreaded.md)
  - [Graceful Shutdown and Cleanup](ch21-03-graceful-shutdown-and-cleanup.md)

- [Apéndice](appendix-00.md)
  - [A - Palabras claves](appendix-01-keywords.md)
  - [B - Operadores y Símbolos](appendix-02-operators.md)
  - [C - Derivable Traits](appendix-03-derivable-traits.md)
  - [D - Useful Development Tools](appendix-04-useful-development-tools.md)
  - [E - Editions](appendix-05-editions.md)
  - [F - Translations of the Book](appendix-06-translation.md)
  - [G - Cómo se hace Rust y “Rust Nightly”](appendix-07-nightly-rust.md)
