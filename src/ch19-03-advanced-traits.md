## Traits Avanzados

Primero cubrimos los traits en la sección ["Traits: Defining Shared
Behavior"][traits-defining-shared-behavior]<!-- ignore --> del Capítulo 10, pero
no discutimos los detalles más avanzados. Ahora que conoces más Rust, podemos
entrar en los detalles más minuciosos.

### Especificando Tipos de Marcador en Definiciones de Traits con Tipos Asociados

Los *tipos asociados* conectan un marcador de tipo con un trait de modo que los
métodos de definición de trait puedan usar estos marcadores de tipo en sus
firmas. El implementador de un trait especificará el tipo concreto que se
utilizará en lugar del tipo de marcador para la implementación particular. De
esa manera, podemos definir un trait que use algunos tipos sin necesidad de
saber exactamente cuáles son esos tipos hasta que se implemente el trait.

Hemos descrito la mayoría de las características avanzadas en este capítulo
como poco necesarias. Los tipos asociados están en algún lugar en el medio: se
utilizan con menos frecuencia que las características explicadas en el resto
del libro, pero con más frecuencia que muchas de las otras características
discutidas en este capítulo.

Un ejemplo de un trait con un tipo asociado es el trait `Iterator` que la
biblioteca estándar proporciona. El tipo asociado se llama `Item` y representa
el tipo de los valores que el tipo que implementa el trait `Iterator` está
iterando. La definición del trait `Iterator` es como se muestra en el Listado
19-12.

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-12/src/lib.rs}}
```

<span class="caption">Listing 19-12: La definición del trait `Iterator` que
tiene un tipo asociado `Item`</span>

El tipo `Item` es un marcador de tipo, y la definición del método `next` muestra
que devolverá valores del tipo `Option<Self::Item>`. Los implementadores del
trait `Iterator` especificarán el tipo concreto para `Item`, y el método `next`
devolverá una `Option` que contiene un valor de ese tipo concreto.

Los tipos asociados pueden parecer un concepto similar a los generics, ya que
estos últimos nos permiten definir una función sin especificar qué tipos puede
manejar. Para examinar la diferencia entre los dos conceptos, veremos una
implementación del trait `Iterator` en un tipo llamado `Counter` que especifica
que el tipo `Item` es `u32`:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-22-iterator-on-counter/src/lib.rs:ch19}}
```

Esta sintaxis parece comparable a la de los generics. Entonces, ¿por qué no
definir simplemente el trait `Iterator` con generics, como se muestra en el
Listado 19-13?

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-13/src/lib.rs}}
```

<span class="caption">Listing 19-13: Una definición hipotética del trait
`Iterator` usando generics</span>

La diferencia es que cuando usamos generics, como en el Listado 19-13, debemos
anotar los tipos en cada implementación; porque también podemos implementar
`Iterator<String> for Counter` o cualquier otro tipo, podríamos tener
múltiples implementaciones de `Iterator` para `Counter`. En otras palabras,
cuando un trait tiene un parámetro genérico, puede implementarse para un tipo
múltiples veces, cambiando los tipos concretos de los parámetros genéricos de
tipo cada vez. Cuando usamos el método `next` en `Counter`, tendríamos que
proporcionar anotaciones de tipo para indicar qué implementación de `Iterator`
queremos usar.

Con los tipos asociados, no necesitamos anotar los tipos porque no podemos
implementar un trait en un tipo múltiples veces. En el Listado 19-12 con la
definición que usa tipos asociados, solo podemos elegir cuál será el tipo de
`Item` una vez, porque solo puede haber un `impl Iterator for Counter`. No
tenemos que especificar que queremos un iterador de valores `u32` en todas
partes que llamamos a `next` en `Counter`.

Los tipos asociados también forman parte del contrato del trait: los
implementadores del trait deben proporcionar un tipo para que se use en lugar
del marcador de tipo. Los tipos asociados a menudo tienen un nombre que
describe cómo se usará el tipo, y documentar el tipo asociado en la
documentación de la API es una buena práctica.

### Parámetros Generics Predeterminados y Sobrecarga de Operadores

Cuando utilizamos parámetros de tipo generic, podemos especificar un tipo 
concreto predeterminado para el tipo generic. Esto elimina la necesidad de que
los implementadores del trait especifiquen un tipo concreto si el tipo
predeterminado funciona. Especificas un tipo predeterminado al declarar un tipo
generic con la sintaxis `<TipoMarcador=TipoConcreto>`.

Un ejemplo excelente de una situación en la que esta técnica es útil es con la
sobrecarga de operadores, en la que personalizas el comportamiento de un
operador (como `+`) en situaciones particulares.

Rust no te permite crear tus propios operadores o sobrecargar operadores
arbitrarios. Pero puedes sobrecargar las operaciones y los traits
correspondientes enumerados en `std::ops` implementando los traits asociados
con el operador. Por ejemplo, en el Listado 19-14 sobrecargamos el operador `+`
para agregar dos instancias de `Point` juntas. Hacemos esto implementando el
trait `Add` en un struct `Point`:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-14/src/main.rs}}
```

