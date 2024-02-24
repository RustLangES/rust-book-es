## Apéndice C: Traits derivables

En varios lugares del libro, hemos discutido el atributo `derive`, que puede
aplicar a una definición de estructura o enumeración. El atributo `derive`
genera código que implementará un rasgo con su propia implementación
predeterminada en el tipo que ha anotado con la sintaxis `derive`.

En este apéndice, proporcionamos una referencia de todos los traits en la
biblioteca estándar que puede usar con `derive`. Cada sección cubre:

- Qué operadores y métodos que derivan este trait se habilitarán
- Qué hace la implementación del trait proporcionado por `derive`
- Qué significa implementar el trait sobre el tipo
- Las condiciones en las que se le permite o no implementar el trait
- Ejemplos de operaciones que requieren el trait

Si desea un comportamiento diferente al proporcionado por el atributo `derive`,
consulte la documentación de la [biblioteca estándar](../std/index.html)<!-- ignore -->
para cada trait para obtener detalles sobre cómo implementarlos manualmente.

Estos traits enumerados aquí son los únicos definidos por la biblioteca
estándar que se pueden implementar en sus tipos usando `derive`. Otros traits
definidos en la biblioteca estándar no tienen un comportamiento predeterminado
sensato, por lo que depende de usted implementarlos de la manera que tenga
sentido para lo que está tratando de lograr.

Un ejemplo de un trait que no se puede derivar es `Display`, que maneja el
formateo para los usuarios finales. Siempre debe considerar la forma apropiada
de mostrar un tipo a un usuario final. ¿Qué partes del tipo puede ver un
usuario final? ¿Qué partes encontrarían relevantes? ¿Qué formato de los datos
sería más relevante para ellos? El compilador Rust no tiene esta idea, por lo
que no puede proporcionar un comportamiento predeterminado apropiado para
usted.

La lista de traits derivables proporcionada en este apéndice no es
exhaustiva: las bibliotecas pueden implementar `derive` para sus propios
traits, lo que hace que la lista de traits que puede usar `derive` sea
realmente abierta. Implementar `derive` implica usar una macro procedural, que
se cubre en la sección [“Macros”][macros]<!-- ignore --> del Capítulo 19.

### `Debug` para el Output del programador

El trait `Debug` permite el formateo de depuración en cadenas de formato, que
indica agregando `:?` dentro de los marcadores `{}`.

El trait `Debug` te permite imprimir instancias de un tipo con fines de
depuración, para que tú y otros programadores que usen tu tipo puedan
inspeccionar una instancia en un punto particular de la ejecución de un
programa.

El trait `Debug` es necesario, por ejemplo, en el uso de la macro `assert_eq!`.
Esta macro imprime los valores de las instancias dadas como argumentos si la
aserción de igualdad falla, por lo que los programadores pueden ver por qué
las dos instancias no eran iguales.

### `PartialEq` y `Eq` para comparaciones de igualdad

El trait `PartialEq` te permite comparar instancias de un tipo para verificar
la igualdad y habilita el uso de los operadores `==` y `!=`.

Derivar `PartialEq` implementa el método `eq`. Cuando se deriva `PartialEq`
en estructuras, dos instancias son iguales solo si _todos_ los campos son
iguales, y las instancias no son iguales si alguno de los campos no es igual.
Cuando se deriva en enumeraciones, cada variante es igual a sí misma y no
igual a las otras variantes.

El trait `PartialEq` es necesario, por ejemplo, con el uso de la macro
`assert_eq!`, que necesita poder comparar dos instancias de un tipo para la
igualdad.

El trait `Eq` no tiene métodos. Su propósito es señalar que para cada valor
del tipo anotado, el valor es igual a sí mismo. El trait `Eq` solo se puede
aplicar a tipos que también implementan `PartialEq`, aunque no todos los tipos
que implementan `PartialEq` pueden implementar `Eq`. Un ejemplo de esto son
los tipos de números de punto flotante: la implementación de los números de
punto flotante establece que dos instancias del valor no es un número (`NaN`)
no son iguales entre sí.

Un ejemplo de cuando se necesita `Eq` es para las claves en un `HashMap<K, V>`
para que el `HashMap<K, V>` pueda decir si dos claves son iguales.

### `PartialOrd` y `Ord` para comparaciones de orden

El trait `PartialOrd` te permite comparar instancias de un tipo para fines de
ordenación. Un tipo que implementa `PartialOrd` se puede usar con los
operadores `<`, `>`, `<=` y `>=`. Solo puede aplicar el trait `PartialOrd` a
tipos que también implementan `PartialEq`.

Derivar `PartialOrd` implementa el método `partial_cmp`, que devuelve un
`Option<Ordering>` que será `None` cuando los valores dados no produzcan un
orden. Un ejemplo de un valor que no produce un orden, a pesar de que la
mayoría de los valores de ese tipo se pueden comparar, es el valor de punto
flotante no es un número (`NaN`). Llamar a `partial_cmp` con cualquier número
de punto flotante y el valor de punto flotante `NaN` devolverá `None`.

Cuando se deriva en structs, `PartialOrd` compara dos instancias comparando el
valor en cada campo en el orden en que aparecen los campos en la definición
del struct. Cuando se deriva en enums, las variantes del enum declaradas
anteriormente en la definición de la enumeración se consideran menores
que las variantes enumeradas más tarde.

