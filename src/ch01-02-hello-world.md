## ¡Hola, mundo!

Ahora que has instalado Rust, es hora de escribir tu primer programa en Rust.
Es tradicional cuando se aprende un nuevo lenguaje escribir un pequeño programa
que imprima el texto `¡Hola, mundo!` en la pantalla, ¡así que haremos lo mismo
aquí!

> Nota: Este libro asume una familiaridad básica con la línea de comandos. Rust
> no asume cosas específicas sobre tu editor o herramientas o dónde vive tu
> código, por lo que si prefieres usar un entorno de desarrollo integrado (IDE)
> en lugar de la línea de comandos, siéntete libre de usar tu IDE favorito. 
> Muchos IDEs ahora tienen algún grado de soporte para Rust; consulta la
> documentación del IDE para obtener más detalles. El equipo de Rust se ha
> centrado en habilitar un gran soporte a IDEs a través de `rust-analyzer`.
> Consulta [Apéndice D][devtools]<!-- ignore --> para obtener más detalles.

### Creando un directorio de proyecto

Comenzarás creando un directorio para almacenar tu código Rust. A Rust no le
importa dónde vive tu código, pero para los ejercicios y proyectos de este libro,
sugerimos que hagas un directorio *proyectos* en tu directorio de inicio y
mantengas todos tus proyectos allí.

Abre una terminal y escribe los siguientes comandos para crear un directorio
*proyectos* y un directorio para el proyecto “¡Hola, mundo!” dentro del
directorio *proyectos*.

Para Linux, macOS y PowerShell en Windows, escribe esto:

```console
$ mkdir ~/proyectos
$ cd ~/proyectos
$ mkdir hola_mundo
$ cd hola_mundo
```

Para Windows CMD, escribe esto:

```cmd
> mkdir "%USERPROFILE%\proyectos"
> cd /d "%USERPROFILE%\proyectos"
> mkdir hola_mundo
> cd hola_mundo
```

### Escribir y ejecutar un programa en Rust

A continuación, crea un nuevo archivo de texto y llámalo *main.rs*. Los archivos
Rust siempre terminan con la extensión *.rs*. Si estás usando más de una palabra
en tu nombre de archivo, la convención es usar un guión bajo para separarlos.
Por ejemplo, usa *hola_mundo.rs* en lugar de *holamundo.rs*.

Ahora abre el archivo *main.rs* que acabas de crear y escribe el código en el
Listado 1-1.

<Listing number="1-1" file-name="main.rs" caption="Un programa que imprime `¡Hola, mundo!`">

```rust
fn main() {
    println!("¡Hola, mundo!");
}
```

</Listing>

Guarda el archivo y vuelve a la ventana de la terminal en el directorio
*~/proyectos/hola_mundo*. En Linux o macOS, escribe los siguientes comandos para
compilar y ejecutar el archivo:

```console
$ rustc main.rs
$ ./main
¡Hola, mundo!
```

En Windows, escribe el comando `.\main.exe` en lugar de `./main`:

```powershell
> rustc main.rs
> .\main.exe
¡Hola, mundo!
```

Independientemente de tu sistema operativo, la cadena `¡Hola, mundo!` debe
imprimirse en la terminal. Si no ves esta salida, consulta la parte
[“Solución de problemas”][troubleshooting]<!-- ignore --> de la sección de
Instalación para obtener formas de obtener ayuda.

Si `¡Hola, mundo!` se imprimió, ¡felicidades! Acabas de escribir oficialmente un
programa en Rust. Eso te convierte en un programador de Rust, ¡bienvenido!

### Anatomía de un programa en Rust

Revisemos este programa “¡Hola, mundo!” en detalle. Aquí está la primera
parte del rompecabezas:

```rust
fn main() {

}
```

Estas líneas definen una función llamada `main`. La función `main` es especial:
siempre es el primer código que se ejecuta en cada programa ejecutable de Rust.
Aquí, la primera línea declara una función llamada `main` que no tiene
parámetros y no devuelve nada. Si hubiera parámetros, irían dentro de los
paréntesis `()`.

El cuerpo de la función está envuelto en `{}`. Rust requiere llaves alrededor de
todos los cuerpos de función. Es buena costumbre colocar la llave de apertura en
la misma línea que la declaración de la función, agregando un espacio entre
ambos.

