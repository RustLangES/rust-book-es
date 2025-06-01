## Uniendo Todo: Futures, Tareas e Hilos

Como vimos en el capítulo anterior, los hilos proporcionan un enfoque para la 
concurrencia. Hemos visto otro enfoque para la concurrencia en este capítulo, 
usando async con futuros y flujos. Puede que te preguntes por qué elegirías uno 
u otro. La respuesta es: ¡depende! Y en muchos casos, la elección no es hilos 
*o* async, sino más bien hilos *y* async.

Muchos sistemas operativos han proporcionado modelos de concurrencia basados en
hilos durante décadas, y muchos lenguajes de programación los admiten como
resultado. Sin embargo, no están exentos de sus compensaciones. En muchos
sistemas operativos, utilizan una buena cantidad de memoria para cada hilo, y
tienen ciertos costos de inicio y cierre. ¡Los hilos también son una opción solo
cuando su sistema operativo y hardware los admiten! A diferencia de las
computadoras de escritorio y móviles convencionales, algunos sistemas embebidos
no tienen un sistema operativo en absoluto, ¡por lo que tampoco tienen hilos!

El modelo async proporciona un conjunto diferente —y en última instancia
complementario— de compensaciones. En el modelo async, las operaciones
concurrentes no requieren sus propios hilos. En su lugar, pueden ejecutarse en
tareas, como cuando usamos `trpl::spawn_task` para iniciar el trabajo desde una
función síncrona a lo largo de la sección de flujos. Una tarea es similar a un
hilo, pero en lugar de ser administrada por el sistema operativo, es 
administrada por código a nivel de biblioteca: el tiempo de ejecución.

En la sección anterior, vimos que podíamos construir un `Stream` usando un canal
async y lanzando una tarea async que podíamos llamar desde el código síncrono.
¡Podríamos hacer exactamente lo mismo con un hilo! En el Listado 17-40, usamos
`trpl::spawn_task` y `trpl::sleep`. En el Listado 17-41, reemplazamos esos con
las API `thread::spawn` y `thread::sleep` de la biblioteca estándar en la 
función `get_intervals`.

