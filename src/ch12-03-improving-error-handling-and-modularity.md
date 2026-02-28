## Refactorizando para mejorar la modularidad y el manejo de errores

Para mejorar nuestro programa, solucionaremos cuatro problemas que tienen que
ver con la estructura del programa y cómo maneja los errores potenciales. En
primer lugar, nuestra función `main` ahora realiza dos tareas: analiza los
argumentos y lee los archivos. A medida que nuestro programa crece, el número
de tareas separadas que maneja la función `main` aumentará. A medida que una
función adquiere responsabilidades, se vuelve más difícil de razonar, más
difícil de probar y más difícil de cambiar sin romper una de sus partes. Es
mejor separar la funcionalidad para que cada función sea responsable de una
tarea.

Este problema también está relacionado con el segundo problema: aunque `query`
y `file_path` son variables de configuración para nuestro programa, variables
como `contents` se utilizan para realizar la lógica del programa. Cuanto más
largo sea `main`, más variables necesitaremos para traer al alcance; Cuantas
más variables tengamos en el alcance, más difícil será realizar un seguimiento
del propósito de cada una. Es mejor agrupar las variables de configuración en
una estructura para que su propósito quede claro.

El tercer problema es que hemos usado `expect` para imprimir un mensaje de
error cuando falla la lectura del archivo, pero el mensaje de error solo
imprime `Should have been able to read the file`. La lectura de un archivo
puede fallar de varias maneras: por ejemplo, el archivo podría faltar, o
podríamos no tener permiso para abrirlo. En este momento, independientemente
de la situación, imprimiríamos el mismo mensaje de error para todo, ¡lo que no
le daría al usuario ninguna información!

Cuarto, usamos `expect` repetidamente para manejar un error, y si el
usuario ejecuta nuestro programa sin especificar suficientes argumentos,
obtendrán un error de `índice fuera de límites` de Rust que no explica
claramente el problema. Sería mejor si todo el código de manejo de errores
estuviera en un solo lugar para que los futuros mantenedores tuvieran un solo
lugar para consultar el código si la lógica de manejo de errores necesitaba
cambiar. Tener todo el código de manejo de errores en un solo lugar también
asegurará que estamos imprimiendo mensajes que serán significativos para
nuestros usuarios finales.

Abordemos estos cuatro problemas refactorizando nuestro proyecto.

### Separacion de preocupaciones para proyectos binarios

El problema organizativo de asignar la responsabilidad de múltiples tareas a la
función `main` es común a muchos proyectos binarios. Como resultado, la
comunidad de Rust ha desarrollado pautas para dividir las preocupaciones
separadas de un programa binario cuando `main` comienza a crecer. Este proceso
tiene los siguientes pasos:

- Divide tu programa en un archivo _main.rs_ y un archivo _lib.rs_ y mueve la 
  lógica de tu programa a _lib.rs_.
- Mientras la lógica de análisis de línea de comandos sea pequeña, puede
  permanecer en _main.rs_.
- Cuando la lógica de análisis de línea de comandos comience a complicarse,
  extráela de _main.rs_ y muévala a _lib.rs_.

Las responsabilidades que quedan en la función `main` después de este proceso
deberían limitarse a lo siguiente:

- Llamar a la lógica de análisis de línea de comandos con los valores de
  argumento
- Configuración de cualquier otra configuración
- Llamando a una función `run` en _lib.rs_
- Manejo del error si `run` devuelve un error

Este patrón se trata de separar las preocupaciones: _main.rs_ maneja la
ejecución del programa, y _lib.rs_ maneja toda la lógica de la tarea en
cuestión. Debido a que no puede probar la función `main` directamente, esta
estructura le permite probar toda la lógica de su programa moviéndola a
funciones en _lib.rs_. El código que permanece en _main.rs_ será lo
suficientemente pequeño como para verificar su corrección leyéndolo. Rehagamos
nuestro programa siguiendo este proceso.

#### Extracción del parser de argumentos

Extraeremos la funcionalidad para analizar los argumentos en una función que
`main` llamará para prepararse para mover la lógica de análisis de línea de
comandos a _src/lib.rs_. La lista 12-5 muestra el nuevo inicio de `main` que
llama a una nueva función `parse_config`, que definiremos en _src/main.rs_ por
el momento.

