## Sintaxis de los Patterns

En esta sección, reunimos toda la sintaxis válida en los patterns y discutimos
por qué y cuándo podría querer usar cada uno.

### Coincidiendo con literales

Como viste en el Capítulo 6, puedes hacer coincidir patterns contra literales
directamente. El siguiente código da algunos ejemplos:

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/no-listing-01-literals/src/main.rs:here}}
```

Este código imprime `one` porque el valor en `x` es 1. Esta sintaxis es útil
cuando quieres que tu código tome una acción si obtiene un valor concreto
particular.

### Coincidiendo con variables nombradas

Las variables nombradas son patterns irrefutables que coinciden con cualquier
valor, y las hemos usado muchas veces en el libro. Sin embargo, hay una
complicación cuando usas variables nombradas en expresiones `match`. Debido a
que `match` inicia un nuevo scope, las variables declaradas como parte de un
pattern dentro de la expresión `match` ocultarán aquellas con el mismo nombre
fuera del constructo `match`, como es el caso de todas las variables. En el
Listado 18-11, declaramos una variable llamada `x` con el valor `Some(5)` y una
variable `y` con el valor `10`. Luego creamos una expresión `match` en el valor
`x`. Mira los patterns en las opciones `match` y `println!` al final, e intenta
averiguar qué imprimirá el código antes de ejecutar este código o leer más.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-11/src/main.rs:here}}
```

<span class="caption">Listing 18-11: Una expresión `match` con una opción que
introduce una variable sombreada (shadowing) `y`</span>

Vamos a repasar lo que sucede cuando se ejecuta la expresión `match`. El pattern
en la primera opción de `match` no coincide con el valor definido de `x`, por
lo que el código continúa.

El pattern en la segunda opción de `match` introduce una nueva variable
llamada `y` que coincidirá con cualquier valor dentro de un valor `Some`.
Debido a que estamos en un nuevo scope dentro de la expresión `match`, esta
es una nueva variable `y`, no la que declaramos al principio con el valor 10.
Este nuevo enlace `y` coincidirá con cualquier valor dentro de un `Some`, que
es lo que tenemos en `x`. Por lo tanto, este nuevo `y` se vincula al valor
interno de `Some` en `x`. Ese valor es `5`, por lo que la expresión para esa
opción se ejecuta e imprime `Matched, y = 5`.

Si `x` hubiera sido un `None` en lugar de `Some(5)`, los patterns en las dos
primeras opciones no habrían coincidido, por lo que el valor habría coincidido
con el guion bajo. No introdujimos la variable `x` en el pattern de la opción
del guion bajo, por lo que el `x` en la expresión sigue siendo el `x` externo
que no ha sido sombreado. En este caso hipotético, el `match` imprimiría
`Default case, x = None`.

Cuando la expresión `match` termina, su scope termina, y también lo hace el
scope del `y` interno. El último `println!` produce `at the end: x = Some(5), y
= 10`.

Para crear una expresión `match` que compare los valores del `x` e `y` externos
en lugar de introducir una variable sombreada, necesitaríamos usar una
condición de guardia de `match`. Hablaremos de las guardias de `match` más
adelante en la sección [“Condicionales adicionales con `match` guards”](#condicionales-adicionales-con-match-guards)<!-- ignore -->

### Múltiples Patterns

En las expresiones `match`, puedes coincidir con múltiples patrones usando la
sintaxis `|`, que es el operador _or_ del pattern. Por ejemplo, en el siguiente
código hacemos coincidir el valor de `x` con las opciones de `match`, el primero
de los cuales tiene una opción _or_, lo que significa que si el valor de `x`
coincide con cualquiera de los valores en esa opción, se ejecutará el código de
esa opción:

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/no-listing-02-multiple-patterns/src/main.rs:here}}
```

Este código imprime `one or two`.

### Coincidiendo con rangos de valores con `..=`

La sintaxis `..=` nos permite emparejar un rango inclusivo de valores. En el
siguiente código, cuando un patrón coincide con cualquiera de los valores
dentro del rango dado, esa opción se ejecutará:

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/no-listing-03-ranges/src/main.rs:here}}
```

Si `x` es 1, 2, 3, 4 o 5, la primera opción coincidirá. Esta sintaxis es más
conveniente para múltiples valores de coincidencia que usar el operador `|`
para expresar la misma idea; si usáramos `|` tendríamos que especificar `1 | 2
| 3 | 4 | 5`. Especificar un rango es mucho más corto, especialmente si
queremos coincidir, digamos, cualquier número entre 1 y 1.000.

