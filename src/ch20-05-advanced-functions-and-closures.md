## Funciones y Closures Avanzados

Esta sección cubre algunas características avanzadas relacionadas con
funciones y closures, incluyendo punteros a funciones y retornar closures.

### Function Pointers

Hemos hablado de cómo pasar closures a funciones; ¡también puedes pasar
funciones regulares a funciones! Esta técnica es útil cuando quieres pasar una
función que ya has definido en lugar de definir un nuevo closure. Las funciones
se coercen al tipo `fn` (con una f minúscula), no confundir con el trait de
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
de números en un vector de strings, podríamos usar un closure, como este:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-15-map-closure/src/main.rs:here}}
```

O podríamos nombrar una función como argumento para `map` en lugar del
closure, como este:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-16-map-function/src/main.rs:here}}
```

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
argumentos para los métodos que toman closures, como se muestra a continuación:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-17-map-initializer/src/main.rs:here}}
```

Aquí creamos instancias de `Status::Value` usando cada valor `u32` en el rango
en el que se llama a `map` usando la función inicializadora de `Status::Value`.
A algunas personas les gusta este estilo, y a otras les gusta usar closures.
Compilan al mismo código, así que usa el estilo que sea más claro para ti.

### Retornando Closures

Los closures se representan mediante traits, lo que significa que no puedes
devolver closures directamente. En la mayoría de los casos en los que podrías
querer devolver un trait, puedes usar en su lugar el tipo concreto que
implementa el trait como el valor de retorno de la función. Sin embargo, no
puedes hacer eso con los closures porque no tienen un tipo concreto que se
pueda devolver; no se te permite usar el puntero a función `fn` como un tipo
de retorno, por ejemplo.

El siguiente código intenta devolver un closure directamente, pero no
compilará:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-18-returns-closure/src/lib.rs}}
```

El error del compilador es el siguiente:

```console
{{#include ../listings/ch20-advanced-features/no-listing-18-returns-closure/output.txt}}
```

¡El error hace referencia nuevamente al trait `Sized`! Rust no sabe cuánto
espacio necesitará para almacenar el closure. Vimos una solución a este
problema anteriormente. Podemos usar un trait object:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-19-returns-closure-trait-object/src/lib.rs}}
```

Este código se compilará correctamente. Para obtener más información sobre los
trait objects, consulta la sección [“Usando trait objects que permiten valores
de diferentes
tipos”][usando-trait-objects-que-permiten-valores-de-diferentes-tipos]
<!-- ignore --> en el Capítulo 19.

¡Ahora veamos las macros!

[advanced-traits]: ch20-03-advanced-traits.html#traits-avanzados
[valores-enum]: ch06-01-defining-an-enum.html#valores-enum
[usando-trait-objects-que-permiten-valores-de-diferentes-tipos]: ch18-02-trait-objects.html#usando-trait-objects-que-permiten-valores-de-diferentes-tipos
