## Validando Referencias con Lifetimes

Los *lifetimes* son otro tipo de genéricos que ya hemos estado usando. En lugar
de asegurarnos de que un tipo tenga el comportamiento que queremos, los
lifetimes aseguran que las referencias sean válidas el tiempo que las
necesitemos.

Un detalle que no discutimos en la sección [“Referencias y
Borrowing"][references-and-borrowing]<!-- ignore --> en el Capítulo 4 es que 
cada referencia en Rust tiene un *lifetime*, que es el alcance para el que esa 
referencia es válida. La mayoría de las veces, los lifetimes son implícitos e 
inferidos, al igual que la mayoría de las veces, los tipos se infieren. 
Solo debemos anotar los tipos cuando son posibles varios tipos. De manera 
similar, debemos anotar los lifetimes cuando los lifetimes de las referencias 
podrían estar relacionados de algunas maneras diferentes. Rust nos obliga a 
anotar las relaciones usando parámetros genéricos de lifetime para garantizar 
que las referencias reales utilizadas en tiempo de ejecución sean 
definitivamente válidas.

Anotar lifetimes no es ni siquiera un concepto que la mayoría de los otros
lenguajes de programación tengan, por lo que esto se sentirá poco familiar.
Aunque no cubriremos los lifetimes en su totalidad en este capítulo,
discutiremos las formas comunes en que podría encontrar la sintaxis de los
lifetimes para que pueda familiarizarse con el concepto.

### Previniendo Referencias Colgantes con Lifetimes

El objetivo principal de los lifetimes es prevenir *referencias colgantes*,
que hacen que un programa haga referencia a datos que no son los que se
pretende referenciar. Considere el programa en el Listado 10-16, que tiene un
scope externo y un scope interno.

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-16/src/main.rs}}
```

<span class="caption">Listing 10-16: Un intento de usar una referencia cuyo 
valor ha quedado fuera del scope</span>

> Nota: Los ejemplos en los Listados 10-16, 10-17 y 10-23 declaran variables
> sin darles un valor inicial, por lo que el nombre de la variable existe en el
> scope externo. A primera vista, esto podría parecer estar en conflicto con el
> hecho de que Rust no tiene valores nulos. Sin embargo, si intentamos usar una
> variable antes de darle un valor, obtendremos un error en tiempo de
> compilación, lo que muestra que Rust de hecho no permite valores nulos.

El scope externo declara una variable llamada `r` sin valor inicial, y el scope
interno declara una variable llamada `x` con el valor inicial de 5. Dentro del
scope interno, intentamos establecer el valor de `r` como una referencia a `x`.
Luego, el scope interno termina, e intentamos imprimir el valor en `r`. Este
código no se compilará porque el valor al que se refiere `r` ha quedado fuera
del scope antes de que intentemos usarlo. Aquí está el mensaje de error:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-16/output.txt}}
```

La variable `x` no “vive lo suficiente”. La razón es que `x` estará fuera del
scope cuando el scope interno termine en la línea 7. Pero `r` todavía es
válido para el scope externo; porque su scope es más grande, decimos que
“vive más tiempo”. Si Rust permitiera que este código funcionara, `r` estaría
referenciando memoria que se desasignó cuando `x` quedó fuera del scope, y
cualquier cosa que intentemos hacer con `r` no funcionaría correctamente. ¿Cómo
determina Rust que este código es inválido? Utiliza el *borrow checker*.

### El Borrow Checker

El compilador de Rust tiene un *borrow checker* que compara scopes para
determinar si todos los *borrows* son válidos. El Listado 10-17 muestra el
mismo código que el Listado 10-16, pero con anotaciones que muestran los
lifetimes de las variables.

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-17/src/main.rs}}
```

<span class="caption">Listing 10-17: Anotaciones de los lifetimes de `r` y
`x`, denominados `'a` y `'b`, respectivamente</span>

Aquí, hemos anotado el lifetime de `r` con `'a` y el lifetime de `x` con `'b`.
Como puede ver, el bloque interno `'b` es mucho más pequeño que el bloque
externo `'a`. En tiempo de compilación, Rust compara el tamaño de los dos
lifetimes y ve que `r` tiene un lifetime de `'a` pero que se refiere a la
memoria con un lifetime de `'b`. El programa es rechazado porque `'b` es más
corto que `'a`: el sujeto de la referencia no vive tanto como la referencia.

