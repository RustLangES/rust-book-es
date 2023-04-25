<!-- Old heading. Do not remove or links may break. -->
<a id="the-match-control-flow-operator"></a>
## `match` El operador de flujo de control (Control Flow Construct)

Rust tiene una construcción de flujo de control extremadamente poderosa llamada
`match` que te permite comparar un valor contra una serie de patrones y luego
ejecutar código basado en qué patrón coincide. Los patrones pueden estar
compuestos de valores literales, nombres de variables, comodines y muchas otras
cosas; El [Capítulo 18][ch18-00-patterns]<!-- ignore --> cubre todos los
diferentes tipos de patrones y lo que hacen. El poder de `match` viene de la
expresividad de los patrones y el hecho de que el compilador confirma que se
tratan todos los casos posibles.

Piensa en una expresión `match` como una máquina de clasificación de monedas:
las monedas deslizan a lo largo de una pista con orificios de diversos tamaños
a lo largo de ella, y cada moneda cae a través del primer orificio que encuentra
que se ajusta a ella. De la misma manera, los valores pasan a través de cada
patrón en un `match`, y en el primer patrón en el que el valor “se ajusta”, el
valor cae en el bloque de código asociado para ser utilizado durante la
ejecución.

Hablando de monedas, ¡usémoslas como un ejemplo usando `match`! Podemos escribir
una función que tome una moneda desconocida de los Estados Unidos y, de una
manera similar a la máquina de conteo, determine qué moneda es y devuelva su
valor en centavos, como se muestra en el Listing 6-3.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-03/src/main.rs:here}}
```

<span class="caption">Listing 6-3: Una expresión enum y `match` que tiene
las variantes del enum como sus patrones</span>

Desglosemos el uso de `match` en la función `value_in_cents`. 
Primero listamos la palabra clave `match` seguida de una expresión, 
que en este caso es el valor `coin`. Esto parece muy similar a una expresión 
condicional utilizada con `if`, pero hay una gran diferencia: con `if`, 
la condición debe evaluar a un valor Booleano, pero aquí puede ser cualquier 
tipo. El tipo de `coin` en este ejemplo es el enum `Coin` que definimos en la 
primera línea.

A continuación, dentro de las llaves de `match`, hay un número de Opciones.
Una Opción tiene dos partes: un patrón y algún código. La primera Opción aquí
tiene un patrón que es el valor `Coin::Penny` y luego el operador `=>` que
separa el patrón y el código a ejecutar. El código en este caso es solo el valor
`1`. Cada Opción está separado del siguiente con una coma.

Cuando la expresión `match` se ejecuta, compara el valor resultante contra el
patrón de cada Opción, en orden. Si un patrón coincide con el valor, se ejecuta
el código asociado con ese patrón. Si ese patrón no coincide con el valor,
la ejecución continúa en la siguiente Opción, como en una máquina de 
clasificación de monedas. Podemos tener tantas Opciones como necesitemos: 
en el Listado 6-3, nuestro `match` tiene cuatro Opciones.

El código asociado con cada Opción es una expresión, y el valor resultante de
la expresión en la Opción coincidente es el valor que se devuelve para la
expresión `match` completa.

Por lo general, no usamos llaves si el código de la Opción de match es
corto, como lo es en el Listado 6-3, donde cada Opción solo devuelve un valor.
Si desea ejecutar varias líneas de código en una Opción de match, debe
usar llaves, y la coma que sigue a la Opción es opcional. Por ejemplo, 
el siguiente código imprime “¡Moneda de la suerte!” cada vez que el método 
se llama con un `Coin::Penny`, pero aún devuelve el último valor del bloque, 
`1`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-08-match-arm-multiple-lines/src/main.rs:here}}
```

### Patrones que vinculan valores

Otra característica útil de las Opciones de match es que pueden vincularse
a las partes del valor que coinciden con el patrón. Esto es cómo podemos extraer
valores de las variantes de enum.

