## Funciones y Closures Avanzados

Esta sección cubre algunas características avanzadas relacionadas con
funciones y closures, incluyendo punteros a funciones y retornar closures.

### Function Pointers

Hemos hablado de cómo pasar closures a funciones; ¡también puedes pasar
funciones regulares a funciones! Esta técnica es útil cuando quieres pasar una
función que ya has definido en lugar de definir un nuevo closure. Las funciones
se coercen al tipo `fn` (con una _f_ minúscula), no confundir con el trait de
cierre `Fn`. El tipo `fn` se llama _puntero a función_. Pasar funciones con
punteros a función te permitirá usar funciones como argumentos para otras
funciones.

La sintaxis para especificar que un parámetro es un puntero a función es
similar a la de los closures, como se muestra en el Listado 20-28, donde hemos
definido una función `add_one` que suma uno a su parámetro. La función
`do_twice` toma dos parámetros: un puntero a función a cualquier función que
tome un parámetro `i32` y devuelva un `i32`, y un valor `i32`. La función
`do_twice` llama a la función `f` dos veces, pasándole el valor `arg`, luego
suma los dos resultados de la llamada a la función. La función `main` llama a
`do_twice` con los argumentos `add_one` y `5`.

<Listing number="20-28" file-name="src/main.rs" caption="Usando el tipo `fn` para aceptar un puntero a function como un argumento">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-28/src/main.rs}}
```

</Listing>

Este código imprime `The answer is: 12`. Especificamos que el parámetro `f` en
`do_twice` es un `fn` que toma un parámetro de tipo `i32` y devuelve un `i32`.
Luego podemos llamar a `f` en el cuerpo de `do_twice`. En `main`, podemos pasar
el nombre de la función `add_one` como el primer argumento a `do_twice`.

A diferencia de los closures, `fn` es un tipo en lugar de un trait, por lo que
especificamos `fn` como el tipo de parámetro directamente en lugar de declarar
un parámetro de tipo genérico con uno de los traits `Fn` como un trait bound.

Los punteros a funciones implementan los tres closure traits (`Fn`, `FnMut` y
`FnOnce`), lo que significa que siempre puedes pasar un puntero a función como
un argumento para una función que espera un closure. Es mejor escribir
funciones usando un tipo generic y uno de los closure traits para que tus
funciones puedan aceptar funciones o closures.

Dicho esto, un ejemplo de dónde querrías aceptar solo `fn` y no closures es
cuando te comunicas con código externo que no tiene closures: las funciones de
C pueden aceptar funciones como argumentos, pero C no tiene closures.

Como ejemplo de dónde podrías usar un closure definido en línea o una función
nombrada, veamos un uso del método `map` proporcionado por el trait `Iterator`
en la biblioteca estándar. Para usar la función `map` para convertir un vector
de números en un vector de strings, podríamos usar un closure, como en el 
Listado 20-29:

<Listing number="20-29" caption="Using a closure with the `map` method to convert numbers to strings">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-29/src/main.rs:here}}
```

</Listing>

O podríamos nombrar una función como argumento para `map` en lugar del
closure. El Listado 20-30 muestra cómo se vería.

<Listing number="20-30" caption="Usando el metodo `String::to_string` para convertir numeros a strings">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-30/src/main.rs:here}}
```

</Listing>

Ten en cuenta que debemos utilizar la sintaxis completamente calificada que
mencionamos anteriormente en la sección [“Traits avanzados”][advanced-traits]

<!-- ignore --> porque hay múltiples funciones disponibles llamadas `to_string`.

Aquí, estamos usando la función `to_string` definida en el trait `ToString`,
que la biblioteca estándar ha implementado para cualquier tipo que implemente
`Display`.

Recuerda la sección [“Valores de Enum”][valores-enum]<!-- ignore --> del
Capítulo 6, que el nombre de cada variante de enum que definimos también se
convierte en una función inicializadora. Podemos usar estas funciones
inicializadoras como punteros a función que implementan los closure traits,
lo que significa que podemos especificar las funciones inicializadoras como
argumentos para los métodos que toman closures, como puedes ver en el 
Listado 20-31:

<Listing number="20-31" caption="Usando un inicializador de enum con el método `map` para crear una instancia de `Status` a partir de números">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-31/src/main.rs:here}}
```

