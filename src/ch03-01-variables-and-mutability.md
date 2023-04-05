## Variables y Mutabilidad

Como se mencionó en la sección [“Almacenando valores con variables”]
[storing-values-with-variables]<!-- ignore -->, por defecto, las variables
son inmutables. Este es uno de los muchos empujes que Rust le da para que
escriba su código de una manera que aproveche la seguridad y la fácil
concurrencia que ofrece Rust. Sin embargo, todavía tiene la opción de hacer
sus variables mutables. Exploremos cómo y por qué Rust le anima a favorecer
inmutabilidad y por qué a veces podría querer optar por no hacerlo. 

Cuando una variable es inmutable, una vez que un valor está vinculado a un
nombre, no puede cambiar ese valor. Para ilustrar esto, genere un nuevo
proyecto llamado *variables* en su directorio *proyectos* usando `cargo new
variables`.

Luego, en su nuevo directorio *variables*, abra *src/main.rs* y reemplace su
código con el siguiente código, que aún no se compilará:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/src/main.rs}}
```

Guarde y ejecute el programa usando `cargo run`. Debería recibir un mensaje de
error relacionado con un error de inmutabilidad, como se muestra en esta
salida:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/output.txt}}
```

Este ejemplo muestra cómo el compilador le ayuda a encontrar errores en sus
programas. Los errores de compilación pueden ser frustrantes, pero realmente
solo significa que su programa aún no está realizando de manera segura lo que
desea que haga; *no* significa que no es un buen programador! Los Rustaceans
experimentados aún reciben errores de compilación.

Recibió el mensaje de error `` cannot assign twice to immutable variable `x`
`` porque intentó asignar un segundo valor a la variable inmutable `x`.

Es importante que obtengamos errores en tiempo de compilación cuando intentamos
cambiar un valor que está designado como inmutable, porque esta situación
muy puede conducir a errores. Si una parte de nuestro código opera bajo la
suposición de que un valor nunca cambiará y otra parte de nuestro código
cambia ese valor, es posible que la primera parte del código no haga lo que
estaba diseñado para hacer. La causa de este tipo de error puede ser difícil
de rastrear después del hecho, especialmente cuando la segunda pieza de código
cambia el valor solo *algunas veces*. El compilador de Rust garantiza que
cuando afirma que un valor no cambiará, realmente no cambiará, por lo que no
tiene que rastrearlo usted mismo. Su código es, por lo tanto, más fácil de
razonar.

Pero la mutabilidad puede ser muy útil y puede hacer que el código sea más
conveniente de escribir. Aunque las variables son inmutables por defecto, puede
hacerlas mutables agregando `mut` delante del nombre de la variable como lo
hizo en el [Capitulo 2][storing-values-with-variables]<!-- ignore -->.
Agregando `mut` también comunica la intención a los lectores futuros del código
indicando que otras partes del código cambiarán el valor de esta variable.

Por ejemplo, cambiemos *src/main.rs* a lo siguiente:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/src/main.rs}}
```

Cuando ejecutamos el programa ahora, obtenemos esto:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/output.txt}}
```

Se nos permite cambiar el valor vinculado a `x` de `5` a `6` cuando se usa
`mut`. En última instancia, decidir si usar o no la mutabilidad depende de
usted y depende de lo que crea que es más claro en esa situación particular.

### Constantes

Al igual que las variables inmutables, las *constantes* son valores que están
vinculados a un nombre y no se les permite cambiar, pero hay algunas
diferencias entre las constantes y las variables.

Primero, no se le permite usar `mut` con constantes. Las constantes no son solo
inmutables por defecto, siempre son inmutables. Declara constantes usando la
palabra clave `const` en lugar de la palabra clave `let`, y el tipo del valor
*debe* estar anotado. Cubriremos los tipos y las anotaciones de tipo en la
siguiente sección, [“Tipos de datos”][data-types]<!-- ignore -->, por lo que no se
preocupe por los detalles ahora. Solo sepa que siempre debe anotar el tipo.

Las constantes se pueden declarar en cualquier ámbito, incluido el ámbito
global, lo que las hace útiles para valores que muchas partes del código
necesitan conocer.

La última diferencia es que las constantes solo se pueden establecer en una
expresión constante, no en el resultado de un valor que solo se podría calcular
en tiempo de ejecución.

Aquí hay un ejemplo de una declaración constante:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