El Listado 10-18 corrige el código para que no tenga una referencia pendiente y
se compile sin errores.

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-18/src/main.rs}}
```

<span class="caption">Listing 10-18: Una referencia válida porque los datos 
tienen un lifetime más largo que la referencia</span>

Aquí, `x` tiene el lifetime `'b` que en este caso es más grande que `'a`. Esto
significa que `r` puede hacer referencia a `x` porque Rust sabe que la
referencia en `r` siempre será válida mientras `x` sea válida.

Ahora que sabemos dónde están los lifetimes de las referencias y cómo Rust
analiza los lifetimes para garantizar que las referencias siempre sean válidas,
exploraremos los lifetimes genéricos de los parámetros y valores de retorno en
el contexto de las funciones.

### Generic Lifetimes en Funciones

Escribiremos una función que devuelva el más largo de dos *string slices*.
Esta función tomará dos *string slices* y devolverá un solo *string slice*.
Después de haber implementado la función `longest`, el código en el Listado
10-19 debería imprimir `The longest string is abcd`.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-19/src/main.rs}}
```

<span class="caption">Listing 10-19: Una función `main` que llama a la
función `longest` para encontrar el más largo de dos string slices</span>

Ten en cuenta que queremos que la función tome *string slices*, que son
referencias, en lugar de *strings*, porque no queremos que la función `longest`
tome posesión de sus parámetros. Consulta la sección [“String Slices as
Parameters”][string-slices-as-parameters]<!-- ignore --> en el Capítulo 4 para
obtener más información sobre por qué los parámetros que usamos en el Listado
10-19 son los que queremos.

Si intentamos implementar la función `longest` como se muestra en el Listado
10-20, no se compilará.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-20/src/main.rs:here}}
```

<span class="caption">Listing 10-20: Una implementación de la función `longest`
que devuelve el más largo de dos string slices pero aún no compila</span>

En su lugar, obtenemos el siguiente error que habla sobre lifetimes:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-20/output.txt}}
```

El texto de ayuda revela que el tipo de retorno necesita un parámetro de
lifetime generic en él porque Rust no puede decir si la referencia que se
devuelve se refiere a `x` o `y`. De hecho, nosotros tampoco lo sabemos, porque
el bloque `if` en el cuerpo de esta función devuelve una referencia a `x` y el
bloque `else` devuelve una referencia a `y`!

Cuando estamos definiendo esta función, no sabemos los valores concretos que
se pasarán a esta función, por lo que no sabemos si se ejecutará el caso `if` o
el caso `else`. Tampoco conocemos los lifetimes concretos de las referencias
que se pasarán, por lo que no podemos mirar los scopes como lo hicimos en los
Listados 10-17 y 10-18 para determinar si la referencia que devolvemos siempre
será válida. El *borrow checker* tampoco puede determinar esto, porque no sabe
cómo se relacionan los lifetimes de `x` e `y` con el lifetime del valor de
retorno. Para corregir este error, agregaremos parámetros de lifetime generics
que definan la relación entre las referencias para que el *borrow checker*
pueda realizar su análisis.

### Sintaxis de las anotaciones de los lifetimes

Las anotaciones de los lifetimes no cambian cuánto tiempo viven las
referencias. En cambio, describen las relaciones de los lifetimes de múltiples
referencias entre sí sin afectar los lifetimes. Al igual que las funciones
pueden aceptar cualquier tipo cuando la firma especifica un parámetro de tipo
genérico, las funciones pueden aceptar referencias con cualquier lifetime
especificando un parámetro de lifetime generic.

Las anotaciones de los lifetimes tienen una sintaxis ligeramente inusual: los
nombres de los parámetros de los lifetimes deben comenzar con un apóstrofe (`'`)
y generalmente son todos en minúsculas y muy cortos, como los tipos generics.
La mayoría de la gente usa el nombre `'a` para la primera anotación de
lifetime. Colocamos las anotaciones de los parámetros de los lifetimes después
del `&` de una referencia, usando un espacio para separar la anotación del tipo
de referencia.

