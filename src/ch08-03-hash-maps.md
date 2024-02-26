## Almacenar Claves con Valores Asociados en HashMaps

La última de nuestras colecciones comunes es el _hash map_. El tipo `HashMap<K,
V>` almacena un mapeo de claves de tipo `K` a valores de tipo `V` usando una
_función hash_, que determina cómo coloca estas claves y valores en la memoria.
Muchos lenguajes de programación admiten este tipo de estructura de datos, pero
a menudo usan un nombre diferente, como hash, map, object, hash table,
diccionario o arreglos asociativos, solo para nombrar algunos.

Los hash maps son útiles cuando desea buscar datos no usando un índice, como
puede hacerlo con vectores, sino usando una clave que puede ser de cualquier
tipo. Por ejemplo, en un juego, podría realizar un seguimiento de la puntuación
de cada equipo en un hash map en el que cada clave es el nombre de un equipo y
los valores son la puntuación de cada equipo. Dado un nombre de equipo, puede
recuperar su puntuación.

Repasaremos la API básica de los hash maps en esta sección, pero muchas más
cosas buenas se esconden en las funciones definidas en `HashMap<K, V>` por la
biblioteca estándar. Como siempre, consulte la documentación de la biblioteca
estándar para obtener más información.

### Creando un nuevo HashMap

Una forma de crear un hash map vacío es usar `new` y agregar elementos con
`insert`. En el Listado 8-20, estamos realizando un seguimiento de las
puntuaciones de dos equipos cuyos nombres son _Blue_ y _Yellow_. El equipo
Blue comienza con 10 puntos y el equipo Yellow comienza con 50.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-20/src/main.rs:here}}
```

<span class="caption">Listado 8-20: Creando un nuevo hash map e insertando
algunas claves y valores</span>

Ten en cuenta que es importante importar primero el módulo `HashMap` de la
biblioteca estándar de colecciones. De nuestras tres colecciones comunes,
ésta es la menos utilizada, por lo que no se incluye automáticamente en las
características del _prelude_. Además, los hash maps tienen menos soporte por
parte de la biblioteca estándar; por ejemplo, no hay una macro incorporada para
construirlos.

Al igual que los vectores, los hash maps almacenan sus datos en el _heap_. Este
`HashMap` tiene claves de tipo `String` y valores de tipo `i32`. Al igual que los
vectores, los hash maps son homogéneos: todas las claves deben tener el mismo
tipo entre sí y todos los valores deben tener el mismo tipo.

### Accediendo a los valores en un HashMap

Podemos obtener un valor de un hash map proporcionando su clave al método `get`
como se muestra en el listado 8-21.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-21/src/main.rs:here}}
```

<span class="caption">Listado 8-21: Acceso al puntaje para el equipo Blue
almacenado en el hash map</span>

Aquí, `score` tendrá el valor que está asociado con el equipo Blue, y el
resultado será `10`. El método `get` devuelve un `Option<&V>`; si no hay un
valor para ese clave en el hash map, `get` devolverá `None`. Este programa
maneja un `Option` llamando a `copied` para obtener un `Option<i32>` en lugar
de un `Option<&i32>`, luego `unwrap_or` para establecer `score` en cero si
`scores` no tiene una entrada para la clave.

Podemos iterar sobre cada par clave/valor en un hash map de manera similar a
como lo hacemos con vectores, usando un ciclo `for`:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-03-iterate-over-hashmap/src/main.rs:here}}
```

Este código imprimirá cada par en un orden arbitrario:

```text
Yellow: 50
Blue: 10
```

### HashMaps y Ownership

Para los tipos que implementan el trait `Copy`, como `i32`, los valores se
copian en el hash map. Para valores de propiedad como `String`, los valores se
moverán y el hash map será el propietario de esos valores, como se demuestra
en el listado 8-22.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-22/src/main.rs:here}}
```

<span class="caption">Listado 8-22: Mostrando que claves y valores son propiedad
del hash map una vez que se insertan</span>

