# Introducción

> Nota: Esta edición del libro es la misma que [The Rust Programming
> Language][nsprust] disponible en formato impreso y ebook de [No Starch
> Press][nsp].

[nsprust]: https://nostarch.com/rust-programming-language-2nd-edition
[nsp]: https://nostarch.com/

Bienvenido a *The Rust Programming Language*, un libro introductorio sobre Rust.
El lenguaje de programación Rust te ayuda a escribir software más rápido y
confiable. La ergonomía de alto nivel y el control de bajo nivel a menudo están
en conflicto en el diseño de lenguajes de programación; Rust desafía ese
conflicto. A través del equilibrio de la capacidad técnica poderosa y una gran
experiencia de desarrollo, Rust te da la opción de controlar los detalles de
bajo nivel (como el uso de memoria) sin todo el problema tradicionalmente
asociado con tal control.

## Para Quién Es Rust

Rust es ideal para muchas personas por una variedad de razones. Veamos algunos
de los grupos más importantes.

### Equipos de Desarrolladores

Rust está demostrando ser una herramienta productiva para colaborar entre
equipos grandes de desarrolladores con diferentes niveles de conocimiento de
programación de sistemas. El código de bajo nivel tiende a tener varios
sutiles errores, que en la mayoría de otros lenguajes solo pueden ser
detectados a través de pruebas extensivas y una revisión cuidadosa del código
por parte de desarrolladores experimentados. En Rust, el compilador juega un
rol de guardián al negarse a compilar código con estos errores elusivos,
incluidos los errores de concurrencia. Trabajando junto al compilador, el
equipo puede dedicar su tiempo a enfocarse en la lógica del programa en lugar
de perseguir errores.

Rust también trae herramientas de desarrollo contemporáneas al mundo de la
programación de sistemas:

* Cargo, el administrador de dependencias y herramienta de compilación
  incluido, hace que agregar, compilar y administrar dependencias sea fácil y
  consistente en todo el ecosistema de Rust.
* La herramienta de formateo Rustfmt garantiza un estilo de codificación
  consistente entre los desarrolladores.
* El servidor de lenguaje Rust proporciona integración con el entorno de
  desarrollo integrado (IDE) para la finalización del código y los mensajes de
  error en línea.

Al usar estas y otras herramientas en el ecosistema de Rust, los desarrolladores
pueden ser productivos mientras escriben código de nivel de sistemas.

### Estudiantes

Rust es para estudiantes y quienes estén interesados en aprender sobre conceptos
de sistemas. Usando Rust, muchas personas han aprendido sobre temas como el
desarrollo de sistemas operativos. La comunidad es muy acogedora y feliz de
responder preguntas de estudiantes. A través de esfuerzos como este libro, los
equipos de Rust quieren hacer que los conceptos de sistemas sean más
accesibles para más personas, especialmente para quienes son nuevos en la
programación.

### Empresas

Cientos de empresas, grandes y pequeñas, usan Rust en producción para una
variedad de tareas, incluidas herramientas de línea de comandos, servicios web,
herramientas de DevOps, dispositivos incrustados, análisis y transcodificación
de audio y video, criptomonedas, bioinformática, motores de búsqueda, aplicaciones
de Internet de las cosas, aprendizaje automático e incluso partes importantes del
navegador web Firefox.

### Desarrolladores de Código Abierto

Rust es para personas que quieren construir el lenguaje de programación Rust,
la comunidad, las herramientas de desarrollo y las bibliotecas. Nos encantaría
que contribuyeras al lenguaje Rust.

### Personas que Valoran la Velocidad y la Estabilidad

Rust es para personas que anhelan velocidad y estabilidad en un lenguaje. Por
velocidad, nos referimos tanto a la rapidez con que el código Rust puede
ejecutarse como a la velocidad con que Rust te permite escribir programas. Las
verificaciones del compilador de Rust garantizan la estabilidad a través de
adiciones de funciones y refactorización. Esto contrasta con el código heredado
quebradizo en lenguajes sin estas verificaciones, que los desarrolladores
a menudo tienen miedo de modificar. Al esforzarse por lograr abstracciones de
costo cero, características de alto nivel que se compilan en código de bajo
nivel tan rápido como el código escrito manualmente, Rust se esfuerza por hacer
que el código seguro sea también código rápido.

El lenguaje Rust también espera apoyar a muchos otros usuarios; los mencionados
aquí son solo algunos de los principales interesados. En general, la mayor
ambición de Rust es eliminar los compromisos que los programadores han
aceptado durante décadas al proporcionar seguridad *y* productividad, velocidad
*y* ergonomía. Pruébalo y ve si sus elecciones funcionan para ti.

## Para Quién Está Este Libro

Este libro asume que has escrito código en otro lenguaje de programación, pero
no hace ninguna suposición sobre cuál es. Hemos intentado hacer que el material
sea ampliamente accesible para aquellos de una amplia variedad de antecedentes
de programación. No pasamos mucho tiempo hablando de lo que *es* la 
programación o cómo pensar sobre ella. Si eres completamente nuevo en la 
programación, sería mejor leer un libro que brinde una introducción específica a
la programación.

## Cómo Usar Este Libro

En general, este libro asume que lo estás leyendo en secuencia, de principio a
fin. Los capítulos posteriores se basan en conceptos de los capítulos
anteriores, y los capítulos anteriores pueden no profundizar en detalles sobre
un tema en particular, pero volverán al tema en un capítulo posterior.

Encontrarás dos tipos de capítulos en este libro: capítulos de conceptos y
capítulos de proyectos. En los capítulos de conceptos, aprenderás sobre un
aspecto de Rust. En los capítulos de proyectos, construiremos programas
pequeños juntos, aplicando lo que has aprendido hasta ahora. Los capítulos 2, 12
y 20 son capítulos de proyectos; el resto son capítulos de conceptos.