Estos son algunos ejemplos: una referencia a un `i32` sin un parámetro de
lifetime, una referencia a un `i32` que tiene un parámetro de lifetime llamado
`'a`, y una referencia mutable a un `i32` que también tiene el lifetime `'a`.

```rust,ignore
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

Una anotación de lifetime en si misma no tiene mucho significado, porque las
anotaciones están destinadas a decirle a Rust cómo los parámetros de lifetime
generics de múltiples referencias se relacionan entre sí. Examinemos cómo las
anotaciones de los lifetimes se relacionan entre sí en el contexto de la
función `longest`.

### Anotaciones de los Lifetimes en las Firmas de las Funciones

Para usar anotaciones de los lifetimes en las firmas de las funciones, primero
necesitamos declarar los parámetros de los lifetimes generic dentro de los
corchetes angulares entre el nombre de la función y la lista de parámetros,
como lo hicimos con los parámetros de tipo generic.

Queremos que la firma exprese la siguiente restricción: la referencia devuelta
será válida siempre que ambos parámetros sean válidos. Esta es la relación
entre los lifetimes de los parámetros y el valor de retorno. Nombraremos al
lifetime `'a` y luego lo agregaremos a cada referencia, como se muestra en el
Listado 10-21.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-21/src/main.rs:here}}
```

<span class="caption">Listing 10-21: La definición de la función `longest`
que especifica que todas las referencias en la firma deben tener el mismo 
lifetime `'a`</span>

Este código debe compilar y producir el resultado que queremos cuando lo
usamos con la función `main` en el Listado 10-19.

La firma de la función ahora le dice a Rust que durante el lifetime `'a`, la
función toma dos parámetros, ambos los cuales son string slices que viven al
menos tanto como el lifetime `'a`. La firma de la función también le dice a
Rust que el string slice devuelto también vivirá al menos tanto como el
lifetime `'a`. En la práctica, significa que el lifetime de la referencia
devuelta por la función `longest` es el mismo que el más pequeño de los
lifetimes de los valores a los que se refieren los argumentos de la función.
Estas relaciones son lo que queremos que Rust use al analizar este código.

Recuerda, cuando especificamos los parámetros de los lifetimes en la firma de
esta función, no estamos cambiando los lifetimes de ninguna de las referencias
que se pasan en o se devuelven. En cambio, estamos especificando que el
*borrow checker* debería rechazar cualquier valor que no cumpla con estas
restricciones. Ten en cuenta que la función `longest` no necesita saber
exactamente cuánto tiempo vivirán `x` e `y`, solo que algún scope puede
sustituirse por `'a` que satisfará esta firma.


Cuando anotamos lifetimes en funciones, las anotaciones van en la firma de la
función, no en el cuerpo de la función. Las anotaciones de los lifetimes se
convierten en parte del contrato de la función, al igual que los tipos en la
firma. Tener las firmas de las funciones que contienen el contrato de los
lifetimes significa que el análisis que hace el compilador de Rust puede ser
más simple. Si hay un problema con la forma en que se anotó una función o la
forma en que se llama, los errores del compilador pueden apuntar a la parte de
nuestro código y las restricciones con más precisión. Si, en cambio, el
compilador de Rust hiciera más inferencias sobre lo que pretendíamos que
fueran las relaciones de los lifetimes, el compilador solo podría señalar el
uso de nuestro código muchas etapas después de la causa del problema.

Cuando pasamos referencias concretas a `longest`, se sustituye un lifetime 
concreto por `'a`. Este lifetime concreto corresponde a la parte del scope de `x` 
que se superpone con el scope de y. En otras palabras, el lifetime 
genérico `'a` adquirirá el lifetime concreto que sea menor entre los lifetimes de 
`x` e `y`. Debido a que hemos anotado la referencia devuelta con el mismo parámetro 
de lifetime `'a`, la referencia devuelta también será válida por la duración del 
lifetime más corta entre `x` e `y`.