<Listing number="12-5" file-name="src/main.rs" caption="Extrayendo una función `parse_config` de `main`">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-05/src/main.rs:here}}
```

</Listing>

En este cambio, aún estamos recopilando los argumentos de la línea de comandos
en un vector, pero en lugar de asignar el valor del argumento en el índice 1 a
la variable `query` y el valor del argumento en el índice 2 a la variable
`file_path` dentro de la función `main`, pasamos todo el vector a la función
`parse_config`. La función `parse_config` luego tiene la lógica que determina
qué argumento va en qué variable y pasa los valores de vuelta a `main`. Todavía
creamos las variables `query` y `file_path` en `main`, pero `main` ya no tiene
la responsabilidad de determinar cómo se corresponden los argumentos de la
línea de comandos y las variables.

Esta reorganización puede parecer excesiva para nuestro pequeño programa, pero
estamos refactorizando en pequeños pasos incrementales. Después de hacer este
cambio, ejecute el programa nuevamente para verificar que el análisis de
argumentos aún funcione. Es bueno verificar su progreso con frecuencia, para
ayudar a identificar la causa de los problemas cuando ocurren.

#### Agrupación de valores de configuración

Podemos dar otro pequeño paso para mejorar aún más la función `parse_config`.
En este momento, estamos devolviendo una tupla, pero luego rompemos esa tupla
en partes individuales nuevamente. Esto es una señal de que tal vez no
tenemos la abstracción correcta todavía.

Otro indicador que muestra que hay margen de mejora es la parte `config` de
`parse_config`, que implica que los dos valores que devolvemos están
relacionados y ambos son parte de un valor de configuración. Actualmente, no
estamos transmitiendo este significado en el struct de los datos que no sea
agrupar los dos valores en una tupla; en su lugar, pondremos los dos valores en
un struct y daremos a cada uno de los campos del struct un nombre significativo.
Hacerlo hará que sea más fácil para los futuros mantenedores de este código
comprender cómo se relacionan los diferentes valores entre sí y cuál es su
propósito.

Listing 12-6 muestra las mejoras a la función `parse_config`.

<Listing number="12-6" file-name="src/main.rs" caption="Refactorizando `parse_config` para que devuelva una instancia de un struct `Config`">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-06/src/main.rs:here}}
```

</Listing>

Hemos agregado un struct llamado `Config` definido para tener campos llamados
`query` y `file_path`. La firma de `parse_config` ahora índica que devuelve un
valor `Config`. En el cuerpo de `parse_config`, donde solíamos devolver
rebanadas de cadena que hacen referencia a valores `String` en `args`, ahora
definimos `Config` para contener valores `String` de propiedad. La variable
`args` en `main` es el propietario de los valores de argumento y solo permite
que la función `parse_config` los pida prestados, lo que significa que
violaríamos las reglas de préstamo de Rust si `Config` intentara tomar
posesión de los valores en `args`.

Hay varias formas de administrar los datos de `String`; la más fácil, aunque
algo ineficiente, es llamar al método `clone` en los valores. Esto hará una
copia completa de los datos para que la instancia de `Config` posea, lo que
toma más tiempo y memoria que almacenar una referencia a los datos de string.
Sin embargo, clonar los datos también hace que nuestro código sea muy
sencillo porque no tenemos que administrar los lifetimes de las referencias; en
estas circunstancias, renunciar a un poco de rendimiento para ganar simplicidad
es un intercambio válido.

> ### Los intercambios de usar `clone`
>
> Hay una tendencia entre muchos Rustaceans a evitar usar `clone` para
> solucionar problemas de propiedad debido a su costo de tiempo de ejecución.
> En [el capítulo 13][ch13]<!-- ignore -->, aprenderá a usar métodos más
> eficientes en este tipo de situaciones. Pero por ahora, está bien copiar
> algunas cadenas para seguir progresando porque solo harás estas copias una
> vez y tu ruta de archivo y cadena de consulta son muy pequeñas. Es mejor
> tener un programa que funcione un poco ineficiente que intentar
> hiperoptimizar el código en tu primer paso. A medida que adquieras más
> experiencia con Rust, será más fácil comenzar con la solución más eficiente,
>  pero por ahora, es perfectamente aceptable llamar a `clone`.

