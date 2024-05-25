## Usando el Pasaje de Mensajes para Transferir Datos entre Hilos

Un enfoque cada vez más popular para garantizar una concurrencia segura es
*message passing*, donde los hilos o actores se comunican enviándose mensajes
que contienen datos. Aquí está la idea en un eslogan de [la documentación del
lenguaje Go](https://golang.org/doc/effective_go.html#concurrency): “No se
comunica compartiendo memoria; en su lugar, comparta memoria comunicándose”.

Para lograr la concurrencia mediante el envío de mensajes, la biblioteca
estándar de Rust proporciona una implementación de *canales*. Un canal es un
concepto de programación general por el cual se envían datos de un hilo a
otro.

Puede imaginar un canal en programación como un canal direccional de agua, como
un arroyo o un río. Si pones algo como un patito de goma en un río, viajará
aguas abajo hasta el final de la vía fluvial.

Un canal tiene dos partes: un transmisor y un receptor. La mitad del
transmisor es la ubicación aguas arriba donde pones patitos de goma en el río,
y la mitad del receptor es donde termina el patito de goma aguas abajo. Una
parte de su código llama a métodos en el transmisor con los datos que desea
enviar, y otra parte verifica el extremo receptor para ver si llegan mensajes.
Se dice que un canal está *cerrado* si se elimina la mitad del transmisor o
del receptor.

Aquí, iremos desarrollando un programa que tiene un hilo para generar valores
y enviarlos por un canal, y otro hilo que recibirá los valores e imprimirá
por pantalla. Enviaremos valores simples entre hilos usando un canal para
ilustrar la característica. Una vez que esté familiarizado con la técnica,
podría usar canales para cualquier hilo que necesite comunicarse entre sí,
como un sistema de chat o un sistema donde muchos hilos realizan partes de un
cálculo y envían las partes a un hilo que agrega los resultados.

Primero, en el Listado 16-6, crearemos un canal pero no haremos nada con él.
Tenga en cuenta que esto aún no se compilará porque Rust no puede determinar qué
tipo de valores queremos enviar por el canal.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-06/src/main.rs}}
```

<span class="caption">Listing 16-6: Creando un canal y asignando las dos mitades
a `tx` y `rx`</span>

Creamos un nuevo canal usando la función `mpsc::channel`; `mpsc` significa
*multiple producer, single consumer* (múltiples productores, un solo
consumidor). En resumen, la forma en que la biblioteca estándar de Rust
implementa los canales significa que un canal puede tener múltiples extremos
de *envío* que producen valores, pero solo un extremo de *recepción* que
consume esos valores. Imagínese varios arroyos que fluyen juntos en un gran
río: todo lo que se envía por cualquiera de los arroyos terminará en un río al
final. Comenzaremos con un solo productor por ahora, pero agregaremos
múltiples productores cuando hagamos que este ejemplo funcione.

La función `mpsc::channel` devuelve una tupla, donde el primer elemento es el
extremo de envío, y el segundo elemento es el extremo de recepción. Las
abreviaturas `tx` y `rx` se usan tradicionalmente en muchos campos para
*transmisor* y *receptor* respectivamente, por lo que nombramos nuestras
variables de esa manera para indicar cada extremo. Estamos usando una
sentencia `let` con un patrón que deconstruye las tuplas; discutiremos el uso
de patrones en las sentencias `let` y la deconstrucción en el Capítulo 18. Por
ahora, sepa que usar una sentencia `let` de esta manera es un enfoque
conveniente para extraer las piezas de la tupla devuelta por `mpsc::channel`.

Movamos el extremo de envío a un hilo generado y hagamos que envíe un string
para que el hilo generado se comunique con el hilo principal, como se muestra
en el Listado 16-7. Esto es como poner un patito de goma en el río aguas arriba
o enviar un mensaje de chat de un hilo a otro.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-07/src/main.rs}}
```

<span class="caption">Listing 16-7: Moviendo `tx` a un hilo generado y enviar
“hi”</span>

Nuevamente, estamos usando `thread::spawn` para crear un nuevo hilo y luego
usando `move` para mover `tx` al cierre para que el hilo generado posea `tx`.
El hilo generado necesita poseer el transmisor para poder enviar mensajes a
través del canal. El transmisor tiene un método `send` que toma el valor que
queremos enviar. El método `send` devuelve un tipo `Result<T, E>`, por lo que
si el receptor se ha eliminado y no hay ningún lugar para enviar un valor, la
operación de envío devolverá un error. En este ejemplo, estamos llamando a
`unwrap` para que se produzca un pánico en caso de error. Pero en una
aplicación real, lo manejaríamos correctamente: vuelva al Capítulo 9 para
revisar las estrategias para el manejo adecuado de errores.

