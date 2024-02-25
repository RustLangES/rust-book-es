## Almacenando listas de valores con vectores

El primer tipo de colección que veremos es `Vec<T>`, también conocido como un
_vector_. Los vectores te permiten almacenar más de un valor en una sola
estructura de datos que pone todos los valores uno al lado del otro en la
memoria. Los vectores solo pueden almacenar valores del mismo tipo. Son útiles
cuando tienes una lista de elementos, como las líneas de texto en un archivo o
los precios de los artículos en un carrito de compras.

### Creando un nuevo vector

Para crear un nuevo vector vacío, llamamos a la función `Vec::new`, como se
muestra en el Listing 8-1.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-01/src/main.rs:here}}
```

<span class="caption">Listing 8-1: Creando un nuevo vector vacío para mantener
valores de tipo `i32`</span>

Ten en cuenta que agregamos una anotación de tipo aquí. Como no estamos
insertando ningún valor en este vector, Rust no sabe qué tipo de elementos
queremos almacenar. Este es un punto importante. Los vectores se implementan
usando genéricos; cubriremos cómo usar genéricos con tus propios tipos en el
Capítulo 10. Por ahora, sepa que el tipo `Vec<T>` proporcionado por la
biblioteca estándar puede contener cualquier tipo. Cuando creamos un vector
para contener un tipo específico, podemos especificar el tipo dentro de
corchetes angulares. En el Listing 8-1, le hemos dicho a Rust que el `Vec<T>`
en `v` contendrá elementos del tipo `i32`.

A menudo, crearás un `Vec<T>` con valores iniciales y Rust inferirá el tipo de
valor que deseas almacenar, por lo que rara vez necesitarás hacer esta
anotación de tipo. Rust proporciona convenientemente la macro `vec!`, que
creará un nuevo vector que contenga los valores que le des. El Listing 8-2
crea un nuevo `Vec<i32>` que contiene los valores `1`, `2` y `3`. El tipo
entero es `i32` porque ese es el tipo entero predeterminado, como discutimos
en la sección ["Tipos de datos"][data-types]<!-- ignore --> del Capítulo 3.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-02/src/main.rs:here}}
```

<span class="caption">Listing 8-2: Creando un nuevo vector que contiene
valores</span>

Debido a que hemos dado valores iniciales `i32`, Rust puede inferir que el tipo
de `v` es `Vec<i32>`, y la anotación de tipo no es necesaria. A continuación,
veremos cómo modificar un vector.

### Actualizando un vector

Para crear un vector y luego agregar elementos a él, podemos usar el método
`push`, como se muestra en el Listing 8-3.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-03/src/main.rs:here}}
```

<span class="caption">Listing 8-3: Usando el método `push` para añadir valores
a un vector</span>

Como con cualquier variable, si queremos poder cambiar su valor, necesitamos
hacerlo mutable usando la palabra clave `mut`, como se discutió en el Capítulo
3\. Los números que colocamos dentro son todos del tipo `i32`, y Rust infiere
esto de los datos, por lo que no necesitamos la anotación `Vec<i32>`.

### Leyendo elementos de vectores

Hay dos formas de hacer referencia a un valor almacenado en un vector: a través
de la indexación o usando el método `get`. En los siguientes ejemplos,
hemos anotado los tipos de los valores que se devuelven de estas funciones para
obtener una mayor claridad.

En el Listing 8-4 se muestran ambos métodos de acceso a un valor en un vector,
con sintaxis de indexación y el método `get`.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-04/src/main.rs:here}}
```

<span class="caption">Listing 8-4: Usando la sintaxis de indexación o el método
`get` accediendo a un objeto en un vector</span>

Ten en cuenta algunos detalles aquí. Usamos el valor de índice `2` para obtener
el tercer elemento porque los vectores se indexan por número, comenzando en
cero. Usar `&` y `[]` nos da una referencia al elemento en el índice. Cuando
usamos el método `get` con el índice pasado como argumento, obtenemos un
`Option<&T>` que podemos usar con `match`.

