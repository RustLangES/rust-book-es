## Funciones

Las Funciones son son muy comunes en el código de Rust. Ya has visto una de las
funciones más importantes del lenguaje: la función `main`, que es el punto de
entrada de muchos programas. También has visto la palabra clave `fn`, que te
permite declarar nuevas funciones.

El código en Rust usa *snake case* como estilo convencional para las funciones
y nombres de variables, en el que todas las letras son minúsculas y los
guiones bajos separan las palabras. Aquí hay un programa que contiene un
ejemplo de definición de una función:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-16-functions/src/main.rs}}
```

Definimos una función en Rust escribiendo `fn` seguido del nombre de la función
y un conjunto de paréntesis. Las llaves indican al compilador donde comienza y
termina el cuerpo de la función.

Podemos llamar a cualquier función que hayamos definido escribiendo su nombre
seguido de un conjunto de paréntesis. Como `another_function` está definida en
el programa, se puede llamar desde dentro de la función `main`. Ten en cuenta
que definimos `another_function` *después* de la función `main` en el código
fuente; también podríamos haberla definido antes. Rust no se preocupa por dónde
definimos nuestras funciones, sino que estén definidas en algún lugar de un
alcance que pueda ser visto por el que llama.

Empecemos un nuevo proyecto binario llamado *functions* para explorar las
funciones más a fondo. Coloca el ejemplo de `another_function` en
*src/main.rs* y ejecútalo. Deberías ver la siguiente salida:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-16-functions/output.txt}}
```

Las líneas se ejecutan en el orden en que aparecen en la función `main`. Primero
se imprime el mensaje “Hello, world!”, y luego se llama a `another_function` y
se imprime su mensaje.

### Parámetros

Podemos definir funciones para que tengan *parámetros*, que son variables
especiales que forman parte de la firma de una función. Cuando una función
tiene parámetros, puedes proporcionarle valores concretos para esos
parámetros. Técnicamente, los valores concretos se llaman *argumentos*, pero en
la conversación informal, la gente tiende a usar las palabras *parámetro* y
*argumento* indistintamente para las variables en la definición de una función
o los valores concretos que se pasan cuando llamas a una función.

En esta versión de `another_function` agregamos un parámetro:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/src/main.rs}}
```

Intenta ejecutar este programa; deberías obtener la siguiente salida:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/output.txt}}
```

La declaración de `another_function` tiene un parámetro llamado `x`. El tipo de
`x` se especifica como `i32`. Cuando pasamos `5` a `another_function`, la
macro `println!` pone `5` donde estaba el par de llaves que contenía `x` en la
cadena de formato.

En las firmas de las funciones, *debes* declarar el tipo de cada parámetro. Esta
es una decisión deliberada en el diseño de Rust: requerir anotaciones de tipo en
las definiciones de las funciones significa que el compilador casi nunca necesita
que las uses en otro lugar del código para averiguar qué tipo quieres decir. El
compilador también puede dar mensajes de error más útiles si sabe qué tipos de
función espera.

Al definir múltiples parámetros, separa las declaraciones de parámetros con
comas, como esto:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/src/main.rs}}
```

Este ejemplo crea una función llamada `print_labeled_measurement` con dos
parámetros. El primer parámetro se llama `value` y es un `i32`. El segundo se
llama `unit_label` y es de tipo `char`. Luego, la función imprime texto que
contiene tanto el `value` como el `unit_label`.

Intentemos ejecutar este código. Reemplaza el programa actualmente en tu
proyecto *functions* en el archivo *src/main.rs* con el ejemplo anterior y
ejecútalo usando `cargo run`:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/output.txt}}
```

Porque llamamos a la función con `5` como el valor para `value` y `'h'` como el
valor para `unit_label`, la salida del programa contiene esos valores.

### Sentencias y Expresiones

Los cuerpos de las funciones están compuestos por una serie de sentencias
opcionalmente terminadas en una expresión. Hasta ahora, las funciones que hemos
visto no incluyen una expresión final, pero has visto una expresión como parte
de una sentencia. Debido a que Rust es un lenguaje basado en expresiones, esta
es una distinción importante de entender. Otros lenguajes no tienen las mismas
distinciones, así que veamos qué son las sentencias y las expresiones y cómo
sus diferencias afectan a los cuerpos de las funciones.

* **Sentencias** son instrucciones que realizan alguna acción y no devuelven un
  valor.
* **Expresiones** evalúan a un valor resultante. Veamos algunos ejemplos.

