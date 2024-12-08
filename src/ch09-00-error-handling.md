# Manejo de Errores

Los errores son un hecho de la vida en el software, por lo que Rust tiene una
serie de características para manejar situaciones en las que algo sale mal. En
muchos casos, Rust te obliga a reconocer la posibilidad de un error y tomar
alguna acción antes de que tu código se compile. ¡Este requisito hace que su
programa sea más robusto al garantizar que descubrirá errores y los manejará
adecuadamente antes de implementar su código en producción!

Rust agrupa los errores en dos categorías principales: errores *recuperables* e
*irrecuperables*. Para un error recuperable, como un error de *archivo no
encontrado*, lo más probable es que solo queramos informar el problema al
usuario y volver a intentar la operación. Los errores irreversibles siempre son
síntomas de errores, como intentar acceder a una ubicación más allá del final
de un arreglo, por lo que queremos detener inmediatamente el programa.

La mayoría de los lenguajes no distinguen entre estos dos tipos de errores y los
manejan de la misma manera, utilizando mecanismos como excepciones. Rust no
tiene excepciones. En cambio, tiene el tipo `Result<T, E>` para errores
recuperables y el macro `panic!` que detiene la ejecución cuando el programa
encuentra un error irrecuperable. Este capítulo cubre primero la llamada a
`panic!` y luego habla sobre la devolución de valores `Result<T, E>`.
Además, exploraremos consideraciones al decidir si intentar recuperarse de un
error o detener la ejecución.
