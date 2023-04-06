## Tipos de datos

Cada valor en Rust es de un cierto *tipo de dato*, que le dice a Rust qué tipo
de dato se está especificando para que sepa cómo trabajar con ese dato. Veremos
dos subconjuntos de tipos de datos: escalar y compuesto.

Tenga en cuenta que Rust es un lenguaje *estáticamente tipado*, lo que significa
que debe conocer los tipos de todas las variables en tiempo de compilación. El
compilador generalmente puede inferir qué tipo queremos usar en función del
valor y cómo lo usamos. En los casos en que muchos tipos son posibles, como
cuando convertimos un `String` en un tipo numérico usando `parse` en la
[sección “Comparando la Adivinanza con el Número Secreto”][comparing-the-guess-to-the-secret-number]
<!-- ignore --> del capítulo 2, debemos agregar una anotación de tipo, como
esta:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

Si no agregamos la anotación de tipo `: u32` mostrada en el código anterior,
Rust mostrará el siguiente error, lo que significa que el compilador necesita
más información de nosotros para saber qué tipo queremos usar:

```console
{{#include ../listings/ch03-common-programming-concepts/output-only-01-no-type-annotations/output.txt}}
```

Verá diferentes anotaciones de tipo para otros tipos de datos.

### Tipos Escalares

Un tipo *escalar* representa un solo valor. Rust tiene cuatro tipos escalares
principales: enteros, números de punto flotante, booleanos y caracteres. Puede
reconocerlos de otros lenguajes de programación. Vamos a ver cómo funcionan en
Rust.

#### Tipos de Enteros

Un *entero* es un número sin componente fraccionario. Usamos un tipo de entero
en el capítulo 2, el tipo `u32`. Esta declaración de tipo indica que el valor
con el que está asociado debe ser un entero sin signo (los tipos de enteros con
signo comienzan con `i` en lugar de `u`) que ocupa 32 bits de espacio. La tabla
3-1 muestra los tipos de enteros integrados en Rust. Podemos usar cualquiera de
estas variantes para declarar el tipo de un valor entero.

<span class="caption">Tabla 3-1: Tipos Enteros en Rust</span>

| Tamaño  | Signed  | Unsigned |
|---------|---------|----------|
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |

Cada variante puede ser *signed* (con signo) o *unsigned* (sin signo) y tiene 
un tamaño explícito. *Signed* y *unsigned* se refieren a si es posible que el
número sea negativo, es decir, si el número necesita tener un signo con él 
(signed) o si solo será positivo y por lo tanto puede representarse sin signo
(unsigned). Es como escribir números en papel: cuando el signo importa,
un número se muestra con un signo más o un signo menos; sin embargo, cuando es
seguro suponer que el número es positivo, se muestra sin signo. 
Los números con signo se almacenan usando la
[representación de complemento a dos][twos-complement]<!-- ignore -->.

Cada variante con signo puede almacenar números de -(2<sup>n - 1</sup>) 
a 2<sup>n - 1</sup> - 1, donde *n* es el número de bits que usa la variante. 
Así, un `i8` puede almacenar números de -(2<sup>7</sup>) a 2<sup>7</sup> - 1, 
lo que equivale a -128 a 127. Las variantes sin signo pueden almacenar números 
de 0 a 2<sup>n</sup> - 1, por lo que un `u8` puede almacenar números de 0 a 2<sup>8</sup> - 1, 
lo que equivale a 0 a 255.

Además, los tipos `isize` y `usize` dependen de la arquitectura de la 
computadora en la que se ejecuta su programa, que se denota en la tabla como 
“arch”: 64 bits si está en una arquitectura de 64 bits y 32 bits si está en una 
arquitectura de 32 bits.

Puede escribir literales enteros en cualquiera de las formas que se muestran en
la Tabla 3-2. Tenga en cuenta que los literales numéricos que pueden ser
múltiples tipos numéricos permiten un sufijo de tipo, como `57u8`, para
designar el tipo. Los literales numéricos también pueden usar `_` como un
separador visual para facilitar la lectura del número, como `1_000`, que tendrá
el mismo valor que si hubiera especificado `1000`.

<span class="caption">Tabla 3-2: Literales enteros en Rust</span>

