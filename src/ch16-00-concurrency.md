# Concurrencia sin miedo

Manejar la programación concurrente de forma segura y eficiente es otro de los
principales objetivos de Rust. La *programación concurrente*, donde diferentes
partes de un programa se ejecutan de forma independiente, y la *programación
paralela*, donde diferentes partes de un programa se ejecutan al mismo tiempo,
son cada vez más importantes a medida que más computadoras aprovechan sus
múltiples procesadores. Históricamente, la programación en estos contextos ha
sido difícil y propensa a errores: ¡Rust espera cambiar eso!

Inicialmente, el equipo de Rust pensó que garantizar la seguridad de la memoria
y prevenir los problemas de concurrencia eran dos desafíos separados que se
resolverían con diferentes métodos. Con el tiempo, el equipo descubrió que los
sistemas de propiedad y tipos son un conjunto de herramientas poderosas para
ayudar a administrar la seguridad de la memoria *y* los problemas de
concurrencia. Al aprovechar la propiedad y la comprobación de tipos, muchos
errores de concurrencia son errores de tiempo de compilación en Rust en lugar
de errores de tiempo de ejecución. Por lo tanto, en lugar de hacer que pase
mucho tiempo tratando de reproducir las circunstancias exactas en las que se
produce un error de concurrencia en tiempo de ejecución, el código incorrecto
se negará a compilar y presentará un error que explica el problema. Como
resultado, puede corregir su código mientras lo está trabajando en lugar de
potencialmente después de que se haya enviado a producción. Hemos apodado este
aspecto de Rust como *concurrencia sin miedo*. La concurrencia sin miedo le
permite escribir código que no tiene errores sutiles y es fácil de refactorizar
sin introducir nuevos bugs.

> Nota: Para simplificar, nos referiremos a muchos de los problemas como
> *concurrentes* en lugar de ser más precisos y decir *concurrentes y/o
> paralelos*. Para este capítulo, por favor sustituye mentalmente concurrentes 
> y/o paralelos cada vez que usemos concurrentes. En el próximo capítulo, donde 
> la distinción es más importante, seremos más específicos.

Muchos lenguajes son dogmáticos sobre las soluciones que ofrecen para manejar
problemas concurrentes. Por ejemplo, Erlang tiene una funcionalidad elegante
para la concurrencia de paso de mensajes, pero solo tiene formas oscuras de
compartir estado entre hilos. Soportar solo un subconjunto de soluciones
posibles es una estrategia razonable para los lenguajes de más alto nivel,
porque un lenguaje de más alto nivel promete beneficios al renunciar a cierto
control para obtener abstracciones. Sin embargo, se espera que los lenguajes de
nivel inferior proporcionen la solución con el mejor rendimiento en cualquier
situación dada y tengan menos abstracciones sobre el hardware. Por lo tanto,
Rust ofrece una variedad de herramientas para modelar problemas de la manera
que sea apropiada para su situación y requisitos.

Aquí están los temas que cubriremos en este capítulo:

* Cómo crear hilos para ejecutar múltiples piezas de código al mismo tiempo
* *Message-passing* concurrencia, donde los canales envían mensajes entre hilos
* *Shared-state* concurrencia, donde múltiples hilos tienen acceso a alguna
  pieza de datos
* Los traits `Sync` y `Send`, que extienden las garantías de concurrencia de
  Rust a los tipos definidos por el usuario, así como a los tipos proporcionados
  por la biblioteca estándar