Veamos cómo las anotaciones de los lifetimes restringen la función `longest`
pasando referencias que tienen diferentes lifetimes concretos. El Listado
10-22 es un ejemplo sencillo.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-22/src/main.rs:here}}
```

<span class="caption">Listing 10-22: Usando la función `longest` con referencias
a valores `String` que tienen diferentes lifetimes concretos</span>

En este ejemplo, `string1` es válida hasta el final del scope externo, `string2`
es válida hasta el final del scope interno, y `result` referencia algo que es
válido hasta el final del scope interno. Ejecuta este código, y verás que el
*borrow checker* lo aprueba; se compilará e imprimirá `The longest string is
long string is long`.

A continuación, intentemos un ejemplo que muestre que el lifetime de la
referencia en `result` debe ser el más pequeño de los dos argumentos.
Moveremos la declaración de la variable `result` fuera del scope interno, pero
dejaremos la asignación del valor a `result` dentro del scope interno. Luego
moveremos la llamada a `println!` que usa `result` fuera del scope interno,
después de que el scope interno haya terminado. El código del Listado 10-23 no
compilará.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-23/src/main.rs:here}}
```

<span class="caption">Listing 10-23: Intentando utilizar `result` después de que
`string2` haya quedado fuera del scope</span>

Cuando intentamos compilar este código, obtenemos este error:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-23/output.txt}}
```

El error muestra que para que `result` sea válido para la instrucción 
`println!`, `string2` tendría que ser válido hasta el final del scope externo.
Rust sabe esto porque anotamos los lifetimes de los parámetros de la función y
los valores de retorno usando el mismo parámetro de lifetime `'a`.

Como humanos, podemos mirar este código y ver que `string1` es más larga que
`string2` y por lo tanto `result` contendrá una referencia a `string1`. Debido a
que `string1` aún no ha quedado fuera del scope, una referencia a `string1`
todavía será válida para la instrucción `println!`. Sin embargo, el compilador
no puede ver que la referencia sea válida en este caso. Le hemos dicho a Rust
que el lifetime de la referencia devuelta por la función `longest` es el mismo
que el más pequeño de los lifetimes de las referencias pasadas. Por lo tanto,
el *borrow checker* rechaza el código del Listado 10-23 como posiblemente
conteniendo una referencia no válida.

Intenta diseñar más experimentos que varíen los valores y los lifetimes de las
referencias que se pasan a la función `longest` y cómo se usa la referencia
devuelta. Haz hipótesis sobre si tus experimentos pasarán el *borrow checker*
antes de compilar; luego comprueba si tienes razón!

### Pensando en términos de lifetimes

La forma en que necesitas especificar los parámetros de los lifetimes depende
de lo que tu función esté haciendo. Por ejemplo, si cambiamos la implementación
de la función `longest` para que siempre devuelva el primer parámetro en lugar
de la referencia a la cadena más larga, no necesitaríamos especificar un
lifetime en el parámetro `y`. El siguiente código compilará:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-08-only-one-reference-with-lifetime/src/main.rs:here}}
```

Hemos especificado un parámetro de lifetime `'a` para el parámetro `x` y el tipo
de retorno, pero no para el parámetro `y` porque el lifetime de `y` no tiene 
ninguna relación con el lifetime de `x` o el valor de retorno.

Cuando se devuelve una referencia desde una función, el parámetro del lifetime
para el tipo de retorno debe coincidir con el parámetro del lifetime de uno de 
los parámetros. Si la referencia devuelta no se refiere a uno de los parámetros,
debe referirse a un valor creado dentro de esa función. Sin embargo, esto sería
una referencia colgante porque el valor quedará fuera del scope al final de la 
función. Considera esta implementación intentada de la función `longest` que no
se compilará:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/src/main.rs:here}}
```

Aquí, aunque hemos especificado un parámetro de lifetime `'a` para el tipo de
retorno, esta implementación no se compilará porque el lifetime del valor 
retornado no está relacionado en absoluto con el lifetime de los parámetros. 
Este es el mensaje de error que obtenemos:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/output.txt}}
```

El problema es que `result` sale del scope y se limpia al final de la función
`longest`. También estamos tratando de devolver una referencia a `result` desde
la función. No hay forma de especificar parámetros de lifetime que cambien la 
referencia colgante, y Rust no nos permitirá crear una referencia colgante. En
este caso, la mejor solución sería devolver un tipo de dato propiedad en lugar 
de una referencia para que la función que llama sea responsable de limpiar el 
valor.

En última instancia, la sintaxis de lifetime se trata de conectar las duraciones
de vida de varios parámetros y valores de retorno de funciones. Una vez que se 
conectan, Rust tiene suficiente información para permitir operaciones seguras en
memoria y prohibir operaciones que puedan crear punteros colgantes o que de otro
modo violen la seguridad de la memoria.

### Anotaciones de lifetime en definiciones de struct

Hasta ahora, los structs que hemos definido contienen tipos de ownership.
Podemos definir structs que contengan referencias, pero en ese caso necesitamos
agregar una anotación de lifetime en cada referencia en la definición del 
struct. El Listado 10-24 tiene un struct llamado `ImportantExcerpt` que contiene
una string slice.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-24/src/main.rs}}
```

