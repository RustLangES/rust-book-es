## Apéndice E - Ediciones

En el Capítulo 1, viste que `cargo new` agrega un poco de metadatos a tu archivo
*Cargo.toml* sobre una edición. ¡Este apéndice habla sobre lo que eso significa!

El lenguaje Rust y el compilador tienen un ciclo de lanzamiento de seis semanas,
lo que significa que los usuarios obtienen un flujo constante de nuevas
características. Otros lenguajes de programación lanzan cambios más grandes con
menos frecuencia; Rust lanza actualizaciones más pequeñas con más frecuencia.
Después de un tiempo, todos estos pequeños cambios se suman. Pero de una
versión a otra, puede ser difícil mirar hacia atrás y decir: “Wow, entre Rust
1.10 y Rust 1.31, Rust ha cambiado mucho!”

Cada dos o tres años, el equipo de Rust produce una nueva *edición* de Rust.
Cada edición reúne las características que han aterrizado en un paquete claro
con documentación y herramientas completamente actualizadas. Las nuevas
ediciones se envían como parte del proceso de lanzamiento habitual de seis
semanas.

Las ediciones sirven para diferentes propósitos para diferentes personas:

* Para los usuarios activos de Rust, una nueva edición reúne los cambios
  incrementales en un paquete fácil de entender.
* Para los no usuarios, una nueva edición señala que se han realizado algunos
  avances importantes, lo que podría hacer que Rust valga la pena volver a
  mirar.
* Para aquellos que desarrollan Rust, una nueva edición proporciona un punto de
  reunión para todo el proyecto.

En el momento de escribir esto, hay cuatro ediciones de Rust disponibles: Rust
2015, Rust 2018, Rust 2021 y Rust 2024. Este libro está escrito utilizando los
modismos de la edición Rust 2024.

La clave `edition` en *Cargo.toml* indica qué edición debe usar el compilador
para su código. Si la clave no existe, Rust usa `2015` como el valor de la
edición por razones de compatibilidad con versiones anteriores.

Cada proyecto puede optar por una edición que no sea la edición predeterminada
2015. Las ediciones pueden contener cambios incompatibles, como incluir una
nueva palabra clave que entra en conflicto con los identificadores en el código.
Sin embargo, a menos que opte por esos cambios, su código seguirá compilando
incluso cuando actualice la versión del compilador Rust que usa.

Todas las versiones del compilador Rust admiten cualquier edición que existía
antes del lanzamiento de ese compilador, y pueden vincular cajas de cualquier
edición compatible entre sí. Los cambios de edición solo afectan la forma en
que el compilador analiza inicialmente el código. Por lo tanto, si está usando
Rust 2015 y una de sus dependencias usa Rust 2018, su proyecto se compilará y
podrá usar esa dependencia. La situación opuesta, donde su proyecto usa Rust
2018 y una dependencia usa Rust 2015, también funciona.

Para ser claros: la mayoría de las características estarán disponibles en todas
las ediciones. Los desarrolladores que usen cualquier edición de Rust seguirán
viendo mejoras a medida que se realicen nuevos lanzamientos estables. Sin
embargo, en algunos casos, principalmente cuando se agregan nuevas palabras
clave, algunas nuevas características pueden estar disponibles solo en ediciones
posteriores. Deberá cambiar de edición si desea aprovechar dichas
características.

Para más detalles, la [*Guía de edición*](https://doc.rust-lang.org/stable/edition-guide/)
es un libro completo sobre ediciones que enumera las diferencias entre ediciones
y explica cómo actualizar automáticamente su código a una nueva edición a través
de `cargo fix`.