No podemos usar `field_name` y `field_value` después de que se hayan movido al
hash map con la llamada a `insert`.

Si insertamos referencias a valores en el hash map, los valores no se moverán
al hash map. Los valores a los que apuntan las referencias deben ser válidos
al menos mientras el hash map sea válido. Hablaremos más sobre estos problemas
en la sección [“Validando referencias con Lifetimes”][validando-referencias-con-lifetimes]<!-- ignore --> en el
Capítulo 10.

### Actualizando un HashMap

Aunque la cantidad de pares clave/valor es creciente, cada clave única solo puede
tener un valor asociado con ella a la vez (pero no viceversa: por ejemplo, el
equipo Blue y el equipo Yellow podrían tener el valor 10 almacenados en el hash
map `scores`).

Cuando queremos cambiar los datos en un hash map, tenemos que decidir cómo
manejar el caso en el que una clave ya tiene un valor asignado. Podrías
reemplazar el valor antiguo por el nuevo valor, ignorando completamente el
valor antiguo. Podrías mantener el valor antiguo e ignorar el nuevo valor,
agregando el nuevo valor solo si la clave _no_ tiene ya un valor. O podrías
combinar el valor antiguo y el nuevo valor. ¡Veamos cómo hacer cada una de
estas!

#### Reemplazando un valor

Si insertamos una clave y un valor en un hash map y luego insertamos esa misma
clave con un valor diferente, el valor asociado con esa clave se reemplazará.
Aunque el código en el listado 8-23 llama a `insert` dos veces, el hash map
solo contendrá un par clave/valor porque estamos insertando el valor para la clave
del equipo Blue dos veces.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-23/src/main.rs:here}}
```

<span class="caption">Listado 8-23: Reemplazando un valor almacenado con una
clave en particular</span>

Este código imprimirá `{"Blue": 25}`. El valor original de `10` ha sido
sobrescrito.

<!-- Old headings. Do not remove or links may break. -->

<a id="only-inserting-a-value-if-the-key-has-no-value"></a>

#### Insertando una Key y un valor solo si una Key no está presente

Es común verificar si una clave en particular ya existe en el hash map con un
valor y luego realizar las siguientes acciones: si la clave existe en el hash
map, el valor existente debe permanecer tal como está. Si la clave no existe,
insertarla junto con su valor.

Los hash maps tienen una API especial para esto llamada `entry` que toma la clave
que desea verificar como parámetro. El valor de retorno del método `entry` es
un enum llamado `Entry` que representa un valor que puede o no existir. Digamos
que queremos verificar si la clave para el equipo Yellow tiene un valor
asociado. Si no lo tiene, queremos insertar el valor 50, y lo mismo para el
equipo Blue. Usando la API `entry`, el código se ve como el listado 8-24.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-24/src/main.rs:here}}
```

<span class="caption">Listado 8-24: Usando el método `entry` para insertar solo
si la clave aún no tiene un valor</span>

El método `or_insert` en `Entry` está definido para devolver una referencia
mutable al valor correspondiente a la clave `Entry` si esa clave existe, y si no,
inserta el parámetro como el nuevo valor para esta clave y devuelve una
referencia mutable al nuevo valor. Esta técnica es mucho más limpia que
escribir la lógica nosotros mismos y, además, juega mejor con el borrow
checker.

Ejecutar el código en el listado 8-24 imprimirá `{"Yellow": 50, "Blue": 10}`.
La primera llamada a `entry` insertará la clave para el equipo Yellow con el
valor 50 porque el equipo Yellow no tiene un valor todavía. La segunda llamada
a `entry` no cambiará el hash map porque el equipo Blue ya tiene el valor 10.

#### Actualizando un valor basado en el valor anterior

