## `RefCell<T>` y el Patrón de Mutabilidad Interior

La mutabilidad interna es un patrón de diseño en Rust que te permite mutar datos
incluso cuando hay referencias inmutables a esos datos; normalmente, esta acción
está prohibida por las reglas de borrowing. Para mutar datos, el patrón utiliza
código `unsafe` dentro de una estructura de datos para flexibilizar las reglas
habituales de Rust que rigen la mutabilidad y el borrowing. El código unsafe
indica al compilador que estamos verificando las reglas manualmente en lugar de
confiar en que el compilador las verifique por nosotros; discutiremos el código
unsafe con más detalle en el Capítulo 19.

Podemos utilizar tipos que utilizan el patrón de mutabilidad interna solo cuando
podemos asegurar que las reglas de borrowing se seguirán en tiempo de ejecución,
aunque el compilador no pueda garantizarlo. El código `unsafe` involucrado se
envuelve entonces en una API segura, y el tipo externo sigue siendo inmutable.

Vamos a explorar este concepto al examinar el tipo `RefCell<T>` que sigue el
patrón de mutabilidad interna.

### Cumpliendo las reglas de borrowing en tiempo de ejecución con `RefCell<T>`

A diferencia de `Rc<T>`, el tipo `RefCell<T>` representa un único ownership
sobre los datos que contiene. Entonces, ¿qué hace que `RefCell<T>` sea diferente
de un tipo como `Box<T>`? Recuerda las reglas de borrowing que aprendiste en el
Capítulo 4:

- En cualquier momento dado, puedes tener _o bien_ una referencia mutable _o
  bien_ cualquier número de referencias inmutables.
- Las referencias siempre deben ser válidas.

Con referencias y `Box<T>`, las invariantes de las reglas de borrowing se hacen
cumplir en tiempo de compilación. Con `RefCell<T>`, estas invariantes se hacen
cumplir _en tiempo de ejecución_. Con referencias, si rompes estas reglas,
obtendrás un error de compilación. Con `RefCell<T>`, si rompes estas reglas, tu
programa entrará en panic y saldrá.

La ventaja de comprobar las reglas de borrowing en tiempo de compilación es que
los errores se detectarán antes en el proceso de desarrollo, y no hay impacto en
el rendimiento en tiempo de ejecución porque todo el análisis se completa de
antemano. Por estas razones, comprobar las reglas de borrowing en tiempo de
compilación es la mejor opción en la mayoría de los casos, por lo que esta es la
opción predeterminada de Rust.

La ventaja de comprobar las reglas de borrowing en tiempo de ejecución es que
se permiten ciertos escenarios seguros de memoria, donde habrían sido
rechazados por las comprobaciones en tiempo de compilación. El análisis estático,
como el compilador de Rust, es inherentemente conservador. Algunas propiedades
del código son imposibles de detectar analizando el código: el ejemplo más
famoso es el Problema de la Parada, que está fuera del alcance de este libro,
pero es un tema interesante para investigar.

Debido a que algunos análisis son imposibles, si el compilador de Rust no puede
estar seguro de que el código cumple con las reglas de ownership, podría
rechazar un programa correcto; de esta manera, es conservador. Si Rust aceptara
un programa incorrecto, los usuarios no podrían confiar en las garantías que
Rust hace. Sin embargo, si Rust rechaza un programa correcto, el programador se
verá perjudicado, pero no puede ocurrir nada catastrófico. El tipo `RefCell<T>`
es útil cuando estás seguro de que tu código sigue las reglas de borrowing, pero
el compilador no puede entenderlo y garantizarlo.

Similar a `Rc<T>`, `RefCell<T>` solo se usa en escenarios de un solo hilo y te
dará un error de tiempo de compilación si intentas usarlo en un contexto
multihilo. Hablaremos de cómo obtener la funcionalidad de `RefCell<T>` en un
programa multihilo en el Capítulo 16.

Aquí tienes un resumen de las razones para elegir `Box<T>`, `Rc<T>` o
`RefCell<T>`:

