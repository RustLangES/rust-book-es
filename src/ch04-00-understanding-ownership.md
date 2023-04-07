# Entendiendo el Ownership

El Ownership es la característica más única de Rust y tiene implicaciones
profundas para el resto del lenguaje. Permite a Rust hacer garantías de
seguridad de memoria sin necesidad de un recolector de basura, por lo que es
importante entender cómo funciona el Ownership. En este capítulo, hablaremos
sobre el Ownership así como varias características relacionadas: préstamo 
(borrowing), slices, y cómo Rust organiza los datos en la memoria.