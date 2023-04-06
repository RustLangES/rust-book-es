## Comentarios

Todos los programadores se esfuerzan por hacer que su código sea fácil de
entender, pero a veces se requiere una explicación adicional. En estos casos,
los programadores dejan *comentarios* en su código fuente que el compilador
ignorará pero que las personas que lean el código fuente pueden encontrar
útiles.

Aquí hay un comentario simple:

```rust
// hola, mundo
```

En Rust, el estilo de comentario idiomático comienza un comentario con dos
barras inclinadas y el comentario continúa hasta el final de la línea. Para
comentarios que se extienden más allá de una sola línea, deberá incluir `//` en
cada línea, así:

```rust
// Así que estamos haciendo algo complicado aquí, lo suficientemente largo
// como para necesitar varias líneas de comentarios para hacerlo. ¡Uf!
// ¡Espero que este comentario explique lo que está sucediendo!
```

Los comentarios también se pueden colocar al final de las líneas que contienen
código:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-24-comments-end-of-line/src/main.rs}}
```

Pero más a menudo verás que se usan en este formato, con el comentario en una
línea separada por encima del código que está anotando:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-25-comments-above-line/src/main.rs}}
```

Rust también tiene otro tipo de comentario, comentarios de documentación, que
discutiremos en la sección [“Publicando una Caja en Crates.io”][publishing]
<!-- ignore --> del Capítulo 14.

[publishing]: ch14-02-publishing-to-crates-io.html
