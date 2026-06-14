## Todos los lugares donde se pueden usar Patterns

Los Patterns aparecen en varios lugares en Rust, ¡y los has estado usando mucho
sin darte cuenta! Esta sección discute todos los lugares donde los Patterns son
válidos.

### `match` Arms

Como se discutió en el Capítulo 6, usamos patrones en la expresiones `match`. 
Formalmente, las expresiones `match` se definen como la palabra clave `match`, 
un valor para hacer coincidir y una o más opciones de coincidencia que consisten 
en un patrón y una expresión para ejecutar si el valor coincide con el patrón de 
esa opción, así:

<!--
  Manually formatted rather than using Markdown intentionally: Markdown does not
  support italicizing code in the body of a block like this!
-->

<pre><code>match <em>VALUE</em> {
    <em>PATTERN</em> => <em>EXPRESSION</em>,
    <em>PATTERN</em> => <em>EXPRESSION</em>,
    <em>PATTERN</em> => <em>EXPRESSION</em>,
}</code></pre>

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

### Sentencias `let`

Antes de este capítulo, solo habíamos hablado explícitamente sobre el uso de 
patrones con `match` e `if let`, pero en realidad también hemos estado usando 
patrones en otros lugares, incluyendo las sentencias `let`. Por ejemplo, 
considera esta sencilla asignación de variable con `let`:

```rust
let x = 5;
```

¡Cada vez que has usado una sentencia `let` como esta, has estado utilizando 
patrones, aunque quizás no te hayas dado cuenta! De forma más formal, una 
sentencia `let` tiene la siguiente forma:

<!--
  Manually formatted rather than using Markdown intentionally: Markdown does not
  support italicizing code in the body of a block like this!
-->

<pre>
<code>let <em>PATTERN</em> = <em>EXPRESSION</em>;</code>
</pre>

En sentencias como `let x = 5;`, donde hay un nombre de variable en la posición 
del PATRÓN, dicho nombre es simplemente una forma particularmente simple de 
patrón. Rust compara la expresión contra el patrón y asigna cualquier nombre que 
encuentre. Así, en el ejemplo `let x = 5;`, `x` es un patrón que significa: 
«vincula lo que coincida aquí a la variable `x`». Como el nombre `x` constituye 
todo el patrón, este patrón significa efectivamente: «vincula cualquier valor a 
la variable `x`, sin importar cuál sea».

Para ver más claramente el aspecto de coincidencia de patrones en `let`, 
considera el Listado 19-1, que utiliza un patrón con `let` para desestructurar 
una tupla.

<Listing number="19-1" caption="Uso de un patrón para desestructurar una tupla y crear tres variables de una sola vez">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-01/src/main.rs:here}}
```

</Listing>

Aquí hacemos coincidir una tupla con un patrón. Rust compara el valor 
`(1, 2, 3)` con el patrón `(x, y, z)` y observa que coinciden, ya que ambos 
tienen la misma cantidad de elementos. Entonces Rust vincula `1` a `x`, `2` a 
`y` y `3` a `z`. Puedes pensar en este patrón de tupla como si contuviera tres 
patrones de variables individuales anidados en su interior.

Si la cantidad de elementos en el patrón no coincide con la cantidad de 
elementos de la tupla, el tipo general no coincidirá y obtendremos un error de 
compilación. Por ejemplo, el Listado 19-2 muestra un intento de desestructurar 
una tupla de tres elementos en dos variables, lo cual no funcionará.


<Listing number="19-2" caption="Construcción incorrecta de un patrón cuyas variables no coinciden con la cantidad de elementos de la tupla">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-02/src/main.rs:here}}
```

</Listing>

Intentar compilar este código produce un error de tipos.

Para corregir el error, podríamos ignorar uno o más de los valores de la tupla 
usando `_` o `..`, como veremos en la sección «Ignorar valores en un patrón». Si 
el problema es que tenemos demasiadas variables en el patrón, la solución es 
hacer que los tipos coincidan eliminando variables hasta que la cantidad de 
variables sea igual a la cantidad de elementos de la tupla.

### Expresiones condicionales `if let`

En el Capítulo 6 vimos cómo utilizar expresiones `if let`, principalmente como 
una forma más corta de escribir el equivalente a un `match` que solo maneja un 
caso. Opcionalmente, un `if let` puede tener un bloque `else` correspondiente 
que contenga el código a ejecutar cuando el patrón del `if let` no coincida.

El Listado 19-3 muestra que también es posible combinar expresiones `if let`, 
`else if` y `else if let`. Esto nos proporciona más flexibilidad que una 
expresión `match`, en la que solo podemos expresar un único valor para 
compararlo contra los patrones. Además, Rust no exige que las condiciones de 
una serie de ramas `if let`, `else if` y `else if let` estén relacionadas entre 
sí.

El código en el Listado 19-3 determina de qué color hacer su fondo en función
de una serie de comprobaciones para varias condiciones. Para este ejemplo,
hemos creado variables con valores codificados que un programa real podría
recibir de la entrada del usuario.

