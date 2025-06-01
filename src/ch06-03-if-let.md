## Flujo de Control Conciso con `if let` y `let else`

La sintaxis `if let` te permite combinar `if` y `let` en una forma menos
verbosa de manejar valores que coinciden con un patrón mientras se ignoran el
resto. Considera el programa en el Listado 6-6 que coincide con un valor
`Option<u8>` en la variable `config_max` pero solo quiere ejecutar el código si
el valor es la variante `Some`.

<Listing number="6-6" caption="Un `match` que solo se preocupa por ejecutar código cuando el valor es `Some`">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-06/src/main.rs:here}}
```

</Listing>

Si el valor es `Some`, imprimimos el valor en la variante `Some` vinculando el
valor a la variable `max` en el patrón. No queremos hacer nada con el valor
`None`. Para satisfacer la expresión `match`, tenemos que agregar `_ => ()`
después de procesar solo una variante, lo cual es un código de plantilla
molesto para agregar.

En su lugar, podríamos escribir esto de una manera más corta usando `if let`.
El siguiente código se comporta de la misma manera que el `match` en el Listado
6-6:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-12-if-let/src/main.rs:here}}
```

La sintaxis `if let` toma un patrón y una expresión separados por un signo
igual. Funciona de la misma manera que un `match`, donde la expresión se da al
`match` y el patrón es su primer brazo. En este caso, el patrón es `Some(max)`,
y el `max` se vincula al valor dentro del `Some`. Luego podemos usar `max` en
el cuerpo del bloque `if let` de la misma manera que usamos `max` en el brazo
`match` correspondiente. El código en el bloque `if let` solo se ejecuta si el
valor coincide con el patrón.

Usar `if let` significa menos escritura, menos indentación y menos código 
repetitivo. Sin embargo, pierdes la verificación exhaustiva que hace cumplir
`match`. Elegir entre `match` e `if let` depende de lo que estés haciendo en tu
situación particular y de si ser más conciso a cambio de la verificación 
exhaustiva es un intercambio adecuado.

En otras palabras, puedes pensar en `if let` como una sintaxis dulce para un
`match` que ejecuta código cuando el valor coincide con un patrón y luego
ignora todos los demás valores.

Podemos incluir un `else` con un `if let`. El bloque de código que va con el
`else` es el mismo que el bloque de código que iría con el caso `_` en la
expresión `match` que es equivalente al `if let` y `else`. Recuerda la
definición de `Coin` en el Listado 6-4, donde la variante `Quarter` también
tenía un valor `UsState`. Si quisiéramos contar todas las monedas que no son
cuartos que vemos mientras también anunciamos el estado de los cuartos, 
podríamos hacerlo con una expresión `match`, como esta:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-13-count-and-announce-match/src/main.rs:here}}
```

O podríamos usar un `if let` y `else`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-14-count-and-announce-if-let-else/src/main.rs:here}}
```

## Manteniéndonos en el "Camino Feliz" con `let...else`

Un patrón común es realizar algún cálculo cuando un valor está presente y
devolver un valor predeterminado de lo contrario. Continuando con nuestro
ejemplo de monedas con un valor `UsState`, si quisiéramos decir algo gracioso
dependiendo de cuán viejo era el estado en el cuarto, podríamos introducir un
método en `UsState` para verificar la edad de un estado, como este:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-07/src/main.rs:state}}
```

Luego podríamos usar `if let` para coincidir con el tipo de moneda,
introduciendo una variable `state` dentro del cuerpo de la condición, como en el
Listado 6-7.

<Listing number="6-7" caption="Verificar si un estado existía en 1900 usando condicionales anidados dentro de un `if let`." file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-07/src/main.rs:describe}}
```

</Listing>

Esto hace el trabajo, pero ha empujado el trabajo al cuerpo de la declaración
`if let`, y si el trabajo a realizar es más complicado, podría ser difícil
seguir exactamente cómo se relacionan las ramas de nivel superior. También
podríamos aprovechar el hecho de que las expresiones producen un valor
ya sea para producir el `state` del `if let` o para devolver temprano, como en
el Listado 6-8. (¡Por supuesto, podrías hacer algo similar con un `match`!)

<Listing number="6-8" caption="Usando `if let` para producir un valor o devolver temprano." file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-08/src/main.rs:describe}}
```

</Listing>

Esto es un poco molesto de seguir a su manera, ¡sin embargo! Una rama del `if
let` produce un valor, y la otra regresa de la función por completo. 

Para hacer que este patrón común sea más agradable de expresar, Rust tiene
`let...else`. La sintaxis `let...else` toma un patrón en el lado izquierdo y una
expresión en el lado derecho, muy similar a `if let`, pero no tiene una rama
`if`, solo una rama `else`. Si el patrón coincide, vinculará el valor del
patrón en el ámbito externo. Si el patrón _no_ coincide, el programa fluirá
hacia el brazo `else`, que debe devolver de la función.

En el Listado 6-9, puedes ver cómo se ve el Listado 6-8 al usar `let...else`
en lugar de `if let`. Observa que se mantiene "en el camino feliz" en el cuerpo
de la función de esta manera, sin tener un flujo de control significativamente
diferente para dos ramas de la manera en que lo hizo el `if let`.

<Listing number="6-9" caption="Usando `let...else` para aclarar el flujo a través de la función." file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-09/src/main.rs:describe}}
```

</Listing>

Si se encuentra en una situación en la cual tu programa tiene lógica que es
demasiado verbosa para expresar usando un `match`, recuerda que `if let` y 
`let...else` está en tu caja de herramientas de Rust también.

## Resumen

Ahora hemos cubierto cómo usar enums para crear tipos personalizados que pueden
ser uno de un conjunto de valores enumerados. Hemos mostrado cómo el tipo
`Option<T>` de la biblioteca estándar te ayuda a usar el sistema de tipos para
prevenir errores. Cuando los valores de enum tienen datos dentro de ellos,
podemos usar `match` o `if let` para extraer y usar esos valores, dependiendo de
cuántos casos necesites manejar.

Tus programas Rust ahora pueden expresar conceptos en tu dominio usando
structs y enums. Crear tipos personalizados para usar en tu API
asegura la seguridad de tipos: el compilador se asegurará de que tus funciones
solo obtengan valores del tipo que cada función espera.

En orden de proveer una API bien organizada a tus usuarios que sea
sencilla de usar y solo exponga exactamente lo que tus usuarios necesitarán,
ahora vamos a ver los módulos de Rust.
