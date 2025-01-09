## Refutabilidad: Si un Pattern Puede Fallar al Hacer Match

Los patterns se dividen en dos formas: refutables e irrefutables. Los patterns
que coinciden con cualquier valor posible son *irrefutables*. Un ejemplo sería
`x` en la declaración `let x = 5;` porque `x` coincide con cualquier cosa y,
por lo tanto, no puede fallar al hacer match. Los patterns que pueden fallar al
hacer match para algunos valores posibles son *refutables*. Un ejemplo sería
`Some(x)` en la expresión `if let Some(x) = a_value` porque si el valor en la
variable `a_value` es `None` en lugar de `Some`, el pattern `Some(x)` no
coincidirá.

Los parámetros de funciones, las declaraciones `let` y los bucles `for` solo
pueden aceptar patterns irrefutables, porque el programa no puede hacer nada
significativo cuando los valores no coinciden. Las expresiones `if let` y
`while let` aceptan patterns refutables e irrefutables, pero el compilador
advierte contra los patterns irrefutables porque, por definición, están
destinados a manejar posibles fallas: la funcionalidad de una condicional está
en su capacidad de realizar de manera diferente dependiendo del éxito o el
fracaso.

En general, no debería preocuparse por la distinción entre patterns refutables
e irrefutables; sin embargo, debe estar familiarizado con el concepto de
refutabilidad para poder responder cuando lo vea en un mensaje de error. En
esos casos, deberá cambiar el pattern o la construcción que está utilizando el
pattern, según el comportamiento previsto del código.

Veamos un ejemplo de lo que sucede cuando intentamos usar un pattern refutable
donde Rust requiere un pattern irrefutable y viceversa. El Listado 19-8 muestra
una declaración `let`, pero para el pattern hemos especificado `Some(x)`, un
pattern refutable. Como puede imaginar, este código no se compilará.

<Listing number="19-8" caption="Intentando utilizar un pattern refutable con `let`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-08/src/main.rs:here}}
```

</Listing>

Si `some_option_value` fuera un valor `None`, no coincidiría con el pattern
`Some(x)`, lo que significa que el pattern es refutable. Sin embargo, la 
declaración `let` solo puede aceptar un pattern irrefutable porque no hay nada
válido que el código pueda hacer con un valor `None`. En tiempo de compilación,
Rust se quejará de que hemos intentado usar un pattern refutable donde se
requiere un pattern irrefutable:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-08/output.txt}}
```

Debido a que no hemos cubierto (¡y no pudimos cubrir!) Cada valor válido con el
pattern `Some(x)`, Rust produce un error del compilador.

Si tenemos un pattern refutable donde se necesita un patrón irrefutable, 
podemos solucionarlo cambiando el código que utiliza el patrón: en lugar de 
usar `let`, podemos usar `if let`. Entonces, si el pattern no coincide, el 
código simplemente omitirá el código entre llaves, dándole una forma de 
continuar válidamente. El Listado 19-9 muestra cómo solucionar el código del 
Listado 19-8.

<Listing number="19-9" caption="Usando `if let` y un bloque con patterns refutables en lugar de `let`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-09/src/main.rs:here}}
```

</Listing>

¡Le hemos dado una solución al código! Este código es perfectamente válido ahora.
Sin embargo, significa que no podemos usar un pattern irrefutable sin recibir un
error. Si le damos a `if let` un pattern que siempre coincidirá, como `x`, como
se muestra en el Listado 18-10, el compilador dará una advertencia.

<Listing number="19-10" caption="Intentando usar un pattern irrefutable con `if let`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-10/src/main.rs:here}}
```

</Listing>

Rust se queja de que no tiene sentido usar `if let` con un pattern 
irrefutable:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-10/output.txt}}
```

Por esta razón, las opciones del match deben usar patterns refutables, excepto
por la última opción, que debe coincidir con cualquier valor restante con un
pattern irrefutable. Rust nos permite usar un pattern irrefutable en un `match`
con solo un brazo, pero esta sintaxis no es particularmente útil y podría
reemplazarse con una declaración `let` más simple.

Ahora que sabes dónde usar patterns y la diferencia entre patterns refutables e
irrefutables, cubramos toda la sintaxis que podemos usar para crear patterns.
