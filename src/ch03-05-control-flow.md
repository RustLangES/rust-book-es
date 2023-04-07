## Flujo de Control

La capacidad de ejecutar algún código dependiendo de si una condición es `true`
y ejecutar algún código repetidamente mientras una condición es `true` son
bloques de construcción básicos en la mayoría de los lenguajes de programación.
Las construcciones más comunes que le permiten controlar el flujo de ejecución
del código Rust son las expresiones `if` y los bucles.

### Expresiones `if`

Una expresión `if` le permite dividir su código según las condiciones.
Proporciona una condición y luego dice: “Si se cumple esta condición, ejecute
este bloque de código. Si la condición no se cumple, no ejecute este bloque de
código.”

Cree un nuevo proyecto llamado *branches* en su directorio *projects* para
explorar la expresión `if`. En el archivo *src/main.rs*, ingrese lo siguiente:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/src/main.rs}}
```

Todas las expresiones `if` comienzan con la palabra clave `if`, seguida de una
condición. En este caso, la condición verifica si la variable `number` tiene un
valor menor que 5. Colocamos el bloque de código para ejecutar si la condición
es `true` inmediatamente después de la condición dentro de llaves. Los bloques
de código asociados con las condiciones en las expresiones `if` a veces se
llaman *brazos*, al igual que los brazos en las expresiones `match` que
discutimos en la sección
[“Comparando la Adivinanza 
con el Número Secreto”][comparing-the-guess-to-the-secret-number]<!--ignore --> 
del Capítulo 2.

Opcionalmente, también podemos incluir una expresión `else`, que elegimos
hacer aquí, para dar al programa un bloque de código alternativo para ejecutar
si la condición se evalúa como `false`. Si no proporciona una expresión `else`
y la condición es `false`, el programa solo omitirá el bloque `if` y continuará
con el siguiente fragmento de código.

Intente ejecutar este código; Debería ver la siguiente salida:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/output.txt}}
```

Vamos a intentar cambiar el valor de `number` a un valor que haga que la
condición sea `false` para ver qué sucede:

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/src/main.rs:here}}
```

Ejecute el programa nuevamente y observe la salida:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/output.txt}}
```

También vale la pena señalar que la condición en este código *debe* ser un
`bool`. Si la condición no es un `bool`, obtendremos un error. Por ejemplo,
intente ejecutar el siguiente código:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/src/main.rs}}
```

La condición `if` se evalúa como un valor de `3` esta vez, y Rust arroja un
error:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/output.txt}}
```

El error indica que Rust esperaba un `bool` pero obtuvo un entero. A diferencia
de los lenguajes como Ruby y JavaScript, Rust no intentará convertir
automáticamente los tipos no booleanos en un booleano. Debe ser explícito y
siempre proporcionar a `if` un booleano como su condición. Si queremos que el
bloque de código `if` se ejecute solo cuando un número no sea igual a `0`, por
ejemplo, podemos cambiar la expresión `if` a lo siguiente:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-29-if-not-equal-0/src/main.rs}}
```

Ejecutando este código imprimirá `number was something other than zero`.

#### Manejo de múltiples condiciones con `else if`

Puede usar múltiples condiciones combinando `if` y `else` en una expresión
`else if`. Por ejemplo:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/src/main.rs}}
```

Este programa tiene cuatro posibles caminos que puede tomar. Después de
ejecutarlo, debería ver la siguiente salida:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/output.txt}}
```

Cuando se ejecuta este programa, verifica cada expresión `if` en orden y
ejecuta el primer cuerpo para el cual la condición se evalúa como `true`. Tenga
en cuenta que incluso si 6 es divisible por 2, no vemos la salida `number is
divisible by 2`, ni vemos el texto `number is not divisible by 4, 3, or 2` del
bloque `else`. Esto se debe a que Rust solo ejecuta el bloque para la primera
condición `true`, y una vez que encuentra una, ni siquiera verifica el resto.

El uso de demasiadas expresiones `else if` puede ensuciar su código, por lo que
si tiene más de una, es posible que desee refactorizar su código. El capítulo 6
describe una poderosa construcción de ramificación de Rust llamada `match` para
estos casos.

#### Usando `if` en una declaración `let`

Dado que `if` es una expresión, podemos usarlo en el lado derecho de una
declaración `let` para asignar el resultado a una variable, como en el Listado
3-2.

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-02/src/main.rs}}
```

<span class="caption">Listado 3-2: Asignando el resultado de una expresión
`if` a una variable</span>

La variable `number` estará vinculada a un valor basado en el resultado de la
expresión `if`. Ejecute este código para ver qué sucede:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-02/output.txt}}
```

Recuerde que los bloques de código se evalúan en la última expresión de ellos y
los números por sí mismos también son expresiones. En este caso, el valor de
la expresión `if` en su conjunto depende de qué bloque de código se ejecuta.
Esto significa que los valores que tienen el potencial de ser resultados de cada
rama del `if` deben ser del mismo tipo; en el Listado 3-2, los resultados de
ambas ramas del `if` y la rama `else` fueron enteros `i32`. Si los tipos no
coinciden, como en el siguiente ejemplo, obtendremos un error:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/src/main.rs}}
```