| Literales numéricos  | Ejemplo       |
|----------------------|---------------|
| Decimal              | `98_222`      |
| Hex                  | `0xff`        |
| Octal                | `0o77`        |
| Binario              | `0b1111_0000` |
| Byte (`u8` solamente) | `b'A'`        |

Entonces, ¿cómo sabe qué tipo de entero usar? Si no está seguro, los valores
predeterminados de Rust son generalmente buenos lugares para comenzar: los
tipos enteros se configuran predeterminadamente en `i32`. La situación
principal en la que usaría `isize` o `usize` es cuando indexa algún tipo de
colección.

> ##### Desbordamiento de enteros
>
> Digamos que tiene una variable de tipo `u8` que puede contener valores entre 0
> y 255. Si intenta cambiar la variable a un valor fuera de ese rango, como 256,
> *se producirá un desbordamiento de enteros*, que puede resultar en uno de dos
> comportamientos. Cuando está compilando en modo de depuración, Rust incluye
> comprobaciones para el desbordamiento de enteros que hacen que su programa
> *se desborde* en tiempo de ejecución si ocurre este comportamiento.
> Rust usa el término *desbordamiento* cuando un programa sale con un error;
> discutiremos los desbordamientos con más profundidad en la sección [“Errores
> irrecuperables con `panic!`”][unrecoverable-errors-with-panic]<!-- ignore -->
> del Capítulo 9.
>
> Cuando está compilando en modo de lanzamiento con la bandera `--release`,
> Rust *no* incluye comprobaciones para el desbordamiento de enteros que
> provocan desbordamientos. En su lugar, si ocurre un desbordamiento, Rust
> realiza *complemento de dos envolviendo*. En resumen, los valores mayores que
> el valor máximo que el tipo puede contener “se envuelven” al mínimo de los
> valores que el tipo puede contener. En el caso de un `u8`, el valor 256 se
> convierte en 0, el valor 257 se convierte en 1, y así sucesivamente. El
> programa no se desbordará, pero la variable tendrá un valor que probablemente
> no sea el que esperaba que tuviera. Depender del comportamiento de envoltura
> del desbordamiento de enteros se considera un error.
>
> Para manejar explícitamente la posibilidad de desbordamiento, puede usar estas
> familias de métodos proporcionados por la biblioteca estándar para tipos
> numéricos primitivos:
>
> * Envolver en todos los modos con los métodos `wrapping_*`, como
>   `wrapping_add`.
> * Devolver el valor `None` si hay desbordamiento con los métodos `checked_*`.
> * Devolver el valor y un booleano que indica si hubo desbordamiento con los
>   métodos `overflowing_*`.
> * Saturar en los valores mínimos o máximos del valor con los métodos
>   `saturating_*`.

#### Tipos de punto flotante

Rust también tiene dos tipos primitivos para *números de punto flotante*, que
son números con puntos decimales. Los tipos de punto flotante de Rust son `f32`
y `f64`, que tienen 32 bits y 64 bits de tamaño, respectivamente. El tipo
predeterminado es `f64` porque en CPUs modernas, es aproximadamente la misma
velocidad que `f32` pero es capaz de más precisión. Todos los tipos de punto
flotante son con signo.

Aquí hay un ejemplo que muestra números de punto flotante en acción:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-06-floating-point/src/main.rs}}
```

Los números de punto flotante se representan de acuerdo con el estándar
IEEE-754. El tipo `f32` es un punto flotante de precisión simple, y `f64` tiene
doble precisión.

#### Operaciones numéricas

Rust admite las operaciones matemáticas básicas que esperaría para todos los
tipos de números: adición, sustracción, multiplicación, división y resto.
La división entera se trunca hacia cero al entero más cercano. El siguiente
código muestra cómo usaría cada operación numérica en una declaración `let`:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-07-numeric-operations/src/main.rs}}
```

Cada expresión en estas instrucciones usa un operador matemático y se evalúa a
un solo valor, que luego se vincula a una variable. [Apéndice
B][appendix_b]<!-- ignore --> contiene una lista de todos los operadores que
Rust proporciona.

#### El tipo booleano