- `Rc<T>` permite múltiples propietarios de los mismos datos; `Box<T>` y
  `RefCell<T>` tienen un único propietario.
- `Box<T>` permite borrowing inmutable o mutable verificado en tiempo de
  compilación; `Rc<T>` permite solo borrowing inmutable verificado en tiempo de
  compilación; `RefCell<T>` permite borrowing inmutable o mutable verificado en
  tiempo de ejecución.
- Debido a que `RefCell<T>` permite borrowing mutable verificado en tiempo de
  ejecución, puedes mutar el valor dentro de la `RefCell<T>` incluso cuando la
  `RefCell<T>` es inmutable.

Mutar el valor dentro de un valor inmutable es el patrón de _mutabilidad
interior_. Veamos una situación en la que la mutabilidad interior es útil y
examinemos cómo es posible.

### Mutabilidad Interior: Un Borrow Mutable a un Valor Inmutable

Una consecuencia de las reglas de borrowing es que cuando tienes un valor
inmutable, no puedes pedir prestado una referencia mutable a través de ese
valor. Por ejemplo, este código no compilará:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/no-listing-01-cant-borrow-immutable-as-mutable/src/main.rs}}
```

Si intentas compilar este código, obtendrás el siguiente error:

```console
{{#include ../listings/ch15-smart-pointers/no-listing-01-cant-borrow-immutable-as-mutable/output.txt}}
```

Sin embargo, hay situaciones en las que sería útil que un valor se mute a sí
mismo en sus métodos, pero parezca inmutable para otro código. El código fuera
de los métodos del valor no podría mutar el valor. Usar `RefCell<T>` es una
forma de obtener la capacidad de tener mutabilidad interior, pero `RefCell<T>`
no evita las reglas de borrowing por completo: el comprobador de préstamos en el
compilador permite esta mutabilidad interior, y las reglas de borrowing se
comprueban en tiempo de ejecución en su lugar. Si violas las reglas, obtendrás
un `panic!` en lugar de un error del compilador.

Vamos a trabajar a través de un ejemplo práctico donde podemos usar `RefCell<T>`
para mutar un valor inmutable y ver por qué es útil.

#### Un Caso de Uso para la Mutabilidad Interior: Mock Objects

A veces durante el testing, un programador usará un tipo en lugar de otro para
observar un comportamiento particular y afirmar que se implementa correctamente.
Este tipo de marcador de posición se llama _test double_. Piensa en ello en el
sentido de un "doble de riesgo" en la realización de películas, donde una
persona entra y sustituye a un actor para hacer una escena particularmente
difícil. Los test doubles se sustituyen por otros tipos cuando se ejecutan las
pruebas. Los _objetos simulados_ son tipos específicos de test doubles que
registran lo que sucede durante una prueba para que puedas afirmar que se
produjeron las acciones correctas.

Rust no tiene objetos en el mismo sentido que otros lenguajes tienen objetos, y
Rust no tiene funcionalidad de objetos simulados integrada en la biblioteca
estándar como lo hacen otros lenguajes. Sin embargo, definitivamente puedes
crear una struct que sirva para los mismos propósitos que un objeto
simulado.

Aquí está el escenario que vamos a probar: crearemos una biblioteca que realiza
un seguimiento de un valor en relación con un valor máximo, y envía mensajes
en función de la proximidad del valor actual al valor máximo. Esta biblioteca
podría usarse para realizar un seguimiento de la cuota de un usuario para el
número de llamadas a la API que se le permite realizar, por ejemplo.

El objetivo de nuestra biblioteca es proporcionar la funcionalidad de realizar
un seguimiento de qué tan cerca está un valor de su máximo y que mensajes se
deben enviar en qué momentos. Se espera que las aplicaciones que utilicen
nuestra biblioteca proporcionen el mecanismo para enviar los mensajes: la
aplicación podría poner un mensaje en la interfaz de la aplicación, enviar un
correo electrónico, enviar un mensaje de texto o algo más. La biblioteca no
necesita saber ese detalle. Todo lo que necesita es algo que implemente un
trait que proporcionaremos llamado `Messenger`. El listado 15-20 muestra el
código de la biblioteca:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-20/src/lib.rs}}
```

<span class="caption">Listing 15-20: Una biblioteca para realizar un seguimiento
de qué tan cerca está un valor a su valor máximo y emitir advertencias
cuando el valor alcanza ciertos niveles.</span>

Una parte importante de este código es el trait `Messenger`, que tiene un método
llamado `send` que toma una referencia inmutable a `self` y el texto del
mensaje. Este trait es la interfaz que nuestro objeto simulado necesita
implementar para que el simulado se pueda usar de la misma manera que un objeto
real. La otra parte importante es que queremos probar el comportamiento del
método `set_value` en el `LimitTracker`. Podemos cambiar lo que pasamos para el
parámetro `value`, pero `set_value` no devuelve nada para que podamos hacer
afirmaciones. Queremos poder decir que si creamos un `LimitTracker` con algo
que implemente el trait `Messenger` y un valor particular para `max`, cuando
pasemos diferentes números para `value`, se le dice al mensajero que envíe los
mensajes apropiados.

Necesitamos un objeto simulado que, en lugar de enviar un email o un mensaje de
texto cuando llamamos a `send`, solo haga un seguimiento de los mensajes que se
le dice que envíe. Podemos crear una nueva instancia del objeto simulado,
crear un `LimitTracker` que use el objeto simulado, llamar al método
`set_value` en `LimitTracker` y luego verificar que el objeto simulado tenga los
mensajes que esperamos. El listado 15-21 muestra un intento de implementar un
objeto simulado para hacer precisamente eso, pero el borrow checker
no lo permite:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-21/src/lib.rs:here}}
```

<span class="caption">Listing 15-21: Un intento de implementar un `MockMessenger`
que no es permitido por el borrow checker</span>

Este código de test define un struct `MockMessenger` que tiene un campo
`sent_messages` que es una `Vec` de `String` valores. Definimos una función
asociada `new` para que sea conveniente crear nuevos valores `MockMessenger`
que comiencen con una lista vacía de mensajes. Luego implementamos el trait
`Messenger` para `MockMessenger` para que podamos darle un `MockMessenger` a un
`LimitTracker`. En la definición del método `send`, tomamos el mensaje pasado
como parámetro y lo almacenamos en la lista `MockMessenger` de `sent_messages`.

En el test, estamos testeando qué sucede cuando el `LimitTracker` se le dice que
establezca `value` en algo que es más del 75 por ciento del valor `max`. En
primer lugar, creamos un nuevo `MockMessenger`, que comenzará con una lista
vacía de mensajes. Luego creamos un nuevo `LimitTracker` y le damos una
referencia al nuevo `MockMessenger` y un valor `max` de 100. Llamamos al método
`set_value` en el `LimitTracker` con un valor de 80, que es más del 75 por
ciento de 100. Luego afirmamos que la lista de mensajes que el `MockMessenger`
está realizando un seguimiento debería tener ahora un mensaje en ella.

Sin embargo, hay un problema con este test, como se muestra aquí:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-21/output.txt}}
```

