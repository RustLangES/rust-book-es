## Procesando una serie de elementos con Iteradores

El patrón de iterador te permite realizar alguna tarea en una secuencia de
elementos a su vez. Un iterador es responsable de la lógica de iterar sobre
cada elemento y determinar cuándo ha terminado la secuencia. Cuando usas
iterators, no tienes que reimplementar esa lógica tú mismo.

En rust, los iterators son *lazy*, lo que significa que no tienen efecto hasta
que llamas a métodos que consumen el iterador para usarlo. Por ejemplo, el
código en el Listado 13-10 crea un iterador sobre los elementos del vector `v1`
llamando al método `iter` definido en `Vec<T>`. Este código por sí solo no hace
nada útil.

<Listing number="13-10" file-name="src/main.rs" caption="Creando un iterator">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-10/src/main.rs:here}}
```

</Listing>

El iterador es almacenado en la variable `v1_iter`. Una vez que hemos creado un
iterator, podemos usarlo de varias maneras. En el Listado 3-5 del Capítulo 3,
iteramos sobre un array usando un bucle `for` para ejecutar algún código en cada
uno de sus elementos. Bajo el capó, esto crea e implícitamente consume un
iterator, pero pasamos por alto cómo funciona exactamente hasta ahora.

En el ejemplo del Listado 13-11, separamos la creación del iterador del uso del
iterador en el bucle `for`. Cuando el bucle `for` es llamado usando el iterator
en `v1_iter`, cada elemento en el iterador es usado en una iteración del bucle,
lo que imprime cada valor.

<Listing number="13-11" file-name="src/main.rs" caption="Usando un iterador en un bucle `for`">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-11/src/main.rs:here}}
```

</Listing>

En lenguajes que no tienen iterators provistos por sus bibliotecas estándar,
probablemente escribirías esta misma funcionalidad comenzando una variable en
el índice 0, usando esa variable para indexar en el vector para obtener un
valor, e incrementando el valor de la variable en un bucle hasta que alcanzara
el número total de elementos en el vector.

Los iterators manejan toda esa lógica por ti, reduciendo el código repetitivo
que podrías potencialmente arruinar. Los iterators te dan más flexibilidad para
usar la misma lógica con muchos tipos diferentes de secuencias, no solo
estructuras de datos en las que puedes indexar, como los vectores. Examinemos
cómo los iterators hacen eso.

### El trait `Iterator` y el método `next`

Todos los iterators implementan un trait llamado `Iterator` que está definido
en la biblioteca estándar. La definición del trait se ve así:

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```

Observa que esta definición usa una nueva sintaxis: `type Item` y
`Self::Item`, que definen un *associated type* con este trait. Hablaremos sobre
los associated types en profundidad en el Capítulo 20. Por ahora, todo lo que
necesitas saber es que este código dice que implementar el trait `Iterator`
requiere que también definas un tipo `Item`, y este tipo `Item` es usado en el
tipo de retorno del método `next`. En otras palabras, el tipo `Item` será el
tipo retornado del iterator.

El trait `Iterator` solo requiere que los implementadores definan un método:
el método `next`, que retorna un item del iterador a la vez envuelto en `Some`
y, cuando la iteración ha terminado, retorna `None`.

Podemos llamar al método `next` en los iterators directamente; el Listado 13-12
demuestra qué valores son retornados de llamadas repetidas a `next` en el
iterador creado desde el vector.

<Listing number="13-12" file-name="src/lib.rs" caption="Llamando al método `next` en un iterator">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-12/src/lib.rs:here}}
```

</Listing>

Nota que necesitamos hacer `v1_iter` mutable: llamar al método `next` en un
iterador cambia el estado interno que el iterador usa para mantenerse al tanto
de dónde está en la secuencia. En otras palabras, este código *consume*, o usa,
el iterator. Cada llamada a `next` consume un item del iterator. No necesitamos
hacer `v1_iter` mutable cuando usamos un bucle `for` porque el bucle toma
posesión de `v1_iter` y lo hace mutable detrás de escena.

También debemos tener en cuenta que los valores que obtenemos de las llamadas a
`next` son referencias inmutables a los valores en el vector. El método `iter`
produce un iterador sobre referencias inmutables. Si queremos crear un iterator
que tome posesión de `v1` y retorne valores poseídos, podemos llamar a
`into_iter` en lugar de `iter`. De manera similar, si queremos iterar sobre
referencias mutables, podemos llamar a `iter_mut` en lugar de `iter`.

### Métodos que consumen el iterator

El trait `Iterator` tiene una variedad de métodos con implementaciones
predeterminadas provistas por la biblioteca estándar; puedes encontrar
información sobre estos métodos en la documentación de la biblioteca estándar
para el trait `Iterator`. Algunos de estos métodos llaman al método `next` en su
definición, por lo que se requiere que implementes el método `next` al
implementar el trait `Iterator`.

