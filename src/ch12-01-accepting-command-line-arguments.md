## Aceptando argumentos de línea de comandos

Vamos a crear un nuevo proyecto con, como siempre, `cargo new`. Llamaremos a
nuestro proyecto `minigrep` para distinguirlo de la herramienta `grep` que
puede que ya tengas en tu sistema.

```console
$ cargo new minigrep
     Created binary (application) `minigrep` project
$ cd minigrep
```

La primera tarea es hacer que `minigrep` acepte sus dos argumentos de línea de
comandos: la ruta del archivo y una cadena para buscar. Es decir, queremos
poder ejecutar nuestro programa con `cargo run`, dos guiones para indicar que
los siguientes argumentos son para nuestro programa en lugar de para `cargo`,
una cadena para buscar y una ruta a un archivo para buscar, así:

```console
$ cargo run -- searchstring example-filename.txt
```

En este momento, el programa generado por `cargo new` no puede procesar los
argumentos que le damos. Algunas bibliotecas existentes en
[crates.io](https://crates.io/) pueden ayudar a escribir un programa que
acepte argumentos de línea de comandos, pero como estás aprendiendo este
concepto, implementemos esta capacidad nosotros mismos.

### Leyendo los valores de los argumentos

Para permitir que `minigrep` lea los valores de los argumentos de línea de
comandos que le pasamos, necesitaremos la función `std::env::args` proporcionada
en la biblioteca estándar de Rust. Esta función devuelve un iterador de los
argumentos de línea de comandos pasados a `minigrep`. Cubriremos los iteradores
completamente en [el capítulo 13][ch13]<!-- ignore -->. Por ahora, solo
necesitas saber dos detalles sobre los iteradores: los iteradores producen una
serie de valores, y podemos llamar al método `collect` en un iterador para
convertirlo en una colección, como un vector, que contiene todos los elementos
que el iterador produce.

El código en el Listado 12-1 permite que tu programa `minigrep` lea cualquier
argumento de línea de comandos que se le pase y luego recoja los valores en un
vector.

<Listing number="12-1" file-name="src/main.rs" caption="Colectando los argumentos de línea de comandos en un vector e imprimiéndolos">

```rust
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-01/src/main.rs}}
```

</Listing>

Primero, traemos el módulo `std::env` al alcance con una declaración `use` para
que podamos usar su función `args`. Ten en cuenta que la función
`std::env::args` está anidada en dos niveles de módulos. Como discutimos en
[el capítulo 7][ch7-idiomatic-use]<!-- ignore -->, en los casos en que la
función deseada está anidada en más de un módulo, hemos elegido traer el módulo
padre al alcance en lugar de la función. Al hacerlo, podemos usar fácilmente
otras funciones de `std::env`. También es menos ambiguo que agregar
`use std::env::args` y luego llamar a la función con solo `args`, porque
`args` podría confundirse fácilmente con una función definida en el módulo
actual.

> ### La función `args` y Unicode inválido
>
> Ten en cuenta que `std::env::args` lanzará un pánico si algún argumento
> contiene Unicode inválido. Si tu programa necesita aceptar argumentos que
> contengan Unicode inválido, usa `std::env::args_os` en su lugar. Esa función
> devuelve un iterator que produce valores `OsString` en lugar de valores
> `String`. Hemos elegido usar `std::env::args` aquí por simplicidad, porque
> los valores `OsString` difieren según la plataforma y son más complejos de
> trabajar que los valores `String`.

En la primera línea de `main`, llamamos a `env::args` y usamos inmediatamente
`collect` para convertir el iterator en un vector que contiene todos los valores
producidos por el iterator. Podemos usar la función `collect` para crear muchos
tipos de colecciones, por lo que anotamos explícitamente el tipo de `args` para
especificar que queremos un vector de strings. Aunque rara vez necesitamos
anotar tipos en Rust, `collect` es una función que a menudo necesitas anotar
porque Rust no puede inferir el tipo de colección que deseas.

Finalmente, imprimimos el vector usando la macro debug. Intentemos ejecutar el
código primero sin argumentos y luego con dos argumentos:

```console
{{#include ../listings/ch12-an-io-project/listing-12-01/output.txt}}
```

```console
{{#include ../listings/ch12-an-io-project/output-only-01-with-args/output.txt}}
```

EL primer valor en el vector es `"target/debug/minigrep"`, que es el nombre de
nuestro binario. Esto coincide con el comportamiento de la lista de argumentos
en C, lo que permite que los programas usen el nombre por el que fueron
invocados en su ejecución. A menudo es conveniente tener acceso al nombre del
programa en caso de que desees imprimirlo en mensajes o cambiar el
comportamiento del programa según el alias de la línea de comandos que se usó
para invocar el programa. Pero para los propósitos de este capítulo, lo
ignoraremos y solo guardaremos los dos argumentos que necesitamos.

### Guardando los valores de los argumentos en variables

El programa actualmente puede acceder a los valores especificados como
argumentos de línea de comandos. Ahora necesitamos guardar los valores de los
dos argumentos en variables para que podamos usar los valores en el resto del
programa. Hacemos eso en el Listado 12-2.

<Listing number="12-2" file-name="src/main.rs" caption="Creando variables para contener el argumento de consulta y la ruta de archivo">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-02/src/main.rs}}
```

</Listing>

Como vimos cuando imprimimos en el vector, el nombre del programa ocupa el
primer valor del vector en `args[0]`, por lo que estamos comenzando a leer los
argumentos en el índice `1`. El primer argumento `minigrep` que toma es la
cadena que estamos buscando, por lo que ponemos una referencia al primer
argumento en la variable `query`. El segundo argumento será la ruta del archivo,
por lo que ponemos una referencia al segundo argumento en la variable
`file_path`.

Temporalmente, imprimimos los valores de estas variables para demostrar que el
código está funcionando como pretendemos. Ejecutemos el programa nuevamente con
los argumentos `test` y `sample.txt`:

```console
{{#include ../listings/ch12-an-io-project/listing-12-02/output.txt}}
```

¡Genial, el programa está funcionando! Los valores de los argumentos que
necesitamos se están guardando en las variables correctas. Más adelante
agregaremos un manejo de errores para tratar ciertas situaciones erróneas
potenciales, como cuando el usuario no proporciona argumentos; por ahora,
ignoraremos esa situación y trabajaremos en agregar capacidades de lectura de
archivos en su lugar.

[ch13]: ch13-00-functional-features.html
[ch7-idiomatic-use]: ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#creando-rutas-de-use-idiomaticas