</Listing>

Aquí creamos instancias de `Status::Value` usando cada valor `u32` en el rango
en el que se llama a `map` usando la función inicializadora de `Status::Value`.
A algunas personas les gusta este estilo, y a otras les gusta usar closures.
Compilan al mismo código, así que usa el estilo que sea más claro para ti.

### Retornando Closures

Closures are represented by traits, which means you can’t return closures
directly. In most cases where you might want to return a trait, you can instead
use the concrete type that implements the trait as the return value of the
function. However, you can’t do that with closures because they don’t have a
concrete type that is returnable; you’re not allowed to use the function
pointer `fn` as a return type, for example.

Instead, you will normally use the `impl Trait` syntax we learned about in
Chapter 10. You can return any function type, using `Fn`, `FnOnce` and `FnMut`.
For example, the code in Listing 20-32 will work just fine.

<Listing number="20-32" caption="Returning a closure from a function using the `impl Trait` syntax">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-32/src/lib.rs}}
```

</Listing>

Sin embargo, como mencionamos en 
[“Inferencia y anotación de tipos en closures”][closure-types]<!-- ignore --> 
en el Capítulo 13, cada closure también es un tipo distinto por sí mismo. Si 
necesitas trabajar con múltiples funciones que tienen la misma firma pero 
diferentes implementaciones, tendrás que usar un trait object para ellas. 
Considera qué sucede si escribes un código como el que se muestra en el 
Listado 20-33.

<Listing file-name="src/main.rs" number="20-33" caption="Crear un `Vec<T>` de closures definidos por funciones que retornan `impl Fn`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-33/src/main.rs}}
```

</Listing>

Aquí tenemos dos funciones, `returns_closure` y `returns_initialized_closure`, 
que ambas retornan `impl Fn(i32) -> i32`. Observa que los closures que devuelven 
son diferentes, aunque implementan el mismo tipo. Si intentamos compilar esto, 
Rust nos indica que no funcionará:

```text
{{#include ../listings/ch20-advanced-features/listing-20-33/output.txt}}
```

El mensaje de error nos indica que cada vez que retornamos un `impl Trait`, 
Rust crea un *tipo opaco* único, un tipo cuyos detalles no podemos ver ni 
conocer cómo Rust lo construye. Así, aunque estas funciones retornan closures 
que implementan el mismo trait, `Fn(i32) -> i32`, los tipos opacos que Rust 
genera para cada una son distintos. (Esto es similar a cómo Rust produce tipos 
concretos diferentes para bloques `async` distintos, incluso cuando tienen el 
mismo tipo de salida, como vimos en 
[“Trabajando con cualquier número de futuros”][any-number-of-futures] en el 
Capítulo 17). Hemos visto una solución para este problema varias veces: podemos 
usar un trait objeto, como en el Listado 20-34.


<Listing number="20-34" caption="Crear un `Vec<T>` de closures definidos por funciones que retornan `Box<dyn Fn>`, para que todos tengan el mismo tipo">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-34/src/main.rs:here}}
```

</Listing>

Este código se compilará sin problemas. Para más información sobre 
trait objects, consulta la sección [“Usando trait objects que permiten valores 
de diferentes tipos”][using-trait-objects-that-allow-for-values-of-different-types]<!-- ignore --> 
en el Capítulo 18.

¡Ahora, veamos las macros!


[advanced-traits]: ch20-03-advanced-traits.html#traits-avanzados
[enum-values]: ch06-01-defining-an-enum.html#enum-values
[valores-enum]: ch06-01-defining-an-enum.html#valores-enum
[closure-types]: ch13-01-closures.html#closure-type-inference-and-annotation
[any-number-of-futures]: ch17-03-more-futures.html
[usando-trait-objects-que-permiten-valores-de-diferentes-tipos]: ch18-02-trait-objects.html#usando-trait-objects-que-permiten-valores-de-diferentes-tipos