Los métodos que llaman a `next` se llaman *consuming adapters*, porque
consumen el iterador llamando a `next`. Un ejemplo es el método `sum`, que
toma posesión del iterador y lo itera a través de los items llamando a `next`,
así consumiendo el iterator. A medida que itera a través de ellos, agrega cada
item a un total en ejecución y retorna el total cuando la iteración está
completa. El Listado 13-13 tiene una prueba que ilustra el uso del método `sum`:

<Listing number="13-13" file-name="src/lib.rs" caption="Llamando al método `sum` para obtener el total de todos los items en el iterator">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-13/src/lib.rs:here}}
```

</Listing>

No se nos permite usar `v1_iter` después de la llamada a `sum` porque `sum`
toma el ownership del iterador en el que lo llamamos.

### Métodos que producen otros iterators

*Iterator adapters* son métodos definidos en el trait `Iterator` que no
consumen el iterator. En cambio, producen diferentes iterators cambiando algún
aspecto del iterador original.

El Listado 13-14 muestra un ejemplo de llamar al método adapter de iterator
`map` que toma un closure para llamar en cada item y produce un nuevo iterator.
El método `map` retorna un nuevo iterador que ejecuta el closure que le
pasamos en cada item y produce los items resultantes. El closure aquí crea un
nuevo iterador en el que cada item del vector será incrementado en 1:

<Listing number="13-14" file-name="src/main.rs" caption="Llamando al iterador adaptor `map` para crear un nuevo iterator">

```rust,not_desired_behavior
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-14/src/main.rs:here}}
```

</Listing>

Como siempre, este código producirá un warning:

```console
{{#include ../listings/ch13-functional-features/listing-13-14/output.txt}}
```

El código del Listado 13-14 no hace nada; el closure que hemos especificado
nunca es llamado. El warning nos recuerda por qué: los iterador adapters son
perezosos, y necesitamos consumir el iterador aquí.

Para solucionar este warning y consumir el iterator, usaremos el método
`collect`, que usamos en el Capítulo 12 con `env::args` en el Listado 12-1. Este
método consume el iterador y colecciona los valores resultantes en un tipo de
colección.

En el Listado 13-15, recolectamos los resultados de iterar sobre el iterator
que es retornado de la llamada a `map` en un vector. Este vector terminará
conteniendo cada item del vector original, incrementado en 1.

<Listing number="13-15" file-name="src/main.rs" caption="Llamando al método `map` para crear un nuevo iterador y luego llamando al método `collect` para consumir el nuevo iterador y crear un vector">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-15/src/main.rs:here}}
```

</Listing>

Debido a que `map` toma un closure, podemos especificar cualquier operación que
queramos realizar en cada item. Este es un gran ejemplo de cómo los closures te
permiten personalizar algún comportamiento mientras reutilizas el comportamiento
de iteración que el trait `Iterator` provee.

Puedes encadenar múltiples llamadas a iterador adaptors para realizar acciones
complejas de una manera legible. Pero debido a que todos los iterators son
perezosos, tienes que llamar a uno de los métodos adaptadores consumidores para
obtener resultados de las llamadas a iterador adaptors.

### Usando Closures que Capturan su Entorno

Muchos de los iterador adaptors toman closures como argumentos, y comúnmente los
closures que especificaremos como argumentos a iterador adaptors capturarán su
entorno.

Para este ejemplo, usaremos el método `filter` definido en el trait `Iterator`,
que toma un closure que toma un item y retorna un `bool`. Si el closure retorna
`true`, el valor será incluido en el iterador producido. Si el closure retorna
`false`, el valor no será incluido en el iterador producido.

En el Listado 13-16, usamos `filter` con un closure que captura la variable
`shoe_size` de su entorno para iterar sobre una colección de instancias de la
estructura `Shoe`. Retornará solo los zapatos que sean del tamaño especificado.

<Listing number="13-16" file-name="src/lib.rs" caption="Usando el método `filter` con un closure que captura `shoe_size`">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-16/src/lib.rs}}
```

</Listing>

La función `shoes_in_size` toma ownership de un vector de zapatos y un tamaño de
zapato como parámetros. Retorna un vector que contiene solo zapatos del tamaño
especificado.

En el cuerpo de `shoes_in_size`, llamamos a `into_iter` para crear un iterator
que tome ownership del vector. Luego llamamos a `filter` para adaptar ese
iterador en un nuevo iterador que solo contiene elementos para los cuales el
closure retorna `true`.

El closure captura el parámetro `shoe_size` del entorno y compara el valor con
el tamaño de cada zapato, manteniendo solo los zapatos del tamaño especificado.
Finalmente, llamando a `collect` recolectamos los valores retornados por el
iterador adaptado en un vector que es retornado por la función.

El test muestra que cuando llamamos a `shoes_in_size` con un vector de zapatos
y un tamaño de zapato, obtenemos de vuelta solo los zapatos del tamaño
especificado:
