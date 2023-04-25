## Flujo de control Conciso con `if let` (Control Flow con `if let`)

La sintaxis `if let` te permite combinar `if` y `let` en una forma menos
verbosa de manejar valores que coinciden con un patrón mientras se ignoran el
resto. Considera el programa en el Listado 6-6 que coincide con un valor
`Option<u8>` en la variable `config_max` pero solo quiere ejecutar el código si
el valor es la variante `Some`.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-06/src/main.rs:here}}
```

<span class="caption">Listing 6-6: A `match` that only cares about executing
code when the value is `Some`</span>

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
`match` correspondiente. El código en el bloque `if let` no se ejecuta si el
valor no coincide con el patrón.

Usar `if let` significa menos escritura, menos sangría y menos código de
plantilla. Sin embargo, pierdes la verificación exhaustiva que hace cumplir
`match`. Elegir entre `match` y `if let` depende de lo que estés haciendo en tu
situación particular y si ganar concisión es un intercambio apropiado para
perder la verificación exhaustiva.

En otras palabras, puedes pensar en `if let` como una sintaxis dulce para un
`match` que ejecuta código cuando el valor coincide con un patrón y luego
ignora todos los demás valores.

Podemos incluir un `else` con un `if let`. El bloque de código que va con el
`else` es el mismo que el bloque de código que iría con el caso `_` en la
expresión `match` que es equivalente al `if let` y `else`. Recuerda la
definición de `Coin` en el Listado 6-4, donde la variante `Quarter` también
tenía un valor `UsState`. Si quisiéramos contar todas las monedas que no son
cuartos que vemos mientras también anunciamos el estado de los cuartos, podríamos
hacerlo con una expresión `match`, como esta:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-13-count-and-announce-match/src/main.rs:here}}
```

O podríamos usar un `if let` y `else`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-14-count-and-announce-if-let-else/src/main.rs:here}}
```

Si se encuentra en una situación en la cual tu programa tiene lógica que es
demasiado verbosa para expresar usando un `match`, recuerda que `if let` está
en tu caja de herramientas de Rust también.

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