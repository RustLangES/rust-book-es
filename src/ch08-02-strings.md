## Almacenando texto codificado en UTF-8 con Strings

Hemos hablado de strings en el Capítulo 4, pero las veremos con más detalle
Los nuevos Rustaceans suelen quedarse atascados en las cadenas por una
combinación de tres razones: la propensión de Rust a exponer posibles errores,
los strings son una estructura de datos más complicada de lo que muchos
programadores le dan crédito, y UTF-8. Estos factores se combinan de una manera
que puede parecer difícil cuando se viene de otros lenguajes de programación.

Discutiremos strings en el contexto de las colecciones porque las strings se
implementan como una colección de bytes, más algunos métodos para proporcionar
funcionalidad útil cuando esos bytes se interpretan como texto. En esta
sección, hablaremos sobre las operaciones en `String` que cada tipo de
colección tiene, como crear, actualizar y leer. También discutiremos las
formas en que `String` es diferente de las otras colecciones, es decir, cómo
indexar en un `String` se complica por las diferencias entre cómo las personas
y las computadoras interpretan los datos de `String`.

### ¿Qué es un string?

Bien primero definamos lo que queremos decir con el término _string_. Rust solo
tiene un tipo de string en el lenguaje principal, que es el string slice `str`
que generalmente se ve en su forma prestada `&str`. En el Capítulo 4, hablamos
sobre _string slices_, que son referencias a algunos datos de cadena codificados
en UTF-8 almacenados en otro lugar. Las literales de cadena, por ejemplo, se
almacenan en el binario del programa y, por lo tanto, son trozos de cadena.

El tipo `String`, que es proporcionado por la biblioteca estándar en lugar de
codificado en el lenguaje principal, es un tipo de cadena que puede crecer, mutable,
de propiedad, codificado en UTF-8. Cuando los Rustaceans se refieren a "strings" en Rust,
pueden estar refiriéndose a cualquiera de los tipos `String` o `str`, no solo
a uno de esos tipos. Aunque esta sección trata principalmente de `String`, ambos
tipos se usan mucho en la biblioteca estándar de Rust, y tanto `String` como
las rebanadas de cadena son codificadas en UTF-8.

### Creando un nuevo String

Muchas de las mismas operaciones disponibles con `Vec<T>` también están
disponibles con `String`, ya que `String` se implementa en realidad como un
envoltorio alrededor de un vector de bytes con algunas garantías, restricciones
y capacidades adicionales. Un ejemplo de una función que funciona de la misma
manera con `Vec<T>` y `String` es la función `new` para crear una instancia,
que se muestra en el listado 8-11.

<Listing number="8-11" caption="Creando un nuevo y vacío `String`">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-11/src/main.rs:here}}
```

</Listing>

Esta línea crea un nuevo `String` vacío llamado `s`, el cual podemos luego cargar
con datos. A menudo, tendremos algunos datos iniciales que queremos comenzar
en el string. Para eso, usamos el método `to_string`, que está disponible en
cualquier tipo que implemente el trait `Display`, como lo hacen los String
Literals. El listado 8-12 muestra dos ejemplos.

<Listing number="8-12" caption="Usando el método `to_string` para crear un `String` a partir de un string literal">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-12/src/main.rs:here}}
```

</Listing>

Este código crea un string que contiene `initial contents`.

Podemos también usar la función `String::from` para crear un `String` a partir
de un string literal. El código en el listado 8-13 es equivalente al código del
listado 8-12 que usa `to_string`.

<Listing number="8-13" caption="Usando la función `String::from` para crear un `String` a partir de un string literal">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-13/src/main.rs:here}}
```

</Listing>

Debido a que los strings se usan para muchas cosas, podemos usar muchas APIs
genéricas diferentes para strings, lo que nos proporciona muchas opciones.
Algunos de ellos pueden parecer redundantes, ¡pero todos tienen su lugar! En
este caso, `String::from` y `to_string` hacen lo mismo, por lo que elegir
depende del estilo y la legibilidad.

Recuerda que los strings son UTF-8 codificados, por lo que podemos incluir
cualquier dato codificado correctamente en ellos, Como se muestra en el listado
8-14.

<Listing number="8-14" caption="Almacenamiento de saludos en diferentes idiomas en strings">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:here}}
```