No podemos modificar `sent_messages` para realizar un seguimiento de los
mensajes, porque el método `send` toma una referencia inmutable a `self`.
Tampoco podemos tomar la sugerencia del texto de error para usar `&mut self`
en su lugar, porque entonces la firma de `send` no coincidiría con la firma en
la definición del trait `Messenger` (siéntase libre de intentarlo y ver qué
mensaje de error obtiene).

Esta es una situación en la que la mutabilidad interior puede ayudar.
Almacenaremos los `sent_messages` dentro de un `RefCell<T>`, y luego el método
`send` podrá modificar `sent_messages` para almacenar los mensajes que hemos
visto. El listado 15-22 muestra cómo se ve eso:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-22/src/lib.rs:here}}
```

<span class="caption">Listing 15-22: Usando `RefCell<T>` para mutar un valor
interno mientras el valor externo se considera inmutable.</span>

El campo `sent_messages` ahora es de tipo `RefCell<Vec<String>>` en lugar de
`Vec<String>`. En la función `new`, creamos una nueva instancia de
`RefCell<Vec<String>>` alrededor del vector vacío.

En la implementación del método `send`, el primer parámetro sigue siendo un
inmutable borrow de `self`, que coincide con la definición del trait. Llamamos
a `borrow_mut` en el `RefCell<Vec<String>>` en `self.sent_messages` para obtener
una referencia mutable al valor dentro del `RefCell<Vec<String>>`, que es el
vector. Luego podemos llamar a `push` en la referencia mutable al vector para
hacer un seguimiento de los mensajes enviados durante el test.

La última modificación que debemos hacer está en la afirmación: para ver cuántos
elementos hay en el vector interno, llamamos a `borrow` en el
`RefCell<Vec<String>>` para obtener una referencia inmutable al vector.

Ahora que has visto cómo usar `RefCell<T>`, ¡profundicemos en cómo funciona!

#### Haciendo un seguimiento del borrowing en runtime con `RefCell<T>`

Cuando creamos referencias inmutables y mutables, usamos la sintaxis `&` y
`&mut`, respectivamente. Con `RefCell<T>`, usamos los métodos `borrow` y
`borrow_mut`, que son parte de la API segura que pertenece a `RefCell<T>`. El
método `borrow` devuelve el tipo de smart pointer `Ref<T>`, y `borrow_mut`
devuelve el tipo de smart pointer `RefMut<T>`. Ambos tipos implementan `Deref`,
por lo que podemos tratarlos como referencias regulares.

`RefCell<T>` realiza un seguimiento de cuántos smart pointers `Ref<T>` y
`RefMut<T>` están actualmente activos. Cada vez que llamamos a `borrow`, el
`RefCell<T>` aumenta su recuento de cuántos borrowing inmutables están activos.
Cuando un valor `Ref<T>` sale del scope, el recuento de borrowing inmutables
disminuye en uno. Al igual que las reglas de borrowing en tiempo de compilación,
`RefCell<T>` nos permite tener muchos borrowing inmutables o un borrowing
mutable en un momento dado.

Si intentamos romper estas reglas, en lugar de obtener un error del compilador
como lo haríamos con las referencias, la implementación de `RefCell<T>` se
bloqueará en tiempo de ejecución. El listado 15-23 muestra una modificación de
la implementación de `send` en el listado 15-22. Estamos tratando
deliberadamente de crear dos borrowing mutables activos para el mismo scope
para ilustrar que `RefCell<T>` nos impide hacer esto en tiempo de ejecución.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,panics
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-23/src/lib.rs:here}}
```

