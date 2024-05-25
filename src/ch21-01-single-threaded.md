## Construyendo un servidor web de un solo hilo

Comenzaremos haciendo funcionar un servidor web de un solo hilo. Antes de
comenzar, veamos una breve descripción general de los protocolos involucrados
en la construcción de servidores web. Los detalles de estos protocolos están
fuera del alcance de este libro, pero una breve descripción general le dará la
información que necesita.

Los dos protocolos principales involucrados en los servidores web son *Hypertext
Transfer Protocol* *(HTTP)* y *Transmission Control Protocol* *(TCP)*. Ambos
protocolos son protocolos de *solicitud-respuesta*, lo que significa que un
*cliente* inicia solicitudes y un *servidor* escucha las solicitudes y
proporciona una respuesta al cliente. El contenido de esas solicitudes y
respuestas está definido por los protocolos.

TCP es el protocolo de nivel inferior que describe los detalles de cómo la
información pasa de un servidor a otro, pero no especifica qué es esa
información. HTTP se basa en TCP definiendo el contenido de las solicitudes y
respuestas. Técnicamente, es posible usar HTTP con otros protocolos, pero en la
gran mayoría de los casos, HTTP envía sus datos a través de TCP. Trabajaremos
con los bytes sin procesar de las solicitudes y respuestas de TCP y HTTP.

### Escuchando la conexión TCP

Nuestro servidor web debe escuchar una conexión TCP, por lo que esa es la
primera parte en la que trabajaremos. La biblioteca estándar ofrece un módulo
`std::net` que nos permite hacer esto. Hagamos un nuevo proyecto de la manera
habitual:

```console
$ cargo new hello
     Created binary (application) `hello` project
$ cd hello
```

Ahora agreguemos el código en el Listado 20-1 en *src/main.rs* para comenzar.
Este código escuchará en la dirección local `127.0.0.1:7878` para flujos TCP
entrantes. Cuando recibe un flujo entrante, imprimirá `¡Conexión establecida!`.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-01/src/main.rs}}
```

<span class="caption">Listing 20-1: Escuchar transmisiones entrantes e imprimir
un mensaje cuando recibimos una transmisión</span>

Usando `TcpListener`, podemos escuchar conexiones TCP en la dirección
`127.0.0.1:7878`. En la dirección, la sección antes de los dos puntos es una
dirección IP que representa su computadora (esto es lo mismo en todas las
computadoras y no representa la computadora de los autores en particular), y
`7878` es el puerto. Hemos elegido este puerto por dos razones: HTTP no se
acepta normalmente en este puerto, por lo que es poco probable que nuestro
servidor entre en conflicto con cualquier otro servidor web que pueda tener
ejecutándose en su máquina, y 7878 es *rust* escrito en un teléfono.

La función `bind` en este escenario funciona como la función `new` en que
devolverá una nueva instancia de `TcpListener`. La función se llama `bind`
porque, en redes, conectarse a un puerto para escuchar se conoce como “enlazar
a un puerto”.

La función `bind` devuelve un `Result<T, E>`, que indica que es posible que el
enlace falle. Por ejemplo, conectarse al puerto 80 requiere privilegios de
administrador (los no administradores solo pueden escuchar en puertos superiores
a 1023), por lo que si intentáramos conectarnos al puerto 80 sin ser un
administrador, el enlace no funcionaría. El enlace tampoco funcionaría, por
ejemplo, si ejecutáramos dos instancias de nuestro programa y, por lo tanto,
tuvimos dos programas escuchando el mismo puerto. Debido a que estamos
escribiendo un servidor básico solo con fines de aprendizaje, no nos
preocuparemos por manejar este tipo de errores; en su lugar, usamos `unwrap`
para detener el programa si ocurren errores.

El método `incoming` en `TcpListener` devuelve un iterator que nos da una
secuencia de flujos (más específicamente, flujos de tipo `TcpStream`). Un solo
*flujo* representa una conexión abierta entre el cliente y el servidor. Una
*conexión* es el nombre del proceso de solicitud y respuesta completo en el que
un cliente se conecta al servidor, el servidor genera una respuesta y el
servidor cierra la conexión. Como tal, leeremos del `TcpStream` para ver lo que
el cliente envió y luego escribiremos nuestra respuesta en el flujo para enviar
datos de vuelta al cliente. En general, este bucle `for` procesará cada
conexión a su vez y producirá una serie de flujos para que los manejemos.

Por ahora, nuestro manejo del flujo consiste en llamar a `unwrap` para terminar
nuestro programa si el flujo tienen algún error; si no hay errores, el programa
imprime un mensaje. Agregaremos más funcionalidad para el caso de éxito en el
siguiente listado. La razón por la que podríamos recibir errores del método
`incoming` cuando un cliente se conecta al servidor es que en realidad no
iteramos sobre las conexiones. En cambio, iteramos sobre *intentos de
conexión*. La conexión podría no tener éxito por una serie de razones, muchas
de ellas específicas del sistema operativo. Por ejemplo, muchos sistemas
operativos tienen un límite para el número de conexiones abiertas simultáneas
que pueden admitir; los nuevos intentos de conexión más allá de ese número
producirán un error hasta que algunas de las conexiones abiertas se cierren.

¡Intentemos ejecutar este código! Invoca `cargo run` en la terminal y luego
carga *127.0.0.1:7878* en un navegador web. El navegador debería mostrar un
mensaje de error como “Conexión restablecida”, porque el servidor no está
enviando ningún dato actualmente. ¡Pero cuando miras tu terminal, deberías ver
varios mensajes que se imprimieron cuando el navegador se conectó al servidor!

```text
     Running `target/debug/hello`