Cuando intentamos compilar este código, obtendremos un error. Las ramas `if` y
`else` tienen tipos de valor que son incompatibles, y Rust indica exactamente
dónde encontrar el problema en el programa:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/output.txt}}
```

La expresión en el bloque `if` se evalúa como un entero, y la expresión en el
bloque `else` se evalúa como una cadena. Esto no funcionará porque las
variables deben tener un solo tipo, y Rust necesita saber en tiempo de
compilación qué tipo tiene la variable `number`, definitivamente. Conocer el
tipo de `number` permite al compilador verificar que el tipo sea válido en
cualquier lugar que usemos `number`. Rust no podría hacerlo si el tipo de
`number` solo se determinara en tiempo de ejecución; el compilador sería más
complejo y haría menos garantías sobre el código si tuviera que rastrear
diversos tipos hipotéticos para cualquier variable.

### Repetición con bucles

A menudo es útil ejecutar un bloque de código más de una vez. Para esta tarea,
Rust proporciona varios *bucles*, que ejecutarán el código dentro del cuerpo del
bucle hasta el final y luego comenzarán de inmediato desde el principio. Para
experimentar con los bucles, hagamos un nuevo proyecto llamado *loops*.

Rust tiene tres tipos de bucles: `loop`, `while` y `for`. Vamos a probar cada
uno.

#### Repetir código con `loop`

La palabra clave `loop` le dice a Rust que ejecute un bloque de código una y
otra vez para siempre o hasta que le indique explícitamente que se detenga.

Como ejemplo, cambie el archivo *src/main.rs* en su directorio *loops* para
que se vea así:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-loop/src/main.rs}}
```

Cuando ejecutemos este programa, veremos `again!` impreso una y otra vez
continuamente hasta que detengamos manualmente el programa. La mayoría de los
terminales admiten el atajo de teclado <span class="keystroke">ctrl-c</span>
para interrumpir un programa que está atascado en un bucle continuo.
Inténtelo:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-32-loop
cargo run
CTRL-C
-->

```console
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

El símbolo `^C` representa dónde presionó <span
class="keystroke">ctrl-c</span>. Puede que vea o no la palabra `again!`
impresa después del `^C`, dependiendo de dónde estaba el código en el bucle
cuando recibió la señal de interrupción.

Afortunadamente, Rust también proporciona una forma de salir de un bucle
utilizando código. Puede colocar la palabra clave `break` dentro del bucle para
decirle al programa cuándo dejar de ejecutar el bucle. Recuerde que hicimos
esto en el juego de adivinanzas en la sección [“Salir después de una
adivinanza correcta”][quitting-after-a-correct-guess]<!-- ignore --> del
capítulo 2 para salir del programa cuando el usuario ganó el juego adivinando
el número correcto.

También usamos `continue` en el juego de adivinanzas, que en un bucle le dice
al programa que omita cualquier código restante en esta iteración del bucle y
pase a la siguiente iteración.

#### Devolviendo valores de los bucles

Una de las aplicaciones de un `loop` es volver a intentar una operación que
sabe que puede fallar, como verificar si un hilo ha completado su trabajo. Es
posible que también necesite pasar el resultado de esa operación fuera del
bucle al resto de su código. Para hacer esto, puede agregar el valor que desea
devolver después de la expresión `break` que usa para detener el bucle; ese
valor se devolverá fuera del bucle para que pueda usarlo, como se muestra aquí:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-33-return-value-from-loop/src/main.rs}}
```

Antes del bucle, declaramos una variable llamada `counter` e inicializamos en
`0`. Luego declaramos una variable llamada `result` para contener el valor
devuelto del bucle. En cada iteración del bucle, agregamos `1` a la variable
`counter`, y luego verificamos si el `counter` es igual a `10`. Cuando lo es,
usamos la palabra clave `break` con el valor `counter * 2`. Después del bucle,
usamos un punto y coma para terminar la instrucción que asigna el valor a
`result`. Finalmente, imprimimos el valor en `result`, que en este caso es
`20`.

#### Etiquetas de bucle para desambiguar entre varios bucles

Si tiene bucles dentro de bucles, `break` y `continue` se aplican al bucle más
interior en ese punto. Opcionalmente, puede especificar una *etiqueta de bucle*
en un bucle que luego puede usar con `break` o `continue` para especificar que
esas palabras clave se aplican al bucle etiquetado en lugar del bucle más
interior. Las etiquetas de bucle deben comenzar con una comilla simple. Aquí
hay un ejemplo con dos bucles anidados:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/src/main.rs}}
```

El bucle externo tiene la etiqueta `'counting_up`, y contará de 0 a 2. El bucle
interior sin etiqueta cuenta de 10 a 9. El primer `break` que no especifique
una etiqueta solo saldrá del bucle interno. La instrucción `break
'counting_up;` saldrá del bucle externo. Este código imprime:

```console
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/output.txt}}
```

#### Bucles condicionales con `while`

Un programa a menudo necesitará evaluar una condición dentro de un bucle.
Mientras la condición sea `true`, el bucle se ejecuta. Cuando la condición deja
de ser `true`, el programa llama a `break`, deteniendo el bucle. Es posible
implementar un comportamiento como este usando una combinación de `loop`, `if`,
`else` y `break`; puede intentarlo ahora en un programa, si lo desea. Sin
embargo, este patrón es tan común que Rust tiene una construcción de lenguaje
integrada para ello, llamada `while` loop. En el Listado 3-3, usamos `while`
para ejecutar el programa tres veces, contando hacia atrás cada vez, y luego,
después del bucle, imprimir un mensaje y salir.

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-03/src/main.rs}}
```

<span class="caption">Listado 3-3: Usando un bucle `while` para ejecutar código
mientras una condición es verdadera</span>

Esta expresion elimina mucho anidamiento que sería necesario si usara `loop`,
`if`, `else` y `break`, y es más claro. Mientras una condición se evalúa como
`true`, el código se ejecuta; de lo contrario, sale del bucle.

#### Bucle a través de una colección con `for`

Puede elegir usar la construcción `while` para iterar sobre los elementos de una
colección, como una matriz. Por ejemplo, el bucle en el Listado 3-4 imprime
cada elemento en la matriz `a`.

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-04/src/main.rs}}
```

<span class="caption">Listado 3-4: Bucle a través de cada elemento de una
colección usando un bucle `while`</span>

Aquí, el código cuenta hacia arriba a través de los elementos en la matriz. Se
inicia en el índice `0`, y luego se ejecuta hasta que alcanza el índice final
en la matriz (es decir, cuando `index < 5` ya no es `true`). Ejecutar este
código imprimirá cada elemento en la matriz:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-04/output.txt}}
```

Todos los cinco valores de la matriz aparecen en la terminal, como se esperaba.
Aunque `index` llegará a un valor de `5` en algún momento, el bucle deja de
ejecutarse antes de intentar obtener un sexto valor de la matriz.

Sin embargo, este enfoque es propenso a errores; podríamos causar que el
programa se descomponga si el valor del índice o la condición de prueba es
incorrecta. Por ejemplo, si cambia la definición de la matriz `a` para tener
cuatro elementos, pero olvida actualizar la condición a `while index < 4`, el
código se descompondría. También es lento, porque el compilador agrega código
de tiempo de ejecución para realizar la verificación condicional de si el
índice está dentro de los límites de la matriz en cada iteración del bucle.

Como una alternativa más concisa, puede usar un bucle `for` y ejecutar algún
código para cada elemento en una colección. Un bucle `for` se ve como el código
en el Listado 3-5.

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-05/src/main.rs}}
```

<span class="caption">Listado 3-5: Bucle a través de cada elemento de una
colección usando un bucle `for`</span>

Cuando ejecutamos este código, veremos la misma salida que en el Listado 3-4.
Lo más importante es que ahora hemos aumentado la seguridad del código y
eliminado la posibilidad de errores que podrían deberse a ir más allá del final
de la matriz o no ir lo suficientemente lejos y perder algunos elementos.

Usando el bucle `for`, no necesitaría recordar cambiar cualquier otro código si
cambiara el número de valores en la matriz, como lo haría con el método usado en
el Listado 3-4.

La seguridad y concisión de los bucles `for` los convierten en la
estructura de bucle más utilizada en Rust. Incluso en situaciones en las que
quiera ejecutar algún código un cierto número de veces, como en el ejemplo de
cuenta regresiva que usó un bucle `while` en el Listado 3-3, la mayoría de los
Rustaceans usarían un bucle `for`. La forma de hacerlo sería usar un `Range`,
proporcionado por la biblioteca estándar, que genera todos los números en
secuencia a partir de un número y termina antes de otro número.

Así es como se vería la cuenta regresiva usando un bucle `for` y otro método que
aún no hemos hablado, `rev`, para invertir el rango:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-34-for-range/src/main.rs}}
```

Este código es un poco más agradable, ¿verdad?

## Resumen

¡Lo lograste! Este fue un capítulo de gran tamaño: aprendiste sobre variables,
tipos de datos escalares y compuestos, funciones, comentarios, expresiones `if`
y bucles. Para practicar con los conceptos discutidos en este capítulo, intente
construir programas para hacer lo siguiente:

* Convertir temperaturas entre Fahrenheit y Celsius.
* Generar el número de Fibonacci *n*.
* Imprimir las letras de la canción navideña "Los doce días de Navidad",
  aprovechando la repetición en la canción.

Cuando esté listo para continuar, hablaremos sobre un concepto en Rust que
*no* existe comúnmente en otros lenguajes de programación: la propiedad.

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#comparando-la-adivinanza-con-el-número-secreto
[quitting-after-a-correct-guess]:
ch02-00-guessing-game-tutorial.html#salir-después-de-una-adivinanza-correcta
