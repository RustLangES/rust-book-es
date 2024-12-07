## Todos los lugares donde se pueden usar Patterns

Los Patterns aparecen en varios lugares en Rust, ¡y los has estado usando mucho
sin darte cuenta! Esta sección discute todos los lugares donde los Patterns son
válidos.

### Opciones de `match`

Como se discutió en el Capítulo 6, usamos Patterns en las opciones de las
expresiones `match`. Formalmente, las expresiones `match` se definen como la
palabra clave `match`, un valor para hacer coincidir y una o más opciones de
coincidencia que consisten en un patrón y una expresión para ejecutar si el
valor coincide con el patrón de esa opción, así:

```text
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

Por ejemplo, aquí está la expresión `match` del Listado 6-5 que coincide con un
valor `Option<i32>` en la variable `x`:

```rust,ignore
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

Los patterns en esta expresión `match` son el `None` y el `Some(i)` a la
izquierda de cada flecha.

Un requisito para las expresiones `match` es que deben ser _exhaustivas_ en el
sentido de que todas las posibilidades para el valor en la expresión `match`
deben tenerse en cuenta. Una forma de asegurarse de haber cubierto todas las
posibilidades es tener un patrón de captura para el último brazo: por ejemplo,
un nombre de variable que coincida con cualquier valor nunca puede fallar y,
por lo tanto, cubre todos los casos restantes.

El patrón específico `_` coincidirá con cualquier cosa, pero nunca se une a una
variable, por lo que a menudo se usa en la última opción de coincidencia. El
patrón `_` puede ser útil cuando desea ignorar cualquier valor no especificado,
por ejemplo. Cubriremos el patrón `_` con más detalle en la sección [“Ignorar
valores en un patrón”][ignoring-values-in-a-pattern]<!-- ignore --> más adelante
en este capítulo.

### Expresiones condicionales `if let`

En el capítulo 6 discutimos cómo usar expresiones `if let` principalmente como
una forma más corta de escribir el equivalente de un `match` que solo coincide
con un caso. Opcionalmente, `if let` puede tener un `else` correspondiente que
contenga código para ejecutar si el patrón en el `if let` no coincide.

El Listado 18-1 muestra que también es posible mezclar y combinar expresiones
`if let`, `else if` y `else if let`. Hacerlo nos da más flexibilidad que una
expresión `match` en la que solo podemos expresar un valor para comparar con
los patrones. Además, Rust no requiere que las condiciones en una serie de
brazos `if let`, `else if`, `else if let` se relacionen entre sí.

El código en el Listado 18-1 determina de qué color hacer su fondo en función
de una serie de comprobaciones para varias condiciones. Para este ejemplo,
hemos creado variables con valores codificados que un programa real podría
recibir de la entrada del usuario.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-01/src/main.rs}}
```

<span class="caption">Listing 18-1: Combinando
`if let`, `else if`, `else if let`, y `else`</span>

Si el usuario especifica un color favorito, ese color se usa como fondo. Si no
se especifica un color favorito y hoy es martes, el color de fondo es verde.
De lo contrario, si el usuario especifica su edad como una cadena y podemos
analizarla como un número con éxito, el color es púrpura o naranja dependiendo
del valor del número. Si ninguna de estas condiciones se aplica, el color de
fondo es azul.

Una estructura condicional nos permite cumplir con requisitos complejos.
Con los valores codificados que tenemos aquí, este ejemplo imprimirá `Using
purple as the background color`.

Puedes ver que `if let` también puede introducir variables con shadowing de la
misma manera que lo hacen las opciones `match`: la línea `if let Ok(age) =
age` introduce una nueva variable `age` que contiene el valor dentro de la
variante `Ok`. Esto significa que necesitamos colocar la condición `if age > 30`
dentro de ese bloque: no podemos combinar estas dos condiciones en `if let Ok
(age) = age && age > 30`. El `age` sombreado que queremos comparar con 30 no es
válido hasta que comience el nuevo scope con la llave de apertura.

La desventaja de usar expresiones `if let` es que el compilador no verifica la
exhaustividad, mientras que con las expresiones `match` sí lo hace. Si
omitiéramos el último bloque `else` y, por lo tanto, no manejáramos algunos
casos, el compilador no nos alertaría sobre el posible bug de lógica.

### Bucles condicionales `while let`

Similar en su construcción a `if let`, el bucle condicional `while let` permite
que un bucle `while` se ejecute mientras un patrón continúe coincidiendo. En
el Listado 18-2 codificamos un bucle `while let` que usa un vector como una
pila e imprime los valores en el vector en el orden opuesto en el que se
pusieron.

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-02/src/main.rs:here}}
```

<span class="caption">Listing 18-2: Utilizando un bucle `while let` para
imprimir valores mientras `stack.pop()` devuelva `Some`</span>

Este ejemplo imprime 3, 2 y luego 1. El método `pop` toma el último elemento
del vector y devuelve `Some(value)`. Si el vector está vacío, `pop` devuelve
`None`. El bucle `while` continúa ejecutando el código en su bloque siempre que
`pop` devuelva `Some`. Cuando `pop` devuelve `None`, el bucle se detiene.
Podemos usar `while let` para sacar todos los elementos de nuestra pila.

### Bucles `for`