El compilador verifica que el rango no esté vacío en tiempo de compilación, y
debido a que los únicos tipos para los que Rust puede decir si un rango está
vacío o no son los valores numéricos y `char`, los rangos solo están permitidos
con valores numéricos o `char`.

Aquí tienes un ejemplo que utiliza rangos de valores `char`:

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/no-listing-04-ranges-of-char/src/main.rs:here}}
```

Rust puede determinar que `'c'` se encuentra dentro del rango especificado en
el primer pattern y se muestra por pantalla `early ASCII letter`.

### Desestructurando para separar valores

Podemos usar patterns para desestructurar structs, enums y tuplas para utilizar
diferentes partes de estos valores. Veamos cada uno de ellos.

#### Desestructurando Structs

El Listado 18-12 muestra un struct `Point` con dos campos, `x` e `y`, que
podemos desestructurar usando un pattern con una declaración `let`.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-12/src/main.rs}}
```

<span class="caption">Listing 18-12: Desestructurando los campos de un struct
en variables separadas</span>

Este código crea las variables `a` y `b` que coinciden con los valores de los
campos `x` e `y` del struct `p`. Este ejemplo muestra que los nombres de las
variables en el pattern no tienen que coincidir con los nombres de los campos
del struct. Sin embargo, es común que los nombres de las variables coincidan
con los nombres de los campos para facilitar recordar qué variables provienen
de qué campos. Debido a este uso común, y porque escribir `let Point { x: x, y:
y } = p;` contiene mucha duplicación, Rust tiene una abreviatura para los
patterns que coinciden con los campos de los structs: solo necesitas listar el
nombre del campo del struct, y las variables creadas a partir del pattern
tendrán los mismos nombres. El Listado 18-13 se comporta de la misma manera que
el código del Listado 18-12, pero las variables creadas en el pattern `let` son
`x` e `y` en lugar de `a` y `b`.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-13/src/main.rs}}
```

<span class="caption">Listing 18-13: Desestructurando los campos de un struct
utilizando la forma abreviada de los campos struct</span>

Este código crea las variables `x` e `y` que coinciden con los campos `x` e `y`
del struct `p`. El resultado es que las variables `x` e `y` contienen los
valores de los campos `x` e `y` del struct.

También podemos desestructurar y con valores literales como parte del pattern
del struct en lugar de crear variables para todos los campos. Hacerlo nos
permite probar algunos de los campos para valores particulares mientras
creamos variables para desestructurar los otros campos.

En el Listado 18-14, tenemos una expresión `match` que separa los valores de
`Point` en tres casos: puntos que se encuentran directamente en el eje `x` (lo
cual es cierto cuando `y = 0`), en el eje `y` (`x = 0`), o ninguno.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-14/src/main.rs:here}}
```

<span class="caption">Listing 18-14: Desestructurar y coincidir valores
literales en un solo pattern</span>

El primer bloque coincidirá con cualquier punto que se encuentre en el eje `x`
especificando que el campo `y` debe coincidir con el valor `0`. El pattern aún
crea una variable `x` que podemos usar en el código de este bloque.

De manera similar, el segundo bloque coincide con cualquier punto en el eje `y`,
especificando que el campo `x` coincida si su valor es `0` y crea una variable
`y` para el valor del campo `y`. El tercer bloque no especifica literales, por
lo que coincide con cualquier otro `Point` y crea variables para ambos campos
`x` e `y`.

En este ejemplo, el valor `p` coincide con el segundo bloque debido a que `x`
contiene un `0`, por lo que este código imprimirá `On the y axis at 7`.

Recuerda que una expresión `match` detiene la verificación de los bloques una
vez que ha encontrado el primer patrón que coincide, por lo que, aunque `Point
{ x: 0, y: 0 }` está en el eje `x` y en el eje `y`, este código solo imprimirá
`On the x axis at 0`.

#### Desestructurando Enums

Hemos desestructurado enums en este libro (por ejemplo, el Listado 6-5 en el
Capítulo 6), pero aún no hemos discutido explícitamente que el pattern para
desestructurar un enum corresponde a la forma en que se define los datos
almacenados dentro del enum. Como ejemplo, en el Listado 18-15 usamos el enum
`Message` del Listado 6-2 y escribimos un `match` con patterns que desestructuran
cada valor interno.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-15/src/main.rs}}
```

<span class="caption">Listing 18-15: Desestructurando variantes enum que
contienen diferentes tipos de valores</span>

Este código imprimirá `Change the color to red 0, green 160, and blue 255`.
Prueba cambiar el valor de `msg` para ver el código de las otras opciones.

Para variantes de enum sin ningún dato, como `Message::Quit`, no podemos
desestructurar el valor más allá. Solo podemos coincidir con el valor literal
`Message::Quit`, y no hay variables en ese pattern.

Para variantes de enum similares a structs, como `Message::Move`, podemos
usar un pattern similar al que especificamos para coincidir con structs. Después
del nombre de la variante, colocamos llaves y luego enumeramos los campos con
variables para que desarmemos las piezas para usar en el código de esta opción.
Aquí usamos la forma abreviada como lo hicimos en el Listado 18-13.

Para variantes de enum similares a tuplas, como `Message::Write` que contiene
una tupla con un elemento y `Message::ChangeColor` que contiene una tupla con
tres elementos, el pattern es similar al pattern que especificamos para
coincidir con tuplas. El número de variables en el pattern debe coincidir con
el número de elementos en la variante que estamos coincidiendo.

#### Desestructurando Structs y Enums Anidados

Hasta ahora, todos nuestros ejemplos han sido de coincidencia (Match) con structs o enums de un nivel de profundidad,
pero el emparejamiento también puede funcionar en elementos anidados. Por ejemplo, podemos refactorizar el
código del Listado 18-15 para admitir colores RGB y HSV en el mensaje `ChangeColor`, como se muestra en el Listado 18-16.

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-16/src/main.rs}}
```

<span class="caption">Listing 18-16: Matching on nested enums</span>

El pattern de la primera opción en la expresión `match` coincide con la
variante de enum `Message::ChangeColor` que contiene una variante
`Color::Rgb`; luego el pattern se une a los tres valores internos `i32`. El
pattern de la segunda opción también coincide con una variante de enum
`Message::ChangeColor`, pero el enum interno coincide con `Color::Hsv` en su
lugar. Podemos especificar estas condiciones complejas en una expresión
`match`, incluso cuando están involucrados dos enums.

#### Desestructurando Structs y Tuplas

Podemos mezclar, combinar y anidar los patrones de desestructuración de formas
aún más complejas. El siguiente ejemplo muestra una desestructuración
complicada donde anidamos structs y tuplas dentro de una tupla y
desestructuramos todos los valores primitivos:

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/no-listing-05-destructuring-structs-and-tuples/src/main.rs:here}}
```

Este código nos permite descomponer tipos complejos en sus partes componentes
para que podamos usar los valores que nos interesan por separado.

El uso de patrones para desestructurar es una forma conveniente de utilizar
partes de valores, como el valor de cada campo en un struct, por separado.

### Ignorando valores en un patron

Has visto que a veces es útil ignorar valores en un pattern, como en la última
opción de un `match`, para obtener una opción que no hace nada, pero que abarca
todos los posibles valores restantes. Hay varias formas de ignorar valores
completos o partes en un pattern: usando el pattern `_` (que has visto), usando
el pattern `_` dentro de otro pattern, usando un nombre que comienza con un
guion bajo y usando `..` para ignorar las partes restantes de un valor.
Exploraremos cómo y por qué usar cada uno de estos patterns.

#### Ignorando un Valor Completo con `_`

Hemos utilizado el guion bajo como un pattern comodín que coincide con
cualquier valor pero no se enlaza con él. Esto es especialmente útil como la
última opción en una expresión `match`, pero también podemos usarlo en
cualquier pattern, incluyendo los parámetros de una función, como se muestra en
el Listado 18-17.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-17/src/main.rs}}
```

<span class="caption">Listing 18-17: Utilizando `_` en la firma de una
función</span>

Este código ignorará completamente el valor `3` pasado como primer argumento,
e imprimirá `This code only uses the y parameter: 4`.

En la mayoría de los casos, cuando ya no necesitas un parámetro de una función,
deberías cambiar la firma de la función para que no incluya el parámetro no
utilizado. Ignorar un parámetro de una función puede ser especialmente útil en
casos en los que, por ejemplo, estás implementando un trait cuando necesitas
una firma de tipo específico, pero el cuerpo de la función en tu implementación
no necesita uno de los parámetros. Luego evitas obtener una advertencia del
compilador sobre parámetros de función no utilizados, como lo harías si
utilizaras un nombre en su lugar.

