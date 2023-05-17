## Leyendo un archivo

Ahora agregaremos funcionalidad para leer el archivo especificado en el 
argumento `file_path`. Primero, necesitamos un archivo de muestra para probarlo:
¡usaremos un archivo con una pequeña cantidad de texto en varias líneas con 
algunas palabras repetidas! ¡El Listado 12-3 tiene un poema de Emily Dickinson 
que funcionará bien! Cree un archivo llamado *poem.txt* en el nivel raíz de su
proyecto e ingrese el poema "¡Soy nadie! ¿Quién eres tú?"

<span class="filename">Filename: poem.txt</span>

```text
{{#include ../listings/ch12-an-io-project/listing-12-03/poem.txt}}
```

<span class="caption">Listing 12-3: Un poema de Emily Dickinson sirve como
buen caso de test</span>

Con el texto en su lugar, edite *src/main.rs* y agregue código para leer el
archivo, como se muestra en el Listado 12-4.

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/src/main.rs:here}}
```

<span class="caption">Listing 12-4: Leyendo el contenido del archivo 
especificado por el segundo argumento</span>

Primero, importamos la parte relevante de la biblioteca estándar con una
sentencia `use`: necesitamos `std::fs` para manejar archivos.

En `main`, la nueva sentencia `fs::read_to_string` toma el `file_path`, abre
ese archivo y devuelve un `std::io::Result<String>` del contenido del archivo.

Luego, nuevamente agregamos una declaración `println!` temporal que imprime el
valor de `contents` después de que se lee el archivo, para que podamos verificar
que el programa está funcionando hasta ahora.

Ejecutemos este código con cualquier string como primer argumento de la línea de
comandos (porque aún no hemos implementado la parte de búsqueda) y el archivo
*poem.txt* como segundo argumento:

```console
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/output.txt}}
```

¡Genial! El código leyó el archivo y luego imprimió el contenido del archivo.
Pero el código tiene algunas fallas. En este momento, la función `main` tiene
múltiples responsabilidades: en general, las funciones son más claras y más
fáciles de mantener si cada función es responsable de una sola idea. El otro
problema es que no estamos manejando los errores tan bien como podríamos. El
programa todavía es pequeño, por lo que estas fallas no son un gran problema,
pero a medida que el programa crece, será más difícil corregirlos de manera
limpia. Es una buena práctica comenzar a refactorizar desde el principio al
desarrollar un programa, porque es mucho más fácil refactorizar cantidades
menores de código. Haremos eso a continuación.