<span class="caption">Listing 19-14: Implementando el trait `Add` para 
sobrecargar el operador `+` para instancias `Point`</span>

El método `add` suma los valores `x` de dos instancias `Point` y los valores `y`
de dos instancias `Point` para crear una nueva instancia `Point`. El trait `Add`
tiene un tipo asociado llamado `Output` que determina el tipo devuelto desde el
método `add`.

El tipo generic predeterminado en este código está dentro del trait `Add`. Aquí
está su definición:

```rust
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

Este código debería resultar familiar en general: un trait con un método y un
tipo asociado. La nueva parte es `Rhs=Self`: esta sintaxis se llama *parámetros
de tipo predeterminados*. El parámetro de tipo generic `Rhs` (abreviatura de
“lado derecho”) define el tipo del parámetro `rhs` en el método `add`. Si no
especificamos un tipo concreto para `Rhs` cuando implementamos el trait `Add`,
el tipo de `Rhs` será predeterminado a `Self`, que será el tipo en el que
estamos implementando `Add`.

Cuando implementamos `Add` para `Point`, utilizamos el valor predeterminado para
`Rhs` porque queremos agregar dos `Point` instancias. Veamos un ejemplo de
implementación del trait `Add` donde queremos personalizar el tipo `Rhs` en
lugar de usar el predeterminado.

Tenemos dos structs, `Millimeters` y `Meters`, que contienen valores en
unidades diferentes. Este envoltorio ligero de un tipo existente en otro struct
se conoce como el *patrón newtype*, que describimos con más detalle en la
sección [“Usando el Patrón Newtype para Implementar Traits Externos en Tipos
Externos”][newtype]. Queremos agregar valores en milímetros a valores en metros
y que la implementación de `Add` haga la conversión correctamente. Podemos
implementar `Add` para `Millimeters` con `Meters` como `Rhs`, como se muestra en
el Listado 19-15.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-15/src/lib.rs}}
```

<span class="caption">Listing 19-15: Implementando el trait `Add` en
`Millimeters` para sumar `Millimeters` a `Meters`</span>

Para agregar `Millimeters` y `Meters`, especificamos `impl Add<Meters>` para
establecer el valor del parámetro de tipo `Rhs` en lugar de usar el 
predeterminado de `Self`.

Se utilizan los parámetros de tipo predeterminados en dos casos principales:

* Para extender un tipo sin romper el código existente
* Para permitir la personalización en casos específicos que la mayoría de los
  usuarios no necesitarán

El trait `Add` de la biblioteca estándar es un ejemplo del segundo propósito:
normalmente, agregarás dos tipos similares, pero el trait `Add` proporciona la
capacidad de personalizar más allá de eso. El uso de un parámetro de tipo
predeterminado en la definición del trait `Add` significa que no tienes que
especificar el parámetro extra la mayor parte del tiempo. En otras palabras, no
se necesita un poco de boilerplate de implementación, lo que facilita el uso del
trait.

El primer propósito es similar al segundo, pero al revés: si quieres agregar un
parámetro de tipo a un trait existente, puedes darle un valor predeterminado
para permitir la extensión de la funcionalidad del trait sin romper el código
de implementación existente.

### Sintaxis Completamente Calificada para la Desambiguación: Llamando Métodos con el Mismo Nombre

Nada en Rust impide que un trait tenga un método con el mismo nombre que el
método de otro trait, ni Rust te impide implementar ambos traits en un solo
tipo. También es posible implementar un método directamente en el tipo con el
mismo nombre que los métodos de los traits.

Cuando llamas a métodos con el mismo nombre, necesitarás decirle a Rust cuál
quieres usar. Considera el código en el Listado 19-16 donde hemos definido dos
traits, `Pilot` y `Wizard`, que ambos tienen un método llamado `fly`. Luego
implementamos ambos traits en un tipo `Human` que ya tiene un método llamado
`fly` implementado en él. Cada método `fly` hace algo diferente.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-16/src/main.rs:here}}
```

<span class="caption">Listing 19-16: Se definen dos traits para tener un 
método `fly` y se implementan en el tipo `Human`, además se implementa 
directamente un método `fly` en `Human`</span>

Cuando llamamos al método `fly` en una instancia de `Human`, el compilador
por defecto llama al método que está implementado directamente en el tipo, como
se muestra en el Listado 19-17.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-17/src/main.rs:here}}
```

<span class="caption">Listing 19-17: Llamando al método `fly` en una instancia 
de `Human`</span>

Ejecutando este código imprimirá `*waving arms furiously*`, mostrando que Rust
llamó al método `fly` implementado directamente en `Human`.