Como ejemplo, podemos cambiar el código de la función `value_in_cents` para
que, en lugar de devolver un valor, imprima el valor que tiene. Esto nos
permite ver qué moneda tenemos y cuánto vale. Para hacer esto, necesitamos
convertir el código de cada Opción en una expresión, y luego usar una
expresión `println!` en lugar de un valor de retorno. También necesitamos
cambiar el tipo de `value_in_cents` a `()`, ya que no estamos devolviendo un
valor entero, sino que estamos ejecutando código. El código completo se muestra
en el Listing 6-4.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-04/src/main.rs:here}}
```

<span class="caption">Listing 6-4: Un enum `Coin` en el cual la variante 
`Quarter` también contiene un valor `UsState`</span>

Imaginemos que tenemos un amigo que está tratando de coleccionar todas las
monedas de 50 estados. Mientras clasificamos nuestra moneda suelta por tipo de
moneda, también llamaremos al nombre del estado asociado con cada moneda de
50 centavos para que si es uno que no tiene, pueda agregarlo a su colección.

En la expresión `match` en el Listado 6-4, podemos agregar `UsState::Alaska` a
la variante `Coin::Quarter` para crear una nueva variante de `Coin`. Cuando
hacemos esto, el estado de Alaska se adjunta a la moneda. Luego, cuando
ejecutamos el código, podemos ver el valor del estado almacenado en la
moneda de 50 centavos al imprimirlo. El código completo se muestra en el
Listing 6-5.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-09-variable-in-pattern/src/main.rs:here}}
```

Si llamáramos a `value_in_cents(Coin::Quarter(UsState::Alaska))`, `coin` sería
`Coin::Quarter(UsState::Alaska)`. Cuando comparamos ese valor con cada una de
las Opciones de match, ninguno coincide hasta que llegamos a
`Coin::Quarter(state)`. En ese punto, el enlace para `state` será el valor
`UsState::Alaska`. Luego podemos usar ese enlace en la expresión `println!`,
obteniendo así el valor del estado interno de la variante de `Coin` para
`Quarter`.

### Match con `Option<T>`

En la sección anterior, queríamos obtener el valor interno `T` de la variante
`Some` cuando se usaba `Option<T>`; también podemos manejar `Option<T>` usando
`match`, como lo hicimos con el enum `Coin`! En lugar de comparar monedas,
compararemos las variantes de `Option<T>`, pero la forma en que funciona la
expresión `match` sigue siendo la misma.

Digamos que queremos escribir una función que tome un `Option<i32>` y, si
hay un valor dentro, agregue 1 a ese valor. Si no hay un valor dentro, la
función debe devolver el valor `None` y no intentar realizar ninguna
operación.

Esta función es muy fácil de escribir, gracias a `match`, y se verá como el
Listing 6-5.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:here}}
```

<span class="caption">Listing 6-5: Una función que usa una expresión `match` en
un `Option<i32>`</span>

Examinemos la primera ejecución de `plus_one` en más detalle. Cuando llamamos
a `plus_one(five)`, la variable `x` en el cuerpo de `plus_one` tendrá el
valor `Some(5)`. Luego comparamos eso contra cada Opción de match:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

El valor `Some(5)` no coincide con el patrón `None`, por lo que seguimos a la
siguiente Opción:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:second_arm}}
```

¿Coincide `Some(5)` con `Some(i)`? ¡Lo hace! Tenemos la misma variante. Él
`i` se vincula al valor contenido en `Some`, por lo que `i` toma el valor `5`.
Luego se ejecuta el código en la Opción de match, por lo que agregamos 1
al valor de `i` y creamos un nuevo valor `Some` con nuestro total `6` dentro.

Ahora consideremos la segunda llamada a `plus_one` en el Listing 6-5, donde
`x` es `None`. Entramos en el `match` y comparamos con la primera Opción:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

¡Coincide! No hay valor para agregar, por lo que el programa se detiene y
devuelve el valor `None` en el lado derecho de `=>`. Debido a que la primera
Opción coincidió, no se comparan otras Opciones.

Combinando `match` y enums es útil en muchas situaciones. Verás este patrón
mucho en el código Rust: `match` contra un enum, vincula una variable a los
datos internos y luego ejecuta el código en función de él. Es un poco
complicado al principio, pero una vez que te acostumbras, desearás tenerlo en
todos los lenguajes. Es consistentemente un favorito de los usuarios.

### Los matches son exhaustivos

