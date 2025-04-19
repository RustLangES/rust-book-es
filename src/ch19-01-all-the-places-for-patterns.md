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

El Listado 19-1 muestra que también es posible mezclar y combinar expresiones
`if let`, `else if` y `else if let`. Hacerlo nos da más flexibilidad que una
expresión `match` en la que solo podemos expresar un valor para comparar con
los patrones. Además, Rust no requiere que las condiciones en una serie de
brazos `if let`, `else if`, `else if let` se relacionen entre sí.

El código en el Listado 19-1 determina de qué color hacer su fondo en función
de una serie de comprobaciones para varias condiciones. Para este ejemplo,
hemos creado variables con valores codificados que un programa real podría
recibir de la entrada del usuario.

<Listing number="19-1" file-name="src/main.rs" caption="Combinando `if let`, `else if`, `else if let`, y `else`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-01/src/main.rs}}
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
30 no es válido hasta que comience el nuevo alcance con la llave de apertura.

La desventaja de usar expresiones `if let` es que el compilador no verifica la
exhaustividad, mientras que con las expresiones `match` sí lo hace. Si
omitiéramos el último bloque `else` y, por lo tanto, no manejáramos algunos
casos, el compilador no nos alertaría sobre el posible bug de lógica.

### Bucles condicionales `while let`

Similar en su construcción a `if let`, el bucle condicional `while let` permite 
que un bucle `while` se ejecute mientras un patrón continúe coincidiendo. Vimos 
por primera vez un bucle `while let` en el Capítulo 17, donde lo usamos para 
seguir iterando mientras un flujo producía nuevos valores. De manera similar, en 
el Listado 19-2 mostramos un bucle `while let` que espera mensajes enviados 
entre hilos, pero en este caso verificando un `Result` en lugar de una `Option`.

<Listing number="19-2" caption="Usando un bucle `while let` para imprimir valores mientras `rx.recv()` devuelva `Ok`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-02/src/main.rs:here}}
```

</Listing>

Este ejemplo imprime 1, 2 y 3. Cuando vimos `recv` en el Capítulo 16, manejamos 
el error directamente con `unwrap` o interactuamos con él como un iterador 
usando un bucle `for`. Sin embargo, como muestra el Listado 19-2, también 
podemos usar `while let`, ya que el método `recv` devuelve `Ok` mientras el 
remitente siga produciendo mensajes, y luego genera un `Err` cuando el lado del 
remitente se desconecta.

### Bucles `for`

En un bucle `for`, el valor que sigue directamente a la palabra clave `for` es
un pattern. Por ejemplo, en `for x in y` el `x` es el pattern. El Listado 19-3
demuestra cómo usar un pattern en un bucle `for` para destruir, o romper, una
tupla como parte del bucle `for`.

<Listing number="19-3" caption="Usando un pattern en un bucle `for` para desestructurar una tupla">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-03/src/main.rs:here}}
```

</Listing>

El código en el Listado 19-3 imprimirá lo siguiente:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-03/output.txt}}
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
considera el Listado 19-4, que usa un pattern con `let` para destruir una
tupla.

<Listing number="19-4" caption="Usando un pattern para desestructurar una tupla y crear tres variables a la vez">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-04/src/main.rs:here}}
```

</Listing>

Aquí, emparejamos una tupla con un pattern. Rust compara el valor `(1, 2, 3)`
con el pattern `(x, y, z)` y ve que el valor y el pattern coinciden, por lo que
Rust asigna `1` a `x`, `2` a `y` y `3` a `z`. Puedes pensar que este pattern de
tupla anida tres patterns de variable individuales dentro de él.

Si el número de elementos en el pattern no coincide con el número de elementos
en la tupla, el tipo general no coincidirá y obtendremos un error del
compilador. Por ejemplo, el Listado 19-5 muestra un intento de destruir una
tupla con tres elementos en dos variables, lo cual no funcionará.

<Listing number="19-5" caption="Al construir incorrectamente un pattern cuyas variables no coinciden con el número de elementos en la tupla">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-05/src/main.rs:here}}
```

</Listing>

Intentar compilar este código resulta en este error de tipo:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-05/output.txt}}
```

Para solucionar el error, podríamos ignorar uno o más valores en la tupla
utilizando `_` o `..`, como verás en la sección [“Ignorando valores en un
pattern”][ignoring-values-in-a-pattern]<!-- ignore -->. Si el problema es que
tenemos demasiadas variables en el pattern, la solución es hacer que los tipos
coincidan eliminando variables para que el número de variables sea igual al
número de elementos en la tupla.

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