Connection established!
Connection established!
Connection established!
```

A veces, verás múltiples mensajes impresos para una solicitud del navegador; la
razón podría ser que el navegador está haciendo una solicitud para la página
además de una solicitud para otros recursos, como el icono *favicon.ico* que
aparece en la pestaña del navegador.

También podría ser que el navegador esté intentando conectarse al servidor
varias veces porque el servidor no está respondiendo con ningún dato. Cuando
`stream` sale del scope y se descarta al final del bucle, la conexión se cierra
como parte de la implementación de `drop`. Los navegadores a veces tratan con
conexiones cerradas volviendo a intentar, porque el problema podría ser
temporal. ¡El factor importante es que hemos obtenido con éxito un controlador
para una conexión TCP!

Recuerda detener el programa presionando <span class="keystroke">ctrl-c</span>
cuando hayas terminado de ejecutar una versión particular del código. Luego
reinicia el programa invocando el comando `cargo run` después de haber hecho
cambios de código para asegurarte de que estás ejecutando el código más nuevo.

### Leyendo la solicitud

¡Vamos a implementar la funcionalidad para leer la solicitud del navegador!. Para
separar las preocupaciones de obtener primero una conexión y luego tomar alguna
acción con la conexión, iniciaremos una nueva función para procesar conexiones.
En esta nueva función `handle_connection`, leeremos datos del flujo TCP e
imprimiremos para que podamos ver los datos que se envían desde el navegador.
Cambia el código para que se vea como el Listado 20-2.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-02/src/main.rs}}
```

<span class="caption">Listing 20-2: Leyendo desde el `TcpStream` e imprimiendo
los datos</span>

Importamos `std::io::prelude` y `std::io::BufReader` para obtener acceso a los
traits y tipos que nos permiten leer del flujo. En el bucle `for` en la función
`main`, en lugar de imprimir un mensaje que diga que hicimos una conexión,
ahora llamamos a la nueva función `handle_connection` y le pasamos el `stream`.

En la función `handle_connection`, creamos una nueva instancia de `BufReader`
que envuelve una referencia mutable al `stream`. `BufReader` agrega
almacenamiento en búfer al administrar las llamadas a los métodos del trait
`std::io::Read` por nosotros.

Creamos una variable llamada `http_request` para recopilar las líneas de la
solicitud que el navegador envía a nuestro servidor. Indicamos que queremos
recopilar estas líneas en un vector agregando la anotación de tipo `Vec<_>`.

`BufReader` implementa el trait `std::io::BufRead`, que proporciona el método
`lines`. El método `lines` devuelve un iterator de `Result<String,
std::io::Error>` al dividir el flujo de datos cada vez que ve un byte de nueva
línea. Para obtener cada `String`, mapeamos y `unwrap` cada `Result`. El
`Result` podría ser un error si los datos no son válidos UTF-8 o si hubo un
problema al leer del flujo. Nuevamente, un programa de producción debería
manejar estos errores de manera más elegante, pero estamos eligiendo detener el
programa en el caso de error por simplicidad.

El navegador señala el final de una solicitud HTTP enviando dos caracteres de
nueva línea seguidos, por lo que para obtener una solicitud del flujo, tomamos
líneas hasta que obtenemos una línea que es el string vacío. Una vez que hemos
recopilado las líneas en el vector, las imprimimos usando el formato de
depuración bonito para que podamos echar un vistazo a las instrucciones que el
navegador web está enviando a nuestro servidor.