Hemos actualizado `main` para que coloque la instancia de `Config` devuelta por
`parse_config` en una variable llamada `config`, y hemos actualizado el código
que anteriormente usaba las variables separadas `query` y `file_path` para que
ahora use los campos en el struct `Config` en su lugar.

Ahora nuestro código transmite más claramente que `query` y `file_path` están
relacionados y que su propósito es configurar cómo funcionará el programa.
Cualquier código que use estos valores sabe que debe buscarlos en la instancia
`config` en los campos nombrados por su propósito.

#### Creando un constructor para `Config`

Hasta ahora, hemos extraído la lógica responsable de analizar los argumentos de
la línea de comandos de `main` y la hemos colocado en la función `parse_config`.
Hacerlo nos ayudó a ver que los valores `query` y `file_path` estaban
relacionados y que esa relación debería transmitirse en nuestro código. Luego
agregamos un struct `Config` para nombrar el propósito relacionado de `query`
y `file_path` y poder devolver los nombres de los valores como nombres de campo
de struct desde la función `parse_config`.

Así que ahora el propósito de la función `parse_config` es crear una instancia
de `Config`, podemos cambiar `parse_config` de una función normal a una función
llama `new` que es asociada con `Config`. que esté asociada con el struct
`Config`. Haciendo este cambio, el código será más idiomático. Podemos crear
instancias de tipos en la biblioteca estándar, como `String`, llamando a
`String::new`. De manera similar, al cambiar `parse_config` a una función
asociada con `Config`, podremos crear instancias de `Config` llamando a
`Config::new`. El listado 12-7 muestra los cambios que debemos hacer.

