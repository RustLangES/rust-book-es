## ¡Hola, mundo!

Ahora que has instalado Rust, es hora de escribir tu primer programa en Rust.
Es tradicional cuando se aprende un nuevo lenguaje escribir un pequeño programa
que imprima el texto `¡Hola, mundo!` en la pantalla, ¡así que haremos lo mismo
aquí!

> Nota: Este libro asume una familiaridad básica con la línea de comandos. Rust
> no hace demandas específicas sobre su edición o herramientas o dónde vive su
> código, por lo que si prefiere usar un entorno de desarrollo integrado (IDE)
> en lugar de la línea de comandos, sientase libre de usar su IDE favorito. 
> Muchos IDEs ahora tienen algún grado de soporte para Rust; consulte la
> documentación del IDE para obtener más detalles. El equipo de Rust se ha
> centrado en habilitar un gran soporte IDE a través de `rust-analyzer`.
> Consulte [Apéndice D][devtools]<!-- ignore --> para obtener más detalles.

### Creando un directorio de proyecto

Comenzarás creando un directorio para almacenar tu código Rust. No importa a
Rust dónde vive tu código, pero para los ejercicios y proyectos de este libro,
sugerimos que hagas un directorio *proyectos* en tu directorio de inicio y
mantengas todos tus proyectos allí.

Abra una terminal y escriba los siguientes comandos para crear un directorio
*proyectos* y un directorio para el proyecto “¡Hola, mundo!” dentro del
directorio *proyectos*.

Para Linux, macOS y PowerShell en Windows, escriba esto:

```console
$ mkdir ~/proyectos
$ cd ~/proyectos
$ mkdir hola_mundo
$ cd hola_mundo
```

Para Windows CMD, escriba esto:

```cmd
> mkdir "%USERPROFILE%\proyectos"
> cd /d "%USERPROFILE%\proyectos"
> mkdir hola_mundo
> cd hola_mundo
```

### Escribir y ejecutar un programa en Rust

A continuación, cree un nuevo archivo de origen y llámelo *main.rs*. Los archivos
Rust siempre terminan con la extensión *.rs*. Si está usando más de una palabra
en su nombre de archivo, la convención es usar un guión bajo para separarlos.
Por ejemplo, use *hola_mundo.rs* en lugar de *holamundo.rs*.

Ahora abra el archivo *main.rs* que acaba de crear y escriba el código en la

<span class="filename">Nombre de archivo: main.rs</span>

```rust
fn main() {
    println!("¡Hola, mundo!");
}
```

<span class="caption">Listado 1-1: Un programa que imprime `¡Hola, mundo!`</span>

Guarde el archivo y vuelva a la ventana de la terminal en el directorio
*~/proyectos/hola_mundo*. En Linux o macOS, escriba los siguientes comandos para
compilar y ejecutar el archivo:

```console
$ rustc main.rs
$ ./main
¡Hola, mundo!
```

En Windows, escriba el comando `.\main.exe` en lugar de `./main`:

```powershell
> rustc main.rs
> .\main.exe
¡Hola, mundo!
```

Independientemente de su sistema operativo, la cadena `¡Hola, mundo!` debe
imprimirse en la terminal. Si no ve esta salida, consulte la parte
[“Solución de problemas”][troubleshooting]<!-- ignore --> de la sección de
Instalación para obtener formas de obtener ayuda.

Si `¡Hola, mundo!` se imprimió, ¡felicidades! Acabas de escribir oficialmente un
programa en Rust. Eso te convierte en un programador de Rust, ¡bienvenido!

### Anatomía de un programa en Rust

Reviewemos este programa “¡Hola, mundo!” en detalle. Aquí está la primera
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

> Nota: Si desea mantener un estilo estándar en todos los proyectos de Rust, 
> puede usar una herramienta de formateo automático llamada `rustfmt` para
> formatear su código en un estilo particular (más sobre `rustfmt` en
> [Apéndice D][devtools]<!-- ignore -->). El equipo de Rust ha incluido esta
> herramienta con la distribución estándar de Rust, como `rustc`, por lo que
> debería estar instalado en su computadora.

El cuerpo de la función `main` contiene el siguiente código:

```rust
    println!("¡Hola, mundo!");
```

Esta línea hace todo el trabajo en este pequeño programa: imprime texto en la
pantalla. Hay cuatro detalles importantes que hay que notar aquí.

Primero, el estilo de Rust es indentar con cuatro espacios, no con una tabulación.

Segundo, `println!` llama a una macro de Rust. Si hubiera llamado a una función
en su lugar, se habría ingresado como `println` (sin el `!`). Discutiremos las
macros de Rust en más detalle en el Capítulo 19. Por ahora, solo necesita saber
que usar un `!` significa que está llamando a una macro en lugar de una función
normal y que las macros no siempre siguen las mismas reglas que las funciones.

Tercero, ve la cadena `"¡Hola, mundo!"`. Pasamos esta cadena como argumento a
`println!`, y la cadena se imprime en la pantalla.

Cuarto, terminamos la línea con un punto y coma (`;`), lo que indica que esta
expresión ha terminado y la siguiente está lista para comenzar. La mayoría de
las líneas de código de Rust terminan con un punto y coma.

### Compilar y ejecutar son pasos separados

Acaba de ejecutar un programa recién creado, así que examinemos cada paso en el
proceso.

Antes de ejecutar un programa de Rust, debe compilarlo usando el compilador de
Rust ingresando el comando `rustc` y pasándole el nombre de su archivo de
origen, así:


```console
$ rustc main.rs
```

Si tiene un fondo en C o C ++, notará que esto es similar a `gcc` o `clang`.
Después de compilar con éxito, Rust genera un ejecutable binario.

En Linux, macOS y PowerShell en Windows, puede ver el ejecutable ingresando el
comando `ls` en su shell:

```console
$ ls
main  main.rs
```

En Linux y macOS, verá dos archivos. Con PowerShell en Windows, verá los mismos
tres archivos que vería con CMD. Con CMD en Windows, ingresaría lo siguiente:

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

Si su *main.rs* es su programa "¡Hola, mundo!", Esta línea imprime `¡Hola,
mundo!` en su terminal.

Si está más familiarizado con un lenguaje dinámico, como Ruby, Python o
JavaScript, puede que no esté acostumbrado a compilar y ejecutar un programa
como pasos separados. Rust es un lenguaje *compilado de antemano*, lo que
significa que puede compilar un programa y dar el ejecutable a otra persona, y
pueden ejecutarlo incluso sin tener Rust instalado. Si le das a alguien un
archivo *.rb*, *.py* o *.js*, necesitan tener una implementación de Ruby,
Python o JavaScript instalada (respectivamente). Pero en esos idiomas, solo
necesita un comando para compilar y ejecutar su programa. Todo es una
compensación en el diseño del lenguaje.

Solo compilar con `rustc` está bien para programas simples, pero a medida que
su proyecto crece, querrá administrar todas las opciones y facilitar el 
compartir su código. A continuación, le presentaremos la herramienta
Cargo, que le ayudará a escribir programas de Rust reales.

[troubleshooting]: ch01-01-installation.html#troubleshooting
[devtools]: appendix-04-useful-development-tools.md