¡Probemos este código! Inicia el programa y luego carga realiza una solicitud
en un navegador web nuevamente. Ten en cuenta que aún obtendremos una página de
error en el navegador, pero la salida del programa en la terminal se verá
similar a esto:

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/hello`
Request: [
    "GET / HTTP/1.1",
    "Host: 127.0.0.1:7878",
    "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:99.0) Gecko/20100101 Firefox/99.0",
    "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
    "Accept-Language: en-US,en;q=0.5",
    "Accept-Encoding: gzip, deflate, br",
    "DNT: 1",
    "Connection: keep-alive",
    "Upgrade-Insecure-Requests: 1",
    "Sec-Fetch-Dest: document",
    "Sec-Fetch-Mode: navigate",
    "Sec-Fetch-Site: none",
    "Sec-Fetch-User: ?1",
    "Cache-Control: max-age=0",
]
```

Dependiendo de tu navegador, podrías obtener una salida ligeramente diferente.
Ahora que estamos imprimiendo los datos de la solicitud, podemos ver por qué
obtenemos múltiples conexiones desde una solicitud del navegador al mirar la
ruta después de `GET` en la primera línea de la solicitud. Si las conexiones
repetidas están solicitando */*, sabemos que el navegador está tratando de
obtener */* repetidamente porque no está obteniendo una respuesta de nuestro
programa.

Descompongamos estos datos de solicitud para comprender lo que el navegador
está pidiendo a nuestro programa.

### Una mirada más cercana a una solicitud HTTP

HTTP es un protocolo de texto, y una solicitud toma este formato:

```text
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

La primera línea es la *línea de solicitud* que contiene información sobre lo
que el cliente está solicitando. La primera parte de la línea de solicitud
indica el *método* que se está utilizando, como `GET` o `POST`, que describe
cómo el cliente está haciendo esta solicitud. Nuestro cliente usó una solicitud
`GET`, lo que significa que está solicitando información.

La siguiente parte de la línea de solicitud es */*, que indica el
*Uniform Resource Identifier* *(URI)* que el cliente está solicitando: un URI
es casi, pero no exactamente, lo mismo que un *Uniform Resource Locator*
*(URL)*. La diferencia entre URIs y URLs no es importante para nuestros
propósitos en este capítulo, pero la especificación HTTP usa el término URI,
por lo que podemos simplemente sustituir mentalmente URL por URI aquí.

La última parte es la versión de HTTP que utiliza el cliente, y luego la línea
de solicitud termina en una secuencia *CRLF*. (CRLF significa *carriage return*
y *line feed*, que son términos de los días de la máquina de escribir!) La
secuencia CRLF también se puede escribir como `\r\n`, donde `\r` es un retorno
de carro y `\n` es un avance de línea. La secuencia CRLF separa la línea de
solicitud del resto de los datos de la solicitud. Tenga en cuenta que cuando se
imprime el CRLF, vemos que comienza una nueva línea en lugar de `\r\n`.

Al examinar los datos de la línea de solicitud que hemos recibido al ejecutar
nuestro programa hasta ahora, vemos que `GET` es el método, */* es el URI de
solicitud y `HTTP/1.1` es la versión.

Después de la línea de solicitud, las líneas restantes a partir de `Host:` en
adelante son encabezados. Las solicitudes `GET` no tienen cuerpo.

Intenta hacer una solicitud desde un navegador diferente o solicitar una
dirección diferente, como *127.0.0.1:7878/test*, para ver cómo cambian los
datos de la solicitud.

Ahora que sabemos lo que el navegador está solicitando, ¡enviemos algunos
datos de vuelta!

### Escribiendo una respuesta

Vamos a implementar el envío de datos en respuesta a una solicitud del
cliente. Las respuestas tienen el siguiente formato:

```text
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

La primera línea es una *línea de estado* que contiene la versión HTTP 
utilizada en la respuesta, un código de estado numérico que resume el resultado
de la solicitud y una frase de motivo que proporciona una descripción textual
del código de estado. Después de la secuencia CRLF hay encabezados, otra
secuencia CRLF y el cuerpo de la respuesta.

Aquí hay un ejemplo de respuesta que usa la versión HTTP 1.1, tiene un código
de estado 200, una frase de motivo OK, no tiene encabezados y no tiene cuerpo:

```text
HTTP/1.1 200 OK\r\n\r\n
```