La razón por la que Rust proporciona estas dos formas de hacer referencia a un
elemento es para que puedas elegir cómo se comporta el programa cuando intentas
usar un valor de índice fuera del rango de elementos existentes. Como ejemplo,
veamos qué sucede cuando tenemos un vector de cinco elementos y luego intentamos
acceder a un elemento en el índice 100 con cada técnica, como se muestra en el
Listing 8-5.

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-05/src/main.rs:here}}
```

<span class="caption">Listing 8-5: Intentando acceder al elemento en el índice
100 en un vector que contiene 5 elementos</span>

Cuando ejecutamos este código, el primer método `[]` causará que el programa
falle porque intenta acceder a un elemento que no existe. Este método es mejor
usarlo cuando quieres que tu programa se bloquee si hay un intento de acceder a
un elemento más allá del final del vector.

Cuando el método `get` se pasa un índice que está fuera del rango del vector,
simplemente devuelve `None` sin entrar en pánico. Tendrías que usar este método
si acceder a un elemento más allá del rango del vector puede suceder con
frecuencia en circunstancias normales. Tu código tendrá lógica para manejar
tener `Some(&element)` o `None`, como se discutió en el Capítulo 6. Por
ejemplo, el índice podría provenir de una persona que ingresa un número. Si
ingresan accidentalmente un número que es demasiado grande y el programa obtiene
un valor `None`, podrías decirle al usuario cuántos elementos hay en el vector
actual y darle otra oportunidad de ingresar un valor válido. Eso sería más
amigable para el usuario que bloquear el programa debido a un error tipográfico.

Cuando el programa tiene una referencia válida, el borrow checker hace cumplir
las reglas de ownership y borrowing (cubiertas en el Capítulo 4) para asegurar
que esta referencia y cualquier otra referencia a los contenidos del vector
permanezcan válidas. Recuerda la regla que establece que no puedes tener
referencias mutables e inmutables en el mismo ámbito. Esa regla se aplica en el
Listing 8-6, donde tenemos una referencia inmutable al primer elemento en un
vector e intentamos agregar un elemento al final. Este programa no funcionará si
también intentamos referirnos a ese elemento más adelante en la función:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-06/src/main.rs:here}}
```

<span class="caption">Listing 8-6: Intentando agregar un elemento a un vector
mientras se mantiene una referencia a un elemento</span>

Compiling this code will result in this error:

```console
{{#include ../listings/ch08-common-collections/listing-08-06/output.txt}}
```

El código en el Listing 8-6 podría parecer que debería funcionar: ¿por qué una
referencia al primer elemento se preocuparía por los cambios al final del
vector? Este error se debe a la forma en que funcionan los vectores: porque los
vectores colocan los valores uno al lado del otro en la memoria, agregar un
nuevo elemento al final del vector puede requerir asignar nueva memoria y
copiar los elementos antiguos al nuevo espacio, si no hay suficiente espacio
para poner todos los elementos uno al lado del otro donde se almacena el vector
actualmente. En ese caso, la referencia al primer elemento apuntaría a la
memoria desasignada. Las reglas de borrowing evitan que los programas terminen en
esa situación.

> Note: For more on the implementation details of the `Vec<T>` type, see [“The
> Rustonomicon”][nomicon].

### Iterando sobre los valores en un vector

Para acceder a cada elemento en un vector a su vez, iteramos a través de todos
los elementos, en lugar de usar índices para acceder a uno a la vez. El Listing
8-7 muestra cómo usar un bucle `for` para obtener referencias inmutables a cada
elemento en un vector de valores `i32` e imprimirlos.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-07/src/main.rs:here}}
```

<span class="caption">Listing 8-7: Imprimiendo cada elemento en un vector
iterando sobre los elementos usando un ciclo `for`</span>

También podemos iterar sobre referencias mutables a cada elemento en un vector
mutable, lo que nos permite cambiar los valores en un vector en el lugar. El
código en el Listing 8-8 agregará `50` a cada elemento en un vector.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-08/src/main.rs:here}}
```

<span class="caption">Listing 8-8: Iterando sobre referencias mutables a
elementos en un vector</span>

