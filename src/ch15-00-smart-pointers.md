# Smart Pointers

Un *puntero* es un concepto general para una variable que contiene una dirección
en memoria. Esta dirección se refiere, o “apunta a,” algún otro dato. El tipo
más común de puntero en Rust es una referencia, la cual aprendiste en el
Capítulo 4. Las referencias son indicadas por el símbolo `&` y toman prestado
el valor al que apuntan. No tienen ninguna capacidad especial más allá de
referirse a datos, y no tienen sobrecarga.

Los *Smart pointers,* por otro lado, son estructuras de datos que actúan como un
puntero, pero también tienen metadatos y capacidades adicionales. El concepto de
smart pointers no es único de Rust: los smart pointers se originaron en C++ y
existen en otros lenguajes también. Rust tiene una variedad de smart pointers
definidos en la biblioteca estándar que proveen funcionalidad más allá de la
proveída por las referencias. Para explorar el concepto general, veremos un
par de ejemplos diferentes de smart pointers, incluyendo un tipo de puntero
*reference counting*. Este puntero te permite permitir que los datos tengan
múltiples propietarios al mantener un registro del número de propietarios y,
cuando no hay propietarios restantes, limpiar los datos.

Rust, con su concepto de propiedad y préstamo, tiene una diferencia adicional
entre referencias y smart pointers: mientras que las referencias solo toman
prestado los datos, en muchos casos, los smart pointers *son dueños* de los
datos a los que apuntan.

Aunque no los llamamos así en ese momento, ya hemos encontrado algunos smart
pointers en este libro, incluyendo `String` y `Vec<T>` en el Capítulo 8. Ambos
estos tipos cuentan como smart pointers porque poseen algo de memoria y te
permiten manipularla. También tienen metadatos y capacidades o garantías
adicionales. `String`, por ejemplo, almacena su capacidad como metadato y tiene
la capacidad adicional de asegurar que sus datos siempre serán UTF-8 válidos.

Los smart pointers usualmente son implementados usando structs. A diferencia de
un struct ordinaria, los smart pointers implementan los traits `Deref` y
`Drop`. El trait `Deref` permite que una instancia de la struct smart pointer se
comporte como una referencia, así que puedes escribir tu código para trabajar
con referencias o smart pointers. El trait `Drop` te permite personalizar el
código que se ejecuta cuando una instancia del smart pointer sale del scope. En
este capítulo, discutiremos ambos traits y demostraremos por qué son
importantes para los smart pointers.

Dado que el patrón de smart pointer es un patrón de diseño general usado
frecuentemente en Rust, este capítulo no cubrirá todos los smart pointers
existentes. Muchas bibliotecas tienen sus propios smart pointers, e incluso
puedes escribir los tuyos. Cubriremos los smart pointers más comunes en la
biblioteca estándar:

* `Box<T>` para asignar valores en el heap
* `Rc<T>`, un tipo de conteo de referencias que permite múltiples ownerships
* `Ref<T>` y `RefMut<T>`, accedidos a través de `RefCell<T>`, un tipo que
  impone las reglas de borrowing en tiempo de ejecución en lugar de tiempo de
  compilación

Además, cubriremos el patrón *interior mutability* donde un tipo inmutable
expone una API para mutar un valor interior. También discutiremos *reference
cycles*: cómo pueden fugar memoria y cómo prevenirlos.

¡Vamos a profundizar en los smart pointers!