</Listing>

Todos estos strings son valores válidos de `String`.

### Actualizando un String

Un `String` puede crecer en tamaño y su contenido puede cambiar, al igual que
el contenido de un `Vec<T>`, si se introducen más datos en el. Además, puedes usar
convenientemente el operador `+` o el macro `format!` para concatenar valores de
`String`.

#### Agregando a un String con `push_str` y `push`

Podemos hacer crecer un `String` usando el método `push_str` para agregar un
string slice, como se muestra en el listado 8-15.

<Listing number="8-15" caption="Agregando un string slice a un `String` usando el método `push_str`">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-15/src/main.rs:here}}
```

</Listing>

Después de estas dos líneas, `s` contendrá `foobar`. El método `push_str` toma
un string slice porque no necesariamente queremos tomar posesión del parámetro.
Por ejemplo, en el código del listado 8-16, queremos poder usar `s2` después de
agregar su contenido a `s1`.

<Listing number="8-16" caption="Uso de un string slice después de agregar su contenido a un `String`">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-16/src/main.rs:here}}
```

</Listing>

Si el método `push_str` tomara posesión de `s2`, no podríamos imprimir su valor
en la última línea. ¡Sin embargo, este código funciona como esperamos!

El método `push` toma un solo carácter como parámetro y lo agrega al `String`.
El listado 8-17 agrega la letra `l` a un `String` usando el método `push`.

<Listing number="8-17" caption="Agregando un carácter a un valor `String` usando `push`">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-17/src/main.rs:here}}
```

</Listing>

Como resultado, `s` contendrá `lol`.

#### Concatenacion con el operador `+` o la Macro `format!`

A menudo, querrás combinar dos cadenas existentes. Una forma de hacerlo es 
usar el operador `+`, como se muestra en el Listado 8-18.

<Listing number="8-18" caption="Usando el operador `+` para combinar dos valores `String` en un nuevo valor `String`">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-18/src/main.rs:here}}
```

</Listing>

El string `s3` contendrá `Hello, world!`. La razón por la que `s1` ya no es
válido después de la adición, y la razón por la que usamos una referencia a
`s2`, tiene que ver con la firma del método que se llama cuando usamos el
operador `+`. El operador `+` usa el método `add`, cuya firma se ve algo como
esto:

```rust,ignore
fn add(self, s: &str) -> String {
```

En la biblioteca estándar, verás `add` definido usando genéricos y tipos
asociados. Aquí, hemos sustituido tipos concretos, que es lo que sucede cuando
llamamos a este método con valores `String`. Discutiremos los genéricos en el
Capítulo 10. Esta firma nos da las pistas que necesitamos para entender las
partes complicadas del operador `+`.

Primero, `s2` tiene un `&`, lo que significa que estamos agregando una referencia
del segundo string al primer string. Esto se debe al parámetro `s` en la
función `add`: solo podemos agregar un `&str` a un `String`; no podemos agregar
dos valores `String` juntos. Pero espera, el tipo de `&s2` es `&String`, no
`&str`, como se especifica en el segundo parámetro de `add`. ¿Entonces por qué
compila el listado 8-18?

La razón por la que podemos usar `s2` en la llamada a `add` es que el
compilador puede _convertir_ el argumento `&String` en un `&str`. Cuando  
llamamos al método `add`, Rust usa una _coerción de dereferencia_, que aquí
convierte `&s2` en `&s2[..]`. Discutiremos la coerción de dereferencia con más
detalle en el Capítulo 15. Debido a que `add` no toma posesión del parámetro
`s`, `s2` seguirá siendo un `String` válido después de esta operación.

En segundo lugar, podemos ver en la firma que `add` toma el ownership de `self`,
porque `self` no tiene un `&`. Esto significa que `s1` en el listado 8-18 se
moverá a la llamada de `add` y ya no será válido después de eso. Entonces,
aunque `let s3 = s1 + &s2;` parece que copiará ambos strings y creará uno
nuevo, esta declaración realmente toma posesión de `s1`, agrega una copia del
contenido de `s2` y luego devuelve la propiedad del resultado. En otras
palabras, parece que está haciendo muchas copias, pero no lo está; la
implementación es más eficiente que copiar.

Si necesitamos concatenar múltiples strings, el comportamiento del operador `+`
se vuelve difícil de manejar:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-01-concat-multiple-strings/src/main.rs:here}}
```

En este punto, `s` contendrá `tic-tac-toe`. Con todos los caracteres `+` y `"`
es difícil ver qué está pasando. Para una combinación de cadenas más
complicada, podemos usar la macro `format!`:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-02-format/src/main.rs:here}}
```

Este código también establece `s` en `tic-tac-toe`. La macro `format!` funciona
como `println!`, pero en lugar de imprimir la salida en la pantalla, devuelve
un `String` con el contenido. La versión del código que usa `format!` es mucho
más fácil de leer, y el código generado por la macro `format!` usa referencias
para que esta llamada no tome posesión de ninguno de sus parámetros.

### Indexando en Strings

En muchos otros lenguajes de programación, acceder a caracteres individuales en
un string referenciándolos por índice es una operación válida y común. Sin
embargo, si intentas acceder a partes de un `String` usando la sintaxis de
indexación en Rust, obtendrás un error. Considera el código inválido en el
listado 8-19.

<Listing number="8-19" caption="Intentando usar la sintaxis de indexación con un String">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-19/src/main.rs:here}}
```

</Listing>

Este código dará como resultado el siguiente error:

```console
{{#include ../listings/ch08-common-collections/listing-08-19/output.txt}}
```

El error y la nota cuentan la historia: los strings de Rust no admiten
indexación. Pero, ¿por qué no? Para responder a esa pregunta, necesitamos
discutir cómo Rust almacena los strings en la memoria.

#### Representación Interna

Un `String` es un wrapper sobre un `Vec<u8>`. Veamos algunos de nuestros
strings de ejemplo UTF-8 correctamente codificados del listado 8-14. Primero,
este:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:spanish}}
```

En este caso, `len` será `4`, lo que significa que el vector que almacena el
string `“Hola”` tiene 4 bytes de largo. Cada una de estas letras toma un byte
cuando se codifica en UTF-8. La siguiente línea, sin embargo, puede
sorprenderte. (Nota que este string comienza con la letra cirílica *Ze* mayúscula,
no con el número árabe 3.)

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:russian}}
```

Si tu te preguntas que tan largo es el string, podrías decir 12. De hecho, la
respuesta de Rust es 24: ese es el número de bytes que se necesitan para
codificar “Здравствуйте” en UTF-8, porque cada valor escalar Unicode en ese
string toma 2 bytes de almacenamiento. Por lo tanto, un índice en los bytes del
string no siempre se correlacionará con un valor escalar Unicode válido. Para
demostrarlo, considera este código inválido de Rust:

```rust,ignore,does_not_compile
let hello = "Здравствуйте";
let answer = &hello[0];
```

Tu Ahora sabes que `answer` no será `З`, la primera letra. Cuando codificado
en UTF-8, el primer byte de `З` es `208` y el segundo es `151`, por lo que
parecería que `answer` debería ser `208`, pero `208` no es un carácter válido
por sí solo. Devolver `208` probablemente no sea lo que un usuario querría si
pidieran la primera letra de esta cadena; sin embargo, esos son los únicos
datos que Rust tiene en el índice de bytes 0. Los usuarios generalmente no
quieren que se devuelva el valor de byte, incluso si la cadena contiene solo
letras latinas: si `&"hi"[0]` fuera un código válido que devolviera el valor
de byte, devolvería `104`, no `h`.

La respuesta, entonces, es que para evitar devolver un valor inesperado y
causar errores que podrían no descubrirse de inmediato, Rust no compila este
código en absoluto y evita malentendidos al comienzo del proceso de
desarrollo.

#### Bytes, valores escalares y grupos de grafemas

Otro punto sobre UTF-8 es que hay tres formas relevantes de ver las cadenas
desde la perspectiva de Rust: como bytes, valores escalares y grupos de
grafemas (lo más parecido a lo que llamaríamos _letras_).

Si observamos la palabra “नमस्ते” en escritura Devanagari, se almacena como un
vector de valores `u8` que se ve así:

```text
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

Eso es 18 bytes y es como las computadoras almacenan los datos. Si los
observamos como valores escalares Unicode, que es lo que es el tipo `char` de
Rust, esos bytes se ven así:

```text
['न', 'म', 'स', '्', 'त', 'े']
```

Aquí hay seis valores `char`, pero el cuarto y el sexto no son letras: son
diacríticos que no tienen sentido por sí mismos. Finalmente, si los miramos
como grupos de grafemas, obtendríamos lo que una persona llamaría las cuatro
letras que componen la palabra hindi:

```text
["न", "म", "स्", "ते"]
```

Rust proporciona diferentes formas de interpretar los datos de string sin
procesar que las computadoras almacenan para que cada programa pueda elegir la
interpretación que necesita, sin importar en qué idioma humano estén los datos.

Una última razón por la que Rust no permite indexar en un `String` para obtener
un carácter es que se espera que las operaciones de indexación siempre tomen
tiempo constante (O(1)). Pero no es posible garantizar ese rendimiento con un
`String`, porque Rust tendría que recorrer el contenido desde el principio
hasta el índice para determinar cuántos caracteres válidos había.

### Slicing Strings

La indexación en un `String` suele ser una mala idea porque no está claro cuál
debería ser el tipo de retorno de la operación de indexación de string: un
valor de byte, un carácter, un grupo de grafemas o una rebanada de string. Si
realmente necesita usar índices para crear rebanadas de string, por lo tanto,
Rust le pide que sea más específico.

En lugar de indexar usando `[]` con un solo número, puede usar `[]` con un
rango para crear un string slice conteniendo bytes particulares:

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

Aquí, `s` será un `&str` que contiene los primeros cuatro bytes del string. Antes,
mencionamos que cada uno de estos caracteres era de dos bytes, lo que significa
que `s` será `Зд`.

Si intentáramos hacer un slice con solo una parte de los bytes de un carácter,
algo como `&hello[0..1]`, Rust entraría en pánico en tiempo de
ejecución de la misma manera que si se accediera a un índice no válido
en un vector:

```console
{{#include ../listings/ch08-common-collections/output-only-01-not-char-boundary/output.txt}}
```

Debemos tener cuidado cuando creamos string slices, porque hacerlo puede 
bloquear su programa.

### Métodos para iterar sobre Strings

La mejor manera de operar en partes de strings es ser explícito sobre si
desea caracteres o bytes. Para valores escalares Unicode individuales, use el
método `chars`. Llamar a `chars` en “Зд” separa y devuelve dos valores de tipo
`char`, y puede iterar sobre el resultado para acceder a cada elemento:

```rust
for c in "Зд".chars() {
    println!("{c}");
}
```

Este código imprimirá lo siguiente:

```text
З
д
```

Alternativamente, el método `bytes` devuelve cada byte sin procesar, que puede
ser apropiado para su dominio:

```rust
for b in "Зд".bytes() {
    println!("{b}");
}
```

Este código imprimirá los cuatro bytes que componen el string:

```text
208
151
208
180
```

Pero asegúrate de recordar que los valores escalares de Unicode válidos pueden 
estar compuestos por más de un byte.

Obtener grupos de grafemas a partir de cadenas, como en el caso del alfabeto 
Devanagari, es complejo, por lo que esta funcionalidad no está proporcionada por
la biblioteca estándar. Hay paquetes disponibles en 
[crates.io](https://crates.io/)<!-- ignore --> si necesitas esta funcionalidad.

### Los Strings no son tan simples

Para resumir, los strings son complicados. Los diferentes lenguajes de
programación hacen diferentes elecciones sobre cómo presentar esta complejidad
al programador. Rust ha elegido hacer que el manejo correcto de los datos
`String` sea el comportamiento predeterminado para todos los programas de Rust,
lo que significa que los programadores tienen que pensar más en el manejo de
datos UTF-8 por adelantado. Este compromiso expone más de la complejidad de
las cadenas de lo que parece en otros lenguajes de programación, pero evita
que tenga que manejar errores que involucran caracteres no ASCII más adelante
en su ciclo de vida de desarrollo.

La buena noticia es que la biblioteca estándar ofrece mucha funcionalidad
construida a partir de los tipos `String` y `&str` para ayudar a manejar estas
situaciones complejas correctamente. Asegúrese de consultar la documentación
para obtener métodos útiles como `contains` para buscar en un string y
`replace` para sustituir partes de un string por otro string.

Pasemos a algo un poco menos complejo: ¡Hash Maps!
