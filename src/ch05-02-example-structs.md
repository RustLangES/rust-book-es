## Un Programa de Ejemplo Usando Structs

Para entender cuándo podríamos querer usar structs, vamos a escribir un
programa que calcule el área de un rectángulo. Empezaremos usando variables
individuales, y luego refactorizaremos el programa hasta que estemos usando
structs.

Hagamos un nuevo proyecto binario con Cargo llamado *rectangles* que tomará
el ancho y el alto de un rectángulo especificado en píxeles y calculará el área
del rectángulo. La lista 5-8 muestra un programa corto con una forma de hacer
exactamente eso en el *src/main.rs* de nuestro proyecto.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:all}}
```

<span class="caption">Listing 5-8: Calculando el área de un rectángulo
especificado por separado en variables ancho y alto</span>

Ahora, ejecuta este programa usando `cargo run`:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/output.txt}}
```

Este código logra calcular el área del rectángulo llamando a la función `area`
con cada dimensión, pero podemos hacer más para hacer este código claro y
legible.

El problema con este código es evidente en la firma de `area`:

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:here}}
```

La función `area` está supuesta para calcular el área de un rectángulo, pero
la función que escribimos tiene dos parámetros, y no está claro en ningún
lugar de nuestro programa que los parámetros están relacionados. Sería más
legible y más manejable agrupar el ancho y el alto juntos. Ya hemos discutido
una forma de hacerlo en la sección [“El Tipo Tupla”][the-tuple-type]<!-- ignore
--> del Capítulo 3: usando tuplas.

### Refactorizando con Tuplas

Listings 5-9 muestra otra versión de nuestro programa que usa tuplas.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-09/src/main.rs}}
```

<span class="caption">Listing 5-9: Especificando el ancho y alto del
rectángulo con una tupla</span>

En un sentido, este programa es mejor. Las tuplas nos permiten agregar un poco
de estructura, y ahora estamos pasando solo un argumento. Pero en otro sentido,
esta versión es menos clara: las tuplas no nombran sus elementos, por lo que
tenemos que indexar los componentes de la tupla, haciendo que nuestro
cálculo sea menos obvio.

Mezclar el ancho y el alto no importaría para el cálculo del área, pero si
queremos dibujar el rectángulo en la pantalla, ¡importaría! Tendríamos que
tener en cuenta que `width` es el índice de la tupla `0` y `height` es el índice
de la tupla `1`. ¡Esto sería aún más difícil para que otra persona lo
descubriera y lo tuviera en cuenta si usara nuestro código! Debido a que no
hemos transmitido el significado de nuestros datos en nuestro código, ahora es
más fácil introducir errores.

### Refactorizando con Structs: Añadiendo Más Significado

Hemos usado structs para agregar significado al etiquetar los datos. Podemos
transformar la tupla que estamos usando en un struct con un nombre para el
todo y nombres para las partes, como se muestra en la lista 5-10.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-10/src/main.rs}}
```

<span class="caption">Listing 5-10: Definiendo un struct `Rectangle`</span>

Hemos definido un struct y lo hemos llamado `Rectangle`. Dentro de las llaves,
hemos definido los campos como `width` y `height`, ambos de los cuales tienen
el tipo `u32`. Luego, en `main`, hemos creado una instancia particular de
`Rectangle` que tiene un ancho de `30` y un alto de `50`.

Nuestra función `area` ahora toma un argumento que es una referencia a una
instancia de `Rectangle` en lugar de dos parámetros numéricos. En la función,
usamos el punto para acceder a los campos de la instancia de `Rectangle` que
recibimos como argumento. En `main`, creamos una instancia de `Rectangle` y
llamamos a la función `area` con la instancia de `Rectangle` como argumento.

La función `area` accede a los campos de `width` y `height` de la instancia de
`Rectangle` (tenga en cuenta que acceder a los campos de una instancia de
estructura prestada no mueve los valores de los campos, por lo que a menudo
ve préstamos de estructuras). Nuestra firma de función para `area` ahora dice
exactamente lo que queremos: calcular el área de `Rectangle`, usando sus
campos `width` y `height`. Esto conduce a que el ancho y el alto estén
relacionados entre sí, y da nombres descriptivos a los valores en lugar de
usar los valores de índice de tupla de `0` y `1`. ¡Esto es una victoria para
la claridad!

### Añadiendo Funcionalidad Útil con Traits Derivados

Sería útil poder imprimir una instancia de `Rectangle` mientras estamos
depurando nuestro programa y ver los valores de todos sus campos. La lista 5-11
intenta usar la macro [`println!`][println]<!-- ignore --> como hemos usado en
capítulos anteriores. Sin embargo, esto no funcionará.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/src/main.rs}}
```

<span class="caption">Listing 5-11: Intentando imprimir una instancia de 
`Rectangle`</span>

Cuando compilamos este código, obtenemos un error con este mensaje principal:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:3}}
```

La macro `println!` puede hacer muchos tipos de formateo, y por defecto, las
llaves curvas le dicen a `println!` que use el formateo conocido como
`Display`: salida destinada al consumo directo del usuario final. Los tipos
primitivos que hemos visto hasta ahora implementan `Display` por defecto
porque solo hay una forma en que querrías mostrar un `1` u otro tipo
primitivo a un usuario. Pero con las estructuras, la forma en que `println!`
debe formatear la salida es menos clara porque hay más posibilidades de
visualización: ¿Quieres comas o no? ¿Quieres imprimir las llaves curvas? ¿Deben
mostrarse todos los campos? Debido a esta ambigüedad, Rust no intenta adivinar
lo que queremos, y las estructuras no tienen una implementación proporcionada
de `Display` para usar con `println!` y el marcador de posición `{}`.

Si seguimos leyendo los errores, encontraremos esta nota útil:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:9:10}}
```

