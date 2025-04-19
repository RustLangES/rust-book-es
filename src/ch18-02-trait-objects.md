## Usando Trait Objects que permiten valores de diferentes tipos

En el capítulo 8, mencionamos que una limitación de los vectores es que pueden
almacenar elementos de un solo tipo. Creamos una solución en el Listado 8-9
donde definimos un enum `SpreadsheetCell` que tenía variantes para almacenar
enteros, flotantes y texto. Esto significaba que podíamos almacenar diferentes
tipos de datos en cada celda y aun así tener un vector que representara una
fila de celdas. Esta es una solución perfectamente buena cuando nuestros
elementos intercambiables son un conjunto fijo de tipos que conocemos cuando
se compila nuestro código.

Sin embargo, a veces queremos que los usuarios de nuestra biblioteca puedan
ampliar el conjunto de tipos que pueden almacenar en una estructura de datos.
Para mostrar cómo podríamos lograr esto, crearemos una herramienta de
interfaz gráfica de usuario (GUI) de ejemplo que itera a través de una lista
de elementos, llamando a un método `draw` en cada uno para dibujarlo en la
pantalla, una técnica común para las herramientas de GUI. Crearemos una
caja de biblioteca llamada `gui` que contiene la estructura de una biblioteca
GUI. Esta caja podría incluir algunos tipos para que las personas los usen,
como `Button` o `TextField`. Además, los usuarios de `gui` querrán crear sus
propios tipos que se puedan dibujar: por ejemplo, un programador podría
agregar una `Image` y otro podría agregar un `SelectBox`.

No implementaremos una biblioteca GUI completamente desarrollada para este
ejemplo, pero mostraremos cómo encajarían las piezas. En el momento de
escribir la biblioteca, no podemos conocer y definir todos los tipos que
otros programadores podrían querer crear. Pero sí sabemos que `gui` necesita
hacer un seguimiento de muchos valores de diferentes tipos, y necesita llamar
a un método `draw` en cada uno de estos valores de diferentes tipos. No
necesita saber exactamente qué sucederá cuando llamemos al método `draw`, solo
que el valor tendrá ese método disponible para que lo llamemos.

Para hacer esto en un lenguaje con herencia, podríamos definir una clase
llamada `Component` que tenga un método llamado `draw` en ella. Las otras
clases, como `Button`, `Image` y `SelectBox`, heredarían de `Component` y,
por lo tanto, heredarían el método `draw`. Cada uno podría anular el método
`draw` para definir su comportamiento personalizado, pero el marco podría
tratar todos los tipos como si fueran instancias de `Component` y llamar a
`draw` en ellos. Pero como Rust no tiene herencia, necesitamos otra forma de
estructurar la biblioteca `gui` para permitir a los usuarios extenderla con
nuevos tipos.

### Definir un Trait para un comportamiento común

Para implementar el comportamiento que queremos que tenga `gui`, definiremos
un trait llamado `Draw` que tendrá un método llamado `draw`. Luego podemos
definir un vector que tome un _objeto de trait_. Un objeto de trait apunta
tanto a una instancia de un tipo que implementa nuestro trait especificado
como a una tabla utilizada para buscar métodos de trait en ese tipo en tiempo
de ejecución. Creamos un objeto de trait especificando algún tipo de puntero,
como una referencia `&` o un puntero inteligente `Box<T>`, luego la palabra
clave `dyn` y luego especificando el trait relevante. (Hablaremos sobre la
razón por la que los objetos de trait deben usar un puntero en el Capítulo 20
en la sección [“Tipos de tamaño dinámico y el
trait `Sized`.”][dynamically-sized]<!-- ignore -->) Podemos usar objetos de
trait en lugar de un tipo genérico o concreto. Donde sea que usemos un objeto
de trait, el sistema de tipos de Rust se asegurará en tiempo de compilación que
cualquier valor utilizado en ese contexto implemente el trait del objeto de
trait. En consecuencia, no necesitamos conocer todos los tipos posibles en
tiempo de compilación.

Hemos mencionado que, en Rust, nos abstenemos de llamar a los structs y enums
“objetos” para distinguirlos de los objetos de otros lenguajes. En un struct o
enum, los datos en los campos del struct y el comportamiento en los bloques
`impl` están separados, mientras que en otros lenguajes, los datos y el
comportamiento combinados en un solo concepto a menudo se etiquetan como un
objeto. Sin embargo, los objetos de trait son más como objetos en otros
lenguajes en el sentido de que combinan datos y comportamiento. Pero los
objetos de trait difieren de los objetos tradicionales en que no podemos
agregar datos a un objeto de trait. Los objetos de trait no son tan útiles en
general como los objetos en otros lenguajes: su propósito específico es
permitir la abstracción a través del comportamiento común.