Para llamar a los métodos `fly` de los traits `Pilot` o `Wizard`, necesitamos
usar una sintaxis más explícita para especificar cuál método `fly` queremos
llamar. El Listado 19-18 demuestra esta sintaxis.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-18/src/main.rs:here}}
```

<span class="caption">Listing 19-18: Especificando qué método `fly` del trait
queremos llamar</span>

Especificar el nombre del trait antes del nombre del método aclara a Rust qué
implementación del método `fly` queremos llamar. También podríamos escribir
`Human::fly(&person)`; esto es equivalente a `person.fly()`, pero es un poco
más largo de escribir si no necesitamos desambiguar.

Al ejecutar este código imprime lo siguiente:

```console
{{#include ../listings/ch19-advanced-features/listing-19-18/output.txt}}
```

Debido a que el método `fly` toma un parámetro `self`, si tuviéramos dos
*tipos* que implementan el mismo *trait*, Rust podría determinar cuál implementación
del trait utilizar en función del tipo de `self`.

Sin embargo, las funciones asociadas que no son métodos no tienen un parámetro
`self`. Cuando hay múltiples tipos o traits que definen funciones no métodos
con el mismo nombre de función, Rust no siempre sabe a qué tipo te refieres a
menos que uses *sintaxis completamente calificada*. Por ejemplo, en el Listado
19-19 creamos un trait para un refugio de animales que quiere nombrar a todos
los perros bebés *Spot*. Creamos un trait `Animal` con una función no método
asociada `baby_name`. El trait `Animal` se implementa para la estructura `Dog`,
en la que también proporcionamos una función no método asociada `baby_name`
directamente.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-19/src/main.rs}}
```

<span class="caption">Listing 19-19: Un trait con una función asociada y un
tipo con una función asociada del mismo nombre que también implementa el
trait</span>

Implementamos el código para nombrar a todos los cachorros Spot en la función
asociada `baby_name` que se define en `Dog`. El tipo `Dog` también implementa
el trait `Animal`, que describe las características que todos los animales
tienen. Los cachorros de perro se llaman cachorros, y eso se expresa en la
implementación del trait `Animal` en `Dog` en la función `baby_name` asociada
con el trait `Animal`.

En `main`, llamamos a la función `Dog::baby_name`, que llama directamente a la
función asociada definida en `Dog` directamente. Este código imprime lo 
siguiente:

```console
{{#include ../listings/ch19-advanced-features/listing-19-19/output.txt}}
```

El output no es el que queríamos. Queremos llamar a la función `baby_name` que
forma parte del trait `Animal` que implementamos en `Dog`, por lo que el código
imprime `A baby dog is called a puppy`. La técnica de especificar el nombre del
trait que usamos en el Listado 19-18 no ayuda aquí; si cambiamos `main` al
código del Listado 19-20, obtendremos un error de compilación.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-20/src/main.rs:here}}
```

<span class="caption">Listing 19-20: Al intentar llamar a la función
`baby_name` del trait `Animal`, Rust no sabe qué implementación usar</span>

Debido a que `Animal::baby_name` no tiene un parámetro `self` y podría haber
otros tipos que implementen el trait `Animal`, Rust no puede averiguar qué
implementación de `Animal::baby_name` queremos. Obtendremos este error de
compilación:

```console
{{#include ../listings/ch19-advanced-features/listing-19-20/output.txt}}
```

Para desambiguar y decirle a Rust que queremos usar la implementación de
`Animal` para `Dog` en lugar de la implementación de `Animal` para algún otro
tipo, necesitamos usar la sintaxis completamente calificada. El Listado 19-21
demuestra cómo usar la sintaxis completamente calificada.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-21/src/main.rs:here}}
```

<span class="caption">Listing 19-21: Usando la sintaxis completamente
calificada para especificar que queremos llamar a la función `baby_name` del
trait `Animal` implementado en `Dog`</span>

Estamos proporcionando a Rust una anotación de tipo dentro de los corchetes
angulares, lo que indica que queremos llamar al método `baby_name` del trait
`Animal` implementado en `Dog` diciendo que queremos tratar el tipo `Dog` como
un `Animal` para esta llamada de función. Este código ahora imprimirá lo que
queremos:

```console
{{#include ../listings/ch19-advanced-features/listing-19-21/output.txt}}
```

En general, la sintaxis completamente calificada se define de la siguiente

```rust,ignore
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

Para las funciones asociadas que no son métodos, no habría un `receiver`:
solo habría una lista de otros argumentos. Podrías usar la sintaxis
completamente calificada en todas partes donde llames a funciones o métodos.
Sin embargo, se te permite omitir cualquier parte de esta sintaxis que Rust
pueda deducir de otra información en el programa. Solo necesitas usar esta
sintaxis más verbosa en casos en los que haya múltiples implementaciones que
usen el mismo nombre y Rust necesite ayuda para identificar qué implementación
quieres llamar.

### Usando supertraits para requerir la funcionalidad de un trait dentro de otro trait

A veces, es posible que desees escribir una definición de trait que dependa de
otro trait: para que un tipo implemente el primer trait, quieres exigir que 
este tipo también implemente el segundo trait. Esto se hace para que la 
definición de tu trait pueda hacer uso de los elementos asociados del segundo
trait. El trait en el que se basa la definición de tu trait se llama
*supertrait* de tu trait.

Por ejemplo, supongamos que queremos crear un trait llamado `OutlinePrint` con
un método `outline_print` que imprima un valor dado enmarcado entre asteriscos.
Es decir, dado un struct `Point` que implementa el trait de la biblioteca
estándar `Display` para que el resultado sea `(x, y)`, cuando llamemos a
`outline_print` en una instancia de `Point` que tenga `1` para `x` y `3` para
`y`, debería imprimir lo siguiente:

```text
**********
*        *
* (1, 3) *
*        *
**********
```

Al implementar el método `outline_print`, queremos usar la funcionalidad de
`Display`. Por lo tanto, necesitamos indicar que el trait `OutlinePrint` solo
funcionará con tipos que también implementen `Display` y proporcionen la
funcionalidad que `OutlinePrint` necesita. Podemos hacer eso en la definición
del trait especificando `OutlinePrint: Display`. Esta técnica es similar a
agregar un límite de trait al trait. El Listado 19-22 muestra una
implementación del trait `OutlinePrint`.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-22/src/main.rs:here}}
```

<span class="caption">Listing 19-22: Implementando el trait `OutlinePrint`
que requiere la funcionalidad de `Display`</span>

Dado que hemos especificado que `OutlinePrint` requiere el trait `Display`, el
utilizar la función `to_string` que se implementa automáticamente para cualquier
tipo que implemente `Display` está bien. Si intentáramos usar `to_string` sin
agregar dos puntos y especificar el trait `Display` después del nombre del
trait, obtendríamos un error diciendo que no se encontró ningún método llamado
`to_string` para el tipo `&Self` en el scope actual.

Veamos qué sucede cuando intentamos usar `OutlinePrint` en un tipo que
no implementa `Display`, como el struct `Point`:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-02-impl-outlineprint-for-point/src/main.rs:here}}
```

Obtenemos un error que indica que se requiere implementar `Display`, pero no
está implementado:

```console
{{#include ../listings/ch19-advanced-features/no-listing-02-impl-outlineprint-for-point/output.txt}}
```

Para solucionar esto, implementamos `Display` en `Point` y cumplimos con la
restricción que requiere `OutlinePrint`, de la siguiente manera:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-03-impl-display-for-point/src/main.rs:here}}
```

Entonces, al implementar el trait `OutlinePrint` en `Point`, se compilará
exitosamente, y podemos llamar a `outline_print` en una instancia de `Point`
para mostrarla dentro de un contorno de asteriscos.

### Usando el pattern Newtype para implementar traits externos en tipos externos

En el capítulo 10 en la sección [“Implementando un trait en un
tipo”][implementing-a-trait-on-a-type]<!-- ignore -->, mencionamos los orphan
rules que establecen que solo podemos implementar un trait en un tipo si
bien el trait o el tipo son locales a nuestro crate. Es posible evitar esta
restricción usando el *patrón newtype*, que implica crear un nuevo tipo en un
struct de tupla. (Cubrimos los structs de tupla en la sección [“Usando
structs de tupla sin campos nombrados para crear diferentes
tipos”][tuple-structs]<!-- ignore --> del capítulo 5.) El struct de tupla
tendrá un campo y será un envoltorio delgado alrededor del tipo en el que
queremos implementar un trait. Entonces, el tipo de envoltorio es local a
nuestro crate, y podemos implementar el trait en el envoltorio. *Newtype* es
un término que se origina en el lenguaje de programación Haskell. No hay
penalización de rendimiento en tiempo de ejecución por usar este patrón, y el
tipo de wrapper se omite en tiempo de compilación.

Como ejemplo, supongamos que queremos implementar `Display` en `Vec<T>`, lo 
cual nos impide hacerlo directamente debido a regla de los "orphan rules", ya 
que el trait `Display` y el tipo `Vec<T>` están definidos fuera de nuestro
crate. Podemos hacer un struct llamado `Wrapper` que contenga una instancia de
`Vec<T>`. Luego podemos implementar `Display` en `Wrapper` y usar el valor de
`Vec<T>`, como se muestra en el Listado 19-23.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-23/src/main.rs}}
```

<span class="caption">Listing 19-23: Crear un tipo `Wrapper` alrededor de
`Vec<String>` para implementar `Display`</span>

La implementación de `Display` usa `self.0` para acceder al `Vec<T>` interno,
porque `Wrapper` es un struct de tupla y `Vec<T>` es el item en el índice 0 de
la tupla. Luego podemos usar la funcionalidad del tipo `Display` en `Wrapper`.

La desventaja de usar esta técnica es que `Wrapper` es un nuevo tipo, por lo
que no tiene los métodos del valor que contiene. Tendríamos que implementar
todos los métodos de `Vec<T>` directamente en `Wrapper` de tal manera que los
métodos deleguen a `self.0`, lo que nos permitiría tratar a `Wrapper`
exactamente como un `Vec<T>`. Si quisiéramos que el nuevo tipo tenga todos los
métodos del tipo interno, implementar el trait `Deref` (discutido en el
capítulo 15 en la sección [“Tratando a los smart pointers como referencias
regulares con el trait `Deref`”][smart-pointer-deref]<!-- ignore -->) en
`Wrapper` para devolver el tipo interno sería una solución. Si no queremos que
el tipo `Wrapper` tenga todos los métodos del tipo interno, por ejemplo, para
restringir el comportamiento del tipo `Wrapper`, tendríamos que implementar
manualmente solo los métodos que queremos.

El pattern newtype también es útil incluso cuando no se involucran traits.
Ahora cambiemos de enfoque y exploremos algunas formas avanzadas de interactuar
con el sistema de tipos de Rust.

[newtype]: ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
[implementing-a-trait-on-a-type]:
ch10-02-traits.html#implementing-a-trait-on-a-type
[traits-defining-shared-behavior]:
ch10-02-traits.html#traits-defining-shared-behavior
[smart-pointer-deref]: ch15-02-deref.html#treating-smart-pointers-like-regular-references-with-the-deref-trait
[tuple-structs]: ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types
