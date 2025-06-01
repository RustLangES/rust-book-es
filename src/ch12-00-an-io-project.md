# Un proyecto de I/O: Construyendo un programa de línea de comandos

Este capítulo es un resumen de las muchas habilidades que has aprendido hasta
ahora y una exploración de algunas características más de la biblioteca estándar.
Construiremos una herramienta de línea de comandos que interactúa con la entrada
y salida de archivos para practicar algunos de los conceptos de Rust que ahora 
tienes bajo el cinturón.

La velocidad, la seguridad, la salida binaria única y el soporte multiplataforma
de Rust hacen que sea un lenguaje ideal para crear herramientas de línea de
comandos, así que para nuestro proyecto, haremos nuestra propia versión de la
herramienta de búsqueda clásica de la línea de comandos `grep` (**g**lobally
search a **r**egular **e**xpression and **p**rint). En el caso de uso más
simple, `grep` busca un archivo especificado para una cadena especificada.
Para ello, `grep` toma como argumentos una ruta de archivo y una cadena. Luego
lee el archivo, encuentra las líneas en ese archivo que contienen el argumento
de cadena y las imprime.

En el camino, mostraremos cómo hacer que nuestra herramienta de línea de comandos
use las características del terminal que muchas otras herramientas de línea de
comandos usan. Leeremos el valor de una variable de entorno para permitir que el
usuario configure el comportamiento de nuestra herramienta. También imprimiremos
mensajes de error a la consola de error estándar (`stderr`) en lugar de la
salida estándar (`stdout`), para que, por ejemplo, el usuario pueda redirigir
la salida exitosa a un archivo y seguir viendo los mensajes de error en la
pantalla.

Andrew Gallant, miembro de la comunidad de Rust, ya ha creado una versión
completamente funcional y muy rápida de `grep`, llamada `ripgrep`. En
comparación, nuestra versión será bastante simple, pero este capítulo te dará
algunos de los conocimientos básicos que necesitas para entender un proyecto
del mundo real como `ripgrep`.

Nuestro proyecto `grep` combinará una serie de conceptos que has aprendido
hasta ahora:

* Organizar código (usando lo que aprendiste sobre módulos en
  [Capítulo 7][ch7]<!-- ignore -->)
* Uso de vectores y strings (colecciones, [Capítulo 8][ch8]<!-- ignore -->)
* Manejo de errores ([Capítulo 9][ch9]<!-- ignore -->)
* Uso de traits y lifetimes cuando corresponda ([Capítulo 10][ch10]<!-- ignore
  -->)
* Escribiendo tests ([Capítulo 11][ch11]<!-- ignore -->)

También presentaremos brevemente los closures, iterators, y trait objects, que
los capítulos [13][ch13]<!-- ignore --> y [18][ch18]<!-- ignore --> cubrirán en
detalle.

[ch7]: ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
[ch8]: ch08-00-common-collections.html
[ch9]: ch09-00-error-handling.html
[ch10]: ch10-00-generics.html
[ch11]: ch11-00-testing.html
[ch13]: ch13-00-functional-features.html
[ch18]: ch18-00-oop.html
