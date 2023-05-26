## Implementando un patrón de diseño orientado a objetos

El *state pattern* es un patrón de diseño orientado a objetos. La esencia del 
patrón es que definimos un conjunto de estados que un valor puede tener 
internamente. Los estados están representados por un conjunto de *state 
objects*, y el comportamiento del valor cambia según su estado. Vamos a
trabajar a través de un ejemplo de un struct de publicación de blog que
tiene un campo para mantener su estado, que será un state object del conjunto
"borrador", "revisión" o "publicado".

Los state objects comparten funcionalidad: en Rust, por supuesto, usamos
structs y traits en lugar de objetos y herencia. Cada state object es
responsable de su propio comportamiento y de gobernar cuándo debe cambiar a
otro estado. El valor que contiene un state object no sabe nada sobre el
comportamiento diferente de los estados o cuándo hacer la transición entre
estados.

La ventaja de usar el state pattern es que, cuando los requisitos comerciales
del programa cambian, no necesitaremos cambiar el código del valor que
contiene el estado o el código que usa el valor. Solo necesitaremos actualizar
el código dentro de uno de los state objects para cambiar sus reglas o quizás
agregar más state objects.

Primero, vamos a implementar el state pattern de una manera más tradicional
orientada a objetos, luego usaremos un enfoque que es un poco más natural en
Rust. Vamos a profundizar en la implementación incremental de un flujo de
trabajo de publicación de blog usando el state pattern.

La funcionalidad final se verá así:

1. Un post de blog que comienza como un borrador vacío.
2. Cuando se completa el borrador, se solicita una revisión de la publicación.
3. Cuando se aprueba la publicación, se publica.
4. Solo las publicaciones de blog publicadas devuelven contenido para imprimir,
   por lo que las publicaciones no aprobadas no pueden publicarse
   accidentalmente.

Cualquier otro cambio que se intente realizar en una publicación no debería
tener ningún efecto. Por ejemplo, si intentamos aprobar un borrador de blog
antes de haber solicitado una revisión, la publicación debería seguir siendo
un borrador no publicado.

El Listado 17-11 muestra este flujo de trabajo en forma de código: este es un
ejemplo de uso de la API que implementaremos en una crate de biblioteca
llamada `blog`. Esto aún no se compilará porque no hemos implementado el crate
de biblioteca `blog`.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-oop/listing-17-11/src/main.rs:all}}
```

<span class="caption">Listing 17-11: Código que demuestra el comportamiento
deseado que queremos que tenga nuestro crate `blog`</span>

Queremos permitir que el usuario cree una nueva publicación de blog en borrador
con `Post::new`. Queremos permitir que se agregue texto a la publicación del
blog. Si intentamos obtener el contenido de la publicación inmediatamente,
antes de la aprobación, no deberíamos obtener ningún texto porque la publicación
sigue siendo un borrador. Hemos agregado `assert_eq!` en el código con fines de
demostración. Una excelente prueba unitaria para esto sería afirmar que una
publicación de blog en borrador devuelve un string vacío del método `content`,
pero no vamos a escribir pruebas para este ejemplo.

A continuación, queremos permitir una solicitud de revisión de la publicación
y queremos que `content` devuelva un string vacío mientras espera la revisión.
Cuando la publicación reciba la aprobación, debería publicarse, lo que significa
que el texto de la publicación se devolverá cuando se llame a `content`.

Observa que el único tipo con el que estamos interactuando desde el crate es
el tipo `Post`. Este tipo utilizará el state pattern y contendrá un valor que
será uno de los tres state objects que representan los diversos estados
en los que puede estar una publicación: borrador, esperando revisión o
publicado. El cambio de un estado a otro se administrará internamente dentro
del tipo `Post`. Los estados cambian en respuesta a los métodos llamados por
los usuarios de nuestra biblioteca en la instancia `Post`, pero no tienen que
administrar los cambios de estado directamente. Además, los usuarios no pueden
cometer un error con los estados, como publicar una publicación antes de que
se revise.

### Definiendo `Post` y creando una nueva instancia en el estado de borrador

¡Comencemos con la implementación de la biblioteca! Sabemos que necesitamos
un struct `Post` público que contenga algún contenido, por lo que comenzaremos
con la definición del struct y una función pública `new` asociada para crear
una instancia de `Post`, como se muestra en el Listado 17-12. También haremos
un trait privado `State` que definirá el comportamiento que todos los objetos
de estado para un `Post` deben tener.

Luego, `Post` contendrá un trait object de `Box<dyn State>` dentro de un campo
privado llamado `state` para mantener el state object. Verás por qué
`Option<T>` es necesario en un momento.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-12/src/lib.rs}}
```