El Listado 18-3 muestra cómo definir un trait llamado `Draw` con un método
llamado `draw`:

<Listing number="18-3" file-name="src/lib.rs" caption="Definición del trait `Draw`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-03/src/lib.rs}}
```

</Listing>

Esta sintaxis debería verse familiar de nuestras discusiones sobre cómo
definir traits en el Capítulo 10. A continuación viene una sintaxis nueva: el
Listado 18-4 define un struct llamado `Screen` que contiene un vector llamado
`components`. Este vector es de tipo `Box<dyn Draw>`, que es un objeto de
trait; es un sustituto de cualquier tipo dentro de una `Box` que implementa el
trait `Draw`.

<Listing number="18-4" file-name="src/lib.rs" caption="Definición del struct `Screen` con un campo `components` que contiene un vector de trait objects que implementan el trait `Draw`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-04/src/lib.rs:here}}
```

</Listing>

En el struct `Screen` hemos definido un método llamado `run` que llamará al
método `draw` en cada uno de sus `components`, como se muestra en el Listado
18-5:

<Listing number="18-5" file-name="src/lib.rs" caption="Un método `run` en `Screen` que llama al método `draw` en cada componente">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-05/src/lib.rs:here}}
```

</Listing>

Esto funciona de manera diferente a la definición de un struct que usa un
parámetro de tipo generic con trait bound. Un parámetro de tipo generic
solo se puede sustituir con un tipo concreto a la vez, mientras que los
trait objects permiten que varios tipos concretos llenen el trait object
en tiempo de ejecución. Por ejemplo, podríamos haber definido el struct
`Screen` usando un parámetro de tipo generic y un trait bound como en el
Listado 18-6:

<Listing number="18-6" file-name="src/lib.rs" caption="Una implementación alternativa del struct `Screen` y su método `run` usando generics y trait bounds">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-06/src/lib.rs:here}}
```

</Listing>

Esto nos restringe a una instancia de `Screen` que tiene una lista de
componentes de tipo `Button` o de tipo `TextField`. Si solo tendrá
colecciones homogéneas, usar generics y trait bounds es preferible porque las
definiciones se monomorfizarán en tiempo de compilación para usar los tipos
concretos.

Por otro lado, con el método que utiliza trait objects, una instancia de
`Screen` puede contener un `Vec<T>` que contiene una `Box<Button>` así como
una `Box<TextField>`. Veamos cómo funciona esto, y luego hablaremos sobre las
implicaciones de rendimiento en tiempo de ejecución.

### Implementando el trait

Ahora agregaremos algunos tipos que implementen el trait `Draw`.
Proporcionaremos el tipo `Button`. Nuevamente, implementar una biblioteca GUI
está más allá del alcance de este libro, por lo que el método `draw` no tendrá
ninguna implementación útil en su cuerpo. Para imaginar cómo podría ser la
implementación, un struct `Button` podría tener campos para `width`, `height`
y `label`, como se muestra en el Listado 18-7:

<Listing number="18-7" file-name="src/lib.rs" caption="Un `Button` que implementa el trait `Draw`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-07/src/lib.rs:here}}
```

</Listing>

Los campos `width`, `height` y `label` en `Button` serán diferentes de los
campos en otros componentes; por ejemplo, un tipo `TextField` podría tener
esos mismos campos más un campo `placeholder`. Cada uno de los tipos que
queremos dibujar en la pantalla implementará el trait `Draw` pero usará
código diferente en el método `draw` para definir cómo dibujar ese tipo
particular, como lo hace `Button` aquí (sin el código GUI real, como se
mencionó). El tipo `Button`, por ejemplo, podría tener un bloque `impl`
adicional que contenga métodos relacionados con lo que sucede cuando un
usuario hace clic en el botón. Este tipo de métodos no se aplicarán a tipos
como `TextField`.

Si alguien que utiliza nuestra biblioteca decide implementar un struct
`SelectBox` que tiene campos `width`, `height` y `options`, también
implementará el trait `Draw` en el tipo `SelectBox`, como se muestra en el
Listado 18-8:

<Listing number="18-8" file-name="src/main.rs" caption="Otro crate usando `gui` e implementando el trait `Draw` en un struct `SelectBox`">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-08/src/main.rs:here}}
```

</Listing>

El usuario de nuestra biblioteca ahora puede escribir su función `main` para
crear una instancia de `Screen`. A la instancia de `Screen`, pueden agregar
un `SelectBox` y un `Button` colocando cada uno en una `Box<T>` para
convertirse en un trait object. Luego pueden llamar al método `run` en la
instancia de `Screen`, que llamará a `draw` en cada uno de los componentes.
El Listado 18-9 muestra esta implementación:

<Listing number="18-9" file-name="src/main.rs" caption="Usando trait objects para almacenar valores de diferentes tipos que implementan el mismo trait">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-09/src/main.rs:here}}
```

</Listing>

Cuando escribimos la biblioteca, no sabíamos que alguien podría agregar el tipo
`SelectBox`, pero nuestra implementación de `Screen` pudo operar en el nuevo
tipo y dibujarlo porque `SelectBox` implementa el trait `Draw`, lo que significa
que implementa el método `draw`.

Este concepto, de preocuparnos solo por los mensajes a los que responde un valor
en lugar del tipo concreto del valor, es similar al concepto de _duck typing_ en
lenguajes de tipado dinámico: si camina como un pato y grazna como un pato,
¡entonces debe ser un pato! En la implementación de `run` en `Screen` en el
Listado 18-5, `run` no necesita saber cuál es el tipo concreto de cada
componente. No verifica si un componente es una instancia de un `Button` o de
un `SelectBox`, simplemente llama al método `draw` en el componente. Al
especificar `Box<dyn Draw>` como el tipo de los valores en el vector
`components`, hemos definido que `Screen` necesita valores a los que podamos
llamar el método `draw`.

La ventaja de utilizar trait objects y el sistema de tipos de Rust para escribir
código similar al código que utiliza duck typing es que nunca tenemos que
verificar si un valor implementa un método en particular en tiempo de ejecución
o preocuparnos por obtener errores si un valor no implementa un método, pero lo
llamamos de todos modos. Rust no compilará nuestro código si los valores no
implementan los traits que necesitan los trait objects.

Por ejemplo, el Listado 18-10 muestra lo que sucede si intentamos crear una
`Screen` con un `String` como componente:

<Listing number="18-10" file-name="src/main.rs" caption="Intentando utilizar un tipo que no implementa the trait del trait object">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-oop/listing-18-10/src/main.rs}}
```

</Listing>

Obtendremos este error porque `String` no implementa el trait `Draw`:

```console
{{#include ../listings/ch18-oop/listing-18-10/output.txt}}
```

Este error nos indica que o bien estamos pasando algo a `Screen` que no
queríamos pasar y, por lo tanto, deberíamos pasar un tipo diferente o deberíamos
implementar `Draw` en `String` para que `Screen` pueda llamar a `draw` en él.

### Los trait objects realizan _dynamic dispatch_

Recuerda que en la sección [“Performance of Code Using
Generics”][rendimiento-de-codigo-usando-genericos]<!-- ignore --> del Capítulo 10
hablamos sobre el proceso de monomorfización que realiza el compilador cuando
usamos _trait bounds_ en los genéricos: el compilador genera implementaciones
no genéricas de funciones y métodos para cada tipo concreto que usamos en lugar
de un parámetro de tipo genérico. El código que resulta de la monomorfización
está realizando _static dispatch_, que es cuando el compilador sabe qué método
estás llamando en tiempo de compilación. Esto se opone al _dynamic dispatch_,
que es cuando el compilador no puede decir en tiempo de compilación qué método
estás llamando. En los casos de dynamic dispatch, el compilador emite código que
en tiempo de ejecución determinará qué método llamar.

Cuando usamos trait objects, Rust debe usar dynamic dispatch. El compilador no
conoce todos los tipos que podrían usarse con el código que está llamando a
trait objects, por lo que no sabe qué método implementado en qué tipo llamar. En
cambio, en tiempo de ejecución, Rust usa los punteros dentro del trait object
para saber qué método llamar. Esta búsqueda incurre en un costo de tiempo de
ejecución que no ocurre con el static dispatch. Dynamic dispatch también evita
que el compilador elija la opción de _inline_ del código de un método, lo que a
su vez evita algunas optimizaciones, además, Rust tiene ciertas reglas sobre 
dónde se puede y dónde no se puede usar la distribución dinámica, llamadas [_dyn
compatibility_][dyn-compatibility]. Sin embargo, obtuvimos flexibilidad 
adicional en el código que escribimos en el Listado 18-5 y pudimos admitir en el 
Listado 18-9, por lo que es un compromiso a considerar.

[rendimiento-de-codigo-usando-genericos]: ch10-01-syntax.html#rendimiento-de-codigo-usando-genericos
[dynamically-sized]: ch20-04-advanced-types.html#tipos-de-tamano-dinamico-y-el-trait-sized
[dyn-compatibility]: https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility

