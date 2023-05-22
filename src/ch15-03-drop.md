## Ejecutando código en la limpieza con el trait `Drop`

El segundo trait importante para el patrón de smart pointer es `Drop`, el cual
permite personalizar qué pasa cuando un valor está a punto de salir del scope.
Puedes proveer una implementación para el trait `Drop` en cualquier tipo, y ese
código puede ser usado para liberar recursos como archivos o conexiones de
red.

Estamos introduciendo `Drop` en el contexto de smart pointers porque la
funcionalidad del trait `Drop` es casi siempre usada cuando se implementa un
smart pointer. Por ejemplo, cuando un `Box<T>` es dropeado, desasignará el
espacio en el heap al que el box apunta.

En algunos lenguajes, para algunos tipos, el programador debe llamar código
para liberar memoria o recursos cada vez que terminan de usar una instancia de
esos tipos. Ejemplos incluyen manejadores de archivos, sockets, o locks. Si se
olvidan, el sistema podría sobrecargarse y colapsar. En Rust, puedes especificar
que un pedazo particular de código sea ejecutado cada vez que un valor sale del
scope, y el compilador insertará este código automáticamente. Como resultado,
no necesitas ser cuidadoso sobre colocar código de limpieza en todos lados en
un programa que una instancia de un tipo particular está terminada con él—¡aún
no se fugarán recursos!

Puedes especificar el código a ejecutar cuando un valor sale del scope
implementando el trait `Drop`. El trait `Drop` requiere que implementes un
método llamado `drop` que toma una referencia mutable a `self`. Para ver cuándo
Rust llama a `drop`, implementemos `drop` con declaraciones `println!` por
ahora. 

Listing 15-14 muestra una estructura `CustomSmartPointer` cuya única
funcionalidad personalizada es que imprimirá `Dropping CustomSmartPointer!`
cuando la instancia sale del scope, para mostrar cuándo Rust ejecuta la
función `drop`.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-14/src/main.rs}}
```

<span class="caption">Listing 15-14: Un struct `CustomSmartPointer` que
implementa el trait `Drop` donde colocaríamos nuestro código de limpieza</span>

El trait `Drop` está incluido en el prelude, así que no necesitamos traerlo al
scope. Implementamos el trait `Drop` en `CustomSmartPointer` y proveemos una
implementación para el método `drop` que llama a `println!`. El cuerpo de la
función `drop` es donde colocarías cualquier lógica que quisieras correr cuando
una instancia de tu tipo sale del scope. Estamos imprimiendo un texto aquí para
demostrar visualmente cuándo Rust llamará a `drop`.

En `main`, creamos dos instancias de `CustomSmartPointer` y luego imprimimos
`CustomSmartPointers created`. Al final de `main`, nuestras instancias de
`CustomSmartPointer` saldrán del scope, y Rust llamará al código que colocamos
en el método `drop`, imprimiendo nuestro mensaje final. Nota que no necesitamos
llamar al método `drop` explícitamente.

Cuando ejecutemos este programa, veremos el siguiente output:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-14/output.txt}}
```

Rust automáticamente llamó a `drop` para nosotros cuando nuestras instancias
salieron del scope, llamando al código que especificamos. Las variables son
dropeadas en el orden inverso a su creación, así que `d` fue dropeada antes que
`c`. El propósito de este ejemplo es darte una guía visual de cómo funciona el
método `drop`; usualmente especificarías el código de limpieza que tu tipo
necesita correr en lugar de un mensaje de impresión.

### Droppeando un valor temprano con `std::mem::drop`

Desafortunadamente, no es sencillo deshabilitar la funcionalidad automática de
`drop`. Deshabilitar `drop` usualmente no es necesario; el punto entero del
trait `Drop` es que se encarga automáticamente. Ocasionalmente, sin embargo,
podrías querer limpiar un valor temprano. Un ejemplo es cuando usas smart
pointers que manejan locks: podrías querer forzar el método `drop` que libera
el lock para que otro código en el mismo scope pueda adquirir el lock. Rust no
te deja llamar al método `drop` del trait `Drop` manualmente; en lugar de eso
tienes que llamar a la función `std::mem::drop` provista por la librería
estándar si quieres forzar a un valor a ser dropeado antes del final de su
scope.

Si intentamos llamar manualmente al método `drop` del trait `Drop` modificando 
la función `main` del Listado 15-14, como se muestra en el Listado 15-15, 
obtendremos un error del compilador:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-15/src/main.rs:here}}
```

<span class="caption">Listing 15-15: Intento de llamar manualmente al método
`drop` del trait `Drop` para limpiar de forma anticipada</span>

When we try to compile this code, we’ll get this error:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-15/output.txt}}
```

Este mensaje de error indica que no se nos permite llamar a `drop` 
explícitamente. El mensaje de error usa el término *destructor*, que es el 
término general de programación para una función que limpia una instancia. 
Un *destructor* es análogo a un *constructor*, que crea una instancia. 
La función `drop` en Rust es un destructor particular.

Rust no nos deja llamar a `drop` explícitamente porque Rust llamaría
automáticamente a `drop` en el valor al final de `main`. Esto causaría un error
de *double free* porque Rust intentaría limpiar el mismo valor dos veces.

No podemos desactivar la inserción automática de `drop` cuando un valor sale
del scope, y no podemos llamar explícitamente al método `drop`. Así que, si
necesitamos forzar a un valor a ser limpiado temprano, usamos la función
`std::mem::drop`.

La función `std::mem::drop` es diferente del método `drop` en el trait `Drop`.
La llamamos pasando como argumento el valor que queremos forzar a dropear. La
función está en el prelude, así que podemos modificar `main` en el Listado
15-15 para llamar a la función `drop`, como se muestra en el Listado 15-16:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-16/src/main.rs:here}}
```

<span class="caption">Listing 15-16: Llamando a `std::mem::drop` para eliminar
explícitamente un valor antes de que salga del scope</span>

Ejecutar este código imprimirá lo siguiente:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-16/output.txt}}
```

El texto ```Dropping CustomSmartPointer with data `some data`!``` es impreso
entre el texto `CustomSmartPointer created.` y `CustomSmartPointer dropped
before the end of main.`, mostrando que el código del método `drop` es llamado
para dropear `c` en ese punto.

Puedes utilizar código especificado en una implementación del trait `Drop` de
varias maneras para hacer la limpieza conveniente y segura: por ejemplo,
¡podrías usarlo para crear tu propio allocator de memoria! Con el trait `Drop`
y el sistema de ownership de Rust, no tienes que recordar limpiar porque Rust
lo hace automáticamente.

Tampoco tienes que preocuparte por problemas que surjan de limpiar 
accidentalmente valores que aún están en uso: el sistema de ownership que
asegura que las referencias siempre sean válidas también asegura que `drop` sea
llamado solo una vez cuando el valor ya no está siendo usado.

Ahora que hemos examinado `Box<T>` y algunas de las características de los
smart pointers, veamos algunos otros smart pointers definidos en la librería
estándar.
