## Apéndice D - Herramientas de desarrollo útiles

En este apéndice, hablaremos sobre algunas herramientas de desarrollo útiles
que proporciona el proyecto Rust. Veremos el formato automático, formas rápidas
de aplicar correcciones de advertencia, un linter e integración con IDE.

### Formato automático con `rustfmt`

La herramienta `rustfmt` reformatea su código de acuerdo con el estilo de código
de la comunidad. Muchos proyectos colaborativos usan `rustfmt` para evitar
discusiones sobre qué estilo usar al escribir Rust: todos formatean su código
usando la herramienta.

Las instalaciones de Rust incluyen `rustfmt` por defecto, así que ya deberías 
tener los programas `rustfmt` y `cargo-fmt` en tu sistema. Estos dos comandos 
son análogos a `rustc` y `cargo` en el sentido de que `rustfmt` permite un 
control más detallado, mientras que `cargo-fmt` entiende las convenciones de un 
proyecto que usa Cargo. Para formatear cualquier proyecto de Cargo, ingresa lo 
siguiente:

```console
$ cargo fmt
```

Ejecutando este comando reformatea todo el código Rust en la carga útil actual.
Esto solo debería cambiar el estilo de código, no la semántica del código. Para
más información sobre `rustfmt`, vea [su documentación][rustfmt].

[rustfmt]: https://github.com/rust-lang/rustfmt

### Corregir su código con `rustfix`

La herramienta `rustfix` se incluye con las instalaciones de Rust y puede
corregir automáticamente las advertencias del compilador que tienen una forma
clara de corregir el problema que es probablemente lo que desea. Es probable que
haya visto advertencias del compilador antes. Por ejemplo, considere este código:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let mut x = 42;
    println!("{x}");
}
```

Aquí estamos definiendo la variable `x` como mutable, pero en realidad nunca la 
modificamos.  
Rust nos advierte sobre eso:

```console
$ cargo build
   Compiling myprogram v0.1.0 (file:///projects/myprogram)
warning: variable does not need to be mutable
 --> src/main.rs:2:9
  |
2 |     let mut x = 0;
  |         ----^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default
```

Esta advertencia sugiere que eliminemos la palabra clave `mut`.
Podemos aplicar automáticamente esa sugerencia usando la herramienta `rustfix` 
ejecutando el comando `cargo fix`:

```console
$ cargo fix
    Checking myprogram v0.1.0 (file:///projects/myprogram)
      Fixing src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

Cuando volvemos a mirar *src/main.rs*, veremos que `cargo fix` ha cambiado el
código:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = 42;
    println!("{x}");
}
```

La variable `x` ahora es inmutable, y la advertencia ya no aparece.

También puede usar `cargo fix` para transformar su código entre diferentes
ediciones de Rust. Las ediciones se tratan en el [Apéndice E][editions].

### Más lints con Clippy

La herramienta `clippy` es una colección de lints para analizar su código para
que pueda detectar errores comunes y mejorar su código Rust. Clippy está 
incluido en las instalaciones estándar de Rust.

Para ejecutar los lints de Clippy en cualquier proyecto de carga útil, ingrese
lo siguiente:

```console
$ cargo clippy
```

Por ejemplo, digamos que escribe un programa que usa una aproximación de una
constante matemática, como pi, como lo hace este programa:

<Listing file-name="src/main.rs">

```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

</Listing>

Ejecutando `cargo clippy` en este proyecto resulta en este error:

```text
error: approximate value of `f{32, 64}::consts::PI` found
 --> src/main.rs:2:13
  |
2 |     let x = 3.1415;
  |             ^^^^^^
  |
  = note: `#[deny(clippy::approx_constant)]` on by default
  = help: consider using the constant directly
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#approx_constant
```

Este error le informa que Rust ya tiene una constante `PI` más precisa definida
y que su programa sería más correcto si usara la constante en su lugar. Luego
cambiaría su código para usar la constante `PI`. El siguiente código no
produce ningún error ni advertencia de Clippy:

<Listing file-name="src/main.rs">

```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

</Listing>

Para obtener más información sobre Clippy, consulte [su documentación][clippy].

[clippy]: https://github.com/rust-lang/rust-clippy

### Integración de IDE con `rust-analyzer`

Para ayudar a la integración del IDE, la comunidad Rust recomienda usar
[`rust-analyzer`][rust-analyzer]. Esta herramienta es un conjunto de utilidades
centradas en el compilador que habla el [Protocolo del servidor de lenguaje][lsp]
<!-- ignore -->, que es una especificación para que los IDE y los lenguajes de
programación se comuniquen entre sí. Diferentes clientes pueden usar
`rust-analyzer`, como [el complemento del analizador Rust para Visual Studio
Code][vscode].

[lsp]: http://langserver.org/
[vscode]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer

Visite la [página de inicio del proyecto `rust-analyzer`][rust-analyzer]<!-- ignore -->
para obtener instrucciones de instalación, luego instale el soporte del servidor
de lenguaje en su IDE en particular. Su IDE ganará habilidades como
autocompletado, salto a la definición y errores en línea. 

[rust-analyzer]: https://rust-analyzer.github.io
[editions]: appendix-05-editions.md