Intentemos eso. La llamada a la macro `println!` ahora se verá así:
`println!("rect1 es {:?}", rect1);`. Poner el especificador `:?` dentro de
los corchetes curvos le dice a `println!` que queremos usar un formato de
salida llamado `Debug`. El rasgo `Debug` nos permite imprimir nuestra estructura
de una manera que sea útil para los desarrolladores para que podamos ver su
valor mientras depuramos nuestro código.

Compilamos el código con este cambio. ¡Oh, no! Todavía obtenemos un error:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:3}}
```

Pero otra vez, el compilador nos da una nota útil:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:9:10}}
```

Rust *si* incluye la funcionalidad para imprimir información de depuración,
pero tenemos que optar explícitamente para hacer que esa funcionalidad esté
disponible para nuestra estructura. Para hacer eso, agregamos el atributo
externo `#[derive(Debug)]` justo antes de la definición de la estructura, como
se muestra en la lista 5-12.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/src/main.rs}}
```

<span class="caption">Listing 5-12: Agregando el atributo para derivar el trait 
`Debug` e imprimiendo la instancia `Rectangle` usando el formato debug</span>

Ahora, cuando compilamos el código, no obtendremos ningún error, y veremos la
siguiente salida:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/output.txt}}
```

¡Bien! No es la salida más bonita, pero muestra los valores de todos los
campos de esta instancia, lo que definitivamente ayudaría durante la
depuración. Cuando tenemos estructuras más grandes, es útil tener una salida
que sea un poco más fácil de leer; en esos casos, podemos usar `{:#?}` en
lugar de `{:?}` en el string `println!`. En este ejemplo, el uso del estilo
`{:#?}` producirá la siguiente salida:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-02-pretty-debug/output.txt}}
```

Otra forma de imprimir un valor usando el formato `Debug` es usar la macro
[`dbg!`][dbg]<!-- ignore -->, que toma el ownership de una expresión (en
oposición a `println!`, que toma una referencia), imprime el archivo y el
número de línea donde se produce esa llamada a la macro `dbg!` en su código
junto con el valor resultante de esa expresión, y devuelve el ownership del
valor.

> Nota: Llamar a la macro `dbg!` imprime en el flujo de consola de error
> estándar (`stderr`), en oposición a `println!`, que imprime en el flujo de
> consola de salida estándar (`stdout`). Hablaremos más sobre `stderr` y
> `stdout` en la [sección “Escribiendo mensajes de error en el error estándar
> en lugar de la salida estándar” del capítulo 12][err]<!-- ignore -->.

Aquí hay un ejemplo en el que estamos interesados en el valor que se asigna al
campo `width`, así como el valor de todo el struct en `rect1`:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/src/main.rs}}
```

Podemos poner `dbg!` alrededor de la expresión `30 * scale` y, porque `dbg!`
devuelve el ownership del valor de la expresión, el campo `width` tendrá el
mismo valor que si no tuviéramos la llamada `dbg!` allí. No queremos que `dbg!`
tome el ownership de `rect1`, así que usamos una referencia a `rect1` en la
siguiente llamada. Aquí está el output de este ejemplo:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/output.txt}}
```

Podemos ver que la primera parte de la salida proviene de *src/main.rs* línea
10 donde estamos depurando la expresión `30 * scale`, y su valor resultante es
`60` (la formateo `Debug` implementado para enteros es imprimir solo su valor).
La llamada `dbg!` en la línea 14 de *src/main.rs* produce el valor de `&rect1`,
que es la estructura `Rectangle`. Esta salida usa el formateo `Debug` de la
estructura `Rectangle`. La macro `dbg!` puede ser realmente útil cuando está
tratando de averiguar qué está haciendo su código.

Además del trait `Debug`, Rust nos ha proporcionado un número de traits para
que podamos usar con el atributo `derive` que pueden agregar un comportamiento
útil a nuestros tipos personalizados. Esos traits y sus comportamientos se
enumeran en [el Apéndice C][app-c]<!-- ignore -->. Cubriremos cómo implementar
estos traits con un comportamiento personalizado, así como cómo crear sus
propios traits en el Capítulo 10. También hay muchos atributos más allá de
`derive`; para obtener más información, consulte [la sección “Atributos” de la
Referencia de Rust][attributes].

Nuestra función `area` es muy específica: solo calcula el área de
rectángulos. Sería útil vincular este comportamiento más estrechamente a nuestra
estructura `Rectangle` porque no funcionará con ningún otro tipo. Veamos cómo
podemos continuar refactorizando este código al convertir la función `area` en
un *método* `area` definido en nuestro tipo `Rectangle`.

[the-tuple-type]: ch03-02-data-types.html#the-tuple-type
[app-c]: appendix-03-derivable-traits.md
[println]: ../std/macro.println.html
[dbg]: ../std/macro.dbg.html
[err]: ch12-06-writing-to-stderr-instead-of-stdout.html
[attributes]: ../reference/attributes.html
