# Colecciones comunes

La biblioteca estándar de Rust incluye una serie de estructuras de datos muy
útiles llamadas *colecciones*. La mayoría de los otros tipos de datos
representan un valor específico, pero las colecciones pueden contener varios
valores. A diferencia de los tipos de datos built-in array y tupla, los
datos a los que apuntan estas colecciones se almacenan en el montón, lo que
significa que la cantidad de datos no necesita conocerse en el momento de la
compilación y puede crecer o disminuir a medida que se ejecuta el programa. Cada
tipo de colección tiene diferentes capacidades y costos, y elegir uno
apropiado para su situación actual es una habilidad que desarrollará con el
tiempo. En este capítulo, discutiremos tres colecciones que se usan muy a menudo
en los programas Rust:

* Un *vector* le permite almacenar un número variable de valores uno al lado del
  otro.
* Un *string* es una colección de caracteres. Hemos mencionado el tipo `String`
  anteriormente, pero en este capítulo hablaremos de él en profundidad.
* Un *hash map* le permite asociar un valor con una clave particular. Es una
  implementación particular de la estructura de datos más general llamada *map*.

Para aprender sobre los otros tipos de colecciones proporcionados por la
biblioteca estándar, consulte [la documentación][collections].

Discutiremos cómo crear y actualizar vectores, strings y hash maps, así como
lo que hace que cada uno sea especial.

[collections]: https://doc.rust-lang.org/std/collections/index.html
