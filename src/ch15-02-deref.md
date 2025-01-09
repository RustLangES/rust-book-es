## Tratando los Smart Pointers como Referencias Regulares con el Trait `Deref`

Implementar el trait `Deref` te permite personalizar el comportamiento del
_operador de desreferencia_ `*` (no confundir con el operador de multiplicación
o el operador de glob). Al implementar `Deref` de tal manera que un smart
pointer pueda ser tratado como una referencia regular, puedes escribir código
que opere en referencias y usar ese código con smart pointers también.

Primero veamos cómo funciona el operador de desreferencia con referencias
regulares. Luego intentaremos definir un tipo personalizado que se comporte
como `Box<T>`, y veremos por qué el operador de desreferencia no funciona como
una referencia en nuestro tipo recién definido. Exploraremos cómo implementar
el trait `Deref` hace posible que los smart pointers trabajen de manera similar
a las referencias. Luego veremos la característica de _deref coercion_ de Rust
y cómo nos permite trabajar con referencias o smart pointers.

> Nota: Hay una gran diferencia entre el tipo `MyBox<T>` que estamos a punto de
> construir y el tipo `Box<T>` real: nuestra versión no almacenará sus datos en
> el heap. Nos estamos enfocando en este ejemplo en `Deref`, por lo que dónde
> se almacenan los datos es menos importante que el comportamiento similar al
> de un puntero.

<!-- Old link, do not remove -->

<a id="following-the-pointer-to-the-value-with-the-dereference-operator"></a>

### Siguiendo el puntero al valor

Una referencia regular es un tipo de puntero, y una forma de pensar en un
puntero es como una flecha a un valor almacenado en otro lugar. En el Listado
15-6, creamos una referencia a un valor `i32` y luego usamos el operador de
desreferencia para seguir la referencia al valor:

<Listing number="15-6" file-name="src/main.rs" caption="Utilizando el operador de dereferencia para seguir una referencia a un valor `i32`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-06/src/main.rs}}
```

</Listing>

La variable `x` contiene un valor `i32` de `5`. Establecemos `y` igual a una
referencia a `x`. Podemos afirmar que `x` es igual a `5`. Sin embargo, si
queremos hacer una afirmación sobre el valor en `y`, tenemos que usar `*y` para
seguir la referencia al valor al que apunta (de ahí _desreferencia_) para que
el compilador pueda comparar el valor real. Una vez que desreferenciamos `y`,
tenemos acceso al valor entero al que apunta `y` que podemos comparar con `5`.

Si intentamos escribir `assert_eq!(5, y);` en su lugar, obtendríamos este error
de compilación:

```console
{{#include ../listings/ch15-smart-pointers/output-only-01-comparing-to-reference/output.txt}}
```

Comparar un número y una referencia a un número no está permitido porque son
tipos diferentes. Debemos usar el operador de desreferencia para seguir la
referencia al valor al que apunta.

### Usando `Box<T>` como una referencia

Podemos reescribir el código del Listado 15-6 para usar un `Box<T>` en lugar de
una referencia; el operador de desreferencia usado en el `Box<T>` en el Listado
15-7 funciona de la misma manera que el operador de desreferencia usado en la
referencia en el Listado 15-6:

<Listing number="15-7" file-name="src/main.rs" caption="Utilizando el operador de dereferencia en un `Box<i32>`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-07/src/main.rs}}
```

</Listing>

La principal diferencia entre el Listado 15-7 y el Listado 15-6 es que aquí
definimos `y` como una instancia de `Box<T>` apuntando a una copia del valor de
`x` en lugar de ser una referencia que apunta al valor de `x`. En la última
afirmación, podemos usar el operador de desreferencia para seguir el puntero del
`Box<T>` de la misma manera que lo hicimos cuando `y` era una referencia. A
continuación, exploraremos que es lo especial de `Box<T>` que nos permite el
uso del operador de desreferencia al definir nuestro propio tipo.

### Definiendo nuestro propio Smart Pointer

Construyamos un smart pointer similar al tipo `Box<T>` proporcionado por la
biblioteca estándar para experimentar cómo los smart pointers se comportan de
manera diferente a las referencias por defecto. Luego veremos cómo agregar la
capacidad de usar el operador de desreferencia.

El tipo `Box<T>` es finalmente definido como una tupla struct con un elemento,
por lo que el Listado 15-8 define un tipo `MyBox<T>` de la misma manera.
También definiremos una función `new` para que coincida con la función `new`
definida en `Box<T>`.

<Listing number="15-8" file-name="src/main.rs" caption="Definiendo un tipo `MyBox<T>`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-08/src/main.rs:here}}
```

</Listing>

Definimos un struct llamado `MyBox` y declaramos un parámetro generic `T`,
porque queremos que nuestro tipo contenga valores de cualquier tipo. El tipo
`MyBox` es una tupla struct con un elemento de tipo `T`. La función `MyBox::new`
toma un parámetro de tipo `T` y devuelve una instancia de `MyBox` que contiene
el valor pasado.

Vamos a intentar añadir la función `main` del Listado 15-7 al Listado 15-8 y
cambiarla para usar el tipo `MyBox<T>` que hemos definido en lugar de `Box<T>`.
El código en el Listado 15-9 no se compilará porque Rust no sabe cómo
desreferenciar `MyBox`.

<Listing number="15-9" file-name="src/main.rs" caption="Intentando usar `MyBox<T>` de la misma manera en que usamos referencias y `Box<T>`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-09/src/main.rs:here}}
```

</Listing>

Aquí está el error de compilación resultante:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-09/output.txt}}
```

Nuestro tipo `MyBox<T>` no puede ser desreferenciado porque no hemos
implementado esa capacidad en nuestro tipo. Para habilitar la desreferencia con
el operador `*`, implementamos el trait `Deref`.

### Tratando un tipo como una referencia implementando el trait `Deref`

Como discutimos en la sección del Capítulo 10
[“Implementando un Trait en un Tipo”][impl-trait]<!-- ignore--> , para
implementar un trait, necesitamos proporcionar implementaciones para los métodos
requeridos del trait. El trait `Deref`, proporcionado por la biblioteca
estándar, requiere que implementemos un método llamado `deref` que tome `self`
y devuelva una referencia al dato interno. El Listado 15-10 contiene una
implementación de `Deref` para agregar a la definición de `MyBox`:

<Listing number="15-10" file-name="src/main.rs" caption="Implementando `Deref` en `MyBox<T>`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-10/src/main.rs:here}}
```