Cuando se deriva en structs, `PartialOrd` compara dos instancias comparando el
valor en cada campo en el orden en que aparecen los campos en la definición
del struct. Cuando se deriva en enums, las variantes del enum declaradas
anteriormente en la definición de la enumeración se consideran menores
que las variantes enumeradas más tarde.

El trait `PartialOrd` es necesario, por ejemplo, para el método `gen_range`
del crate `rand` que genera un valor aleatorio en el rango especificado por
una expresión de rango.

El trait `Ord` permite saber que para cualquier dos valores del tipo anotado,
existirá un orden válido. El trait `Ord` implementa el método `cmp`, que
devuelve un `Ordering` en lugar de un `Option<Ordering>` porque siempre será
posible un orden válido. Solo puede aplicar el trait `Ord` a tipos que también
implementan `PartialOrd` y `Eq` (y `Eq` requiere `PartialEq`). Cuando se
deriva en structs y enums, `cmp` se comporta de la misma manera que la
implementación derivada para `partial_cmp` con `PartialOrd`.

Un ejemplo de cuando se necesita `Ord` es cuando se almacenan valores en un
`BTreeSet<T>`, una estructura de datos que almacena datos basados en el orden
de clasificación de los valores.

### `Clone` y `Copy` para duplicar valores

El trait `Clone` te permite crear explícitamente una copia profunda de un
valor, y el proceso de duplicación puede implicar la ejecución de código
arbitrario y la copia de datos de la pila. Consulte la sección [“Ways
Variables and Data Interact: Clone”][ways-variables-and-data-interact-clone]<!-- ignore -->
en el Capítulo 4 para obtener más información sobre `Clone`.

Derivar `Clone` implementa el método `clone`, que cuando se implementa para
todo el tipo, llama a `clone` en cada una de las partes del tipo. Esto
significa que todos los campos o valores en el tipo también deben implementar
`Clone` para derivar `Clone`.

Un ejemplo de cuando se requiere `Clone` es cuando se llama al método `to_vec`
en una rebanada. La rebanada no posee las instancias de tipo que contiene, pero
el vector devuelto de `to_vec` necesitará poseer sus instancias, por lo que
`to_vec` llama a `clone` en cada elemento. Por lo tanto, el tipo almacenado en
la rebanada debe implementar `Clone`.

El trait `Copy` te permite duplicar un valor copiando solo los bits almacenados
en la pila; no es necesario ningún código arbitrario. Consulte la sección
[“Stack-Only Data: Copy”][solo-datos-del-stack-copiar]<!-- ignore --> en el Capítulo 4
para obtener más información sobre `Copy`.

El trait `Copy` no define ningún método para evitar que los programadores
sobrecarguen esos métodos y violen la suposición de que no se está ejecutando
código arbitrario. De esa manera, todos los programadores pueden asumir que
copiar un valor será muy rápido.

Puede derivar `Copy` en un tipo solo si todas las partes del tipo implementan
`Copy`. Un tipo que implementa `Copy` también debe implementar `Clone`, porque
un tipo que implementa `Copy` tiene una implementación trivial de `Clone` que
realiza la misma tarea que `Copy`.

El trait `Copy` es rara vez requerido; los tipos que implementan `Copy` tienen
optimizaciones disponibles, lo que significa que no tiene que llamar a `clone`,
lo que hace que el código sea más conciso.

Todo lo posible con `Copy` también se puede lograr con `Clone`, pero el código
podría ser más lento o tener que usar `clone` en lugares.

### `Hash` para mapear un valor a un valor de tamaño fijo

El trait `Hash` te permite tomar una instancia de un tipo de tamaño arbitrario
y asignar esa instancia a un valor de tamaño fijo usando una función hash.
Derivar `Hash` implementa el método `hash`. La implementación derivada del
método `hash` combina el resultado de llamar a `hash` en cada una de las partes
del tipo, lo que significa que todos los campos o valores también deben
implementar `Hash` para derivar `Hash`.

Un ejemplo de cuando se requiere `Hash` es en el almacenamiento de claves en
un `HashMap<K, V>` para almacenar datos de manera eficiente.

### `Default` para valores predeterminados

El trait `Default` te permite crear un valor predeterminado para un tipo.
Derivar `Default` implementa la función `default`. La implementación derivada
de la función `default` llama a la función `default` en cada parte del tipo,
lo que significa que todos los campos o valores en el tipo también deben
implementar `Default` para derivar `Default`.

La función `Default::default` es comúnmente usada en combinación con la
sintaxis de actualización de struct discutida en la sección [“Creating
Instances From Other Instances With
Struct Update Syntax”][creating-instances-from-other-instances-with-struct-update-syntax]<!-- ignore -->
en el Capítulo 5. Puede personalizar algunos campos de un struct y luego
establecer y usar un valor predeterminado para el resto de los campos usando
`..Default::default()`.

El trait `Default` es necesario, por ejemplo, cuando se usa el método
`unwrap_or_default` en instancias de `Option<T>`. Si el `Option<T>` es `None`,
el método `unwrap_or_default` devolverá el resultado de `Default::default` para
el tipo `T` almacenado en el `Option<T>`.

[creating-instances-from-other-instances-with-struct-update-syntax]: ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax
[solo-datos-del-stack-copiar]: ch04-01-what-is-ownership.html#solo-datos-del-stack-copiar
[ways-variables-and-data-interact-clone]: ch04-01-what-is-ownership.html#ways-variables-and-data-interact-clone
[macros]: ch19-06-macros.html#macros