Para cambiar el valor al que se refiere la referencia mutable, tenemos que usar
el operador de desreferencia `*` para llegar al valor en `i` antes de poder
usar el operador `+=`. Hablaremos más sobre el operador de desreferencia en la
sección [“Siguiendo el puntero al valor con el operador de
desreferencia”][deref]<!-- ignore --> del Capítulo 15.

Iterando sobre un vector, ya sea inmutable o mutable, es seguro debido a las
reglas del borrow checker. Si intentáramos insertar o eliminar elementos en los
cuerpos del ciclo `for` en el Listing 8-7 y el Listing 8-8, obtendríamos un
error del compilador similar al que obtuvimos con el código en el Listing 8-6.
La referencia al vector que el ciclo `for` contiene evita la modificación
simultánea de todo el vector.

### Usar un `enum` para almacenar múltiples tipos

Los vectores solo pueden almacenar valores del mismo tipo. Esto puede ser
inconveniente; definitivamente hay casos de uso para necesitar almacenar una
lista de elementos de diferentes tipos. Afortunadamente, las variantes de un
`enum` se definen bajo el mismo tipo de `enum`, por lo que cuando necesitamos
que un tipo represente elementos de diferentes tipos, ¡podemos definir y usar un
`enum`!

Por ejemplo, digamos que queremos almacenar en una lista los elementos de una
tabla de hoja de cálculo: algunas columnas pueden contener números, y otras
cadenas de texto. Podemos definir un `enum` cuyas variantes contendrán los
diferentes tipos de datos, y todas las variantes se considerarán del mismo tipo:
el del `enum`. Luego podemos crear un vector para contener ese `enum` y, por lo
tanto, en última instancia, contener diferentes tipos. Hemos demostrado esto en
el Listing 8-9.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-09/src/main.rs:here}}
```

<span class="caption">Listing 8-9: Definiendo un `enum` para almacenar valores
de diferentes tipos en un vector</span>

Rust necesita saber qué tipos habrá en el vector en tiempo de compilación para
saber exactamente cuánta memoria en el montón se necesitará para almacenar cada
elemento. También debemos ser explícitos sobre qué tipos están permitidos en
este vector. Si Rust permitiera que un vector contenga cualquier tipo, existiría
la posibilidad de que uno o más de los tipos causaran errores con las
operaciones realizadas en los elementos del vector. Usar un `enum` más una
expresión `match` significa que Rust se asegurará en tiempo de compilación de
que se maneje cada caso posible, como se discutió en el Capítulo 6.

Si tu no sabes el conjunto exhaustivo de tipos que un programa obtendrá en
tiempo de ejecución para almacenar en un vector, la técnica de `enum` no
funcionará. En su lugar, puede usar un objeto de rasgo, que cubriremos en el
Capítulo 17.

Ahora que hemos discutido algunas de las formas más comunes de usar vectores,
asegúrese de revisar [la documentación de la API][vec-api]<!-- ignore --> para
todos los muchos métodos útiles definidos en `Vec<T>` por la biblioteca
estándar. Por ejemplo, además de `push`, un método `pop` elimina y devuelve el
último elemento.

### Liberar un vector libera sus elementos

Como cualquier otro `struct`, un vector se libera cuando sale del ámbito, como
se anota en el Listing 8-10.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-10/src/main.rs:here}}
```

<span class="caption">Listing 8-10: Mostrando dónde se colocan el vector y sus
elementos</span>

Cuando se libera el vector, también se libera todo su contenido, lo que
significa que se limpiarán los enteros que contiene. El borrow checker garantiza
que cualquier referencia al contenido de un vector solo se utilice mientras el
vector en sí sea válido.

Pasemos al siguiente tipo de colección: ¡`String`!

[data-types]: ch03-02-data-types.html#tipos-de-datos
[nomicon]: https://doc.rust-lang.org/nomicon/vec/vec.html
[vec-api]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[deref]: ch15-02-deref.html#following-the-pointer-to-the-value-with-the-dereference-operator