<span class="caption">Listing 10-24: Un struct que contiene una referencia, 
lo que requiere una annotation de lifetime</span>

Este struct tiene el campo `part` que contiene un string slice, que es una
referencia. Como con los tipos de datos generics, declaramos el nombre del
parámetro de lifetime genérico dentro de corchetes angulares después del nombre
del struct para que podamos usar el parámetro de lifetime en el cuerpo de la
definición del struct. Esta anotación significa que una instancia de
`ImportantExcerpt` no puede sobrevivir más allá de la referencia que contiene
en su campo `part`.

La función `main` aquí crea una instancia del struct `ImportantExcerpt` que
contiene una referencia a la primera oración de la variable `novel`. La data en
`novel` existe antes de que se cree la instancia de `ImportantExcerpt`. Además,
`novel` no sale del scope hasta después de que la instancia de `ImportantExcerpt`
sale del scope, por lo que la referencia en la instancia de `ImportantExcerpt`
es válida.

### Omisión de lifetime

Has aprendido que cada referencia tiene un lifetime y que debes especificar
parámetros de lifetime para las funciones o structs que usan referencias. Sin
embargo, en el Capítulo 4, tuvimos una función en el Listado 4-9, que se muestra
nuevamente en el Listado 10-25, que se compiló sin anotaciones de lifetime.

<span class="filename">Filename: src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-25/src/main.rs:here}}
```

<span class="caption">Listing 10-25: Una función que definimos en el Listado 4-9
que compiló sin anotaciones de lifetime, a pesar de que el parámetro y el tipo
de retorno son referencias</span>

La razón por la que esta función se compila sin anotaciones de lifetime es
histórica: en las primeras versiones (pre-1.0) de Rust, este código no se
compilaría porque cada referencia necesitaba un lifetime explícito. En ese
momento, la firma de la función se habría escrito así:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```

Después de escribir mucho código en Rust, el equipo de Rust descubrió que los
programadores de Rust estaban ingresando las mismas anotaciones de lifetime una
y otra vez en situaciones particulares. Estas situaciones eran predecibles y
seguían algunos patrones deterministas. Los desarrolladores programaron estos
patrones en el código del compilador para que el borrow checker pudiera inferir
los lifetimes en estas situaciones y no necesitara anotaciones explícitas.

Este fragmento de la historia de Rust es relevante porque es posible que
aparezcan patrones más deterministas y se agreguen al compilador. En el futuro,
se pueden requerir aún menos anotaciones de lifetime.

Los patrones programados en el análisis de referencias de Rust se llaman *reglas
de omisión de lifetime*. Estas no son reglas que los programadores deben    
seguir; son un conjunto de casos particulares que el compilador considerará, y
si su código se ajusta a estos casos, no es necesario que escriba los lifetimes
explícitamente.

Las reglas de omisión no proporcionan inferencia completa. Si Rust aplica
determinísticamente las reglas pero todavía hay ambigüedad sobre qué lifetimes
tienen las referencias, el compilador no adivinará qué lifetime deberían tener
las referencias restantes. En lugar de adivinar, el compilador le dará un error
que puede resolver agregando las anotaciones de lifetime.

Los lifetime en los parámetros de una función o método se llaman lifetime de 
entrada y los lifetime en los valores de retorno se llaman *output lifetimes*.

El compilador usa tres reglas para determinar los lifetime de las referencias 
cuando no hay anotaciones explícitas. La primera regla se aplica a los lifetime 
de entrada, y la segunda y tercera regla se aplican a los *output lifetimes*. Si
el compilador llega al final de las tres reglas y aún hay referencias para las 
que no puede determinar los lifetime, el compilador mostrará un error. Estas 
reglas se aplican tanto a las definiciones de `fn` como a los bloques `impl`.

La primera regla es que el compilador asigna un parámetro de lifetime a cada
parámetro que sea una referencia. En otras palabras, una función con un
parámetro obtiene un parámetro de lifetime: `fn foo<'a>(x: &'a i32)`; una
función con dos parámetros obtiene dos parámetros de lifetime separados: `fn
foo<'a, 'b>(x: &'a i32, y: &'b i32)`; y así sucesivamente.

La segunda regla es que, si hay un parámetro de input de lifetime, ese lifetime
se asigna a todos los parámetros de output de lifetime:`fn foo<'a>(x: &'a i32) 
-> &'a i32`.

La tercera regla es que si hay múltiples parámetros de input de lifetime, pero
uno de ellos es `&self` o `&mut self` porque este es un método, el lifetime de
`self` se asigna a todos los parámetros de output de lifetime. Esto hace que los
métodos sean mucho más agradables de leer y escribir porque se necesitan menos
símbolos.

Imaginemos que somos el compilador. Aplicaremos estas reglas para descubrir los
lifetime de las referencias en la firma de la función `first_word` en el
Listado 10-25. La firma comienza sin ningún lifetime asociado con las
referencias:

```rust,ignore
fn first_word(s: &str) -> &str {
```

Luego el compilador aplica la primera regla, que especifica que cada parámetro
tiene su propio lifetime. Como de costumbre, llamaremos a este lifetime `'a`, 
por lo que ahora la firma es la siguiente:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &str {
```

La segunda regla aplica porque hay exactamente un parámetro de input con 
lifetime. Este segundo conjunto establece que el lifetime del único parámetro de
input se asigna a todos los parámetros de output, por lo que la firma de la
función se convierte en la siguiente:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```

Ahora todas las referencias en esta firma de función tienen lifetime, y el
compilador puede continuar su análisis sin necesidad de que el programador
anote los lifetime en esta firma de función.

Veamos otro ejemplo, esta vez usando la función `longest` que no tenía
parámetros de lifetime cuando comenzamos a trabajar con ella en el Listado
10-20:

```rust,ignore
fn longest(x: &str, y: &str) -> &str {
```

Aplicamos la primera regla: cada parámetro obtiene su propio lifetime. Esta vez
tenemos dos parámetros en lugar de uno, por lo que tenemos dos lifetimes:

```rust,ignore
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

Podemos ver que la segunda regla no se aplica porque hay más de un input 
lifetime. La tercera regla tampoco se aplica porque `longest` es una función
en lugar de un método, por lo que no hay un parámetro de `self`. Después de
trabajar a través de las tres reglas, todavía no hemos descubierto cuál es el
lifetime de retorno. Es por eso que obtuvimos un error al intentar compilar el
código en el Listado 10-20: el compilador trabajó a través de las reglas de
omisión de lifetime, pero aún no pudo descubrir todos los lifetime de las
referencias en la firma.

Dado que la tercera regla solo se aplica realmente en las firmas de los métodos,
veremos los lifetime en ese contexto a continuación para ver por qué la tercera
regla significa que no tenemos que anotar los lifetime en las firmas de los
métodos con mucha frecuencia.

### Anotaciones de lifetime en las definiciones de métodos

Cuando implementamos métodos en un struct con lifetimes, usamos la misma
sintaxis que la de los parámetros de tipo generic que se muestra en el Listado
10-11. Donde declaramos y usamos los parámetros de lifetime depende de si están
relacionados con los campos del struct o con los parámetros y valores de retorno
del método.

Los nombres de lifetime para los campos de una estructura siempre deben declararse
después de la palabra clave `impl` y luego usarse después del nombre del struct,
porque esos lifetime son parte del tipo del struct.

En las firmas de los métodos dentro del bloque `impl`, las referencias pueden
estar vinculadas a los lifetime de los campos del struct, o pueden ser
independientes. Además, las reglas de omisión de lifetime a menudo hacen que no
sean necesarias las anotaciones de lifetime en las firmas de los métodos. Veamos
algunos ejemplos usando el struct llamado `ImportantExcerpt` que definimos en el
Listado 10-24.

En primer lugar, usaremos un método llamado `level` cuyo parámetro es una 
referencia a `self`, y cuyo valor de retorno es un `i32`, que no es una
referencia a nada:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:1st}}
```

La declaración del parámetro de lifetime después de `impl` y su uso después del
nombre del struct son requeridos, pero no estamos obligados a anotar el lifetime
de la referencia a `self` porque se aplica la primera regla de omisión.

Aquí hay un ejemplo donde la tercera regla de omisión de lifetime se aplica:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:3rd}}
```

Hay dos input lifetimes, por lo que Rust aplica la primera regla de omisión de
lifetime y les da a `&self` y `announcement` sus propios lifetimes. Luego,
debido a que uno de los parámetros es `&self`, el tipo de retorno obtiene el
lifetime de `&self`, y todos los lifetimes han sido contabilizados.

### El lifetime static

Un lifetime especial que necesitamos discutir es `'static`, que denota que
la referencia afectada puede vivir durante toda la duración del programa. Todos
los string literals tienen el lifetime `'static`, que podemos anotar de la
siguiente manera:

```rust
let s: &'static str = "I have a static lifetime.";
```

El texto de este string se almacena directamente en el programa binario, que
siempre está disponible. Por lo tanto, la duración de todos los string literals
es `'static`.

Es posible que veas sugerencias para usar el lifetime `'static` en mensajes de
error. Pero antes de especificar `'static` como el lifetime para una referencia,
piensa si la referencia que tienes realmente vive durante toda la duración de tu
programa o no, y si quieres que lo haga. La mayoría de las veces, un mensaje de
error que sugiere el lifetime `'static` resulta de intentar crear una referencia
colgante o una falta de coincidencia de los lifetimes disponibles. En tales
casos, la solución es corregir esos problemas, no especificar el lifetime
`'static`.

## Parámetros de tipo generic, trait bounds y lifetimes juntos

¡Veamos brevemente la sintaxis de especificar parámetros de tipo generic, trait
bounds y lifetimes todo en una función!

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-11-generics-traits-and-lifetimes/src/main.rs:here}}
```

Esta es la función `longest` del Listado 10-21 que devuelve el string más largo
de dos string slices. Pero ahora tiene un parámetro adicional llamado `ann` del
tipo generic `T`, que puede llenarse con cualquier tipo que implemente el trait
`Display` como se especifica en la cláusula `where`. Este parámetro adicional
se imprimirá con `{}`, por lo que es necesario el trait bound `Display`. Debido
a que los lifetimes son un tipo de generic, las declaraciones del parámetro de
lifetime `'a` y el parámetro de tipo generic `T` van en la misma lista dentro
de los corchetes angulares después del nombre de la función.

## Resumen

¡Hemos cubierto mucho en este capítulo! Ahora que conoces los parámetros de
tipo generic, los traits y los trait bounds, y los parámetros de lifetime
generic, estás listo para escribir código sin repetición que funcione en muchas
situaciones diferentes. Los parámetros de tipo generic te permiten aplicar el
código a diferentes tipos. Los traits y los trait bounds garantizan que, aunque
los tipos son generic, tendrán el comportamiento que el código necesita.
Aprendiste cómo usar las anotaciones de lifetime para garantizar que este código
flexible no tendrá referencias colgantes. ¡Y todo este análisis ocurre en tiempo
de compilación, lo que no afecta el rendimiento en tiempo de ejecución!

Aunque no lo creas, hay mucho más que aprender sobre los temas que discutimos en
este capítulo: el Capítulo 17 discute los trait objects, que son otra forma de
usar traits. También hay escenarios más complejos que involucran anotaciones de
lifetime que solo necesitarás en escenarios muy avanzados; para esos, debes leer
la [Referencia de Rust][reference]. Pero a continuación, aprenderás cómo
escribir pruebas en Rust para que puedas asegurarte de que tu código funcione
como debería.

[references-and-borrowing]:
ch04-02-references-and-borrowing.html#references-and-borrowing
[string-slices-as-parameters]:
ch04-03-slices.html#string-slices-as-parameters
[reference]: https://doc.rust-lang.org/reference/index.html