<span class="caption">Listing 17-12: Definición de un struct `Post` y una 
función `new` que crea una nueva instancia de `Post`, un trait `State`, y un 
struct `Draft`</span>

El trait `State` define el comportamiento compartido por los diferentes estados
de una publicación. Los state objects son `Draft`, `PendingReview` y
`Published`, y todos implementarán el trait `State`. Por ahora, el trait no
tiene ningún método, y comenzaremos definiendo solo el estado `Draft` porque
ese es el estado en el que queremos que comience una publicación.

Cuando creamos un nuevo `Post`, estableceremos su campo `state` como un valor
`Some` que contiene un `Box` que apunta a una nueva instancia del struct
`Draft`. Esto asegura que cada vez que creemos una nueva instancia de `Post`,
comenzará como un borrador. Debido a que el campo `state` de `Post` es privado,
¡no hay forma de crear un `Post` en ningún otro estado! En la función 
`Post::new`, establecemos el campo `content` en un nuevo `String` vacío.

### Almacenando el texto del contenido del post

Vimos en el Listado 17-11 que queremos poder llamar a un método llamado 
`add_text` y pasarle un `&str` que luego se agregará como el contenido de texto
de la publicación del blog. Implementaremos esto como un método, en lugar de
exponer el campo `content` como `pub`, para que más tarde podamos implementar
un método que controlará cómo se lee el campo `content`. El método `add_text`
es bastante sencillo, así que agreguemos la implementación en el Listado 17-13
al bloque `impl Post`:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-13/src/lib.rs:here}}
```

<span class="caption">Listing 17-13: Implementando el método `add_text` para
agregar texto al campo `content` de una publicación</span>

El método `add_text` toma una referencia mutable a `self` porque estamos
cambiando la instancia de `Post` en la que estamos llamando `add_text`. Luego
llamamos a `push_str` en el `String` en `content` y pasamos el argumento `text`
para agregar al `content` guardado. Este comportamiento no depende del estado
en el que se encuentre la publicación, por lo que no es parte del state pattern.
El método `add_text` no interactúa con el campo `state` en absoluto, pero es
parte del comportamiento que queremos admitir.

### Asegurando que el contenido de un post en borrador esté vacío

Incluso después de que hayamos llamado `add_text` y agregado algún contenido a
nuestra publicación, todavía queremos que el método `content` devuelva un slice
de string vacío porque la publicación todavía está en el estado de borrador,
como se muestra en la línea 7 del Listado 17-11. Por ahora, implementemos el
método `content` con lo más simple que cumplirá con este requisito: siempre
devolver un string slice vacío. Lo cambiaremos más tarde una vez que
implementemos la capacidad de cambiar el estado de una publicación para que
pueda publicarse. Hasta ahora, las publicaciones solo pueden estar en el estado
de borrador, por lo que el contenido de la publicación siempre debe estar
vacío. El Listado 17-14 muestra esta implementación de marcador de posición:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-14/src/lib.rs:here}}
```

<span class="caption">Listing 17-14: Agregando una implementación provisional
para el método `content` en `Post` que siempre devuelve un string slice vacío
</span>

Con este método `content` añadido, todo en el Listado 17-11 hasta la línea 7
funciona como se pretendía.

### Solicitar una revisión de los cambios de publicación de su estado

