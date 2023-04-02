# Programando un juego de adivinanzas

¡Vamos a empezar con Rust trabajando en un proyecto práctico! Este capítulo te
introduce a algunos conceptos comunes de Rust mostrándote cómo usarlos en un
programa real. ¡Aprenderás sobre `let`, `match`, métodos, funciones asociadas,
paquetes externos y más! En los capítulos siguientes, exploraremos estos
conceptos en más detalle. En este capítulo, solo practicarás los fundamentos.

Implementaremos un clásico problema de programación para principiantes: un
juego de adivinanzas. Así es como funciona: el programa generará un número
entero aleatorio entre 1 y 100. Luego le pedirá al jugador que ingrese una
adivinanza. Después de ingresar una adivinanza, el programa indicará si la
adivinanza es demasiado baja o demasiado alta. Si la adivinanza es correcta, el
juego imprimirá un mensaje de felicitación y saldrá.

## Configurando un nuevo proyecto

Para configurar un nuevo proyecto, vaya al directorio *proyectos* que creó en
el Capítulo 1 y cree un nuevo proyecto usando Cargo, así:

```console
$ cargo new guessing_game
$ cd guessing_game
```

El primer comando, `cargo new`, toma el nombre del proyecto (`guessing_game`) 
como el primer argumento. El segundo comando cambia al directorio del nuevo
proyecto.


Mira el archivo *Cargo.toml* generado:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial
rm -rf no-listing-01-cargo-new
cargo new no-listing-01-cargo-new --name guessing_game
cd no-listing-01-cargo-new
cargo run > output.txt 2>&1
cd ../../..
-->

<span class="filename">Nombre de archivo: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/Cargo.toml}}
```

Como viste en el Capítulo 1, `cargo new` genera un programa “Hola, mundo!” para
ti. Mira el archivo *src/main.rs*:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/src/main.rs}}
```

Ahora compilemos este programa “Hola, mundo!” y ejecutémoslo en el mismo paso
usando el comando `cargo run`:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/output.txt}}
```

El comando `run` es útil cuando necesitas iterar rápidamente en un proyecto,
como haremos en este juego, probando rápidamente cada iteración antes de
pasar a la siguiente.

Vuelve a abrir el archivo *src/main.rs*. Escribirás todo el código en este

## Procesando una adivinanza

La primera parte del programa del juego de adivinanzas pedirá al usuario que
ingrese un valor, procesará ese valor y verificará que el valor esté en el
formato esperado. Para comenzar, permitiremos al jugador ingresar una adivinanza.
Ingresa el código de la Lista 2-1 en *src/main.rs*.

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:all}}
```

<span class="caption">Lista 2-1: Código que obtiene una adivinanza del usuario
y la imprime</span>

Este código contiene mucha información, así que repasémoslo línea por línea. 
Para obtener la entrada del usuario y luego imprimir el resultado como salida,
necesitamos traer la biblioteca de entrada/salida `io` al alcance. La biblioteca
`io` viene de la biblioteca estándar, conocida como `std`:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:io}}
```

Por defecto, Rust tiene un conjunto de elementos definidos en la biblioteca
estándar que trae al alcance de cada programa. Este conjunto se llama
*prelude*, y puedes ver todo lo que contiene [en la documentación de la
biblioteca estándar][prelude].

Si un tipo que quieres usar no está en el prelude, tienes que traer ese tipo
al alcance explícitamente con una declaración `use`. Usar la biblioteca `std::io`
te proporciona una serie de características útiles, incluyendo la capacidad de
aceptar la entrada del usuario.

Como viste en el Capítulo 1, la función `main` es el punto de entrada al
programa:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:main}}
```

La sintaxis `fn` declara una nueva función; los paréntesis, `()`, indican que
no hay parámetros; y la llave, `{`, inicia el cuerpo de la función.

Como también aprendiste en el Capítulo 1, `println!` es una macro que imprime
una cadena en la pantalla:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print}}
```

Este código está imprimiendo una solicitud que indica qué es el juego y está
solicitando la entrada del usuario.

### Almacenando valores con variables

A continuación, crearemos una *variable* para almacenar la entrada del usuario,
como esto:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:string}}
```

¡Ahora el programa está interesante! Hay mucho que está pasando en esta pequeña
línea. Usamos la declaración `let` para crear la variable. Aquí hay otro
ejemplo:

```rust,ignore
let apples = 5;
```

Esta línea crea una nueva variable llamada `apples` y la enlaza con el valor 5.
En Rust, las variables son inmutables por defecto, lo que significa que una vez
que le damos a la variable un valor, el valor no cambiará. Vamos a discutir
este concepto en detalle en la sección [“Variables y Mutabilidad”][variables-
and-mutability]<!-- ignore --> del Capítulo 3. Para hacer una variable mutable,
agregamos `mut` antes del nombre de la variable:

```rust,ignore
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

> Nota: La sintaxis `//` inicia un comentario que continúa hasta el final de la
> línea. Rust ignora todo lo que está en los comentarios. Vamos a discutir los
> comentarios en más detalle en el [Capítulo 3][comments]<!-- ignore -->.

Regresando al programa del juego de adivinanzas, ahora sabes que `let mut guess`
introducirá una variable mutable llamada `guess`. El signo igual (`=`) le dice
a Rust que queremos enlazar algo a la variable ahora. A la derecha del signo
igual está el valor al que `guess` está enlazado, que es el resultado de llamar
a `String::new`, una función que devuelve una nueva instancia de un `String`.
[`String`][string]<!-- ignore --> es un tipo de cadena proporcionado por la
biblioteca estándar que es una parte de texto codificada en UTF-8 que puede
crecer.

La sintaxis `::` en la línea `::new` indica que `new` es una función asociada
del tipo `String`. Una *función asociada* es una función que está implementada
en un tipo, en este caso `String`. Esta función `new` crea una nueva cadena 
vacía. Encontrarás una función `new` en muchos tipos porque es un nombre
común para una función que crea un nuevo valor de algún tipo.

En total, la línea `let mut guess = String::new();` ha creado una variable
mutable que está actualmente enlazada a una nueva instancia vacía de un
`String`. ¡Uf!

### Recibiendo la entrada del usuario

Recuerda que incluimos la funcionalidad de entrada/salida de la biblioteca
estándar con `use std::io;` en la primera línea del programa. Ahora llamaremos
a la función `stdin` del módulo `io`, que nos permitirá manejar la entrada del
usuario:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:read}}
```

Si no hubiéramos importado la biblioteca `io` con `use std::io;` al comienzo del
programa, aún podríamos usar la función escribiendo esta llamada de función
como `std::io::stdin`. La función `stdin` devuelve una instancia de
[`std::io::Stdin`][iostdin]<!-- ignore -->, que es un tipo que representa un
manejador de la entrada estándar para tu terminal.

A continuación, la línea `.read_line(&mut guess)` llama al método
[`read_line`][read_line]<!-- ignore --> en el manejador de entrada estándar para
obtener la entrada del usuario. También estamos pasando `&mut guess` como
argumento a `read_line` para decirle qué cadena almacenar la entrada del
usuario. El trabajo completo de `read_line` es tomar lo que el usuario escribe
en la entrada estándar y agregar eso a una cadena (sin sobrescribir su
contenido), por lo que, por lo tanto, pasamos esa cadena como argumento. La
cadena de argumentos debe ser mutable para que el método pueda cambiar el
contenido de la cadena.

El `&` indica que este argumento es una *referencia*, que te da una forma de
permitir que varias partes de tu código accedan a una pieza de datos sin
necesidad de copiar esos datos en la memoria varias veces. Las referencias son
una característica compleja, y una de las principales ventajas de Rust es lo
seguro y fácil que es usar referencias. No necesitas saber mucho de esos
detalles para terminar este programa. Por ahora, todo lo que necesitas saber es
que, como las variables, las referencias son inmutables por defecto. Por lo
tanto, necesitas escribir `&mut guess` en lugar de `&guess` para hacerlo
mutable. (El capítulo 4 explicará las referencias con más detalle.)

<!-- Old heading. Do not remove or links may break. -->
<a id="handling-potential-failure-with-the-result-type"></a>

### Manejando el posible fallo con `Result`

Todavía estamos trabajando en esta línea de código. Ahora estamos discutiendo
una tercera línea de texto, pero ten en cuenta que aún es parte de una sola
línea lógica de código. La siguiente parte es este método:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:expect}}
```

Podríamos haber escrito este código como:

```rust,ignore
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

Sin embargo, una línea larga es difícil de leer, por lo que es mejor dividirla.
A menudo es sabio introducir un salto de línea y otros espacios en blanco para
ayudar a dividir líneas largas cuando llamas a un método con la sintaxis
`.method_name()`. Ahora discutamos lo que hace esta línea.

Como se mencionó anteriormente, `read_line` coloca lo que el usuario ingresa en
la cadena que le pasamos, pero también devuelve un valor `Result`. [`Result`][
result]<!-- ignore --> es una [*enumeración*][enums]<!-- ignore -->, a menudo
llamada *enum*, que es un tipo que puede estar en uno de varios estados
posibles. Llamamos a cada estado posible a una *variante*.

El [Capítulo 6][enums]<!-- ignore --> cubrirá las enumeraciones con más
detalles. El propósito de estos tipos `Result` es codificar información de
manejo de errores.


Las variantes de `Result` son `Ok` y `Err`. La variante `Ok` indica que la
operación fue exitosa, y dentro de `Ok` está el valor generado con éxito. La
variante `Err` significa que la operación falló, y `Err` contiene información
sobre cómo o por qué la operación falló.

Los valores del tipo `Result`, como los valores de cualquier tipo, tienen
métodos definidos en ellos. Una instancia de `Result` tiene un método
[`expect`][expect]<!-- ignore --> que puedes llamar. Si esta instancia de
`Result` es un valor `Err`, `expect` hará que el programa se bloquee y muestre
el mensaje que pasaste como argumento a `expect`. Si el método `read_line`
devuelve un `Err`, probablemente sea el resultado de un error proveniente del
sistema operativo subyacente. Si esta instancia de `Result` es un valor `Ok`,
`expect` tomará el valor de retorno que `Ok` está sosteniendo y devolverá solo
ese valor para que lo puedas usar. En este caso, ese valor es el número de
bytes en la entrada del usuario.

Si no llamas a `expect`, el programa se compilará, pero obtendrás una advertencia:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-02-without-expect/output.txt}}
```

Rust advierte que no has usado el valor `Result` devuelto por `read_line`,
indicando que el programa no ha manejado un posible error.

La forma correcta de suprimir la advertencia es escribir realmente código de
manejo de errores, pero en nuestro caso solo queremos bloquear este programa
cuando ocurra un problema, por lo que podemos usar `expect`. Aprenderás a
recuperarte de los errores en el [Capítulo 9][recover]<!-- ignore -->.

### Imprimiendo valores con marcadores de posición `println!`

Además del corchete de cierre, solo hay una línea más que discutir en el código
hasta ahora:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print_guess}}
```

Esta línea imprime la cadena que ahora contiene la entrada del usuario. El
conjunto de llaves `{}` es un marcador de posición: piensa en `{}` como pequeñas
pinzas de cangrejo que mantienen un valor en su lugar. Al imprimir el valor de
una variable, el nombre de la variable puede ir dentro de las llaves
curvas. Al imprimir el resultado de evaluar una expresión, coloca llaves
curvas vacías en la cadena de formato, luego sigue la cadena de formato con una
lista separada por comas de expresiones para imprimir en cada marcador de
posición vacío de llaves curvas en el mismo orden. Imprimir una variable y el
resultado de una expresión en una llamada a `println!` se vería así:

```rust
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
```

Este código imprimiría `x = 5 and y + 2 = 12`.

### Probando la primera parte

Probemos la primera parte del juego de adivinanzas. Ejecútalo usando `cargo run`:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-01/
cargo clean
cargo run
input 6 -->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6
```

En este punto, la primera parte del juego está terminada: estamos obteniendo
entrada del teclado y luego la imprimimos.

## Generando un número secreto

A continuación, necesitamos generar un número secreto que el usuario intentará
adivinar. El número secreto debe ser diferente cada vez para que el juego sea
divertido de jugar más de una vez. Usaremos un número aleatorio entre 1 y 100
para que el juego no sea demasiado difícil. Rust aún no incluye la
funcionalidad de números aleatorios en su biblioteca estándar. Sin embargo, el
equipo de Rust proporciona un [`rand` crate][randcrate] con dicha
funcionalidad.

### Usando un Crate para obtener más funcionalidad

Recuerda que un crate es una colección de archivos de código fuente de Rust. El
proyecto que hemos estado construyendo es un *binary crate*, que es un
ejecutable. El crate `rand` es un *library crate*, que contiene código que se
pretende usar en otros programas y no se puede ejecutar por sí solo.

La coordinación de los crates externos de Cargo es donde realmente brilla
Cargo. Antes de poder escribir código que use `rand`, necesitamos modificar el
archivo *Cargo.toml* para incluir el crate `rand` como una dependencia. Abre ese
archivo ahora y agrega la siguiente línea al final, debajo del encabezado de la
sección `[dependencies]` que Cargo creó para ti. Asegúrate de especificar `rand`
exactamente como lo tenemos aquí, con este número de versión, o los ejemplos de
código en este tutorial pueden no funcionar:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">Nombre de archivo: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:8:}}
```

En el archivo *Cargo.toml*, todo lo que sigue a un encabezado es parte de esa
sección que continúa hasta que comienza otra sección. En `[dependencies]` le
dices a Cargo qué crates externos depende tu proyecto y qué versiones de esos
crates requieres. En este caso, especificamos el crate `rand` con el
especificador de versión semántica `0.8.5`. Cargo entiende [Semantic
Versioning][semver]<!-- ignore --> (a veces llamado *SemVer*), que es un
estándar para escribir números de versión. El especificador `0.8.5` es
realmente un atajo para `^0.8.5`, lo que significa cualquier versión que sea
al menos 0.8.5 pero inferior a 0.9.0.

Cargo considera que estas versiones tienen APIs públicas compatibles con la
versión 0.8.5, y esta especificación asegura que obtendrá la última versión de
corrección que aún se compilará con el código de este capítulo. Cualquier
versión 0.9.0 o superior no está garantizada de tener la misma API que lo que
usarán los siguientes ejemplos.

Ahora, sin cambiar ningún código, construyamos el proyecto, como se muestra en
el Listado 2-2.

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
rm Cargo.lock
cargo clean
cargo build -->

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.5
  Downloaded libc v0.2.127
  Downloaded getrandom v0.2.7
  Downloaded cfg-if v1.0.0
  Downloaded ppv-lite86 v0.2.16
  Downloaded rand_chacha v0.3.1
  Downloaded rand_core v0.6.3
   Compiling libc v0.2.127
   Compiling getrandom v0.2.7
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.16
   Compiling rand_core v0.6.3
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
```

<span class="caption">Listado 2-2: La salida de ejecutar `cargo build` después
de agregar el crate rand como una dependencia</span>

Es posible que veas números de versión diferentes (¡pero todos serán
compatibles con el código, gracias a SemVer!) y líneas diferentes (dependiendo
del sistema operativo), y las líneas pueden estar en un orden diferente.

Cuando incluimos una dependencia externa, Cargo obtiene las últimas versiones de
todo lo que la dependencia necesita del *registro*, que es una copia de datos
de [Crates.io][cratesio]. Crates.io es donde las personas en el ecosistema de
Rust publican sus proyectos de Rust de código abierto para que otros los
utilicen.

Después de actualizar el registro, Cargo verifica la sección `[dependencies]`
y descarga cualquier crate que se haya enumerado que aún no se haya
descargado. En este caso, aunque solo enumeramos `rand` como una dependencia,
Cargo también tomó otros crates que `rand` depende para funcionar. Después de
descargar los crates, Rust los compila y luego compila el proyecto con las
dependencias disponibles.

Si ejecuta `cargo build` nuevamente sin hacer ningún cambio, no obtendrá
ninguna salida aparte de la línea `Finished`. Cargo sabe que ya ha descargado y
compilado las dependencias, y no ha cambiado nada sobre ellas en su archivo
*Cargo.toml*. Cargo también sabe que no ha cambiado nada sobre su código, por
lo que tampoco lo vuelve a compilar. Sin nada que hacer, simplemente sale.

Si abre el archivo *src/main.rs*, realiza un cambio trivial y luego lo guarda y
vuelve a construir, solo verá dos líneas de salida:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
touch src/main.rs
cargo build -->

```console
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
```

Estas líneas muestran que Cargo solo actualiza la compilación con su pequeño
cambio en el archivo *src/main.rs*. Sus dependencias no han cambiado, por lo
que Cargo sabe que puede reutilizar lo que ya ha descargado y compilado para
esas.

#### Garantizar compilaciones reproducibles con el archivo *Cargo.lock*

Cargo tiene un mecanismo que le garantiza que puede reconstruir el mismo
artefacto cada vez que usted o cualquier otra persona construye su código:
Cargo solo usará las versiones de las dependencias que haya especificado hasta
que indique lo contrario. Por ejemplo, digamos que la semana que viene sale la
versión 0.8.6 del crate `rand`, y que esa versión contiene una corrección de
error importante, pero también contiene una regresión que romperá su código.
Para manejar esto, Rust crea el archivo *Cargo.lock* la primera vez que ejecuta
`cargo build`, por lo que ahora tenemos esto en el directorio *guessing_game*

Cuando construye un proyecto por primera vez, Cargo determina todas las
versiones de las dependencias que cumplen con los criterios y luego las escribe
en el archivo *Cargo.lock*. Cuando construye su proyecto en el futuro, Cargo
verá que el archivo *Cargo.lock* existe y usará las versiones especificadas
allí en lugar de hacer todo el trabajo de averiguar las versiones nuevamente.
Esto le permite tener una compilación reproducible de forma automática. En
otras palabras, su proyecto permanecerá en 0.8.5 hasta que actualice
explícitamente, gracias al archivo *Cargo.lock*. Debido a que el archivo
*Cargo.lock* es importante para las compilaciones reproducibles, a menudo se
verifica en el control de versiones con el resto del código en su proyecto.

#### Actualizar un crate para obtener una nueva versión