En un bucle `for`, el valor que sigue directamente a la palabra clave `for` es
un pattern. Por ejemplo, en `for x in y` el `x` es el pattern. El Listado 18-3
demuestra cómo usar un pattern en un bucle `for` para destruir, o romper, una
tupla como parte del bucle `for`.

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-03/src/main.rs:here}}
```

<span class="caption">Listing 18-3: Usando un pattern en un bucle `for` para
desestructurar una tupla</span>

El código en el Listado 18-3 imprimirá lo siguiente:

```console
{{#include ../listings/ch18-patterns-and-matching/listing-18-03/output.txt}}
```

Adaptamos un iterator usando el método `enumerate` para que produzca un valor y
el índice de ese valor, colocado en una tupla. El primer valor producido es la
tupla `(0, 'a')`. Cuando este valor se corresponde con el pattern `(index,
value)`, `index` será `0` y `value` será `'a'`, imprimiendo la primera línea
del output.

### Sentencias `let`

Antes de este capítulo, solo habíamos discutido explícitamente el uso de
patterns con `match` e `if let`, pero de hecho, también hemos usado patterns
en otros lugares, incluyendo en las sentencias `let`. Por ejemplo, considera
esta asignación de variable directa con `let`:

```rust
let x = 5;
```

Cada vez que has utilizado una declaración `let` como esta, has estado usando
patterns, aunque es posible que no te hayas dado cuenta. Más formalmente, una
sentencia `let` se ve así:

```text
let PATTERN = EXPRESSION;
```

En declaraciones como `let x = 5;`, con un nombre de variable en el slot
`PATTERN`, el nombre de la variable es solo una forma particularmente simple de
un pattern. Rust compara la expresión con el pattern y asigna cualquier nombre
que encuentre. Entonces, en el ejemplo `let x = 5;`, `x` es un pattern que
significa “vincula lo que coincide aquí a la variable `x`”. Debido a que el
nombre `x` es todo el pattern, este pattern significa efectivamente “vincula
todo a la variable `x`, sea cual sea el valor”.

Para ver más claramente el aspecto de coincidencia de patrones de `let`,
considera el Listado 18-4, que usa un pattern con `let` para destruir una
tupla.

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-04/src/main.rs:here}}
```

<span class="caption">Listing 18-4: Usando un pattern para desestructurar una
tupla y crear tres variables a la vez</span>

Aquí, emparejamos una tupla con un pattern. Rust compara el valor `(1, 2, 3)`
con el pattern `(x, y, z)` y ve que el valor y el pattern coinciden, por lo que
Rust asigna `1` a `x`, `2` a `y` y `3` a `z`. Puedes pensar que este pattern de
tupla anida tres patterns de variable individuales dentro de él.

Si el número de elementos en el pattern no coincide con el número de elementos
en la tupla, el tipo general no coincidirá y obtendremos un error del
compilador. Por ejemplo, el Listado 18-5 muestra un intento de destruir una
tupla con tres elementos en dos variables, lo cual no funcionará.

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-05/src/main.rs:here}}
```

<span class="caption">Listing 18-5: Al construir incorrectamente un pattern
cuyas variables no coinciden con el número de elementos en la tupla</span>

Intentar compilar este código resulta en este error de tipo:

```console
{{#include ../listings/ch18-patterns-and-matching/listing-18-05/output.txt}}
```

Para solucionar el error, podríamos ignorar uno o más valores en la tupla
utilizando `_` o `..`, como verás en la sección [“Ignorando valores en un
pattern”][ignoring-values-in-a-pattern]<!-- ignore -->. Si el problema es que
tenemos demasiadas variables en el pattern, la solución es hacer que los tipos
coincidan eliminando variables para que el número de variables sea igual al
número de elementos en la tupla.

### Parámetros de función

Los parámetros de función también pueden ser patterns. El código del Listado
18-6, que declara una función llamada `foo` que toma un parámetro llamado `x`
de tipo `i32`, debería ser familiar a estas alturas.

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-06/src/main.rs:here}}
```

<span class="caption">Listing 18-6: La firma de una función que utiliza
patterns en los parámetros</span>

¡La parte `x` es un pattern! Como hicimos con `let`, podríamos hacer coincidir
una tupla en los argumentos de una función con el pattern. El Listado 18-7
divide los valores en una tupla a medida que la pasamos a una función.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-07/src/main.rs}}
```

<span class="caption">Listing 18-7: Una función con parámetros que desetructura
una tupla</span>

Este código imprime `Current location: (3, 5)`. El valor `(3, 5)` coincide con
el pattern `(x, y)`, por lo que `x` es `3` y `y` es `5`.

También podemos usar patterns en las listas de parámetros de closures, de la
misma manera que en las listas de parámetros de funciones. Porque los closures
son similares a las funciones, como se discutió en el Capítulo 13.

Hasta ahora, has visto varias formas de usar patrones, pero los patrones no
funcionarán de la misma manera en todos los lugares donde podemos usarlos. En
algunos casos, los patrones deben ser irrefutables; en otras circunstancias,
pueden ser refutables. Discutiremos estos dos conceptos a continuación.

[ignoring-values-in-a-pattern]: ch18-03-pattern-syntax.html#ignorando-valores-en-un-patron