#### Ignorando partes de un valor con un `_` anidado

En este caso, el pattern `_` se utiliza dentro de otro pattern para ignorar
solo una parte del valor. Esto puede ser útil cuando queremos probar solo una
parte del valor, pero no tenemos uso para las otras partes en el código
correspondiente que queremos ejecutar. El Listado 18-18 muestra un código
encargado de gestionar el valor de una configuración. Los requisitos
son que el usuario no debe poder sobrescribir una personalización existente de
una configuración, pero puede eliminar la configuración y asignarle un valor si
actualmente no está establecida.

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-18/src/main.rs:here}}
```

<span class="caption">Listing 18-18: Utilizando un guion bajo dentro de patterns
que coinciden con variantes `Some` cuando no necesitamos usar el valor dentro
del `Some`</span>

Este código imprimirá `setting is None` y luego `setting is Some(5)`. En la
primera opción de `match`, no necesitamos hacer coincidir ni usar los valores
dentro de ninguna de las variantes `Some`, pero si necesitamos comprobar en
el caso en el que tanto `setting_value` como `new_setting_value` son la
variante `Some`. En ese caso, imprimimos la razón por la que no cambiamos
`setting_value`, y no lo cambiamos.

En todos los demás casos (si `setting_value` o `new_setting_value` son `None`)
expresados por el pattern `_` en la segunda opción, queremos permitir que
`new_setting_value` se convierta en `setting_value`.

También podemos usar guiones bajos en múltiples lugares dentro de un solo
pattern para ignorar valores particulares. El Listado 18-19 muestra un ejemplo
de ignorar el segundo y cuarto valores en una tupla de cinco elementos.

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-19/src/main.rs:here}}
```

<span class="caption">Listing 18-19: Ignorando múltiples partes de una tupla</span>

Este código imprimirá `Some numbers: 2, 8, 32`, y los valores 4 y 16 serán
ignorados.

#### Ignorando una variable no utilizada comenzando su nombre con `_`

Si creas una variable, pero no la utilizas en ningún lugar, Rust generalmente
emitirá una advertencia porque una variable no utilizada podría causar un bug.
Sin embargo, a veces es útil poder crear una variable que aún no se utilizará,
como cuando estás prototipando o simplemente comenzando un proyecto. En esta
situación, puedes decirle a Rust que no te advierta sobre la variable no
utilizada comenzando el nombre de la variable con un guion bajo. En el Listado
18-20, creamos dos variables no utilizadas, pero cuando compilamos este código,
solo deberíamos obtener una advertencia sobre una de ellas.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-20/src/main.rs}}
```

<span class="caption">Listing 18-20: Comenzar el nombre de una variable con un
guion bajo para evitar recibir advertencias de variables no utilizadas</span>

Aquí recibimos una advertencia sobre no utilizar la variable `y`, pero no
recibimos una advertencia sobre no utilizar `_x`.

Es importante destacar que hay una diferencia sutil entre usar solo `_` y usar
un nombre que comienza con un guion bajo. La sintaxis `_x` todavía enlaza el
valor a la variable, mientras que `_` no enlaza en absoluto. Para mostrar un
caso en el que esta distinción importa, el Listado 18-21 nos proporcionará un
error.

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-21/src/main.rs:here}}
```

<span class="caption">Listing 18-21: Una variable no utilizada que comienza con
un guion bajo aún vincula el valor, lo que puede tomar ownership del
valor</span>

Recibiremos un error porque el valor de `s` se mueve a `_s`, lo que invalida
usar `s` nuevamente. Sin embargo, usar solo el guion bajo no vincula el valor
en ningún momento. El Listado 18-22 se compilará sin errores porque `s` no se
mueve a `_`.

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-22/src/main.rs:here}}
```

<span class="caption">Listing 18-22: Usar un guion bajo no vincula el
valor</span>

Este código funciona bien porque nunca vinculamos `s` a nada; no se mueve.

#### Ignorando las partes restantes de un valor con `..`

Con los valores que tiene muchas partes, podemos usar la sintaxis `..` para
usar partes específicas e ignorar el resto, evitando la necesidad de enumerar
guiones bajos para cada valor ignorado. El pattern `..` ignora cualquier parte
de un valor que no hayamos coincidido explícitamente en el resto del pattern.
En el Listado 18-23, tenemos un struct `Point` que contiene una coordenada en
el espacio tridimensional. En la expresión `match`, queremos operar solo en la
coordenada `x` e ignorar los valores en los campos `y` y `z`.

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-23/src/main.rs:here}}
```