<span class="caption">Listing 15-23: Creando dos referencias mutables en el
mismo scope para ver que `RefCell<T>` lanzará un panic</span>

Creamos una variable `one_borrow` para el smart pointer `RefMut<T>` devuelto
desde `borrow_mut`. Luego creamos otro borrowing mutable de la misma manera en
la variable `two_borrow`. Esto hace dos referencias mutables en el mismo scope,
lo cual no está permitido. Cuando ejecutamos los tests para nuestra librería, el
código en el listado 15-23 se compilará sin errores, pero el test fallará:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-23/output.txt}}
```

Observa que el código entró en panic con el mensaje `already borrowed:
BorrowMutError`. Así es como `RefCell<T>` maneja las violaciones de las reglas
de borrowing en tiempo de ejecución.

Elegir capturar errores de borrowing en tiempo de ejecución en lugar de en
tiempo de compilación, como lo hemos hecho aquí, significa que potencialmente
encontrarías errores en tu código más tarde en el proceso de desarrollo:
posiblemente no hasta que tu código se implemente en producción. Además, tu
código incurriría en una pequeña penalización de rendimiento en tiempo de
ejecución como resultado de realizar un seguimiento de los borrows en tiempo de
ejecución en lugar de en tiempo de compilación. Sin embargo, usar `RefCell<T>`
hace posible escribir un objeto simulado que pueda modificarse para realizar un
seguimiento de los mensajes que ha visto mientras lo estás usando en un
contexto donde solo se permiten valores inmutables. Puedes usar `RefCell<T>`
a pesar de sus compensaciones para obtener más funcionalidad de la que
proporcionan las referencias regulares.

### Teniendo múltiples propietarios de datos mutables combinando `Rc<T>` y `RefCell<T>`

Una forma común de usar `RefCell<T>` es en combinación con `Rc<T>`. Recuerda
que `Rc<T>` te permite tener múltiples propietarios de algunos datos, pero solo
te da acceso inmutable a esos datos. Si tienes un `Rc<T>` que contiene un
`RefCell<T>`, puedes obtener un valor que puede tener múltiples propietarios y
que puedes mutar.

Por ejemplo, recuerda el ejemplo de la lista de cons en el Listado 15-18 donde
usamos `Rc<T>` para permitir que múltiples listas compartan propiedad de otra
lista. Debido a que `Rc<T>` contiene solo valores inmutables, no podemos cambiar
ninguno de los valores en la lista una vez que los hemos creado. Agreguemos
`RefCell<T>` para obtener la capacidad de cambiar los valores en las listas.
El listado 15-24 muestra que al usar un `RefCell<T>` en la definición de `Cons`,
podemos modificar el valor almacenado en todas las listas:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-24/src/main.rs}}
```

