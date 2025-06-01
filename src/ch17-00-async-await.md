## Fundamentos de la Programación Asíncrona: Async, Await, Futures y Streams

Muchas de las operaciones que pedimos a la computadora realizar pueden tardar un 
tiempo en completarse. Por ejemplo, si utilizas un editor de video para crear un 
video de una celebración familiar, exportarlo podría llevar desde unos minutos 
hasta horas. De manera similar, descargar un video compartido por alguien de tu 
familia podría tomar mucho tiempo. Sería ideal poder hacer algo más mientras 
esperamos que esos procesos prolongados se completen.

La exportación de video utilizará toda la potencia de CPU y GPU que pueda. Si 
solo tuvieras un núcleo de CPU y tu sistema operativo nunca interrumpiera esa 
exportación hasta que se complete, no podrías hacer nada más en tu computadora 
mientras se ejecuta. Eso sería una experiencia bastante frustrante. En cambio, 
el sistema operativo de tu computadora puede — ¡y lo hace! — interrumpir 
invisiblemente la exportación con la suficiente frecuencia para permitirte 
realizar otras tareas mientras tanto.

La descarga de archivos es diferente. No consume mucho tiempo de CPU. En su 
lugar, la CPU necesita esperar a que los datos lleguen desde la red. Aunque 
puedes comenzar a leer los datos una vez que parte de ellos están presentes, 
puede llevar tiempo que el resto llegue. Incluso cuando todos los datos están 
presentes, un video puede ser bastante grande, por lo que podría tomar algo de 
tiempo cargarlo por completo. Quizás solo tome uno o dos segundos, pero eso es 
muchísimo tiempo para un procesador moderno, que puede realizar miles de 
millones de operaciones por segundo. Sería ideal poder usar la CPU para otras 
tareas mientras esperamos que la llamada a la red se complete. Así que, 
nuevamente, el sistema operativo interrumpirá invisiblemente tu programa para 
que otras cosas puedan ocurrir mientras la operación de red sigue en curso.

> Nota: La exportación de video es el tipo de operación que a menudo se describe 
> como “limitada por la CPU” o “limitada por el cálculo” (*CPU-bound* o 
> *compute-bound*). Está limitada por la velocidad con la que la computadora 
> puede procesar datos dentro de la *CPU* o la *GPU*, y cuánto de esa velocidad 
> puede usar. La descarga de video es el tipo de operación que se describe 
> comúnmente como “limitada por IO” (*IO-bound*), porque está restringida por la 
> velocidad de entrada y salida de la computadora. Solo puede ir tan rápido como 
> los datos puedan transmitirse a través de la red.

En ambos ejemplos, las interrupciones invisibles del sistema operativo 
proporcionan una forma de concurrencia. Sin embargo, esa concurrencia solo 
ocurre a nivel de todo un programa: el sistema operativo interrumpe un programa 
para permitir que otros programas realicen trabajo. En muchos casos, como 
entendemos nuestros programas a un nivel mucho más granular que el sistema 
operativo, podemos detectar muchas oportunidades de concurrencia que el sistema 
operativo no puede ver.

Por ejemplo, si estamos creando una herramienta para gestionar descargas de 
archivos, deberíamos ser capaces de escribir nuestro programa de manera que 
iniciar una descarga no bloquee la interfaz de usuario, y los usuarios puedan 
comenzar varias descargas al mismo tiempo. Sin embargo, muchas APIs del sistema 
operativo para interactuar con la red son *bloqueantes*. Es decir, estas APIs 
bloquean el progreso del programa hasta que los datos con los que están 
trabajando estén completamente listos.

> Nota: ¡Así es como funcionan *la mayoría* de las llamadas a funciones, si lo 
piensas bien! Sin embargo, normalmente reservamos el término “bloqueante” para 
llamadas a funciones que interactúan con archivos, la red u otros recursos de la 
computadora, porque en esos casos un programa individual podría beneficiarse de 
que la operación no sea *bloqueante*.

Podríamos evitar bloquear nuestro hilo principal creando un hilo dedicado para 
descargar cada archivo. Sin embargo, eventualmente encontraríamos que el 
sobrecosto de esos hilos es un problema. También sería más conveniente si la 
llamada no fuera bloqueante desde el principio. Por último, pero no menos 
importante, sería mejor si pudiéramos escribir en el mismo estilo directo que 
usamos en el código bloqueante. Algo similar a esto:

```rust,ignore,does_not_compile
let data = fetch_data_from(url).await;
println!("{data}");
```

Eso es exactamente lo que la abstracción de Rust nos ofrece. Antes de ver como 
funciona esto en la práctica, debemos hacer una breve pausa para entender la 
diferencia entre paralelismos y concurrencia.

### Paralelismo y Concurrencia

En el capitulo anterior, tratamos la concurrencia y el paralelismo como 
conceptos mayormente intercambiables. Ahora necesitamos distinguirlos con más 
precisión porque la diferencia se hará más evidente a medida que comencemos a 
trabajar.