Como en la mayoría de los otros lenguajes de programación, un tipo booleano en
Rust tiene dos posibles valores: `true` y `false`. Los booleanos tienen un
byte de tamaño. El tipo booleano en Rust se especifica usando `bool`. Por
ejemplo:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-08-boolean/src/main.rs}}
```

La forma principal de usar valores booleanos es a través de condicionales, como
una expresión `if`. Cubriremos cómo funcionan las expresiones `if` en Rust en
la sección [“Control de flujo”][control-flow]<!-- ignore -->.

#### El tipo de carácter

El tipo `char` de Rust es el tipo alfabético más primitivo del lenguaje. Aquí
hay algunos ejemplos de declarar valores `char`:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-09-char/src/main.rs}}
```

Tenga en cuenta que especificamos literales `char` con comillas simples, en
oposición a literales de cadena, que usan comillas dobles. El tipo `char` de
Rust tiene cuatro bytes de tamaño y representa un valor escalar de Unicode,
lo que significa que puede representar mucho más que solo ASCII. Letras
acentuadas; Caracteres chinos, japoneses y coreanos; Emojis; y espacios de ancho
cero son todos valores `char` válidos en Rust. Los valores escalar de Unicode
van desde `U+0000` a `U+D7FF` y `U+E000` a `U+10FFFF` inclusive. Sin embargo,
un "carácter" no es realmente un concepto en Unicode, por lo que su intuición
humana sobre lo que es un "carácter" puede no coincidir con lo que es un `char`
en Rust. Discutiremos este tema en detalle en [“Almacenar texto codificado en
UTF-8 con cadenas”][strings]<!-- ignore --> en el capítulo 8.

### Tipos compuestos

*Tipos compuestos* pueden agrupar múltiples valores en un solo tipo. Rust
tiene dos tipos compuestos primitivos: tuplas y matrices.

#### El Tipo Tupla

Una *tupla* es una forma general de agrupar juntos un número de valores con
una variedad de tipos en un solo tipo compuesto. Las tuplas tienen una
longitud fija: una vez declaradas, no pueden crecer ni disminuir en tamaño.

Creamos una tupla escribiendo una lista separada por comas de valores dentro de
paréntesis. Cada posición en la tupla tiene un tipo, y los tipos de los
valores diferentes en la tupla no tienen que ser los mismos. Hemos agregado
anotaciones de tipo opcionales en este ejemplo:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-10-tuples/src/main.rs}}
```

La variable `tup` se vincula a toda la tupla porque se considera que una tupla
es un único elemento compuesto. Para obtener los valores individuales de una
tupla, podemos usar el emparejamiento de patrones para descomponer un valor de
tupla, así:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-11-destructuring-tuples/src/main.rs}}
```

Este programa primero crea una tupla y la vincula a la variable `tup`. Luego
usa un patrón con `let` para tomar `tup` y convertirla en tres variables
separadas, `x`, `y` y `z`. Esto se llama *desestructuración* porque rompe la
única tupla en tres partes. Finalmente, el programa imprime el valor de `y`,
que es `6.4`.

También podemos acceder directamente a un elemento de la tupla usando un punto
(`.`) seguido del índice del valor que queremos acceder. Por ejemplo:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-12-tuple-indexing/src/main.rs}}
```

Este programa crea la tupla `x` y luego accede a cada elemento de la tupla
usando sus respectivos índices. Al igual que la mayoría de los lenguajes de
programación, el primer índice en una tupla es 0.

La tupla sin ningún valor tiene un nombre especial, *unit*. Este valor y su
tipo correspondiente están escritos ambos como `()` y representan un valor
vacío o un tipo de retorno vacío. Las expresiones devuelven implícitamente el
valor unit si no devuelven ningún otro valor.

#### El Tipo Arreglo

Otra forma de tener una colección de múltiples valores es con un *arreglo*.
A diferencia de una tupla, cada elemento de un arreglo debe tener el mismo
tipo. A diferencia de los arreglos en algunos otros lenguajes, los arreglos en
Rust tienen una longitud fija.

Escribimos los valores en un arreglo como una lista separada por comas dentro
de corchetes:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-13-arrays/src/main.rs}}
```