En el Listado 16-8, recibiremos el valor enviado en el hilo principal. Esto es
como recibir el patito de goma en el río aguas abajo o recibir un mensaje de
chat.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-08/src/main.rs}}
```

<span class="caption">Listing 16-8: Recibiendo el valor “hi” en el hilo thread
e imprimiéndolo</span>

El receptor tiene dos métodos útiles: `recv` y `try_recv`. Estamos usando
`recv`, abreviatura de *receive* (recibir), que bloqueará la ejecución del
hilo principal y esperará hasta que se envíe un valor por el canal. Una vez que
se envía un valor, `recv` lo devolverá en un `Result<T, E>`. Cuando el
transmisor se cierra, `recv` devolverá un error para indicar que no se
enviarán más valores.

El método `try_recv` no bloquea, sino que en su lugar devuelve un `Result<T,
E>` inmediatamente: un valor `Ok` que contiene un mensaje si hay uno
disponible y un valor `Err` si no hay mensajes esta vez. Usar `try_recv` es
útil si este hilo tiene otro trabajo que hacer mientras espera mensajes:
podríamos escribir un bucle que llame a `try_recv` cada cierto tiempo, maneje
un mensaje si hay uno disponible y, de lo contrario, haga otro trabajo por un
tiempo hasta que vuelva a verificar.

Hemos usado `recv` en este ejemplo por simplicidad; no tenemos otro trabajo
para que haga el hilo principal que esperar mensajes, por lo que bloquear el
hilo principal es apropiado.

Cuando ejecutamos el código en el Listado 16-8, veremos el valor impreso desde
el hilo principal:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Got: hi
```

¡Perfecto!

### Canales y transferencia de Ownership

Las reglas de ownership juegan un papel vital en el envío de mensajes porque
ayudan a escribir código concurrente seguro. Prevenir errores en la
programación concurrente es la ventaja de pensar en el ownership en todos sus
programas Rust. Hagamos un experimento para mostrar cómo los canales y el
ownership funcionan juntos para evitar problemas: intentaremos usar un valor
`val` en el hilo generado *después* de haberlo enviado por el canal. Intente
compilar el código en el Listado 16-9 para ver por qué este código no está
permitido:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-09/src/main.rs}}
```

<span class="caption">Listing 16-9: Attempting to use `val` after we’ve sent it
down the channel</span>

Aquí, intentamos imprimir `val` después de haberlo enviado por el canal a
través de `tx.send`. Permitir esto sería una mala idea: una vez que el valor
se ha enviado a otro hilo, ese hilo podría modificarlo o eliminarlo antes de
que intentemos usar el valor nuevamente. Potencialmente, las modificaciones de
otro hilo podrían causar errores o resultados inesperados debido a datos
inconsistentes o inexistentes. Sin embargo, Rust nos da un error si intentamos
compilar el código en el Listado 16-9:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-09/output.txt}}
```

Nuestro error de concurrencia ha causado un error en tiempo de compilación. La
función `send` toma la propiedad de su parámetro, y cuando se mueve el valor,
el receptor se hace cargo de él. Esto nos impide usar accidentalmente el valor
nuevamente después de enviarlo; el sistema de propiedad verifica que todo
esté bien.

### Enviando múltiples valores y viendo al receptor esperando

El código en el Listado 16-8 compiló y se ejecutó, pero no nos mostró
claramente que dos hilos separados estaban hablando entre sí a través del
canal. En el Listado 16-10 hemos realizado algunas modificaciones que
demostrarán que el código en el Listado 16-8 se está ejecutando
concurrentemente: el hilo generado ahora enviará varios mensajes y se
pausará durante un segundo entre cada mensaje.

<span class="filename">Filename: src/main.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-10/src/main.rs}}
```

<span class="caption">Listing 16-10: Enviando múltiples mensajes y pausando
entre cada uno</span>

Esta vez, el hilo generado tiene un vector de strings que queremos enviar al
hilo principal. Iteramos sobre ellos, enviando cada uno individualmente, y
pausamos entre cada uno llamando a la función `thread::sleep` con un valor
`Duration` de 1 segundo.

En el hilo principal, ya no estamos llamando explícitamente a la función
`recv`: en su lugar, estamos tratando `rx` como un iterator. Para cada valor
recibido, lo imprimimos. Cuando el canal está cerrado, la iteración terminará.

Al ejecutar el código del Listado 16-10, debería ver el siguiente resultado con
una pausa de 1 segundo entre cada línea:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Got: hi
Got: from
Got: the
Got: thread
```

Debido a que no tenemos ningún código que pause o retrase el bucle `for` en el
hilo principal, podemos decir que el hilo principal está esperando recibir
valores del hilo generado.

### Creando múltiples productores clonando el transmisor

Anteriormente mencionamos que `mpsc` era un acrónimo de *multiple producer,
single consumer* (múltiples productores, un solo consumidor). Pongamos `mpsc`
en uso y expandamos el código en el Listado 16-10 para crear múltiples hilos
que envíen valores al mismo receptor. Podemos hacerlo clonando el transmisor,
como se muestra en el Listado 16-11:

<span class="filename">Filename: src/main.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-11/src/main.rs:here}}
```

<span class="caption">Listing 16-11: Envío de múltiples mensajes de múltiples
productores</span>

Esta vez, antes de crear el primer hilo generado, llamamos a `clone` en el
transmisor, lo que nos dará un nuevo transmisor que podemos pasar al primer
hilo generado. Pasamos el transmisor original a un segundo hilo generado. Esto
nos da dos hilos, cada uno enviando mensajes diferentes al receptor.

Cuando ejecutamos el código, tu output debería verse así:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Got: hi
Got: more
Got: from
Got: messages
Got: for
Got: the
Got: thread
Got: you
```

Es posible que veas los valores en otro orden según tu sistema. Esto es lo que 
hace que la concurrencia sea tan interesante como difícil. Si experimentas con
`thread::sleep`, dándole varios valores en los diferentes hilos, cada ejecución
será más no determinista y creará una salida diferente cada vez.

Ahora que hemos visto cómo funcionan los canales, veamos un método diferente de
concurrencia.