Cuando *quiera* actualizar un crate, Cargo proporciona el comando `update`,
que ignorará el archivo *Cargo.lock* y determinará todas las últimas versiones
que cumplan con sus especificaciones en *Cargo.toml*. Cargo luego escribirá
esas versiones en el archivo *Cargo.lock*. De lo contrario, de forma 
predeterminada, Cargo solo buscará versiones mayores que 0.8.5 y menores que
0.9.0. Si el crate `rand` ha lanzado las dos nuevas versiones 0.8.6 y 0.9.0,
vería lo siguiente si ejecutara `cargo update`:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo update
assuming there is a new 0.8.x version of rand; otherwise use another update
as a guide to creating the hypothetical output shown here -->

```console
$ cargo update
    Updating crates.io index
    Updating rand v0.8.5 -> v0.8.6
```

Cargo ignora el lanzamiento 0.9.0. En este punto, también notaría un cambio en
su archivo *Cargo.lock* que indica que la versión del crate `rand` que ahora
está usando es 0.8.6. Para usar la versión 0.9.0 o cualquier versión en la
serie 0.9.*x*, tendría que actualizar el archivo *Cargo.toml* para que se
vea así:

```toml
[dependencies]
rand = "0.9.0"
```

La próxima vez que ejecute `cargo build`, Cargo actualizará el registro de
crates disponibles y volverá a evaluar sus requisitos de `rand` de acuerdo con
la nueva versión que ha especificado.

Hay mucho más que decir sobre [Cargo][doccargo]<!-- ignore --> y [su
ecosistema][doccratesio]<!-- ignore -->, que discutiremos en el Capítulo 14, pero
por ahora, eso es todo lo que necesita saber. Cargo hace muy fácil reutilizar
bibliotecas, por lo que los Rustaceans pueden escribir proyectos más pequeños
que se ensamblan a partir de un número de paquetes.

### Generar un número aleatorio

Comencemos a usar `rand` para generar un número para adivinar. El siguiente
paso es actualizar *src/main.rs*, como se muestra en el Listado 2-3.

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:all}}
```

<span class="caption">Listado 2-3: Agregando código para generar un número
aleatorio</span>

Primero agregamos la línea `use rand::Rng;`. El trait `Rng` define métodos que
los generadores de números aleatorios implementan, y este trait debe estar en
el alcance para que podamos usar esos métodos. El Capítulo 10 cubrirá los
traits en detalle.

A continuación, estamos agregando dos líneas en el medio. En la primera línea,
llamamos a la función `rand::thread_rng` que nos da el generador de números
aleatorios particular que vamos a usar: uno que es local al hilo de ejecución
actual y está sembrado por el sistema operativo. Luego llamamos al método
`gen_range` en el generador de números aleatorios. Este método está definido
por el trait `Rng` que traemos al alcance con la declaración `use rand::Rng;`.
El método `gen_range` toma una expresión de rango como argumento y genera un
número aleatorio en el rango. El tipo de expresión de rango que estamos
utilizando aquí toma la forma `start..=end` y es inclusivo en los límites
inferior y superior, por lo que necesitamos especificar `1..=100` para solicitar
un número entre 1 y 100.

> Nota: No sabrá solo qué traits usar y qué métodos y funciones llamar desde un
> crate, por lo que cada crate tiene documentación con instrucciones para
> usarlo. Otra característica interesante de Cargo es que ejecutar el comando
> `cargo doc --open` construirá la documentación proporcionada por todas sus
> dependencias localmente y la abrirá en su navegador. Si está interesado en
> otra funcionalidad en el crate `rand`, por ejemplo, ejecute `cargo doc
> --open` y haga clic en `rand` en la barra lateral a la izquierda.

La segunda línea nueva imprime el número secreto. Esto es útil mientras
desarrollamos el programa para poder probarlo, pero lo eliminaremos de la
versión final. ¡No es mucho un juego si el programa imprime la respuesta tan
pronto como comienza!

Intente ejecutar el programa varias veces:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-03/
cargo run
4
cargo run
5
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5
```

Debería obtener números aleatorios diferentes, y todos deberían ser números
entre 1 y 100. ¡Gran trabajo!

## Comparando la Adivinanza con el Número Secreto

Ahora que tenemos la entrada del usuario y un número aleatorio, podemos
compararlos. Ese paso se muestra en el Listado 2-4. Tenga en cuenta que este
código aún no se compilará, como explicaremos.