El capítulo 1 explica cómo instalar Rust, cómo escribir un programa “Hola,
mundo!” Y cómo usar Cargo, el administrador de paquetes y herramienta de
compilación de Rust. El capítulo 2 es una introducción práctica a la escritura
de un programa en Rust, teniendo que construir un juego de adivinanzas. Aquí
tratamos los conceptos a un nivel alto, y capítulos posteriores proporcionarán
detalles adicionales. Si quieres ponerte manos a la obra de inmediato, el
capítulo 2 es el lugar para eso. El capítulo 3 cubre las características de
Rust que son similares a las de otros lenguajes de programación, y en el
capítulo 4 aprenderás sobre el sistema de propiedad de Rust. Si eres un
aprendiz particularmente meticuloso que prefiere aprender todos los detalles
antes de pasar al siguiente, es posible que desees omitir el capítulo 2 y
dirigirte directamente al capítulo 3, regresando al capítulo 2 cuando te 
gustaría trabajar en un proyecto aplicando los detalles que has aprendido.

El capítulo 5 discute las estructuras y los métodos, y el capítulo 6 cubre las
enumeraciones, las expresiones `match`, y la construcción de flujo de control
`if let`. Usarás estructuras y enumeraciones para crear tipos personalizados en
Rust.

En el capítulo 7, aprenderás sobre el sistema de módulos de Rust y sobre las
reglas de privacidad para organizar tu código y su interfaz de programación de
aplicaciones (API) pública. El capítulo 8 discute algunas estructuras de datos
de colección comunes que proporciona la biblioteca estándar, como vectores,
cadenas y mapas hash. El capítulo 9 explora la filosofía y técnicas de
manejo de errores de Rust.

El capítulo 10 se adentra en las genericidades, las características y las
vidas, que te dan el poder de definir código que se aplique a varios tipos. El
capítulo 11 trata sobre las pruebas, que incluso con las garantías de seguridad
de Rust, es necesario para asegurar que la lógica de tu programa sea correcta.
En el capítulo 12, construiremos nuestra propia implementación de un subconjunto
de la funcionalidad del comando de línea de comandos `grep` que busca texto
dentro de archivos. Para esto, usaremos muchos de los conceptos que
discutimos en los capítulos anteriores.

El capítulo 13 explora las clausuras y los iteradores: características de Rust
que provienen de los lenguajes de programación funcional. En el capítulo 14,
examinaremos Cargo en más profundidad y hablaremos sobre las mejores prácticas
para compartir tus bibliotecas con otros. El capítulo 15 discute los punteros
inteligentes que proporciona la biblioteca estándar y las características que
habilitan su funcionalidad.

En el capítulo 16, recorreremos diferentes modelos de programación concurrente
y hablaremos sobre cómo Rust te ayuda a programar en múltiples hilos sin
temor. El capítulo 17 examina cómo los idiomas de Rust se comparan con los
principios de programación orientada a objetos con los que puede estar
familiarizado.

El capítulo 18 es una referencia sobre los patrones y el emparejamiento de
patrones, que son formas poderosas de expresar ideas en todo el programa de
Rust. El capítulo 19 contiene un banquete de temas avanzados de interés,
incluyendo Rust inseguro, macros y más sobre tiempos de vida, características, 
tipos, funciones y clausuras.

En el capítulo 20, ¡completaremos un proyecto en el que implementaremos un
servidor web de múltiples hilos de bajo nivel!

Finalmente, algunos apéndices contienen información útil sobre el lenguaje en
un formato más de referencia. El apéndice A cubre las palabras clave de Rust,
el apéndice B cubre los operadores y símbolos de Rust, el apéndice C cubre las
características derivables proporcionadas por la biblioteca estándar, el
apéndice D cubre algunas herramientas de desarrollo útiles, y el apéndice E
explica las ediciones de Rust. En el apéndice F, puede encontrar traducciones
del libro, y en el apéndice G cubriremos cómo se hace Rust y qué es Rust
nightly.

No hay una forma incorrecta de leer este libro: ¡si quieres adelantarte, hazlo!
Es posible que debas volver a los capítulos anteriores si experimentas
alguna confusión. Pero haz lo que funcione para ti.

<span id="ferris"></span>

Una parte importante del proceso de aprendizaje de Rust es aprender a leer los
mensajes de error que muestra el compilador: estos te guiarán hacia el código
funcional. Por lo tanto, proporcionaremos muchos ejemplos que no se compilan
junto con el mensaje de error que mostrará el compilador en cada situación.
Ten en cuenta que si ingresas y ejecutas un ejemplo aleatorio, ¡es posible que
no se compile! Asegúrate de leer el texto circundante para ver si el ejemplo
que estás intentando ejecutar está destinado a error. Ferris también te ayudará
a distinguir el código que no está destinado a funcionar:

| Ferris                                                                                                           | Significado                                          |
|------------------------------------------------------------------------------------------------------------------|--------------------------------------------------|
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain" alt="Ferris with a question mark"/>            | Este código no compila!                       |
| <img src="img/ferris/panics.svg" class="ferris-explain" alt="Ferris throwing up their hands"/>                   | ¡Este código provoca pánico!               |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain" alt="Ferris with one claw up, shrugging"/> | Este código no produce el comportamiento deseado. |

En la mayoría de las situaciones, te guiaremos a la versión correcta de cualquier
código que no se compile.

## Código fuente

Los archivos de origen de los que se genera este libro se pueden encontrar en
[GitHub][book].

[book]: https://github.com/Phosphorus-M/rust-book-es/tree/main/src