A continuación, necesitamos agregar funcionalidad para solicitar una revisión
de una publicación, lo que debería cambiar su estado de `Draft` a
`PendingReview`. El Listado 17-15 muestra este código:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-15/src/lib.rs:here}}
```

<span class="caption">Listing 17-15: Implementando los métodos `request_review`
en `Post` y el trait `State`</span>

Agregamos un método público llamado `request_review` a `Post` que toma una
referencia mutable a `self`. Luego llamamos a un método interno `request_review`
en el estado actual de `Post`, y este segundo método `request_review` consume
el estado actual y devuelve un nuevo estado.

Agregamos el método `request_review` al trait `State`; todos los tipos que
implementan el trait ahora deberán implementar el método `request_review`.
Tenga en cuenta que en lugar de tener `self`, `&self` o `&mut self` como el
primer parámetro del método, tenemos `self: Box<Self>`. Esta sintaxis significa
que el método solo es válido cuando se llama en un `Box` que contiene el tipo.
Esta sintaxis toma posesión de `Box<Self>`, invalidando el estado anterior para
que el valor de estado de `Post` pueda transformarse en un nuevo estado.

Para consumir el antiguo estado, el método `request_review` debe tomar 
ownership del valor de estado. Aquí es donde entra en juego la `Option` en el
campo `state` de `Post`: llamamos al método `take` para sacar el valor `Some`
del campo `state` y dejar un `None` en su lugar, porque Rust no nos permite
tener campos no poblados en los structs. Esto nos permite mover el valor
`state` fuera de `Post` en lugar de pedir borrowing. Luego estableceremos el
valor `state` de la publicación en el resultado de esta operación.

Necesitamos establecer `state` como `None` temporalmente en lugar de 
establecerlo directamente con código como 
`self.state = self.state.request_review();` para obtener la propiedad del
valor `state`. Esto asegura que `Post` no pueda usar el valor `state` antiguo
después de que lo hayamos transformado en un nuevo estado.

El método `request_review` en `Draft` devuelve una nueva instancia de un nuevo 
struct llamado `PendingReview`, que representa el estado cuando un post está 
esperando una revisión. El struct `PendingReview` también implementa
el método `request_review`, pero no hace ninguna transformación. En cambio,
devuelve a sí mismo, porque cuando solicitamos una revisión en una publicación
que ya está en el estado `PendingReview`, debe permanecer en el estado
`PendingReview`.

Ahora podemos comenzar a ver las ventajas del state pattern: el método 
`request_review` en `Post` es el mismo sin importar su valor `state`. Cada
estado es responsable de sus propias reglas.

Dejaremos el método `content` en `Post` tal como está, devolviendo un string
slice vacío. Ahora podemos tener un `Post` en el estado `PendingReview` así
como en el estado `Draft`, pero queremos el mismo comportamiento en el estado
`PendingReview`. ¡El Listado 17-11 ahora funciona hasta la línea 10!

<!-- Old headings. Do not remove or links may break. -->
<a id="adding-the-approve-method-that-changes-the-behavior-of-content"></a>

### Agregando `approve` para cambiar el comportamiento de `content`

El método `approve` será similar al método `request_review`: establecerá el
valor de `state` al estado que el estado actual indique que debería tener 
cuando ese estado sea aprobado, como se muestra en el Listado 17-16:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-16/src/lib.rs:here}}
```

<span class="caption">Listing 17-16: Implementando el método `approve` en 
`Post` y el trait `State`</span>

Agregamos el método `approve` al trait `State` y agregamos un nuevo struct
que implementa el trait `State`, el estado `Published`.

De manera similar a cómo funciona `request_review` en `PendingReview`, si 
llamamos al método `approve` en un estado `Draft`, no tendrá efecto porque
`approve` devolverá `self`. Cuando llamamos a `approve` en `PendingReview`,
devuelve una nueva instancia de `Published` struct. El struct `Published`
implementa el trait `State`, y para ambos el método `request_review` y el
método `approve`, devuelve a sí mismo, porque la publicación debe permanecer
en el estado `Published` en esos casos.

Ahora debemos actualizar el método `content` en `Post`. Queremos que el valor
devuelto por `content` dependa del estado actual de `Post`, por lo que vamos
a hacer que `Post` delegue a un método `content` definido en su `state`, como
se muestra en el Listado 17-17:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-oop/listing-17-17/src/lib.rs:here}}
```

<span class="caption">Listing 17-17: Actualizando el método `content` en `Post` 
para delegar en un método `content` en `State`</span>

Debido a que el objetivo es mantener todas estas reglas dentro de los structs
que implementan `State`, llamamos a un método `content` en el valor en `state`
y pasamos la instancia de publicación (es decir, `self`) como argumento. Luego
devolvemos el valor devuelto del uso del método `content` en el valor `state`.

Llamamos al método `as_ref` en un `Option` porque queremos una referencia al
valor dentro del `Option` en lugar del ownership del valor. Debido a que
`state` es un `Option<Box<dyn State>>`, cuando llamamos a `as_ref`, se
devuelve una `Option<&Box<dyn State>>`. Si no llamamos a `as_ref`, obtendríamos
un error porque no podemos mover `state` fuera del `&self` prestado del
parámetro de la función.

Luego llamamos al método `unwrap`, el cual sabemos que nunca generará un error,
porque los métodos en `Post` aseguran que `state` siempre contendrá un valor
`Some` cuando esos métodos finalicen. Este es uno de los casos que mencionamos
en la sección [“Casos en los que tienes más información que el
compilador”][more-info-than-rustc]<!-- ignore --> del Capítulo 9 cuando
sabemos que un valor `None` nunca es posible, aunque el compilador no puede
entender eso.

En este punto, cuando llamamos a `content` en el `&Box<dyn State>`, la coerción
de dereferencia entrará en vigencia en el `&` y el `Box`, por lo que el método
`content` se llamará en el tipo que implementa el trait `State`. Eso significa
que debemos agregar `content` a la definición del trait `State`, y allí es
donde pondremos la lógica para qué contenido devolver dependiendo de qué
estado tengamos, como se muestra en el Listado 17-18:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-18/src/lib.rs:here}}
```