Considera las diferentes maneras en que un equipo podría dividir el trabajo en 
un proyecto de software. Podríamos asignar múltiples tareas a una sola persona, 
o podríamos asignar una tarea por miembro del equipo, o podríamos usar una 
combinación de ambos enfoques.

Cuando una persona trabaja en varias tareas diferentes antes de completar alguna 
de ellas, esto es *concurrencia*. Tal vez tienes dos proyectos diferentes 
abiertos en tu computadora, y cuando te aburres o te atascas en uno, cambias al 
otro. Eres solo una persona, por lo que no puedes avanzar en ambas tareas al 
mismo tiempo exacto, pero puedes hacer varias cosas a la vez, progresando en 
múltiples tareas al cambiar entre ellas.

<figure>

<img alt="Flujo de trabajo concurrente" src="img/trpl17-01.svg" class="center" />

<figcaption>Figura 17-1: Un flujo de trabajo concurrente, cambiando entre la Tarea A y la Tarea B.</figcaption>

</figure>

Cuando acuerdas dividir un grupo de tareas entre las personas del equipo, con
cada persona tomando una tarea y trabajando en ella sola, esto es *paralelismo*.
Cada persona en el equipo puede avanzar al mismo tiempo. 

<figure>

<img src="img/trpl17-02.svg" class="center" alt="Un diagrama con cajas etiquetadas como Tarea A y Tarea B, con rombos dentro que representan subtareas. Hay flechas que apuntan de A1 a A2, de A2 a A3, de A3 a A4, de B1 a B2 y de B2 a B3. No hay flechas que crucen entre las cajas de Tarea A y Tarea B." />

<img alt="Flujo de trabajo paralelo" src="img/trpl17-02.svg" class="center" />

<figcaption>Figura 17-2: Un flujo de trabajo paralelo, donde el trabajo se realiza en la Tarea A y la Tarea B de forma independiente.</figcaption>

</figure>

Con ambas situaciones, es posible que debas coordinar entre diferentes tareas.
Quizás *pensaste* que la tarea en la que una persona estaba trabajando era
totalmente independiente del trabajo de todos los demás, pero en realidad
necesita algo terminado por otra persona en el equipo. Alguno de los trabajos
podría hacerse en paralelo, pero en realidad, algo de eso era *serial*: solo
podría ocurrir en serie, una cosa tras otra, como en la Figura 17-3.

<figure>

<img src="img/trpl17-03.svg" class="center" alt="Un diagrama con cajas etiquetadas como Tarea A y Tarea B, con rombos dentro que representan subtareas. Hay flechas que van de A1 a A2, de A2 a un par de líneas verticales gruesas como un símbolo de “pausa”, de ese símbolo a A3, de B1 a B2, de B2 a B3 (que está debajo de ese símbolo), de B3 a A3 y de B3 a B4.
" />

<figcaption>Figura 17-3: Un flujo de trabajo parcialmente paralelo, donde el trabajo se realiza en la Tarea A y la Tarea B de forma independiente hasta que la tarea A3 está bloqueada en los resultados de la tarea B3.</figcaption>

</figure>

Así mismo, puedes darte cuenta de que una de tus propias tareas depende de otra
de tus tareas. Ahora tu trabajo concurrente también se ha vuelto serial.

El paralelismo y la concurrencia también pueden interceptarse entre sí. Si
descubres que un colega está atascado hasta que termines una de tus tareas,
probablemente centrarás todos tus esfuerzos en esa tarea para “desbloquear” a tu
colega. Tú y tu compañero de trabajo ya no pueden trabajar en paralelo, y tú
tampoco puedes trabajar en forma concurrente en tus propias tareas.

Las mismas dinámicas básicas entran en juego con el software y el hardware. En
una máquina con un solo núcleo de CPU, la CPU solo puede hacer una operación a 
la vez, pero aún puede trabajar de manera concurrente. Utilizando herramientas
como hilos, procesos y async, la computadora puede pausar una actividad y 
cambiar a otras antes de volver eventualmente a esa primera actividad 
nuevamente. En una máquina con múltiples núcleos de CPU, también puede hacer
trabajo en paralelo. Un núcleo puede estar haciendo una cosa mientras que otro
núcleo hace algo completamente no relacionado, y eso realmente sucede al mismo
tiempo.

Cuando trabajamos con async en Rust, siempre estamos tratando con concurrencia.
Dependiendo del hardware, el sistema operativo y el tiempo de ejecución async
que estamos utilizando — ¡más sobre los tiempos de ejecución async en breve! —
esa concurrencia también puede usar paralelismo bajo el capó.

Ahora, ¡sumérgete en cómo funciona la programación async en Rust! En el resto de
este capítulo, vamos a:

* ver cómo usar la sintaxis `async` y `await` de Rust
* explorar cómo usar el modelo async para resolver algunos de los mismos
  desafíos que vimos en el Capítulo 16
* ver cómo el multiprocesamiento y async proporcionan soluciones 
  complementarias, que incluso puedes usarlas juntas en muchos casos