Hemos usado realmente sentencias y expresiones. Crear una variable y asignarle
un valor con la palabra clave `let` es una sentencia. En el Listado 3-1,
`let y = 6;` es una sentencia.

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-01/src/main.rs}}
```

<span class="caption">Listado 3-1: Una declaración de la función `main` que contiene una sentencia</span>

Las definiciones de las funciones también son sentencias; todo el ejemplo
anterior es una sentencia en sí misma.

Las sentencias no devuelven valores. Por lo tanto, no puedes asignar una
sentencia `let` a otra variable, como intenta hacer el siguiente código; 
obtendrás un error:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/src/main.rs}}
```

Cuando ejecutes este programa, el error que obtendrás se verá así:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/output.txt}}
```

La sentencia `let y = 6` no devuelve un valor, por lo que no hay nada para lo
que `x` pueda enlazar. Esto es diferente a lo que ocurre en otros lenguajes,
como C y Ruby, donde la asignación devuelve el valor de la asignación. En esos
lenguajes, puedes escribir `x = y = 6` y tener tanto `x` como `y` el valor `6`;
eso no es el caso en Rust.

Las expresiones evalúan a un valor y componen la mayor parte del resto del
código que escribirás en Rust. Considera una operación matemática, como `5 + 6`,
que es una expresión que evalúa al valor `11`. Las expresiones pueden ser parte
de las sentencias: en el Listado 3-1, el `6` en la sentencia `let y = 6;` es
una expresión que evalúa al valor `6`. Llamar a una función es una expresión.
Llamar a una macro es una expresión. Un nuevo bloque de ámbito creado con
llaves es una expresión, por ejemplo:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-20-blocks-are-expressions/src/main.rs}}
```

Esta expresión:

```rust,ignore
{
    let x = 3;
    x + 1
}
```

es un bloque que, en este caso, evalúa a `4`. Ese valor se enlaza a `y` como
parte de la sentencia `let`. Ten en cuenta que la línea `x + 1` no tiene un
punto y coma al final, lo que es diferente a la mayoría de las líneas que has
visto hasta ahora. Las expresiones no incluyen punto y coma al final. Si
agregas un punto y coma al final de una expresión, la conviertes en una
sentencia, y entonces no devolverá un valor. Ten esto en cuenta a medida que
exploras los valores de retorno de las funciones y las expresiones a continuación.

### Funciones con valores de retorno

Las funciones pueden devolver valores al código que las llama. No nombramos los
valores de retorno, pero debemos declarar su tipo después de una flecha (`->`).
En Rust, el valor de retorno de la función es sinónimo del valor de la última
expresión en el bloque del cuerpo de una función. Puedes devolver temprano de
una función usando la palabra clave `return` y especificando un valor, pero la
mayoría de las funciones devuelven la última expresión implícitamente. Aquí
hay un ejemplo de una función que devuelve un valor:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/src/main.rs}}
```

No hay llamadas a funciones, macros, ni siquiera sentencias `let` en la función
`five` - solo el número `5` por sí solo. Esa es una función perfectamente
válida en Rust. Ten en cuenta que también se especifica el tipo de retorno de
la función, como `-> i32`. Intenta ejecutar este código; la salida debería
verse así:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/output.txt}}
```

El `5` en `five` es el valor de retorno de la función, por eso el tipo de
retorno es `i32`. Veamos esto con más detalle. Hay dos partes importantes:
primero, la línea `let x = five();` muestra que estamos usando el valor de
retorno de una función para inicializar una variable. Debido a que la función
`five` devuelve un `5`, esa línea es la misma que la siguiente:

```rust
let x = 5;
```

Segundo, la función `five` no tiene parámetros y define el tipo del valor de
retorno, pero el cuerpo de la función es un solitario `5` sin punto y coma
porque es una expresión cuyo valor queremos devolver.

Veamos otro ejemplo:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-22-function-parameter-and-return/src/main.rs}}
```

La ejecución de este código imprimirá `The value of x is: 6`. Pero si colocamos
un punto y coma al final de la línea que contiene `x + 1`, cambiándolo de una
expresión a una sentencia, obtendremos un error:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/src/main.rs}}
```

La compilación de este código produce un error, como sigue:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/output.txt}}
```

El mensaje de error principal, `mismatched types`, revela el problema principal
con este código. La definición de la función `plus_one` dice que devolverá un
`i32`, pero las sentencias no evalúan un valor, lo que se expresa por `()`, el
tipo unitario. Por lo tanto, no se devuelve nada, lo que contradice la
definición de la función y da como resultado un error. En esta salida, Rust
proporciona un mensaje para posiblemente ayudar a corregir este problema:
sugiere eliminar el punto y coma, lo que arreglaría el error.