Los arreglos son útiles cuando desea que sus datos se asignen en el stack (pila)
en lugar del heap (monticulo) (hablaremos más sobre el stack y el heap en el 
[Capítulo 4][stack-and-heap]<!-- ignore -->) o cuando desea asegurarse de que
siempre tenga un número fijo de elementos. Sin embargo, un arreglo no es tan
flexible como el tipo vector. Un *vector* es un tipo de colección similar
proporcionado por la biblioteca estándar que *puede* crecer o reducir su tamaño.
Si no está seguro de si debe usar un arreglo o un vector, es probable que deba
usar un vector. El [Capítulo 8][vectors]<!-- ignore --> discute los vectores en
más detalle.

Sin embargo, los arreglos son más útiles cuando sabe que el número de elementos
no cambiará. Por ejemplo, si está utilizando los nombres del mes en un
programa, probablemente usaría un arreglo en lugar de un vector porque sabe que
siempre contendrá 12 elementos:

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

Escribe el tipo de un arreglo usando corchetes con el tipo de cada elemento,
un punto y coma y luego el número de elementos en el arreglo, así:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Aquí, `i32` es el tipo de cada elemento. Después del punto y coma, el número
`5` indica que el arreglo contiene cinco elementos.

También puede inicializar un arreglo para contener el mismo valor para cada
elemento especificando el valor inicial, seguido de un punto y coma y luego la
longitud del arreglo en corchetes, como se muestra aquí:

```rust
let a = [3; 5];
```

El arreglo llamado `a` contendrá `5` elementos que inicialmente se establecerán
en el valor `3`. Esto es lo mismo que escribir `let a = [3, 3, 3, 3, 3];` pero
de una manera más concisa.

##### Accediendo a los Elementos del Arreglo


Un arreglo es un solo fragmento de memoria de un tamaño conocido y fijo que se
puede asignar en el stack. Puede acceder a los elementos de un arreglo usando
indexación, así:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-14-array-indexing/src/main.rs}}
```

En este ejemplo, la variable llamada `first` obtendrá el valor `1` porque ese
es el valor en el índice `[0]` en el arreglo. La variable llamada `second`
obtendrá el valor `2` del índice `[1]` en el arreglo.

##### Acceso Inválido a los Elementos del Arreglo

Veamos qué sucede si intenta acceder a un elemento de un arreglo que está más
allá del final del arreglo. Digamos que ejecuta este código, similar al juego
de adivinanzas del Capítulo 2, para obtener un índice de arreglo del usuario:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore,panics
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access/src/main.rs}}
```

Este código se compila con éxito. Si ejecuta este código usando `cargo run` y
ingresa `0`, `1`, `2`, `3` o `4`, el programa imprimirá el valor
correspondiente en ese índice en el arreglo. Si en cambio ingresa un número
más allá del final del arreglo, como `10`, verá una salida como esta:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access
cargo run
10
-->

```console
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

El programa resultó en un error de *ejecución* en el punto de uso de un valor
inválido en la operación de indexación. El programa salió con un mensaje de
error y no ejecutó la última instrucción `println!`. Cuando intenta acceder a un
elemento usando indexación, Rust verificará que el índice que ha especificado
es menor que la longitud del arreglo. Si el índice es mayor o igual que la
longitud, Rust entrará en pánico. Esta verificación debe ocurrir en tiempo de
ejecución, especialmente en este caso, porque el compilador no puede
posiblemente saber qué valor ingresará un usuario cuando ejecuten el código
después.

Este es un ejemplo de los principios de seguridad de la memoria de Rust en
acción. En muchos lenguajes de bajo nivel, esta clase de verificación no se
realiza, y cuando proporciona un índice incorrecto, se puede acceder a la memoria
inválida. Rust lo protege contra este tipo de error al salir inmediatamente en
vez de permitir el acceso a la memoria y continuar. El Capítulo 9 discute más de
la gestión de errores de Rust y cómo puede escribir código legible y seguro que
ni se pone en pánico ni permite el acceso a la memoria inválida.

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#comparando-la-adivinanza-con-el-número-secreto
[twos-complement]: https://es.wikipedia.org/wiki/Complemento_a_dos
[control-flow]: ch03-05-control-flow.html#control-flow
[strings]: ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
[stack-and-heap]: ch04-01-what-is-ownership.html#the-stack-and-the-heap
[vectors]: ch08-01-vectors.html
[unrecoverable-errors-with-panic]: ch09-01-unrecoverable-errors-with-panic.html
[appendix_b]: appendix-02-operators.md