<span class="caption">Listing 17-18: Agregando el método `content` al trait
`State`</span>

Agregamos una implementación predeterminada para el método `content` que
devuelve un string slice vacío. Eso significa que no necesitamos implementar
`content` en los structs `Draft` y `PendingReview`. El struct `Published`
anulará el método `content` y devolverá el valor en `post.content`.

Es importante destacar que necesitamos anotaciones de lifetime en este método,
como discutimos en el Capítulo 10. Estamos tomando una referencia a un `post`
como argumento y devolviendo una referencia a una parte de ese `post`, por lo
que el lifetime de la referencia devuelta está relacionado con el tiempo
de vida del argumento `post`.

¡Y hemos terminado! ¡Todo lo que se muestra en el Listado 17-11 ahora funciona!
Hemos implementado el patrón de estado con las reglas del flujo de trabajo de
la publicación de blog. La lógica relacionada con las reglas vive en los
objetos de estado en lugar de estar dispersa en `Post`.

> #### ¿Por qué no un enum?
>
> Puede que te hayas preguntado por qué no usamos un `enum` con los diferentes
> estados posibles de la publicación como variantes. Esa es ciertamente una
> solución posible, ¡pruébala y compara los resultados finales para ver cuál
> prefieres! Una desventaja de usar un enum es que cada lugar que verifica el
> valor del enum necesitará una expresión `match` o similar para manejar cada
> variante posible. Esto podría ser más repetitivo que esta solución de trait
> object.

### Trade-offs del State Pattern

Hemos demostrado que Rust es capaz de implementar el State Pattern orientado a
objetos para encapsular los diferentes tipos de comportamiento que un post
debería tener en cada estado. Los métodos en `Post` no saben nada sobre los
diferentes comportamientos. La forma en que organizamos el código, solo
tenemos que mirar en un solo lugar para conocer las diferentes formas en que
un post publicado puede comportarse: la implementación del trait `State` en el
struct `Published`.

Si creáramos una implementación alternativa que no usara el State Pattern,
en su lugar podríamos usar expresiones `match` en los métodos de `Post` o
incluso en el código `main` que verifica el estado del post y cambia el
comportamiento en esos lugares. ¡Eso significaría que tendríamos que mirar en
varios lugares para comprender todas las implicaciones de un post que se
encuentra en el estado publicado! ¡Esto solo aumentaría cuanto más estados
agregáramos: cada una de esas expresiones `match` necesitaría otra opción!

Con el State Pattern, los métodos `Post` y los lugares donde usamos `Post` no
necesitan expresiones `match`, y para agregar un nuevo estado, solo 
necesitaríamos agregar un nuevo struct e implementar los métodos del trait en 
ese struct.

La implementación utilizando el State Pattern es fácil de extender para agregar
más funcionalidad. Para ver la simplicidad de mantener el código que usa el
State Pattern, prueba algunas de estas sugerencias:

* Agrega un método `reject` que cambia el estado de un post de `PendingReview`
  a `Draft`.
* Requiere dos llamadas a `approve` antes de que el estado pueda cambiar a
  `Published`.
* Permite a los usuarios agregar contenido de texto solo cuando un post está en
  el estado `Draft`. Sugerencia: haz que el objeto de estado sea responsable de
  lo que podría cambiar sobre el contenido, pero no sea responsable de modificar
  el `Post`.

Un inconveniente del State Pattern es que, debido a que los estados implementan
las transiciones entre estados, algunos de los estados están acoplados entre sí.
Si agregamos otro estado entre `PendingReview` y `Published`, como `Scheduled`,
tendríamos que cambiar el código en `PendingReview` para hacer la transición a
`Scheduled` en su lugar. Sería menos trabajo si `PendingReview` no necesitara
cambiar con la adición de un nuevo estado, pero eso significaría cambiar a
otro patrón de diseño.

