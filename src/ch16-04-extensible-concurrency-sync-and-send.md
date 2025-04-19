## Concurrencia extensible con los traits `Sync` y `Send`

Curiosamente, el lenguaje Rust tiene _muy_ pocas características de
concurrencia. Casi todas las características de concurrencia de las que hemos
hablado hasta ahora en este capítulo han sido parte de la biblioteca estándar,
no del lenguaje. Sus opciones para manejar la concurrencia no se limitan al
lenguaje o a la biblioteca estándar; puede escribir sus propias características
de concurrencia o usar las escritas por otros.

Sin embargo, dos conceptos de concurrencia están integrados en el lenguaje: los
traits `Sync` y `Send` de `std::marker`.

### Permitiendo la transferencia de Ownership entre hilos con `Send`

El trait `Send` indica que la propiedad de un valor se puede transferir entre
hilos. Casi todos los tipos son `Send`, con algunas excepciones notables, como
`Rc<T>`, que no es `Send` porque si clonara un valor de `Rc<T>` y tratara de
transferir la propiedad del clon a otro hilo, ambos hilos podrían actualizar el
recuento de referencias al mismo tiempo. Por esta razón, `Rc<T>` está
implementado para su uso en situaciones de un solo hilo donde no desea pagar la
penalización de rendimiento segura para subprocesos.

Por lo tanto, el sistema de tipos y los límites de los traits de Rust garantizan
que nunca pueda enviar accidentalmente un valor `Rc<T>` a través de hilos de
forma insegura. Cuando intentamos hacer esto en el Listado 16-14, obtuvimos el
error `the trait Send is not implemented for Rc<Mutex<i32>>`. Cuando cambiamos a
`Arc<T>`, que es `Send`, el código se compiló.

Cualquier tipo compuesto enteramente de tipos `Send` se marca automáticamente
como `Send` también. Casi todos los tipos primitivos son `Send`, aparte de los
punteros sin procesar, que discutiremos en el Capítulo 20.

### Permitiendo el acceso desde múltiples hilos con `Sync`

El trait `Sync` indica que es seguro que el tipo que implementa `Sync` se
referencie desde múltiples hilos. En otras palabras, cualquier tipo `T` es
`Sync` si `&T` (una referencia inmutable a `T`) es `Send`, lo que significa que
la referencia se puede enviar de forma segura a otro hilo. De manera similar a
`Send`, los tipos primitivos son `Sync`, y los tipos compuestos enteramente de
tipos que son `Sync` también son `Sync`.

El smart pointer `Rc<T>` tampoco es `Sync` por las mismas razones por las que
no es `Send`. El tipo `RefCell<T>` (del que hablamos en el Capítulo 15) y la
familia de tipos relacionados `Cell<T>` no son `Sync`. La implementación de la
comprobación de préstamos que hace `RefCell<T>` en tiempo de ejecución no es
segura para subprocesos. El smart pointer `Mutex<T>` es `Sync` y se puede usar
para compartir el acceso con múltiples hilos como viste en la sección [“Compartir
un `Mutex<T>` entre múltiples
hilos”][compartir-un-mutext-entre-varios-hilos]<!-- ignore -->.

### Implementar `Send` y `Sync` manualmente es inseguro

Debido a que los tipos que están compuestos de los traits `Send` y `Sync` se
automatizan también `Send` y `Sync`, no tenemos que implementar esos traits
manualmente. Como marcadores de traits, ni siquiera tienen ningún método para
implementar. Son útiles para hacer cumplir invariantes relacionados con la
concurrencia.

Implementar manualmente estos traits implica implementar código inseguro de
Rust. Hablaremos sobre el uso de código inseguro de Rust en el Capítulo 20; por
ahora, la información importante es que la construcción de nuevos tipos
concurrentes que no están compuestos de partes `Send` y `Sync` requiere un
pensamiento cuidadoso para mantener las garantías de seguridad. [“The
Rustonomicon”][nomicon] tiene más información sobre estas garantías y cómo
mantenerlas.

## Resumen

No es la última vez que verás la concurrencia en este libro: todo el siguiente 
capitulo esta enfocado en programación asíncrona, y el proyecto del Capítulo 21 
usará los conceptos de este capítulo en una situación más realista que los 
ejemplos más pequeños que se discuten aquí.

Como se mencionó anteriormente, debido a que muy poco de cómo Rust maneja la
concurrencia es parte del lenguaje, muchas soluciones de concurrencia se
implementan como cajones. Estos evolucionan más rápido que la biblioteca
estándar, así que asegúrese de buscar en línea las cajas actuales de última
generación para usar en situaciones de múltiples subprocesos.

La biblioteca estándar de Rust proporciona canales para el paso de mensajes y
tipos de smart pointer, como `Mutex<T>` y `Arc<T>`, que son seguros de usar en
contextos concurrentes. El sistema de tipos y el borrow checker garantizan que
el código que usa estas soluciones no terminará con carreras de datos o
referencias no válidas. Una vez que haya compilado su código, puede estar
seguro de que se ejecutará felizmente en múltiples hilos sin los tipos de
errores difíciles de rastrear comunes en otros lenguajes. La programación
concurrente ya no es un concepto del que tener miedo: ¡adelante y haga que sus
programas sean concurrentes, sin miedo!

A continuación, hablaremos sobre las formas idiomáticas de modelar problemas y
estructurar soluciones a medida que sus programas Rust se vuelven más grandes.
Además, discutiremos cómo los ideales de Rust se relacionan con los que
puede estar familiarizado con la programación orientada a objetos.

[compartir-un-mutext-entre-varios-hilos]: ch16-03-shared-state.html#compartir-un-mutext-entre-varios-hilos
[nomicon]: https://doc.rust-lang.org/nomicon/index.html