El código de estado 200 es la respuesta de éxito estándar. El texto es una
respuesta HTTP exitosa. ¡Escribamos esto en el flujo como nuestra respuesta a
una solicitud exitosa! Desde la función `handle_connection`, elimine el
`println!` que estaba imprimiendo los datos de la solicitud y reemplácelo con
el código en el Listado 20-3.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-03/src/main.rs:here}}
```

<span class="caption">Listing 20-3: Escribiendo una pequeña respuesta HTTP
exitosa en el flujo de datos</span>

El primer cambio introduce la variable `response`, que contiene los datos
del mensaje de éxito. Luego, llamamos a `as_bytes` en nuestra `response` para
convertir los datos de string en bytes. El método `write_all` en `stream` toma
un `&[u8]` y envía esos bytes directamente por la conexión. Debido a que la
operación `write_all` podría fallar, usamos `unwrap` en cualquier resultado de
error como antes. Nuevamente, en una aplicación real agregarías manejo de
errores aquí.

Con estos cambios, ejecutemos nuestro código y hagamos una solicitud. Como ya
no estamos imprimiendo ningún dato en la terminal, no veremos ninguna salida
aparte de la salida generada por Cargo. Cuando cargues *127.0.0.1:7878* en un
navegador web, deberías ver una página en blanco en lugar de un error. ¡Acabas
de codificar a mano la recepción de una solicitud HTTP y el envío de una
respuesta!

### Devolviendo HTML real

Vamos a implementar la funcionalidad para devolver algo más que una página en
blanco. Crea el nuevo archivo *hello.html* en la raíz de tu directorio del
proyecto, no en el directorio *src*. Puedes introducir cualquier HTML que
quieras; el Listado 20-4 muestra una posibilidad.

<span class="filename">Filename: hello.html</span>

```html
{{#include ../listings/ch20-web-server/listing-20-05/hello.html}}
```

<span class="caption">Listing 20-4: Un ejemplo de archivo HTML para devolver en 
una respuesta</span>

Esto es un documento HTML5 mínimo con un encabezado y un poco de texto. Para
devolver esto desde el servidor cuando se recibe una solicitud, modificaremos
`handle_connection` como se muestra en el Listado 20-5 para leer el archivo
HTML, agregarlo a la respuesta como un cuerpo y enviarlo.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-05/src/main.rs:here}}
```

<span class="caption">Listing 20-5: Enviando el contenido de *hello.html* como 
el cuerpo de la respuesta</span>

Hemos agregado `fs` a la declaración `use` para traer el módulo del sistema de
archivos de la biblioteca estándar al scope. El código para leer el contenido
de un archivo a una cadena debería ser familiar; lo usamos en el Capítulo 12
cuando leímos el contenido de un archivo para nuestro proyecto de I/O en el
Listado 12-4.

A continuación, utilizamos `format!` para agregar el contenido del archivo como
el cuerpo de la respuesta de éxito. Para asegurar una respuesta HTTP válida,
agregamos el encabezado `Content-Length` que se establece en el tamaño del
cuerpo de nuestra respuesta, en este caso el tamaño de `hello.html`.

Ejecuta este código con `cargo run` y carga *127.0.0.1:7878* en tu navegador;
¡Deberías ver tu HTML renderizado!

Actualmente, estamos ignorando los datos de la solicitud en `http_request` y
enviando de vuelta el contenido del archivo HTML incondicionalmente. Eso
significa que si intentas solicitar *127.0.0.1:7878/something-else* en tu
navegador, aún obtendrás esta misma respuesta HTML. En este momento, nuestro
servidor es muy limitado y no hace lo que hacen la mayoría de los servidores
web. Queremos personalizar nuestras respuestas dependiendo de la solicitud y
solo enviar el archivo HTML para una solicitud bien formada a */*.

### Validando la solicitud y respondiendo selectivamente

En este momento, nuestro servidor web devolverá el HTML del archivo sin 
importar lo que el cliente haya solicitado. Agreguemos funcionalidad para
verificar que el navegador esté solicitando */* antes de devolver el archivo
HTML y devolver un error si el navegador solicita cualquier otra cosa. Para
esto necesitamos modificar `handle_connection`, como se muestra en el Listado
20-6. Este nuevo código verifica el contenido de la solicitud recibida contra
lo que sabemos que se parece una solicitud para */* y agrega bloques `if` y
`else` para tratar las solicitudes de manera diferente.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-06/src/main.rs:here}}
```

<span class="caption">Listing 20-6: Tratar las solicitudes a */* de manera 
diferente a las demás solicitudes</span>

Solo vamos a analizar la primera línea de la solicitud HTTP, por lo que en 
lugar de leer toda la solicitud en un vector, estamos llamando a `next` para
obtener el primer elemento del iterator. El primer `unwrap` se encarga de la
`Option` y detiene el programa si el iterator no tiene elementos. El segundo
`unwrap` maneja el `Result` y tiene el mismo efecto que el `unwrap` que estaba
en el `map` agregado en el Listado 20-2.

A continuación, verificamos si la `request_line` es igual a la línea de
solicitud de una solicitud GET a la ruta */**. Si es así, el bloque `if`
devuelve el contenido de nuestro archivo HTML.

Si la `request_line` no es igual a la línea de solicitud GET al camino */*,
significa que hemos recibido alguna otra solicitud. Agregaremos código al
bloque `else` en un momento para responder a todas las demás solicitudes.

Ejecuta este código ahora y solicita *127.0.0.1:7878*; deberías ver el HTML
en *hello.html*. Si haces cualquier otra solicitud, como 
*127.0.0.1:7878/something-else*, obtendrás un error de conexión como los que
viste al ejecutar el código en el Listado 20-1 y el Listado 20-2.

Ahora agreguemos el código del Listado 20-7 al bloque `else` para devolver
una respuesta con el código de estado 404, que indica que el contenido de la
solicitud no se encontró. También devolveremos un poco de HTML para una página
que se renderizará en el navegador indicando la respuesta al usuario final.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-07/src/main.rs:here}}
```

<span class="caption">Listing 20-7: Respondiendo con el código de estado 404 y
una página de error si se solicita algo distinto a */*</span>

Aquí, nuestra respuesta tiene una línea de estado con el código de estado 404
y la frase de motivo `NOT FOUND`. El cuerpo de la respuesta será el HTML en el
archivo *404.html*. Necesitarás crear un archivo *404.html* junto a
*hello.html* para la página de error; nuevamente, siéntete libre de usar
cualquier HTML que desees o usa el HTML de ejemplo en el Listado 20-8.

<span class="filename">Filename: 404.html</span>

```html
{{#include ../listings/ch20-web-server/listing-20-07/404.html}}
```

<span class="caption">Listing 20-8: Contenido de ejemplo para la página que
se enviará como respuesta en cualquier caso de error 404</span>

Con estos cambios, ejecuta tu servidor nuevamente. Al solicitar *127.0.0.1:7878*
deberías obtener el contenido de *hello.html*, y cualquier otra solicitud,
como *127.0.0.1:7878/foo*, debería devolver el HTML de *404.html*.

### Un toque de refactorización

En este momento, los bloques `if` y `else` tienen mucha repetición: ambos
están leyendo archivos y escribiendo el contenido de los archivos en el
stream. Las únicas diferencias son la línea de estado y el nombre del archivo.
Hagamos que el código sea más conciso extrayendo esas diferencias en líneas
`if` y `else` separadas que asignarán los valores de la línea de estado y el
nombre del archivo a variables; luego podemos usar esas variables
incondicionalmente en el código para leer el archivo y escribir la respuesta.
El Listado 20-9 muestra el código resultante después de reemplazar los grandes
bloques `if` y `else`.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-09/src/main.rs:here}}
```

<span class="caption">Listing 20-9: Refactorizando los bloques `if` y `else`
para que contengan solo el código que difiere entre los dos casos</span>

Ahora los bloques `if` y `else` solo devuelven los valores apropiados para la
línea de estado y el nombre de archivo en una tupla; luego usamos la
destructuración para asignar estos dos valores a `status_line` y `filename`
usando un patrón en la declaración `let`, como se discutió en el Capítulo 18.

El código previamente duplicado ahora está fuera de los bloques `if` y `else`
y usa las variables `status_line` y `filename`. Esto hace que sea más fácil
ver la diferencia entre los dos casos, y significa que solo tenemos un lugar
para actualizar el código si queremos cambiar la forma en que funciona la
lectura de archivos y la escritura de respuestas. El comportamiento del código
en el Listado 20-9 será el mismo que el del Listado 20-7.

¡Increíble! Ahora tenemos un servidor web simple en aproximadamente 40 líneas
de código Rust que responde a una solicitud con una página de contenido y
responde a todas las demás solicitudes con una respuesta 404.

Actualmente, nuestro servidor se ejecuta en un solo hilo, lo que significa que
solo puede atender una solicitud a la vez. Analicemos cómo esto puede ser un
problema al simular algunas solicitudes lentas. Luego lo arreglaremos para que
nuestro servidor pueda manejar múltiples solicitudes a la vez.