<Listing number="17-41" caption="Usar las API `std::thread` en lugar de las API async `trpl` para la función `get_intervals`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-41/src/main.rs:threads}}
```

</Listing>

Si ejecutas esto, la salida es idéntica. ¡Y fíjate en cuánto cambia aquí desde
la perspectiva del código que llama! Además, aunque una de nuestras funciones
lanzó una tarea async en el tiempo de ejecución y la otra lanzó un hilo del
sistema operativo, los flujos resultantes no se vieron afectados por las
diferencias.

A pesar de las similitudes, estos dos enfoques se comportan de manera muy
diferente, aunque podríamos tener dificultades para medirlo en este ejemplo muy
simple. Podríamos lanzar millones de tareas async en cualquier computadora
personal moderna. ¡Si intentáramos hacer eso con hilos, literalmente nos 
quedaríamos sin memoria!

Sin embargo, hay una razón por la que estas API son tan similares. Los hilos
actúan como un límite para conjuntos de operaciones síncronas; la concurrencia 
es posible *entre* hilos. Las tareas actúan como un límite para conjuntos de
operaciones *asíncronas*; la concurrencia es posible tanto *entre* como *dentro*
de las tareas, porque una tarea puede cambiar entre futuros en su cuerpo.
Finalmente, los futuros son la unidad de concurrencia más granular de Rust, y
cada futuro puede representar un árbol de otros futuros. El tiempo de ejecución
— específicamente, su ejecutor — administra las tareas, y las tareas administran
los futuros. En ese sentido, las tareas son similares a hilos livianos
administrados por el tiempo de ejecución con capacidades adicionales que 
provienen de ser administrados por un tiempo de ejecución en lugar del sistema
operativo.

Esto no significa que las tareas async siempre sean mejores que los hilos, al
igual que los hilos no siempre son mejores que las tareas.

La concurrencia con hilos es en ciertos aspectos un modelo de programación más
simple que la concurrencia con `async`. Eso puede ser una fortaleza o una
debilidad. Los hilos son algo así como “disparar y olvidar”, no tienen un
equivalente nativo a un futuro, por lo que simplemente se ejecutan hasta su
finalización, sin interrupciones excepto por el sistema operativo en sí mismo.
Es decir, no tienen soporte integrado para la *concurrencia intra-tarea* de la
forma en que lo hacen los futuros. Los hilos en Rust tampoco tienen mecanismos
para la cancelación —un tema que no hemos cubierto en profundidad en este
capítulo, pero que es implícito en el hecho de que cada vez que terminamos un
futuro, su estado se limpió correctamente.

Estas limitaciones también hacen que los hilos sean más difíciles de componer 
que los futuros. Es mucho más difícil, por ejemplo, usar hilos para construir
ayudantes como el `timeout` que construimos en [“Construyendo nuestras propias
abstracciones async”][combining-futures] o el método `throttle` que usamos con
flujos en [“Componiendo flujos”][streams]. El hecho de que los futuros sean
estructuras de datos más ricas significa que se pueden componer de manera más
natural, como hemos visto.

Las tareas dan *control adicional* sobre los futuros, permitiéndote elegir dónde
y cómo agrupar los futuros. Y resulta que los hilos y las tareas a menudo
funcionan muy bien juntos, porque las tareas pueden (al menos en algunos
tiempos de ejecución) moverse entre hilos. No lo hemos mencionado hasta ahora,
pero bajo el capó, el `Runtime` que hemos estado usando, incluidas las funciones
`spawn_blocking` y `spawn_task`, es multihilo de forma predeterminada. ¡Muchos
tiempos de ejecución utilizan un enfoque llamado *robo de trabajo* para mover
tareas de manera transparente entre hilos basándose en la utilización actual de
los hilos, con el objetivo de mejorar el rendimiento general del sistema. Para
construir eso, en realidad se requieren hilos *y* tareas, y por lo tanto
futuros.

Como una forma predeterminada de pensar es cuando:

- Si el trabajo es *muy paralelizable*, como procesar un montón de datos donde
  cada parte puede procesarse por separado, los hilos son una mejor elección.
- Si el trabajo es *muy concurrente*, como manejar mensajes de un montón de
  fuentes diferentes que pueden llegar en intervalos o tasas diferentes, async
  es una mejor elección.

Y si necesitas una mezcla de paralelismo y concurrencia, no tienes que elegir
entre hilos y async. Puedes usarlos juntos libremente, dejando que cada uno
sirva la parte en la que es mejor. Por ejemplo, el Listado 17-42 muestra un
ejemplo bastante común de este tipo de mezcla en código Rust del mundo real.

<Listing number="17-42" caption="Enviar mensajes con código bloqueante en un hilo y esperar los mensajes en un bloque async" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-42/src/main.rs:all}}
```

</Listing>

Comenzamos creando un canal async. Luego lanzamos un hilo que toma posesión del
lado emisor del canal. Dentro del hilo, enviamos los números del 1 al 10, y
dormimos durante un segundo entre cada uno. Finalmente, ejecutamos un futuro
creado con un bloque async pasado a `trpl::run` tal como lo hemos hecho a lo
largo del capítulo. En ese futuro, esperamos esos mensajes, al igual que en los
otros ejemplos de paso de mensajes que hemos visto.

Para volver a los ejemplos con los que abrimos el capítulo: podrías imaginar
ejecutar un conjunto de tareas de codificación de video usando un hilo
dedicado, porque la codificación de video está limitada por la computación, pero
notificar a la interfaz de usuario que esas operaciones se han completado con un
canal async. ¡Los ejemplos de este tipo de mezcla abundan!

## Resumen

Esto no es lo último que verás de la concurrencia en este libro: el proyecto en
el Capítulo 21 utilizará los conceptos de este capítulo en una situación más
realista que los ejemplos más pequeños discutidos aquí —y comparará de manera
más directa cómo se ve resolver este tipo de problemas con hilos vs. con tareas 
y futuros.

Ya sea con hilos, con futuros y tareas, o con la combinación de todos ellos, 
Rust te brinda las herramientas que necesitas para escribir código concurrente
seguro y rápido —ya sea para un servidor web de alto rendimiento o un sistema
operativo embebido.

A continuación, hablaremos sobre formas idiomáticas de modelar problemas y
estructurar soluciones a medida que tus programas Rust se vuelven más grandes.
Además, discutiremos cómo se relacionan los ídolos de Rust con los que podrías
estar familiarizado de la programación orientada a objetos.


[ch16]: http://localhost:3000/ch16-00-concurrency.html
[combining-futures]: ch17-03-more-futures.html#building-our-own-async-abstractions
[streams]: ch17-04-streams.html#componiendo-streams