<Listing number="12-7" file-name="src/main.rs" caption="Cambiando `parse_config` a `Config::new`">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-07/src/main.rs:here}}
```

</Listing>

Hemos actualizado `main` donde estábamos llamando a `parse_config` para que en
su lugar llame a `Config::new`. Hemos cambiado el nombre de `parse_config` a
`new` y lo hemos movido dentro de un bloque `impl`, que asocia la función `new`
con `Config`. Intenta compilar este código nuevamente para asegurarte de que
funciona.

### Arreglando el manejo de errores

Ahora trabajaremos en la corrección de nuestro manejo de errores. Recuerda que
intentar acceder a los valores en el vector `args` en el índice 1 o el índice
2 hará que el programa entre en pánico si el vector contiene menos de tres
elementos. Intenta ejecutar el programa sin ningún argumento; se verá así:

```console
{{#include ../listings/ch12-an-io-project/listing-12-07/output.txt}}
```

La línea `index out of bounds: the len is 1 but the index is 1` es un mensaje
de error destinado a los programadores. No ayudará a nuestros usuarios finales
a comprender lo que deben hacer en su lugar. Arreglemos eso ahora.

#### Mejorando el mensaje de error

En el Listado 12-8, agregamos una verificación en la función `new` que
verificará que el slice sea lo suficientemente largo antes de acceder al índice
1 y 2. Si el slice no es lo suficientemente largo, el programa entra en pánico
y muestra un mensaje de error mejor.

<Listing number="12-8" file-name="src/main.rs" caption="Agregando una verificación para el número de argumentos">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-08/src/main.rs:here}}
```

</Listing>

Este código es similar a [la función `Guess::new` que escribimos en el Listado
9-13][ch9-custom-types]<!-- ignore -->, donde llamamos a `panic!` cuando el
argumento `value` estaba fuera del rango de valores válidos. En lugar de
verificar un rango de valores aquí, estamos verificando que la longitud de
`args` sea al menos `3` y el resto de la función puede operar bajo la suposición
de que esta condición se ha cumplido. Si `args` tiene menos de tres elementos,
esta condición será verdadera y llamaremos a la macro `panic!` para finalizar
el programa inmediatamente.

Con estas pocas líneas de código adicionales en `new`, ejecutemos el programa
sin ningún argumento nuevamente para ver cómo se ve el error ahora:

```console
{{#include ../listings/ch12-an-io-project/listing-12-08/output.txt}}
```

Este output es mejor: ahora tenemos un mensaje de error razonable. Sin embargo,
también tenemos información superflua que no queremos dar a nuestros usuarios.
Quizás usar la técnica que usamos en el Listado 9-13 no es la mejor para usar
aquí: una llamada a `panic!` es más apropiada para un problema de programación
que para un problema de uso,
[como se discutió en el Capítulo 9][ch9-error-guidelines]<!-- ignore -->.
En su lugar, usaremos la otra técnica que aprendiste en el Capítulo 9:
[devolver un `Result`][ch9-result]<!-- ignore --> que indique el éxito o un
error.

<!-- Old headings. Do not remove or links may break. -->


<a id="returning-a-result-from-new-instead-of-calling-panic"></a>

#### Devolver un `Result` en lugar de llamar a `panic!`

En su lugar, podemos devolver un `Result` que contendrá una instancia de
`Config` en el caso de éxito y describirá el problema en el caso de error.
También cambiaremos el nombre de la función de `new` a `build` porque muchos
programadores esperan que las funciones `new` nunca fallen. Cuando
`Config::build` se comunique con `main`, podemos usar el tipo `Result` para
señalar que hubo un problema. Luego podemos cambiar `main` para convertir una
variante `Err` en un error más práctico para nuestros usuarios sin el texto
circundante sobre `thread 'main'` y `RUST_BACKTRACE` que una llamada a `panic!`
provoca.

El Listado 12-9 muestra los cambios que debemos hacer en el valor de retorno de
la función que ahora llamamos `Config::build` y el cuerpo de la función
necesario para devolver un `Result`. Ten en cuenta que esto no se compilará
hasta que actualicemos `main` también, lo cual haremos en el siguiente
listado.

<Listing number="12-9" file-name="src/main.rs" caption="Devolviendo un `Result` desde `Config::build`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-09/src/main.rs:here}}
```

</Listing>

Nuestra función `build` devuelve un `Result` con una instancia de `Config` en
el caso de éxito y una referencia a un string en el caso de error. Nuestros
valores de error siempre serán string literals que tengan el lifetime `'static`.

Hemos hecho dos cambios en el cuerpo de la función: en lugar de llamar a
`panic!` cuando el usuario no pasa suficientes argumentos, ahora devolvemos un
valor `Err`, y hemos envuelto el valor de retorno `Config` en un `Ok`. Estos
cambios hacen que la función se ajuste a su nueva firma de tipo.

Devolviendo un valor `Err` desde `Config::build` permite que la función
`main` maneje el `Result` devuelto por la función `build` y salga del proceso
de manera más limpia en el caso de error.

<!-- Old headings. Do not remove or links may break. -->


<a id="calling-confignew-and-handling-errors"></a>

#### Llamando a `Config::build` y manejando errores

Para manejar el caso de error e imprimir un mensaje amigable para el usuario,
necesitamos actualizar `main` para manejar el `Result` que devuelve
`Config::build`, como se muestra en el Listado 12-10. También tomaremos la
responsabilidad de salir de la herramienta de línea de comandos con un código
de error distinto de cero de `panic!` e implementarlo a mano. Un estado de
salida distinto de cero es una convención para señalar al proceso que llamó a
nuestro programa que el programa salió con un estado de error.

<Listing number="12-10" file-name="src/main.rs" caption="Saliendo con un código de error si falla la construcción de una `Config`">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-10/src/main.rs:here}}
```

</Listing>

En este listado, hemos usado un método que aún no hemos cubierto en detalle:
`unwrap_or_else`, que está definido en `Result<T, E>` por la biblioteca
estándar. Usar `unwrap_or_else` nos permite definir un manejo de errores
personalizado que no sea `panic!`. Si el `Result` es un valor `Ok`, el
comportamiento de este método es similar a `unwrap`: devuelve el valor interno
que `Ok` está envolviendo. Sin embargo, si el valor es un valor `Err`, este
método llama al código en el _closure_, que es una función anónima que
definimos y pasamos como argumento a `unwrap_or_else`. Cubriremos los closures
con más detalle en el [Capítulo 13][ch13]<!-- ignore -->. Por ahora, solo
necesitas saber que `unwrap_or_else` pasará el valor interno del `Err`, que en
este caso es el string estático `"not enough arguments"` que agregamos en el
Listado 12-9, a nuestro closure en el argumento `err` que aparece entre las
barras verticales `|`. El código en el closure imprime el valor de `err` cuando
se ejecuta.

Hemos agregado una nueva línea `use` para traer `process` de la biblioteca
estándar al alcance. El código en el closure que se ejecutará en el caso de
error es solo de dos líneas: imprimimos el valor de `err` y luego llamamos a
`process::exit`. La función `process::exit` detendrá el programa
inmediatamente y devolverá el número que se pasó como código de estado de
salida. Esto es similar al manejo basado en `panic!` que usamos en el Listado
12-8, pero ya no obtenemos todo el output extra. ¡Probémoslo!

```console
{{#include ../listings/ch12-an-io-project/listing-12-10/output.txt}}
```

¡Genial! Este output es mucho más amigable para nuestros usuarios.

### Extrayendo la lógica de `main`

Ahora que hemos terminado de refactorizar el análisis de configuración, pasemos
a la lógica del programa. Como dijimos en [“Separación de preocupaciones para
proyectos
binarios”](#separacion-de-preocupaciones-para-proyectos-binarios)<!-- ignore -->
, extraeremos una función llamada `run` que contendrá toda la lógica actualmente
en la función `main` que no está involucrada con la configuración o el manejo
de errores. Cuando terminemos, `main` será conciso y fácil de verificar por
inspección, y podremos escribir pruebas para toda la otra lógica.

El Listado 12-11 muestra la función `run` extraída. Por ahora, solo estamos
haciendo la pequeña mejora incremental de extraer la función. Todavía estamos
definiendo la función en _src/main.rs_.

<Listing number="12-11" file-name="src/main.rs" caption="Extrayendo una función `run` conteniendo el resto de la lógica del programa">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-11/src/main.rs:here}}
```

</Listing>

La función `run` ahora contiene toda la lógica restante de `main`, comenzando
desde la lectura del archivo. La función `run` toma la instancia de `Config`
como argumento.

#### Devolviendo errores desde la función `run`

Con la lógica del programa restante separada en la función `run`, podemos
mejorar el manejo de errores, como hicimos con `Config::build` en el Listado
12-9. En lugar de permitir que el programa entre en pánico llamando a
`expect`, la función `run` devolverá un `Result<T, E>` cuando algo salga mal.
Esto nos permitirá consolidar aún más la lógica que rodea el manejo de errores
en `main` de una manera amigable para el usuario. El Listado 12-12 muestra los
cambios que debemos hacer en la firma y el cuerpo de `run`.

<Listing number="12-12" file-name="src/main.rs" caption="Cambiando la función `run` para devolver `Result`">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-12/src/main.rs:here}}
```

</Listing>

Hemos realizado tres cambios significativos aquí. Primero, cambiamos el tipo de
retorno de la función `run` a `Result<(), Box<dyn Error>>`. Esta función
anteriormente devolvía el tipo unitario, `()`, y lo mantenemos como el valor
devuelto en el caso `Ok`.

Para el tipo de error, usamos el _trait object_ `Box<dyn Error>` (y hemos
traído `std::error::Error` al alcance con una declaración `use` en la parte
superior). Cubriremos los _trait objects_ en el [Capítulo 18][ch18]<!-- ignore
-->. Por ahora, solo sepa que `Box<dyn Error>` significa que la función
devolverá un tipo que implementa el trait `Error`, pero no tenemos que
especificar qué tipo particular será el valor de retorno. Esto nos da
flexibilidad para devolver valores de error que pueden ser de diferentes tipos
en diferentes casos de error. La palabra clave `dyn` es una abreviación de 
*“dynamic”*.

Segundo, hemos eliminado la llamada a `expect` en favor del operador `?`, como
hablamos en el [Capítulo 9][ch9-question-mark]<!-- ignore -->. En lugar de
`panic!` en un error, `?` devolverá el valor de error de la función actual para
que el llamador lo maneje.

Tercero, la función `run` ahora devuelve un valor `Ok` en caso de éxito. Hemos
declarado con éxito la función `run` como `()` en la firma, lo que significa
que necesitamos envolver el valor unitario en el valor `Ok`. Esta sintaxis
`Ok(())` puede parecer un poco extraña al principio, pero usar `()` de esta
manera es la forma idiomática de indicar que estamos llamando a `run` solo por
sus efectos secundarios; no devuelve un valor que necesitamos.

Cuando ejecutamos el código, se compila, pero no muestra nada:

```console
{{#include ../listings/ch12-an-io-project/listing-12-12/output.txt}}
```

Rust nos dice que nuestro código ignoró el valor `Result` y el valor `Result`
podría indicar que ocurrió un error. Pero no estamos comprobando si hubo un
error o no, ¡y el compilador nos recuerda que probablemente quisimos tener algo
de código de manejo de errores aquí! Corrijamos ese problema ahora.

#### Manejando errores devueltos por `run` en `main`

Comprobaremos los errores y los manejaremos usando una técnica similar a la que
usamos con `Config::build` en el Listado 12-10, pero con una ligera diferencia:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-01-handling-errors-in-main/src/main.rs:here}}
```

Usamos `if let` en lugar de `unwrap_or_else` para verificar si `run` devuelve un
valor `Err` y llamar a `process::exit(1)` si lo hace. La función `run` no
devuelve un valor que queremos `unwrap` de la misma manera que `Config::build`
devuelve la instancia de `Config`. Debido a que `run` devuelve `()` en el caso
de éxito, solo nos importa detectar un error, por lo que no necesitamos que
`unwrap_or_else` devuelva el valor desempaquetado, que solo sería `()`.

Los cuerpos de las funciones `if let` y `unwrap_or_else` son los mismos en
ambos casos: imprimimos el error y salimos.

### Dividiendo el código en un crate de biblioteca

Nuestro proyecto `minigrep` se ve bien hasta ahora. Ahora dividiremos el archivo
_src/main.rs_ y pondremos parte del código en el archivo _src/lib.rs_. De esa
manera podemos probar el código y tener un archivo _src/main.rs_ con menos
responsabilidades.

Vamos a mover todo el código que no sea la función `main` de _src/main.rs_ a
_src/lib.rs_:

- La función `run`
- Las declaraciones `use` relevantes
- La definición de `Config`
- La función `Config::build`

El contenido de _src/lib.rs_ debería tener la firma que se muestra en el
Listado 12-13 (omitimos los cuerpos de las funciones por brevedad). Ten en
cuenta que esto no se compilará hasta que modifiquemos _src/main.rs_ en el
Listado 12-14.

<Listing number="12-13" file-name="src/lib.rs" caption="Moviendo `Config` y `run` a _src/lib.rs_">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-13/src/lib.rs:here}}
```

</Listing>

Hemos hecho uso de la palabra clave `pub`: en `Config`, en sus campos y en su
método `build`, y en la función `run`. ¡Ahora tenemos un crate de biblioteca que
tiene una API pública que podemos probar!.

Ahora necesitamos traer el código que movimos a _src/lib.rs_ al scope del crate
binario en _src/main.rs_, como se muestra en el Listado 12-14.

<Listing number="12-14" file-name="src/main.rs" caption="Usando el crate biblioteca `minigrep` en _src/main.rs_">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-14/src/main.rs:here}}
```