El nombre de la constante es `THREE_HOURS_IN_SECONDS` y su valor se establece
en el resultado de multiplicar 60 (el número de segundos en un minuto) por 60
(el número de minutos en una hora) por 3 (el número de horas que queremos
contar en este programa). La convención de nombramiento de Rust para constantes
es usar mayúsculas con guiones bajos entre palabras. El compilador es capaz de
evaluar un conjunto limitado de operaciones en tiempo de compilación, lo que
nos permite elegir escribir este valor de una manera que sea más fácil de
entender y verificar, en lugar de establecer esta constante en el valor 10,800.
Vea la [sección de la Referencia de Rust sobre la evaluación constante]
[const-eval] para más información sobre qué operaciones se pueden 
usar al declarar constantes.

Las constantes son válidas durante todo el tiempo que se ejecuta un programa,
dentro del ámbito en el que se declararon. Esta propiedad hace que las
constantes sean útiles para los valores en el dominio de su aplicación que
varias partes del programa podrían necesitar conocer, como el número máximo de
puntos que cualquier jugador de un juego puede obtener o la velocidad de la
luz.

Nombrar valores codificados en su programa como constantes es útil para
transmitir el significado de ese valor a los futuros mantenedores del código.
También ayuda a tener solo un lugar en su código en el que necesitaría cambiar
si el valor codificado tuviera que actualizarse en el futuro.

### Shadowing

Como vio en el tutorial del juego de adivinanzas en [Capítulo
2][comparing-the-guess-to-the-secret-number]<!-- ignore -->, puede declarar una
nueva variable con el mismo nombre que una variable anterior. Los Rustaceans
dicen que la primera variable es *ocultada* por la segunda, lo que significa
que la segunda variable es lo que el compilador verá cuando use el nombre de la
variable. En efecto, la segunda variable oculta la primera, tomando
cualquier uso del nombre de la variable para sí misma hasta que se haga
*shadowing* sobre la misma variable o el ámbito finalice. 
Podemos ocultar una variable usando el mismo nombre de variable y repitiendo 
el uso de la palabra clave `let` de la siguiente manera:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/src/main.rs}}
```

Este programa primero vincula `x` a un valor de `5`. Luego crea una nueva
variable `x` repitiendo `let x =`, tomando el valor original y agregando `1`
para que el valor de `x` sea entonces `6`. Luego, dentro de un ámbito interno
creado con las llaves, la tercera declaración `let` también proyecta `x` y
crea una nueva variable, multiplicando el valor anterior por `2` para darle a
`x` un valor de `12`. Cuando ese ámbito finaliza, la proyección interna finaliza
y `x` vuelve a ser `6`. Cuando ejecutamos este programa, se mostrará lo
siguiente:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/output.txt}}
```

El *Shadowing* es diferente de marcar una variable como `mut` porque obtendremos
un error de tiempo de compilación si accidentalmente intentamos volver a
asignar esta variable sin usar la palabra clave `let`. Al usar `let`, podemos
realizar algunas transformaciones en un valor, pero la variable debe ser
inmutable después de que se hayan completado esas transformaciones.

La otra diferencia entre `mut` y el *shadowing* es que, debido a que
efectivamente estamos creando una nueva variable cuando usamos la palabra clave
`let` nuevamente, podemos cambiar el tipo de valor pero reutilizar el mismo
nombre. Por ejemplo, digamos que nuestro programa le pide al usuario que muestre
cuántos espacios desea entre algún texto ingresando caracteres de espacio, y
luego queremos almacenar esa entrada como un número:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-04-shadowing-can-change-types/src/main.rs:here}}
```

La primera variable `spaces` es de tipo *string* y la segunda variable `spaces`
es de tipo *numerico*. El *shadowing* nos ahorra tener que pensar en nombres
diferentes, como `spaces_str` y `spaces_num`; en su lugar, podemos reutilizar
el nombre más simple `spaces`. Sin embargo, si intentamos usar `mut` para esto,
como se muestra aquí, obtendremos un error de tiempo de compilación:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/src/main.rs:here}}
```

El error dice que no se permite mutar el tipo de una variable:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/output.txt}}
```

Ahora que hemos explorado cómo funcionan las variables, veamos más tipos de
datos que pueden tener.

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#comparando-la-adivinanza-con-el-número-secreto
[data-types]: ch03-02-data-types.html#data-types
[storing-values-with-variables]: 
ch02-00-guessing-game-tutorial.html#almacenando-valores-con-variables
[const-eval]: ../reference/const_eval.html