</Listing>

La sintaxis `type Target = T;` define un tipo asociado que será utilizado por el
trait `Deref`. Los tipos asociados son una forma ligeramente diferente de
declarar un parámetro genérico, pero no necesitas preocuparte por ellos por
ahora; los cubriremos con más detalle en el Capítulo 20.

Rellenamos el cuerpo del método `deref` con `&self.0` para que `deref` devuelva
una referencia al valor al que queremos acceder con el operador `*`. Recordemos
de la sección [“Usando Tuplas Structs sin Campos Nombrados para Crear Diferentes
Tipos”][tuple-structs]<!-- ignore --> del Capítulo 5 que `.0` accede al primer
valor en una tupla struct. ¡La función `main` en el Listado 15-9 que llama a `*`
en el valor `MyBox<T>` ahora compila, y las afirmaciones pasan!

Sin el trait `Deref`, el compilador no sabe cómo desreferenciar referencias `&`.
El método `deref` le da al compilador la capacidad de tomar un valor de
cualquier tipo que implemente `Deref` y llamar al método `deref` para obtener
una referencia `&` que sabe cómo desreferenciar.

Cuando ingresamos `*y` en el Listado 15-9, en realidad Rust ejecuta este código:

```rust,ignore
*(y.deref())
```

Rust sustituye el operador `*` con una llamada al método `deref`, y luego
realiza una desreferenciación directa, por lo que no tenemos que pensar si
necesitamos llamar al método `deref`. Esta característica de Rust nos permite
escribir código que funciona de manera idéntica si tenemos una referencia
regular o un tipo que implementa `Deref`.

La razón por la cual el método `deref` devuelve una referencia a un valor, y
por qué la desreferenciación simple fuera de los paréntesis en `*(y.deref())`
todavía es necesaria, tiene que ver con el sistema de propiedad. Si el método
`deref` devolviera el valor directamente en lugar de una referencia al valor,
el valor se movería fuera de `self`. No queremos tomar posesión del valor
interno dentro de `MyBox<T>` en este caso o en la mayoría de los casos en los
que usamos el operador de desreferencia.

Nota que el operador `*` es reemplazado con una llamada al método `deref` y
luego una llamada al operador `*` solo una vez, cada vez que usamos un `*` en
nuestro código. Debido a que la sustitución del operador `*` no se repite
infinitamente, terminamos con datos de tipo `i32`, que coincide con el `5` en
`assert_eq!` en el Listado 15-9.

### Coerciones implicitas de Deref con funciones y metodos

La _coerción Deref_ convierte una referencia a un tipo que implementa el trait
`Deref` en una referencia a otro tipo. Por ejemplo, la coerción Deref puede
convertir `&String` en `&str` porque `String` implementa el trait `Deref` de
manera que devuelve `&str`. La coerción Deref es una conveniencia que Rust
realiza en los argumentos de las funciones y métodos, y solo funciona en tipos
que implementan el trait `Deref`. Sucede automáticamente cuando pasamos una
referencia al valor de un tipo particular como argumento a una función o método
que no coincide con el tipo de parámetro en la definición de la función o
método. Una secuencia de llamadas al método `deref` convierte el tipo que
proporcionamos en el tipo que necesita el parámetro.