</Listing>

Agregamos una línea `use minigrep::Config` para traer el tipo `Config` desde el
crate de biblioteca al scope del crate binario, y agregamos el prefijo
`minigrep::` a la llamada a `run`. Ahora toda la funcionalidad debería estar
conectada y debería funcionar. Ejecuta el programa con `cargo run` y asegúrate
de que todo funcione correctamente.

¡Uf! Eso fue mucho trabajo, pero nos hemos preparado para el éxito en el
futuro. Ahora es mucho más fácil manejar errores, y hemos hecho que el código
sea más modular. Casi todo nuestro trabajo se hará en _src/lib.rs_ a partir de
ahora.

¡Aprovechemos esta nueva modularidad haciendo algo que habría sido difícil con
el código antiguo, pero es fácil con el nuevo código: escribiremos algunas
pruebas!

[ch13]: ch13-00-functional-features.html
[ch9-custom-types]: ch09-03-to-panic-or-not-to-panic.html#creacion-de-tipos-personalizados-para-validacion
[ch9-error-guidelines]: ch09-03-to-panic-or-not-to-panic.html#pautas-para-el-manejo-de-errores
[ch9-result]: ch09-02-recoverable-errors-with-result.html
[ch18]: ch18-00-oop.html
[ch9-question-mark]: ch09-02-recoverable-errors-with-result.html#un-atajo-para-propagar-errores-el-operador-