Otro inconveniente es que hemos duplicado algo de lógica. Para eliminar parte
de la duplicación, podríamos intentar hacer implementaciones predeterminadas
para los métodos `request_review` y `approve` en el trait `State` que devuelvan
`self`; sin embargo, esto violaría la seguridad del objeto, porque el trait no
sabe exactamente cuál será el `self` concreto. Queremos poder usar `State` como
un objeto de trait, por lo que sus métodos deben ser seguros para el objeto.

Otra duplicación incluye las implementaciones similares de los métodos
`request_review` y `approve` en `Post`. Ambos métodos delegan a la
implementación del mismo método en el valor del campo `state` de `Option` y
establecen el nuevo valor del campo `state` en el resultado. Si tuviéramos
muchos métodos en `Post` que siguieran este patrón, podríamos considerar
definir un macro para eliminar la repetición (ver la sección [“Macros”][macros]
en el Capítulo 19).

Al implementar el State Pattern exactamente como se define en lenguajes 
orientados a objetos, no estamos aprovechando al máximo las fortalezas de Rust.
Veamos algunos cambios que podemos hacer en el crate `blog` que pueden hacer
que los estados y transiciones no válidos sean errores de tiempo de 
compilación.

#### Codificando estados y comportamiento como tipos

Vamos a mostrarte cómo replantear el State Patter para obtener un conjunto
diferente de compensaciones. En lugar de encapsular los estados y las
transiciones por completo para que el código externo no tenga conocimiento de
ellos, codificaremos los estados en diferentes tipos. En consecuencia, el
sistema de verificación de tipos de Rust evitará los intentos de usar 
publicaciones borradores donde solo se permiten publicaciones publicadas 
emitiendo un error del compilador.

Consideremos la primera parte de `main` en el Listado 17-11:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch17-oop/listing-17-11/src/main.rs:here}}
```

Todavía permitimos la creación de nuevas publicaciones en el estado de borrador
usando `Post::new` y la capacidad de agregar texto al contenido de la 
publicación. Pero en lugar de tener un método `content` en una publicación en
borrador que devuelva un string vacío, haremos que las publicaciones en
borrador no tengan el método `content` en absoluto. De esa manera, si
intentamos obtener el contenido de una publicación en borrador, obtendremos un
error del compilador que nos dice que el método no existe. Como resultado,
será imposible mostrar accidentalmente el contenido de la publicación en
borrador en producción, porque ese código ni siquiera se compilará. El Listado
17-9 muestra la definición de un struct `Post` y un struct `DraftPost`, así
como métodos en cada uno:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-19/src/lib.rs}}
```

<span class="caption">Listing 17-19: Un `Post` con un método `content` y un
`DraftPost` sin un método `content`</span>

Tanto los structs `Post` como `DraftPost` tienen un campo privado `content`
que almacena el texto de la publicación del blog. Los structs ya no tienen el
campo `state` porque estamos moviendo la codificación del estado a los tipos
de los structs. El struct `Post` representará una publicación publicada, y
tiene un método `content` que devuelve el `content`.

Todavía tenemos una función `Post::new`, pero en lugar de devolver una 
instancia de `Post`, devuelve una instancia de `DraftPost`. Debido a que
`content` es privado y no hay funciones que devuelvan `Post`, no es posible
crear una instancia de `Post` en este momento.

El struct `DraftPost` tiene un método `add_text`, por lo que podemos agregar
texto al campo `content` como antes. Sin embargo, ten en cuenta que `DraftPost`
no tiene un método `content` definido. Entonces, ahora el programa garantiza
que todas las publicaciones comienzan como publicaciones en borrador, y las
publicaciones en borrador no tienen su contenido disponible para mostrar.
Cualquier intento de evitar estas restricciones dará como resultado un error
del compilador.

#### Implementando transiciones como transformaciones en diferentes tipos