<span class="caption">Listing 18-23: Ignorando todos los campos de un `Point`
excepto `x` mediante el uso de `..`</span>

Listamos el valor `x` y luego simplemente incluimos el pattern `..`. Esto es
más rápido que tener que listar `y: _` y `z: _`, particularmente cuando
estamos trabajando con structs que tienen muchos campos en situaciones en las
que solo uno o dos campos son relevantes.

La sintaxis `..` se expandirá a tantos valores como sea necesario. El Listado
18-24 muestra cómo usar `..` con una tupla.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-24/src/main.rs}}
```

<span class="caption">Listing 18-24: Coincidir solo con el primer y último
valor en una tupla e ignorar todos los demás valores</span>

En este código, el primer y último valor se coinciden con `first` y `last`. El
`..` coincidirá con cualquier número de valores entre el primero y el último.

Sin embargo, el uso de `..` debe ser inequívoco. Si no está claro qué valores
deben coincidir y cuáles deben ignorarse, Rust nos dará un error. El Listado
18-25 muestra un ejemplo de usar `..` de manera ambigua, por lo que no se
compilará.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-25/src/main.rs}}
```

<span class="caption">Listing 18-25: Un intento de usar `..` de manera
ambigua</span>

Cuando compilamos este ejemplo, obtenemos este error:

```console
{{#include ../listings/ch18-patterns-and-matching/listing-18-25/output.txt}}
```

Es imposible para Rust determinar cuántos valores en la tupla ignorar antes de
hacer coincidir un valor con `second` y luego cuántos valores más ignorar
después. Este código podría significar que queremos ignorar `2`, vincular
`second` a `4` y luego ignorar `8`, `16` y `32`; o que queremos ignorar `2` y
`4`, vincular `second` a `8` y luego ignorar `16` y `32`; y así sucesivamente.
El nombre de la variable `second` no significa nada especial para Rust, por lo
que obtenemos un error del compilador porque usar `..` en dos lugares como este
es ambiguo.

### Condicionales adicionales con Match Guards

Un _match guard_ es una condición adicional `if`, especificada después del
pattern en una opción `match`, que también debe coincidir para que se elija
esa opción. Los match guards son útiles para expresar ideas más complejas que
las que permite un pattern solo.

La condición puede utilizar variables creadas en el pattern. El Listado 18-26
muestra un `match` donde la primera opción tiene el pattern `Some(x)` y también
tiene un match guard de `if x % 2 == 0` (que será verdadero si el número es
par).

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-26/src/main.rs:here}}
```

<span class="caption">Listing 18-26: Agregando un match guard a un
pattern</span>

Este ejemplo imprimirá `The number 4 is even`. Cuando `num` se compara con el
pattern en la primera opción, coincide, porque `Some(4)` coincide con `Some(x)`.
Luego, el match guard verifica si el resto de dividir `x` por 2 es igual a 0,
y porque lo es, se selecciona la primera opción.

Si `num` hubiera sido `Some(5)`, el match guard en la primera opción habría
sido falso porque el resto de 5 dividido por 2 es 1, que no es igual a 0. Rust
entonces pasaría a la segunda opción, que coincidiría porque la segunda opción
no tiene un match guard y, por lo tanto, coincide con cualquier variante `Some`.

No hay forma de expresar la condición `if x % 2 == 0` dentro de un pattern, por
lo que el match guard nos da la capacidad de expresar esta lógica. La
desventaja de esta expresividad adicional es que el compilador no intenta
verificar la exhaustividad cuando están involucradas las expresiones de match
guard.

En el Listado 18-11, mencionamos que podríamos usar match guards para resolver
nuestro problema de shadowing de pattern. Recordemos que creamos una nueva
variable dentro del pattern en la expresión `match` en lugar de usar la
variable fuera del `match`. Esa nueva variable significaba que no podíamos
probar contra el valor de la variable externa. El Listado 18-27 muestra cómo
podemos usar un match guard para solucionar este problema.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-27/src/main.rs}}
```

<span class="caption">Listing 18-27: Utilizando un match guard para probar
la igualdad con una variable externa</span>

