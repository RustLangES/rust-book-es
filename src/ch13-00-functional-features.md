# Características funcionales del lenguaje: Iterators y Closures

El diseño de Rust se ha inspirado en muchos lenguajes y técnicas existentes, 
y una influencia significativa es la *programación funcional*. La programación 
en un estilo funcional a menudo incluye el uso de funciones como valores 
pasándolas en argumentos, devolviéndolas de otras funciones, asignándolas a 
variables para su ejecución posterior, y así sucesivamente.

En este capítulo, no debatiremos la cuestión de lo que es o no es la
programación funcional, sino que discutiremos algunas características de Rust
que son similares a las características de muchos lenguajes a menudo
denominados funcionales.

Más específicamente, cubriremos:

* *Closures*, una construcción similar a una función que puede almacenarse en una
  variable
* *Iterators*, una forma de procesar una serie de elementos
* Cómo usar closures e iterators para mejorar el proyecto I/O en el Capítulo 12
* ¡El rendimiento de los closures e iterators (Spoiler alert: son más rápidos de
  lo que podrías pensar!)

Ya hemos cubierto algunas otras características de Rust, como el *pattern
matching* y los *enums*, que también están influenciados por el estilo
funcional. Debido a que dominar los closures e iterators es una parte
importante de escribir código Rust idiomático y rápido, dedicaremos todo este
capítulo a ellos.