Entonces, ¿cómo obtenemos una publicación publicada? Queremos hacer cumplir
la regla de que una publicación en borrador debe ser revisada y aprobada antes
de que pueda publicarse. Una publicación en el estado de revisión pendiente
todavía no debe mostrar ningún contenido. Implementemos estas restricciones
agregando otro struct, `PendingReviewPost`, definiendo el método `request_review`
en `DraftPost` para devolver un `PendingReviewPost`, y definiendo un método
`approve` en `PendingReviewPost` para devolver un `Post`, como se muestra en
el Listado 17-20:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-20/src/lib.rs:here}}
```

<span class="caption">Listing 17-20: Un `PendingReviewPost` que se crea 
llamando a `request_review` en `DraftPost` y un método `approve` que convierte
un `PendingReviewPost` en un `Post` publicado</span>

Los métodos `request_review` y `approve` toman ownership de `self`, consumiendo
así las instancias de `DraftPost` y `PendingReviewPost` y transformándolas en
un `PendingReviewPost` y un `Post` publicado, respectivamente. De esta manera,
no tendremos ninguna instancia de `DraftPost` persistente después de haber
llamado a `request_review` en ellas, y así sucesivamente. El struct
`PendingReviewPost` no tiene un método `content` definido en él, por lo que
intentar leer su contenido da como resultado un error del compilador, como con
`DraftPost`. Debido a que la única forma de obtener una instancia de `Post`
publicada que tiene un método `content` definido es llamar al método `approve`
en un `PendingReviewPost`, y la única forma de obtener un `PendingReviewPost`
es llamar al método `request_review` en un `DraftPost`, ahora hemos codificado
el workflow de la publicación del blog en el sistema de tipos.

Pero también debemos hacer algunos cambios pequeños en `main`. Los métodos
`request_review` y `approve` devuelven nuevas instancias en lugar de modificar
el struct en el que se llaman, por lo que debemos agregar más asignaciones de
sombreado `let post =` para guardar las instancias devueltas. Tampoco podemos
tener las afirmaciones sobre el contenido de las publicaciones en borrador y
revisión pendiente sean strings vacíos, ni los necesitamos: ya no podemos
compilar el código que intenta usar el contenido de las publicaciones en esos
estados. El código actualizado en `main` se muestra en el Listado 17-21:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch17-oop/listing-17-21/src/main.rs}}
```

<span class="caption">Listing 17-21: Modificaciones a `main` para usar la nueva
implementación del workflow de la publicación del blog</span>

Las modificaciones que hicimos a `main` para reasignar `post` significan que
esta implementación ya no sigue el patrón de estado orientado a objetos: las
transformaciones entre los estados ya no están encapsuladas completamente
dentro de la implementación de `Post`. Sin embargo, nuestra ganancia es que
los estados inválidos ahora son imposibles debido al sistema de tipos y la
comprobación de tipos que ocurre en tiempo de compilación. Esto garantiza que
ciertos errores, como la visualización del contenido de una publicación no
publicada, se descubrirán antes de que lleguen a producción.

Prueba las tareas sugeridas al comienzo de esta sección en el crate `blog` tal
como está después del Listado 17-21 para evaluar el diseño de esta versión del
código. Ten en cuenta que es posible que algunas de las tareas ya estén 
completadas en este diseño.

Hemos visto que aunque Rust es capaz de implementar patrones de diseño 
orientados a objetos, también están disponibles en Rust otros patrones, como
la codificación del estado en el sistema de tipos. Estos patrones tienen
diferentes compensaciones. Aunque es posible que estés muy familiarizado con
los patrones orientados a objetos, repensar el problema para aprovechar las
características de Rust puede proporcionar beneficios, como prevenir algunos
errores en tiempo de compilación. Los patrones orientados a objetos no siempre
serán la mejor solución en Rust debido a ciertas características, como el
ownership, que los lenguajes orientados a objetos no tienen.

## Resumen

Sin importar si consideras a Rust como un lenguaje orientado a objetos después
de leer este capítulo, ahora sabes que puedes usar objetos de tipo trait para
obtener algunas características orientadas a objetos en Rust. La 
despatronización dinámica puede brindarle a tu código cierta flexibilidad a 
cambio de un poco de rendimiento en tiempo de ejecución. Puedes usar esta
flexibilidad para implementar patrones orientados a objetos que pueden ayudar
a la mantenibilidad de tu código. Rust también tiene otras características,
como el ownership, que los lenguajes orientados a objetos no tienen. Un patrón
orientado a objetos no siempre será la mejor manera de aprovechar las
fortalezas de Rust, pero es una opción disponible.

A continuación, veremos los patterns, que son otra de las características de 
Rust que permiten mucha flexibilidad. Hemos visto brevemente los patterns a lo
largo del libro, pero aún no hemos visto su capacidad total. ¡Vamos allá! 

[more-info-than-rustc]: ch09-03-to-panic-or-not-to-panic.html#cases-in-which-you-have-more-information-than-the-compiler
[macros]: ch19-06-macros.html#macros