Este código imprimirá `Default case, x = Some(5)`. El pattern en la segunda
opción no introduce una nueva variable `y` que sombree la variable externa `y`,
por lo que podemos usar la variable externa `y` en el match guard. En lugar de
especificar el pattern como `Some(y)`, que habría sombreado la variable externa
`y`, especificamos `Some(n)`. Esto crea una nueva variable `n` que no sombrea
nada porque no hay una variable `n` fuera del `match`.

El match guard `if n == y` no es un pattern y, por lo tanto, no introduce nuevas
variables. Este `y` _es_ el `y` externo en lugar de un nuevo `y` sombreado, y
podemos buscar un valor que tenga el mismo valor que el `y` externo comparando
`n` con `y`.

También puedes usar el operador _or_ `|` en un match guard para especificar
múltiples patterns; la condición del match guard se aplicará a todos los
patterns. El Listado 18-28 muestra la precedencia al combinar un pattern que
usa `|` con un match guard. La parte importante de este ejemplo es que el
match guard `if y` se aplica a `4`, `5` y `6`, aunque podría parecer que `if y`
solo se aplica a `6`.

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-28/src/main.rs:here}}
```

<span class="caption">Listing 18-28: Combinando múltiples patterns con un match
guard</span>

La condición de match establece que la opción solo coincide si el valor de `x`
es igual a `4`, `5` o `6` _y_ si `y` es `true`. Cuando se ejecuta este código,
el pattern de la primera opción coincide porque `x` es `4`, pero el match guard
`if y` es falso, por lo que no se elige la primera opción. El código pasa a la
segunda opción, que coincide, y este programa imprime `no`. La razón es que la
condición `if` se aplica a todo el pattern `4 | 5 | 6`, no solo al último valor
`6`. En otras palabras, la precedencia de un match guard en relación con un
pattern se comporta así:

```text
(4 | 5 | 6) if y => ...
```

en lugar de esto:

```text
4 | 5 | (6 if y) => ...
```

Después de ejecutar el código, el comportamiento de precedencia es evidente: si
el match guard se aplicara solo al último valor en la lista de valores
especificados usando el operador `|`, la opción habría coincidido y el programa
habría impreso `yes`.

### `@` Bindings

El operador `@`, conocido como _at_, nos permite crear una variable que almacena
un valor al mismo tiempo que lo comprobamos para una coincidencia de pattern.
En el Listado 18-29, queremos probar que el campo `id` de un `Message::Hello`
está dentro del rango `3..=7`. También queremos vincular el valor a la variable
`id_variable` para poder usarlo en el código asociado con la opción. Podríamos
nombrar esta variable `id`, igual que el campo, pero para este ejemplo usaremos
un nombre diferente.

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-29/src/main.rs:here}}
```

<span class="caption">Listing 18-29: Usando `@` para enlazar un valor en un
pattern mientras también lo testeamos</span>

Este ejemplo imprimirá `Found an id in range: 5`. Al especificar `id_variable 
@` antes del rango `3..=7`, estamos capturando cualquier valor que coincida con
el rango mientras también probamos que el valor coincidió con el pattern de
rango.

En la segunda opción, donde solo tenemos especificado un rango en el patrón,
el código asociado a la opción no tiene una variable que contenga el valor real
del campo `id`. El campo `id` podría haber sido `10`, `11` o `12`, pero el
código asociado al pattern no sabe cuál es. El código del pattern no puede usar
el valor del campo `id` porque no hemos guardado el valor `id` en una variable.

En la última opción, donde hemos especificado una variable sin un rango, sí
tenemos el valor disponible para usar en el código de la opción en una variable
llamada `id`. La razón es que hemos usado la sintaxis abreviada del campo del struct. Pero no hemos aplicado ninguna prueba al valor en el campo `id` en
esta opción, como hicimos con las dos primeras opciones: cualquier valor
coincidiría con este pattern.

Usar `@` nos permite probar un valor y guardarlo en una variable dentro de un
mismo pattern.

## Resumen

Los patterns en Rust son muy útiles para distinguir entre diferentes tipos de
datos. Cuando se usan en expresiones `match`, Rust garantiza que tus patterns
cubran todos los valores posibles, o tu programa no se compilará. Los patterns
en las declaraciones `let` y en los parámetros de las funciones hacen que esos
constructos sean más útiles, permitiendo la deconstrucción de valores en partes
más pequeñas al mismo tiempo que se asignan a variables. Podemos crear patterns
simples o complejos para adaptarse a nuestras necesidades.

A continuación, para el penúltimo capítulo del libro, exploraremos algunos
aspectos avanzados de varias características de Rust.