<Listing number="19-3" file-name="src/main.rs" caption="Combinando `if let`, `else if`, `else if let`, y `else`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-03/src/main.rs}}
```

</Listing>

Si el usuario especifica un color favorito, ese color se usa como fondo. Si no
se especifica un color favorito y hoy es martes, el color de fondo es verde.
De lo contrario, si el usuario especifica su edad como una cadena y podemos
analizarla como un número con éxito, el color es púrpura o naranja dependiendo
del valor del número. Si ninguna de estas condiciones se aplica, el color de
fondo es azul.

Una estructura condicional nos permite cumplir con requisitos complejos.
Con los valores codificados que tenemos aquí, este ejemplo imprimirá `Using
purple as the background color`.

Puedes ver que `if let` también puede introducir nuevas variables con shadowing 
de la misma manera que lo hacen las opciones `match`: la línea 
`if let Ok(age) = age` introduce una nueva variable `age` que contiene el valor 
dentro de la variante `Ok`. Esto significa que necesitamos colocar la condición 
`if age > 30` dentro de ese bloque: no podemos combinar estas dos condiciones en 
`if let Ok(age) = age && age > 30`. El `age` sombreado que queremos comparar con 
30 no es válido hasta que comience el nuevo scope (ámbito) con la llave de 
apertura.

La desventaja de usar expresiones `if let` es que el compilador no verifica la
exhaustividad, mientras que con las expresiones `match` sí lo hace. Si
omitiéramos el último bloque `else` y, por lo tanto, no manejáramos algunos
casos, el compilador no nos alertaría sobre el posible bug de lógica.

### Bucles condicionales `while let`

Similar en su construcción a `if let`, el bucle condicional `while let` permite 
que un bucle `while` se ejecute mientras un patrón continúe coincidiendo. Vimos 
por primera vez un bucle `while let` en el Capítulo 17, donde lo usamos para 
seguir iterando mientras un flujo producía nuevos valores. De manera similar, en 
el Listado 19-4 mostramos un bucle `while let` que espera mensajes enviados 
entre hilos, pero en este caso verificando un `Result` en lugar de una `Option`.

<Listing number="19-4" caption="Usando un bucle `while let` para imprimir valores mientras `rx.recv()` devuelva `Ok`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-04/src/main.rs:here}}
```

</Listing>

Este ejemplo imprime 1, 2 y 3. Cuando vimos `recv` en el Capítulo 16, manejamos 
el error directamente con `unwrap` o interactuamos con él como un iterador 
usando un bucle `for`. Sin embargo, como muestra el Listado 19-4, también 
podemos usar `while let`, ya que el método `recv` devuelve `Ok` mientras el 
remitente siga produciendo mensajes, y luego genera un `Err` cuando el lado del 
remitente se desconecta.

### Bucles `for`

En un bucle `for`, el valor que sigue directamente a la palabra clave `for` es
un pattern. Por ejemplo, en `for x in y` el `x` es el pattern. El Listado 19-5
demuestra cómo usar un pattern en un bucle `for` para destructurar, o romper, 
una tupla como parte del bucle `for`.

<Listing number="19-5" caption="Usando un pattern en un bucle `for` para desestructurar una tupla">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-05/src/main.rs:here}}
```

</Listing>

El código en el Listado 19-5 imprimirá lo siguiente:


```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-05/output.txt}}
```

Adaptamos un iterator usando el método `enumerate` para que produzca un valor y
el índice de ese valor, colocado en una tupla. El primer valor producido es la
tupla `(0, 'a')`. Cuando este valor se corresponde con el pattern `(index,
value)`, `index` será `0` y `value` será `'a'`, imprimiendo la primera línea
del output.

### Parámetros de función

Los parámetros de función también pueden ser patterns. El código del Listado
19-6, que declara una función llamada `foo` que toma un parámetro llamado `x`
de tipo `i32`, debería ser familiar a estas alturas.

<Listing number="19-6" caption="La firma de una función que utiliza patterns en los parámetros">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-06/src/main.rs:here}}
```

</Listing>

¡La parte `x` es un pattern! Como hicimos con `let`, podríamos hacer coincidir
una tupla en los argumentos de una función con el pattern. El Listado 19-7
divide los valores en una tupla a medida que la pasamos a una función.

<Listing number="19-7" file-name="src/main.rs" caption="Una función con parámetros que desetructura una tupla">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-07/src/main.rs}}
```

</Listing>

Este código imprime `Current location: (3, 5)`. El valor `(3, 5)` coincide con
el pattern `(x, y)`, por lo que `x` es `3` y `y` es `5`.

También podemos usar patterns en las listas de parámetros de closures, de la
misma manera que en las listas de parámetros de funciones. Porque los closures
son similares a las funciones, como se discutió en el Capítulo 13.

Hasta ahora, has visto varias formas de usar patrones, pero los patrones no
funcionarán de la misma manera en todos los lugares donde podemos usarlos. En
algunos casos, los patrones deben ser irrefutables; en otras circunstancias,
pueden ser refutables. Discutiremos estos dos conceptos a continuación.

[ignoring-values-in-a-pattern]: ch19-03-pattern-syntax.html#ignorando-valores-en-un-patron