Hay otro aspecto de `match` que debemos discutir: los patrones de las Opciones
deben cubrir todas las posibilidades. Considera esta versión de nuestra
función `plus_one`, que tiene un error y no se compila:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/src/main.rs:here}}
```

No manejamos el caso `None`, por lo que este código causará un error.
Afortunadamente, es un error que Rust sabe cómo detectar. Si intentamos
compilar este código, obtendremos este error:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/output.txt}}
```

Rust sabe que no cubrimos todos los casos posibles, e incluso sabe qué
patrón olvidamos! Los matches en Rust son *exhaustivos*: debemos agotar
todas las posibilidades para que el código sea válido. Especialmente en el
caso de `Option<T>`, cuando Rust nos impide olvidar manejar el caso `None`,
nos protege de asumir que tenemos un valor cuando podríamos tener nulo,
haciendo así imposible el error de mil millones de dólares discutido
anteriormente.

### Patrones de captura y el Placeholder `_`

Usando enums, también podemos tomar acciones especiales para algunos valores
particulares, pero para todos los demás valores, tomar una acción 
predeterminada. Imagina que estamos implementando un juego donde, si sacas un 
3 en un lanzamiento de dados, tu jugador no se mueve, sino que obtiene un nuevo 
sombrero elegante. Si sacas un 7, tu jugador pierde un sombrero elegante. 
Para todos los demás valores, tu jugador se mueve esa cantidad de espacios 
en el tablero de juego. Aquí hay un `match` que implementa esa lógica, con el 
resultado del lanzamiento de dados codificado en lugar de un valor aleatorio, 
y toda la lógica representada por funciones sin cuerpos porque implementarlas 
realmente está fuera del alcance de este ejemplo:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-15-binding-catchall/src/main.rs:here}}
```

Para las primeras dos Opciones, los patrones son los valores literales `3` y
`7`. Para la última Opción que cubre cualquier otro valor posible, el patrón
es la variable que hemos elegido para nombrar `other`. El código que se
ejecuta para la Opción `other` usa la variable pasándola a la función
`move_player`.

Este código compila, aunque no hemos enumerado todos los posibles valores que
puede tener un `u8`, porque el último patrón coincidirá con todos los valores
no especificados específicamente. Este patrón de captura cumple con el
requisito de que `match` debe ser exhaustivo. Ten en cuenta que tenemos que
poner la Opción de captura al final porque los patrones se evalúan en orden. Si
ponemos la Opción de captura antes, las otras Opciones nunca se ejecutarían, por
lo que Rust nos advertirá si agregamos Opciones después de un catch-all!

Rust también tiene un patrón que podemos usar cuando queremos un catch-all,
pero no queremos *usar* el valor en el patrón catch-all: `_` es un patrón
especial que coincide con cualquier valor y no se vincula a ese valor. Esto le
dice a Rust que no vamos a usar el valor, por lo que Rust no nos advertirá
sobre una variable no utilizada.

Vamos a cambiar las reglas del juego. Ahora, si sacas un 3 o un 7, debes tirar
de nuevo. Ya no necesitamos usar el valor general, por lo que puede cambiar
nuestro código para usar `_` en lugar de la variable llamada `other`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-16-underscore-catchall/src/main.rs:here}}
```

Este ejemplo también cumple con el requisito de exhaustividad porque estamos 
explícitamente ignorando todos los demás valores en la última Opción; no hemos
olvidado nada.

Finalmente, cambiaremos las reglas del juego una vez más para que nada más
ocurra en tu turno si sacas algo que no sea un 3 o un 7. Podemos expresar eso
usando el valor de unidad (el tipo de tupla vacía que mencionamos en [“El tipo
de tupla”][tuples]<!-- ignore --> sección) como el código que va con la Opción
`_`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-17-underscore-unit/src/main.rs:here}}
```

Aquí, le decimos a Rust explícitamente que no vamos a usar ningún otro valor
que no coincida con un patrón en una Opción anterior, y no queremos ejecutar
ningún código en este caso.

Hay más sobre patrones y coincidencias que cubriremos en el [Capítulo
18][ch18-00-patterns]<!-- ignore -->. Por ahora, vamos a pasar a la sintaxis
`if let` que puede ser útil en situaciones en las que la expresión `match` es
un poco larga.

[tuples]: ch03-02-data-types.html#the-tuple-type
[ch18-00-patterns]: ch18-00-patterns.html