La coerción Deref se agregó a Rust para que los programadores que escriben
llamadas a funciones y métodos no necesiten agregar tantas referencias y
desreferencias explícitas con `&` y `*`. La característica de coerción Deref
también nos permite escribir más código que puede funcionar para referencias o
smart pointers.

Para ver la coerción Deref en acción, usemos el tipo `MyBox<T>` que definimos
en el Listado 15-8 y la implementación de `Deref` que agregamos en el Listado
15-10. El Listado 15-11 muestra la definición de una función que tiene un
parámetro de tipo string slice:

<Listing number="15-11" file-name="src/main.rs" caption="Una función `hello` que tiene el parámetro `name` de tipo `&str`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-11/src/main.rs:here}}
```

</Listing>

Llamamos a la función `hello` con un string slice como un argumento, como
`hello("Rust");` por ejemplo. La coerción Deref hace posible llamar a `hello`
con una referencia a un valor de tipo `MyBox<String>`, como se muestra en el
Listado 15-12:

<Listing number="15-12" file-name="src/main.rs" caption="Llamando a `hello` con una referencia a un valor `MyBox<String>`, lo cual funciona debido a la coerción deref">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-12/src/main.rs:here}}
```

</Listing>

Aquí estamos llamando a la función `hello` con el argumento `&m`, que es una
referencia a un valor `MyBox<String>`. Debido a que implementamos el trait
`Deref` en `MyBox<T>` en el Listado 15-10, Rust puede convertir `&MyBox<String>`
en `&String` llamando a `deref`. La biblioteca estándar proporciona una
implementación de `Deref` en `String` que devuelve una cadena de texto, y esto
está en la documentación de la API de `Deref`. Rust llama a `deref` nuevamente
para convertir el `&String` en `&str`, que coincide con la definición de la
función `hello`.

Si Rust no implementara la coerción Deref, tendríamos que escribir el código
en el Listado 15-13 en lugar del código en el Listado 15-12 para llamar a
`hello` con un valor de tipo `&MyBox<String>`.

<Listing number="15-13" file-name="src/main.rs" caption="El código que tendríamos que escribir si Rust no tuviera deref coerción">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-13/src/main.rs:here}}
```

</Listing>

El `(*m)` desreferencia el `MyBox<String>` en un `String`. Luego, el `&` y
`[..]` toman un string slice del `String` que es igual a todo el string para
coincidir con la firma de `hello`. Este código sin coerciones de desreferencia
es más difícil de leer, escribir y entender con todos estos símbolos
involucrados. La coerción Deref permite que Rust maneje estas conversiones
automáticamente.

Cuando el trait `Deref` está definido para el tipo involucrado, Rust analizará
los tipos y usará `Deref::deref` tantas veces como sea necesario para obtener
una referencia que coincida con el tipo del parámetro. El número de veces que
`Deref::deref` necesita ser insertado se resuelve en tiempo de compilación, por
lo que no hay penalización en tiempo de ejecución por aprovechar la coerción
Deref!

### Cómo interactúa la coerción Deref con la mutabilidad

Similar a cómo usas el trait `Deref` para anular el operador `*` en
referencias inmutables, puedes usar el trait `DerefMut` para anular el operador
`*` en referencias mutables.

Rust realiza la coerción Deref cuando encuentra tipos e implementaciones de
traits en tres casos:

- De `&T` a `&U` cuando `T: Deref<Target=U>`
- De `&mut T` a `&mut U` cuando `T: DerefMut<Target=U>`
- De `&mut T` a `&U` cuando `T: Deref<Target=U>`

Los dos primeros casos son iguales entre sí, excepto que el segundo implementa
mutabilidad. El primer caso establece que si tienes un `&T`, y `T` implementa
`Deref` a algún tipo `U`, puedes obtener un `&U` de forma transparente. El
segundo caso establece que la misma coerción de desreferencia ocurre para
referencias mutables.

El tercer caso es más complicado. Rust también convertirá una referencia mutable
en una inmutable. Pero lo contrario no es posible: una referencia inmutable
nunca se puede convertir en una referencia mutable. Debido a las reglas de
borrowing, si tienes una referencia mutable, esa referencia debe ser la única
referencia a ese dato (de lo contrario, el programa no se compilaría). Convertir
una referencia mutable a una inmutable nunca romperá las reglas de borrowing.
Convertir una referencia inmutable a una mutable requeriría que la referencia
inmutable inicial sea la única referencia inmutable a esos datos, pero las
reglas de borrowing no garantizan eso. Por lo tanto, Rust no puede hacer la
suposición de que convertir una referencia inmutable en una mutable es posible.

[impl-trait]: ch10-02-traits.html#implementando-un-trait-en-un-tipo
[tuple-structs]: ch05-01-defining-structs.html#usando-structs-de-tuplas-sin-campos-nombrados-para-crear-diferentes-tipos