<span class="filename">Nombre del archivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-04/src/main.rs:here}}
```

<span class="caption">Listado 2-4: Manejo de los posibles valores de retorno
de la comparación de dos números</span>

Primero agregamos otra declaración `use`, que trae un tipo llamado
`std::cmp::Ordering` al alcance de la biblioteca estándar. El tipo `Ordering`
es otro enum y tiene las variantes `Less`, `Greater` y `Equal`. Estos son los
tres resultados posibles cuando compara dos valores.

Luego agregamos cinco nuevas líneas al final que usan el tipo `Ordering`. El
método `cmp` compara dos valores y se puede llamar en cualquier cosa que se
pueda comparar. Toma una referencia a lo que quiera comparar: aquí está
comparando `guess` con `secret_number`. Luego devuelve una variante del enum
`Ordering` que importamos al alcance con la declaración `use`. Usamos una
expresión [`match`][match]<!-- ignore --> para decidir qué hacer a continuación
basándonos en qué variante de `Ordering` se devolvió de la llamada a `cmp` con
los valores en `guess` y `secret_number`.

Una expresión `match` está compuesta por *brazos*. Un brazo consta de un
*patrón* para coincidir y el código que se debe ejecutar si el valor dado a
`match` se ajusta al patrón del brazo. Rust toma el valor dado a `match` y
busca cada patrón de brazo en orden. Los patrones y la construcción `match` son
potentes características de Rust: le permiten expresar una variedad de
situaciones que su código puede encontrar y se aseguran de que los maneje
todos. Estas características se cubrirán en detalle en el Capítulo 6 y el
Capítulo 18, respectivamente.

Vamos a repasar un ejemplo con la expresión `match` que usamos aquí. Digamos
que el usuario ha adivinado 50 y el número secreto generado aleatoriamente
esta vez es 38.

Cuando el código compara 50 con 38, el método `cmp` devolverá
`Ordering::Greater` porque 50 es mayor que 38. La expresión `match` obtiene el
valor `Ordering::Greater` y comienza a verificar el patrón de cada brazo. Mira
el patrón del primer brazo, `Ordering::Less`, y ve que el valor
`Ordering::Greater` no coincide con `Ordering::Less`, ¡así que ignora el código
en ese brazo y se mueve al siguiente brazo! El patrón del siguiente brazo es
`Ordering::Greater`, ¡que *sí* coincide con `Ordering::Greater`! El código
asociado en ese brazo se ejecutará y mostrará `Too big!` en la pantalla. La
expresión `match` termina después de la primera coincidencia exitosa, ¡así que
no mirará el último brazo en este escenario.

Sin embargo, el código del Listado 2-4 aún no se compilará. Vamos a intentarlo:

<!--
The error numbers in this output should be that of the code **WITHOUT** the
anchor or snip comments
-->

```console
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-04/output.txt}}
```

El núcleo del error indica que hay *tipos no coincidentes*. Rust tiene un
sistema de tipos fuerte y estático. Sin embargo, también tiene inferencia de
tipo. Cuando escribimos `let mut guess = String::new()`, Rust pudo inferir que
`guess` debería ser un `String` y no nos obligó a escribir el tipo. El
`secret_number`, por otro lado, es un tipo de número. Algunos de los tipos de
números de Rust pueden tener un valor entre 1 y 100: `i32`, un número de 32 bits;
`u32`, un número sin signo de 32 bits; `i64`, un número de 64 bits; así como
otros. A menos que se especifique lo contrario, Rust predetermina un `i32`, que
es el tipo de `secret_number` a menos que agregue información de tipo en otro
lugar que haga que Rust infiera un tipo numérico diferente. La razón del error
es que Rust no puede comparar una cadena y un tipo numérico.

Finalmente, queremos convertir la `String` que el programa lee como entrada en
un tipo de número real para que podamos compararlo numéricamente con el número
secreto. Lo hacemos agregando esta línea al cuerpo de la función `main`:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/src/main.rs:here}}
```

La línea es:

```rust,ignore
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

Creamos una variable llamada `guess`. Pero espera, ¿no tiene el programa ya una
variable llamada `guess`? Lo hace, pero Rust nos permite redefinir el valor
anterior de `guess` con uno nuevo. Este concepto en Rust se le conoce como 
*Shadowing*, nos permite volver a usar el nombre de la variable `guess` 
en lugar de obligarnos a crear dos variables únicas, como `guess_str` 
y `guess`, por ejemplo. Lo cubriremos con más detalle en el 
[Capítulo 3][shadowing]<!-- ignore -->, pero por ahora, sé que esta
característica se usa a menudo cuando desea convertir un valor de un tipo a
otro tipo.

Enlazamos esta nueva variable a la expresión `guess.trim().parse()`. La `guess`
en la expresión se refiere a la variable `guess` original que contenía la
entrada como una cadena. El método `trim` en una instancia `String` eliminará
cualquier espacio en blanco al principio y al final, lo que debemos hacer para
poder comparar la cadena con el `u32`, que solo puede contener datos numéricos.
El usuario debe presionar <span class="keystroke">enter</span> para satisfacer
`read_line` e ingresar su conjetura, lo que agrega un carácter de nueva línea
a la cadena. Por ejemplo, si el usuario escribe <span class="keystroke">5</span>
y presiona <span class="keystroke">enter</span>, `guess` se ve así: `5\n`. El
`\n` representa "nueva línea". (En Windows, presionar <span
class="keystroke">enter</span> resulta en un retorno de carro y una nueva
línea, `\r\n`). El método `trim` elimina `\n` o `\r\n`, lo que resulta en solo
`5`.

El [método `parse` en las cadenas][parse]<!-- ignore --> convierte una cadena
en otro tipo. Aquí, lo usamos para convertir de una cadena a un número. Debemos
decirle a Rust el tipo de número exacto que queremos usando `let guess: u32`.
Los dos puntos (`:`) después de `guess` le dicen a Rust que anotaremos el tipo
de variable. Rust tiene algunos tipos de número integrados; el `u32` visto
aquí es un entero sin signo de 32 bits. Es una buena opción predeterminada para
un número positivo pequeño. Aprenderá sobre otros tipos de números en el
[Capítulo 3][integers]<!-- ignore -->.

Además, la anotación `u32` en este programa de ejemplo y la comparación con
`secret_number` significa que Rust inferirá que `secret_number` también
debería ser `u32`. ¡Entonces la comparación será entre dos valores del mismo
tipo!

El método `parse` solo funcionará en caracteres que se puedan convertir
lógicamente en números y, por lo tanto, pueden causar fácilmente errores. Si,
por ejemplo, la cadena contiene `A👍%`, no habría manera de convertir eso en un
número. Debido a que podría fallar, el método `parse` devuelve un tipo `Result`,
tal como lo hace el método `read_line` (discutido anteriormente en
[“Manejo de posibles fallas con `Result`”](#handling-potential-failure-with-result)<!-- ignore-->).
Trataremos este `Result` de la misma manera usando el método `expect` de nuevo.
Si `parse` devuelve una variante `Err` del tipo `Result` porque no pudo crear
un número a partir de la cadena, la llamada `expect` hará que el juego se
bloquee y muestre el mensaje que le damos. Si `parse` puede convertir
exitosamente la cadena en un número, devolverá la variante `Ok` del tipo
`Result`, y `expect` devolverá el número que queremos del valor `Ok`.

¡Corramos el programa ahora!


<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/
cargo run
  76
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```

¡Bien! Aunque se agregaron espacios antes de la adivinanza, el programa aún
sabía que el usuario adivinó 76. Ejecute el programa varias veces para
verificar el comportamiento diferente con diferentes tipos de entrada: adivine
el número correctamente, adivine un número que sea demasiado alto y adivine un
número que sea demasiado bajo.

Tenemos la mayoría del juego funcionando ahora, pero el usuario solo puede
adivinar una vez. ¡Cambiamos eso agregando un bucle!

## Permitir múltiples adivinanzas con bucles


La palabra clave `loop` crea un bucle infinito. Agregaremos un bucle para darle
a los usuarios más oportunidades para adivinar el número:

<span class="filename">Filename: src/main.rs</span>

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-04-looping/src/main.rs:here}}
```

Como puede ver, hemos movido todo desde la solicitud de entrada de adivinanzas
hacia adelante en un bucle. Asegúrese de indentar las líneas dentro del bucle
otras cuatro veces y ejecute el programa nuevamente. ¡El programa ahora pedirá
otra adivinanza para siempre, lo que introduce un nuevo problema! ¡Parece que el
usuario no puede salir!

El usuario siempre podría interrumpir el programa usando el atajo de teclado
<span class="keystroke">ctrl-c</span>. Pero hay otra forma de escapar de este
monstruo insaciable, como se mencionó en la discusión de `parse` en
[“Comparando la adivinanza con el número secreto”](#comparando-la-adivinanza-con-el-número-secreto)<!--
ignore -->: si el usuario ingresa una respuesta que no es un número, el
programa se bloqueará. Podemos aprovechar eso para permitir que el usuario
salga, como se muestra aquí:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-04-looping/
cargo run
(too small guess)
(too big guess)
(correct guess)
quit
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
quit
thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/main.rs:28:47
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Al escribir `quit` se cerrará el juego, pero como notará, también lo hará al
ingresar cualquier otra entrada que no sea un número. Esto es lo menos
óptimo, para decir lo menos; queremos que el juego también se detenga cuando se
adivine el número correcto.

### Salir después de una adivinanza correcta

Programemos el juego para que se cierre cuando el usuario gane agregando una
declaración `break`:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-05-quitting/src/main.rs:here}}
```

Agregando la línea `break` después de `You win!` hace que el programa salga del
bucle cuando el usuario adivina el número secreto correctamente. Salir del
bucle también significa salir del programa, porque el bucle es la última parte
de `main`.

### Manejo de entrada no válida

Para mejorar aún más el comportamiento del juego, en lugar de bloquear el
programa cuando el usuario ingresa un número no válido, hagamos que el juego
ignore un número no válido para que el usuario pueda seguir adivinando. Podemos
hacer eso alterando la línea donde `guess` se convierte de un `String` a un
`u32`, como se muestra en el Listado 2-5.

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:here}}
```

<span class="caption">Listado 2-5: Ignorar una adivinanza que no es un número
y pedir otra adivinanza en lugar de bloquear el programa</span>

Cambiamos de una llamada `expect` a una expresión `match` para pasar de
bloquear el programa en un error a manejar el error. Recuerde que `parse`
devuelve un tipo `Result` y `Result` es un enum que tiene las variantes `Ok` y
`Err`. Aquí estamos usando una expresión `match`, como hicimos con el resultado
`Ordering` del método `cmp`.

Si `parse` es capaz de convertir exitosamente la cadena en un número, devolverá
un valor `Ok` que contiene el número resultante. Ese valor `Ok` coincidirá con
el patrón de la primera rama y la expresión `match` devolverá el valor `num`
que `parse` produjo y puso dentro del valor `Ok`. Ese número terminará en el
lugar correcto en la nueva variable `guess` que estamos creando.

Si `parse` *no* es capaz de convertir la cadena en un número, devolverá un
valor `Err` que contiene más información sobre el error. El valor `Err` no
coincide con el patrón `Ok(num)` en la primera rama de `match`, pero sí
coincide con el patrón `Err(_)` en la segunda rama. El guión bajo, `_`, es un
valor de captura; en este ejemplo, estamos diciendo que queremos coincidir con
todos los valores `Err`, sin importar qué información tengan dentro. ¡Así que
el programa ejecutará el código de la segunda rama, `continue`, que le dice al
programa que vaya a la siguiente iteración del `loop` y pida otra adivinanza.
¡Así que, efectivamente, el programa ignora todos los errores que `parse` puede
encontrar!

Ahora todo en el programa debería funcionar como se espera. Vamos a probarlo:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-05/
cargo run
(too small guess)
(too big guess)
foo
(correct guess)
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 4.45s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!
```

¡Genial! Con un pequeño ajuste final, terminaremos el juego de adivinanzas.
Recuerde que el programa todavía está imprimiendo el número secreto. Eso
funcionó bien para las pruebas, pero arruina el juego. Vamos a eliminar el
`println!` que muestra el número secreto. El listado 2-6 muestra el código
final.

<span class="filename">Nombre del archivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-06/src/main.rs}}
```

<span class="caption">Listado 2-6: Código completo del juego de adivinanzas</span>

En este punto, ha construido exitosamente el juego de adivinanzas. ¡Felicidades!

## Resumen

Este proyecto fue una manera práctica de introducirle a muchos nuevos conceptos
de Rust: `let`, `match`, funciones, el uso de paquetes externos, y más. En los
próximos capítulos, aprenderá sobre estos conceptos en más detalle. El capítulo
3 cubre conceptos que la mayoría de los lenguajes de programación tienen, como
variables, tipos de datos y funciones, y muestra cómo usarlos en Rust. El
capítulo 4 explora la propiedad, una característica que hace que Rust sea
diferente de otros lenguajes. El capítulo 5 discute las estructuras y la
sintaxis de los métodos, y el capítulo 6 explica cómo funcionan los enums.

[prelude]: https://doc.rust-lang.org/std/prelude/index.html
[variables-and-mutability]: ch03-01-variables-and-mutability.html#variables-and-mutability
[comments]: ch03-04-comments.html
[string]: https://doc.rust-lang.org/std/string/struct.String.html
[iostdin]: https://doc.rust-lang.org/std/io/struct.Stdin.html
[read_line]: https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line
[result]: https://doc.rust-lang.org/std/result/enum.Result.html
[enums]: ch06-00-enums.html
[expect]: https://doc.rust-lang.org/std/result/enum.Result.html#method.expect
[recover]: ch09-02-recoverable-errors-with-result.html
[randcrate]: https://crates.io/crates/rand
[semver]: http://semver.org
[cratesio]: https://crates.io/
[doccargo]: http://doc.crates.io
[doccratesio]: http://doc.crates.io/crates-io.html
[match]: ch06-02-match.html
[shadowing]: ch03-01-variables-and-mutability.html#shadowing
[parse]: https://doc.rust-lang.org/std/primitive.str.html#method.parse
[integers]: ch03-02-data-types.html#integer-types