<span class="caption">Listing 15-24: Usando `Rc<RefCell<i32>>` para crear una
`List` que podemos modificar.</span>

Creamos un valor que es una instancia de `Rc<RefCell<i32>>` y lo almacenamos en
una variable llamada `value` para que podamos acceder a él directamente más
tarde. Luego creamos una `List` en `a` con una variante `Cons` que contiene
`value`. Necesitamos clonar `value` para que tanto `a` como `value` tengan
ownership del valor interno `5` en lugar de transferir el ownership de `value`
a `a` o tener `a` pedir prestado de `value`.

Envolvemos la lista `a` en un `Rc<T>` para que cuando creemos las listas `b` y
`c`, ambas puedan referirse a `a`, que es lo que hicimos en el listado 15-18.

Después de haber creado las listas en `a`, `b` y `c`, queremos agregar 10 al
valor en `value`. Hacemos esto llamando a `borrow_mut` en `value`, que usa la
característica de dereferenciación automática que discutimos en el capítulo 5
(ver la sección [“¿Dónde está el operador `->`?”][donde-esta-el-operador--]<!--
ignore -->) para desreferenciar el `Rc<T>` al valor interno `RefCell<T>`.
El método `borrow_mut` devuelve un smart pointer `RefMut<T>`, y usamos el
operador de desreferenciación en él y cambiamos el valor interno.

Cuando imprimimos `a`, `b` y `c`, podemos ver que todos tienen el valor
modificado de 15 en lugar de 5:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-24/output.txt}}
```

¡Esta técnica es bastante genial! Al usar `RefCell<T>`, tenemos un valor
`List` externamente inmutable. Pero podemos usar los métodos en `RefCell<T>`
que proporcionan acceso a su mutabilidad interior para que podamos modificar
nuestros datos cuando sea necesario. Las comprobaciones en tiempo de ejecución
de las reglas de borrowing nos protegen de las condiciones de carrera en los
datos y, a veces, vale la pena intercambiar un poco de velocidad por esta
flexibilidad en nuestras estructuras de datos. ¡Ten en cuenta que `RefCell<T>`
no funciona para código multihilo! `Mutex<T>` es la versión segura para hilos
de `RefCell<T>` y discutiremos `Mutex<T>` en el capítulo 16.

[donde-esta-el-operador--]: ch05-03-method-syntax.html#donde-esta-el-operador--