> Nota: Si deseas mantener un estilo estándar en todos los proyectos de Rust, 
> puedes usar una herramienta de formateo automático llamada `rustfmt` para
> formatear tu código en un estilo particular (más sobre `rustfmt` en
> [Apéndice D][devtools]<!-- ignore -->). El equipo de Rust ha incluido esta
> herramienta con la distribución estándar de Rust, como `rustc`, por lo que
> debería estar instalado en tu computadora.

El cuerpo de la función `main` contiene el siguiente código:

```rust
    println!("¡Hola, mundo!");
```

Esta línea hace todo el trabajo en este pequeño programa: imprime texto en la
pantalla. Hay cuatro detalles importantes que hay que notar aquí.

Primero, el estilo de Rust es indentar con cuatro espacios, no con una tabulación.

Segundo, `println!` llamamos a una macro de Rust. Si hubiéramos llamado a una
función en su lugar, habríamos ingresado `println` (sin el `!`). Discutiremos las
macros de Rust en más detalle en el Capítulo 19. Por ahora, solo necesitas saber
que usar un `!` significa que estamos llamando a una macro en lugar de una función
normal y que las macros no siempre siguen las mismas reglas que las funciones.

Tercero, ve la cadena `"¡Hola, mundo!"`. Pasamos esta cadena como argumento a
`println!`, y la cadena se imprime en la pantalla.

Cuarto, terminamos la línea con un punto y coma (`;`), lo que indica que esta
expresión ha terminado y la siguiente está lista para comenzar. La mayoría de
las líneas de código de Rust terminan con un punto y coma.

### Compilar y ejecutar son pasos separados

Acabas de ejecutar un programa recién creado, así que examinemos cada paso en el
proceso.

Antes de ejecutar un programa de Rust, debes compilarlo usando el compilador de
Rust ingresando el comando `rustc` y pasándole el nombre de tu archivo de
código fuente, así:


```console
$ rustc main.rs
```

Si tienes un trasfondo en C o C ++, notarás que esto es similar a `gcc` o
`clang`. Después de compilar con éxito, Rust genera un ejecutable binario.

En Linux, macOS y PowerShell en Windows, puedes ver el ejecutable ingresando el
comando `ls` en tu shell:

```console
$ ls
main  main.rs
```

En Linux y macOS, verás dos archivos. Con PowerShell en Windows, verás los mismos
tres archivos que verías con CMD. Con CMD en Windows, ingresarías lo siguiente:

```cmd
> dir /B %= la /B significa que solo mostrara los nombres de los archivos =%
main.exe
main.pdb
main.rs
```

Esto muestra el archivo de código fuente con la extensión *.rs*, el archivo
ejecutable (*main.exe* en Windows, pero *main* en todas las otras plataformas),
y, cuando se usa Windows, un archivo que contiene información de depuración con
la extensión *.pdb*. Desde aquí, ejecuta el archivo *main* o *main.exe*, así:

```console
$ ./main # o .\main.exe en Windows
```

Si tu *main.rs* es tu programa "¡Hola, mundo!", Esta línea imprime `¡Hola,
mundo!` en tu terminal.

Si estás más familiarizado con un lenguaje dinámico, como Ruby, Python o
JavaScript, puede que no estés acostumbrado a compilar y ejecutar un programa
como pasos separados. Rust es un lenguaje *compilado de antemano*, lo que
significa que puedes compilar un programa y dar el ejecutable a otra persona, y
pueden ejecutarlo incluso sin tener Rust instalado. Si le das a alguien un
archivo *.rb*, *.py* o *.js*, necesitan tener una implementación de Ruby,
Python o JavaScript instalada (respectivamente). Pero en esos lenguajes, sólo
necesitas un comando para compilar y ejecutar tu programa. Todo depende de las
concesiones hechas al momento de diseñar un lenguaje.

Solo compilar con `rustc` está bien para programas simples, pero a medida que
tu proyecto crece, querrás administrar todas las opciones y facilitar el 
compartir tu código. A continuación, te presentaremos la herramienta
Cargo, que te ayudará a escribir programas de Rust reales.

[troubleshooting]: ch01-01-installation.html#solucion-de-problemas
[devtools]: appendix-04-useful-development-tools.html