Otro caso común para los hash maps es buscar un valor para una clave y luego
actualizar ese valor en función del valor anterior. Por ejemplo, el listado 8-25
muestra un código que cuenta cuántas veces aparece cada palabra en algún texto.
Usamos un hash map con las palabras como claves y aumentamos el valor para
mantener un recuento de cuántas veces hemos visto esa palabra. Si es la primera
vez que vemos una palabra, primero insertaremos el valor 0.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-25/src/main.rs:here}}
```

<span class="caption">Listado 8-25: Contando ocurrencias de palabras usando un
hash map que almacena palabras y cuenta</span>

Este código imprimirá `{"world": 2, "hello": 1, "wonderful": 1}`. Es posible
que veas los mismos pares clave/valor en un orden diferente: recuerda la sección
[“Accediendo a valores en un hash map”][access]<!-- ignore --> que iterar sobre
un hash map ocurre en un orden arbitrario.

El método `split_whitespace` devuelve un iterator sobre sub-slices, separados
por espacios en blanco, del valor en `text`. El método `or_insert` devuelve una
referencia mutable (`&mut V`) al valor para la clave especificada. Aquí
almacenamos esa referencia mutable en la variable `count`, por lo que para
asignar a ese valor, primero debemos desreferenciar `count` usando el asterisco
(`*`). La referencia mutable sale del ámbito al final del ciclo `for`, por lo
que todos estos cambios son seguros y permitidos por las reglas del borrowing.

### Funciones de Hashing

Por defecto, `HashMap` usa una función de hashing llamada _SipHash_ que puede
proporcionar resistencia a ataques de Denegación de Servicio (DoS) que
involucran tablas hash[^siphash]<!-- ignore -->. Este no es el algoritmo de
hashing más rápido disponible, pero el compromiso por una mejor seguridad que
viene con la caída en el rendimiento vale la pena. Si perfilas tu código y
encuentras que la función de hash predeterminada es demasiado lenta para tus
propósitos, puedes cambiar a otra función especificando un hasher diferente. Un
_hasher_ es un tipo que implementa el trait `BuildHasher`. Hablaremos sobre
traits y cómo implementarlos en el Capítulo 10. No necesariamente tienes que
implementar tu propio hasher desde cero;
[crates.io](https://crates.io/)<!-- ignore -->
tiene bibliotecas compartidas por otros usuarios de Rust que proporcionan
hashes que implementan muchos algoritmos de hashing comunes.

[^siphash]: [https://en.wikipedia.org/wiki/SipHash](https://en.wikipedia.org/wiki/SipHash)

## Resumen

Los vectores, los strings y los hash maps proporcionan una funcionalidad
importante que necesitarás cuando quieras almacenar, acceder y modificar datos.
Aquí hay algunos ejercicios que ahora deberías estar equipado para resolver:

- Dada una lista de enteros, usa un vector y devuelve la mediana (cuando se
  ordena, el valor en la posición media) y la moda (el valor que ocurre con más
  frecuencia; un hash map será útil aquí) de la lista.
- Convierte strings a pig latin. La primera consonante de cada palabra se
  mueve al final de la palabra y se agrega "ay", por lo que "primero" se
  convierte en "rimepay". Sin embargo, si la palabra comienza con una vocal,
  simplemente agregue "hay" al final de la palabra ("manzanaay"). ¡Ten en
  cuenta las reglas de UTF-8!
- Usando un hash map y vectores, cree un texto de interfaz para permitir que un
  usuario agregue nombres de empleados a un departamento en una empresa. Por
  ejemplo, "Agregar Sally a Ingeniería" o "Agregar Amir a Ventas". Luego,
  permita que el usuario recupere una lista de todas las personas en un
  departamento o todas las personas en la empresa por departamento, ordenadas
  alfabéticamente.

La documentación de la biblioteca estándar describe métodos que los vectores,
strings y hash maps tienen que ser útiles para estos ejercicios.

Nos estamos adentrando en programas más complejos en los que las operaciones
pueden fallar, por lo que es un momento perfecto para discutir el manejo de
errores. ¡Haremos eso a continuación!

[validando-referencias-con-lifetimes]: ch10-03-lifetime-syntax.html#validando-referencias-con-lifetimes
[access]: #accediendo-a-los-valores-en-un-hashmap
