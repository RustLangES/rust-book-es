## Paths para referirse a un item en el árbol de módulos

Para mostrarle a Rust dónde encontrar un item en el árbol de módulos, usamos un
path de la misma manera que usamos un path para navegar un sistema de archivos.
Para llamar una función, necesitamos saber su path.

Un path puede tomar dos formas:

* Un *path absoluto* comienza desde la raíz del crate; para código de un crate
  externa, el path absoluto comienza con el nombre del crate, y para código del
  crate actual, comienza con el literal `crate`.

* Un *path relativo* comienza desde el módulo actual y usa `self`, `super`, o un
  identificador en el módulo actual.

Tanto los paths absolutos como los relativos están seguidos por uno o más
identificadores separados por dos puntos dobles (`::`).

Volviendo al Listing 7-1, digamos que queremos llamar a la función
`add_to_waitlist` desde la función `eat_at_restaurant` definida en el crate
root. Este es el mismo que preguntar: ¿cuál es el path de la función
`add_to_waitlist`? El Listing 7-3 contiene el Listing 7-1 con algunos de los
módulos y funciones removidas.

Mostraremos dos formas de llamar a la función `add_to_waitlist` desde una nueva
función `eat_at_restaurant` definida en el crate root. Estos paths son
correctos, pero hay otro problema que impide que este ejemplo compile tal cual.
Explicaremos por qué en un momento.

La función `eat_at_restaurant` es parte de la API pública del crate de nuestra
librería, así que la marcamos con la palabra clave `pub`. En la sección
[“Exponiendo Paths con la palabra clave `pub`”][pub]<!-- ignore -->, iremos en
más detalle sobre `pub`.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-03/src/lib.rs}}
```

<span class="caption">Listing 7-3: Llamando a la función `add_to_waitlist` usando
paths absolutos y relativos</span>

La primera vez que llamamos a la función `add_to_waitlist` en `eat_at_restaurant`,
usamos un path absoluto. La función `add_to_waitlist` está definida en el mismo
crate que `eat_at_restaurant`, lo que significa que podemos usar la palabra
clave `crate` para comenzar un path absoluto. Luego incluimos cada uno de los
módulos sucesivos hasta que llegamos a `add_to_waitlist`. Puedes imaginar un
sistema de archivos con la misma estructura: especificaríamos el path
`/front_of_house/hosting/add_to_waitlist` para ejecutar el programa
`add_to_waitlist`; usar el nombre `crate` para comenzar desde la raíz del crate
es como usar `/` para comenzar desde la raíz del sistema de archivos en tu
shell.

La segunda vez que llamamos a `add_to_waitlist` en `eat_at_restaurant`, usamos
un path relativo. El path comienza con `front_of_house`, el nombre del módulo
definido al mismo nivel del árbol de módulos que `eat_at_restaurant`. Aquí el
equivalente en el sistema de archivos sería usar el path
`front_of_house/hosting/add_to_waitlist`. Comenzar con el nombre del módulo
significa que el path es relativo.

Elegir si usar un path relativo o absoluto es una decisión que tomarás basado
en tu proyecto, y depende de si es más probable que muevas la definición de un
item de código separadamente o junto con el código que usa el item. Por
ejemplo, si movemos el módulo `front_of_house` y la función `eat_at_restaurant`
a un módulo llamado `customer_experience`, necesitaríamos actualizar el path
absoluto a `add_to_waitlist`, pero el path relativo seguiría siendo válido.
Sin embargo, si movemos la función `eat_at_restaurant` separadamente a un
módulo llamado `dining`, el path absoluto a la llamada de `add_to_waitlist`
seguiría siendo el mismo, pero el path relativo necesitaría ser actualizado.
Nuestra preferencia en general es especificar paths absolutos porque es más
probable que queramos mover definiciones de código y llamadas de items
independientemente.

¡Intentemos compilar el Listing 7-3 y averigüemos por qué aún no compila! El
error que obtenemos se muestra en el Listing 7-4.

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-03/output.txt}}
```

<span class="caption">Listing 7-4: Errores de compilación al hacer building del 
código del Listing 7-3</span>

El mensaje de error dice que el módulo `hosting` es privado. En otras palabras,
tenemos los paths correctos para el módulo `hosting` y la función
`add_to_waitlist`, pero Rust no nos deja usarlos porque no tiene acceso a las
secciones privadas. En Rust, todos los items (funciones, métodos, structs,
enums, módulos, y constantes) son privados a los módulos padres por defecto. Si
quieres hacer un item como una función o struct privado, lo pones en un módulo.

Los items en un módulo privado no pueden ser accedidos por un path externo
absoluto, porque el módulo padre no puede ver dentro de los módulos privados de
sus hijos. El módulo padre puede ver el contenido de sus módulos hijos porque
los módulos hijos están dentro del módulo padre. Para continuar con nuestra
metáfora, piensa en las reglas de privacidad como la oficina de atrás de un
restaurante: lo que pasa ahí es privado para los clientes del restaurante, pero
los gerentes de la oficina pueden ver y hacer todo en el restaurante que
operan.

Rust elige tener el sistema de módulos funcionando de esta forma para que
ocultar detalles de implementación internos sea lo predeterminado. De esta
forma, sabes qué partes del código interno puedes cambiar sin romper el código
externo. Sin embargo, Rust te da la opción de exponer partes internas del código
de los módulos hijos a los módulos ancestros externos usando la palabra clave
`pub` para hacer un item público.

### Exponiendo Paths con la palabra clave `pub`

Volviendo al error en el Listing 7-4 que nos dijo que el módulo `hosting` es
privado, queremos que la función `eat_at_restaurant` en el módulo padre tenga
acceso a la función `add_to_waitlist` en el módulo hijo, así que marcamos el
módulo `hosting` con la palabra clave `pub`, como se muestra en el Listing 7-5.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-05/src/lib.rs}}
```

<span class="caption">Listing 7-5: Declarando el módulo `hosting` como `pub` 
para usarlo desde `eat_at_restaurant`</span>

Desafortunadamente, el código en el Listing 7-5 aún resulta en un error, como se
muestra en el Listing 7-6.

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-05/output.txt}}
```

<span class="caption">Listing 7-6: Errores de compilación al hacer building del 
código del Listing 7-5</span>

¿Qué pasó? Agregar la palabra clave `pub` al frente del módulo `hosting` hace
que el módulo sea público. Con este cambio, si podemos acceder a
`front_of_house`, podemos acceder a `hosting`. Pero el *contenido* de `hosting`
sigue siendo privado; hacer el módulo público no hace que su contenido sea
público. La palabra clave `pub` en un módulo solo permite que el código en sus
módulos ancestros se refiera a él, no acceder a su código interno. Debido a que
los módulos son contenedores, no hay mucho que podamos hacer solo haciendo que
el módulo sea público; necesitamos ir más allá y elegir hacer que uno o más de
los items dentro del módulo sean públicos también.

El error en el Listing 7-6 dicen que la función `add_to_waitlist` es privada.
Las reglas de privacidad se aplican a structs, enums, funciones, y métodos, así
como a módulos.

Para hacer que la función `add_to_waitlist` sea pública, necesitamos agregar la
palabra clave `pub` antes de su definición, como se muestra en el Listing 7-7.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-07/src/lib.rs}}
```

<span class="caption">Listing 7-7: Agregar la keyword `pub` a `mod hosting`
y `fn add_to_waitlist` nos permite llamar a la función desde 
`eat_at_restaurant`</span>

¡Ahora el código compilará! Para ver por qué agregar la palabra clave `pub` nos
permite usar estos paths en `add_to_waitlist` con respecto a las reglas de
privacidad, veamos los paths absolutos y relativos.

En el path absoluto, comenzamos con `crate`, la raíz de nuestro árbol de módulos
de la caja. El módulo `front_of_house` está definido en la raíz de la caja. Si
bien `front_of_house` no es público, porque la función `eat_at_restaurant` está
definida en el mismo módulo que `front_of_house` (es decir, `eat_at_restaurant`
y `front_of_house` son hermanos), podemos referirnos a `front_of_house` desde
`eat_at_restaurant`. A continuación está el módulo `hosting` marcado con `pub`.
Podemos acceder al módulo padre de `hosting`, por lo que podemos acceder a
`hosting`. ¡Finalmente, la función `add_to_waitlist` está marcada con `pub` y
podemos acceder a su módulo padre, por lo que está llamada a función funciona!

En el path relativo, la lógica es la misma que el path absoluto, excepto por el
primer paso: en lugar de comenzar desde la raíz de la caja, el path comienza
desde `front_of_house`. El módulo `front_of_house` está definido dentro del
mismo módulo que `eat_at_restaurant`, por lo que el path relativo que comienza
desde el módulo en el que se define `eat_at_restaurant` funciona. Luego,
porque `hosting` y `add_to_waitlist` están marcados con `pub`, el resto del path
funciona, ¡y está llamada a función es válida!

Si planeas compartir tu caja de biblioteca para que otros proyectos puedan usar
tu código, tu API pública es tu contrato con los usuarios de tu caja que
determina cómo pueden interactuar con tu código. Hay muchas consideraciones
sobre cómo administrar los cambios en tu API pública para que sea más fácil que
la gente dependa de tu caja. Estas consideraciones están fuera del alcance de
este libro; si estás interesado en este tema, consulta [The Rust API
Guidelines][api-guidelines].

> #### Buenas prácticas para paquetes con un binario y una biblioteca
>
> Mencionamos que un paquete puede contener tanto un binario *src/main.rs* como
> una biblioteca *src/lib.rs*, y ambos tendrán el nombre del paquete de forma
> predeterminada. Típicamente, los paquetes con este patrón de contener tanto
> una biblioteca como un binario tendrán solo el código suficiente en el binario
> para iniciar un ejecutable que llame al código con la biblioteca. Esto permite
> que otros proyectos se beneficien de la mayor funcionalidad que proporciona el
> paquete, porque el código de la biblioteca se puede compartir.
>
> El árbol de módulos debería ser definido en *src/lib.rs*. Luego, cualquier
> item público puede ser usado en el binario comenzando los paths con el nombre
> del paquete. El binario se convierte en un usuario de la biblioteca de la
> misma forma que un crate completamente externo usaría la biblioteca: solo
> puede usar la API pública. Esto te ayuda a diseñar una buena API; no solo eres
> el autor, ¡también eres un cliente!
>
> En el [Capítulo 12][ch12]<!-- ignore -->, demostraremos esta práctica
> organizativa con un programa de línea de comandos que contendrá tanto un
> paquete binario como una biblioteca.

### Comenzando paths relativos con `super`

Podemos construir paths relativos que comienzan en el módulo padre, en lugar del
módulo actual o la raíz de la caja, usando `super` al comienzo del path. Esto es
como comenzar un path del sistema de archivos con la sintaxis `..`. Usar `super`
nos permite hacer referencia a un item que sabemos que está en el módulo padre,
lo que puede facilitar la reorganización del árbol de módulos cuando el módulo
está estrechamente relacionado con el padre, pero el padre podría moverse a
otro lugar en el árbol de módulos algún día.

Considere el código en el Listing 7-8 que modela la situación en la que un chef
arregla un pedido incorrecto y lo trae personalmente al cliente. La función
`fix_incorrect_order` definida en el módulo `back_of_house` llama a la función
`deliver_order` definida en el módulo padre especificando el path a
`deliver_order` comenzando con `super`:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-08/src/lib.rs}}
```

<span class="caption">Listing 7-8: Llamar a una función usando un path relativo
que comienza con `super`</span>

La función `fix_incorrect_order` está en el módulo `back_of_house`, por lo que
podemos usar `super` para ir al módulo padre de `back_of_house`, que en este
caso es `crate`, la raíz. Desde allí, buscamos `deliver_order` y lo encontramos.
¡Éxito! Pensamos que el módulo `back_of_house` y la función `deliver_order`
probablemente permanecerán en la misma relación entre sí y se moverán juntos si
decidimos reorganizar el árbol de módulos de la caja. Por lo tanto, usamos
`super` para tener menos lugares para actualizar el código en el futuro si este
código se mueve a un módulo diferente.

### Haciendo públicos los structs y enums

También podemos usar `pub` para designar structs y enums como públicos, pero hay
algunos detalles adicionales para el uso de `pub` con structs y enums. Si
usamos `pub` antes de una definición de struct, hacemos que el struct sea
público, pero los campos del struct seguirán siendo privados. Podemos hacer que
cada campo sea público o no caso por caso. En el Listing 7-9, hemos definido un
struct `back_of_house::Breakfast` público con un campo `toast` público pero un
campo `seasonal_fruit` privado. Esto modela el caso en un restaurante donde el
cliente puede elegir el tipo de pan que viene con una comida, pero el chef
decide qué fruta acompaña la comida según lo que está en temporada y en stock.
La fruta disponible cambia rápidamente, por lo que los clientes no pueden
elegir la fruta o incluso ver qué fruta obtendrán.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-09/src/lib.rs}}
```

<span class="caption">Listing 7-9: Un struct con algunos campos públicos y 
algunos campos privados</span>

Debido a que el campo `toast` es público, podemos cambiar el valor de `toast`
en una instancia de `Breakfast` en la función `eat_at_restaurant` en el Listing
7-10. Ten en cuenta que no podemos usar el campo `seasonal_fruit` en
`eat_at_restaurant` porque `seasonal_fruit` es privado. ¡Intenta descomentar la
línea que modifica el valor del campo `seasonal_fruit` para ver qué error
obtiene!

Además, ten en cuenta que debido a que `back_of_house::Breakfast` tiene un
campo privado, el struct debe proporcionar una función asociada pública que
construya una instancia de `Breakfast` (lo hemos llamado `summer` aquí). Si
`Breakfast` no tuviera tal función, no podríamos crear una instancia de
`Breakfast` en `eat_at_restaurant` porque no podríamos establecer el valor del
campo privado `seasonal_fruit` en `eat_at_restaurant`.

Por el contrario, si hacemos un enum público, todos sus variantes son públicas.
Solo necesitamos el `pub` antes de la palabra clave `enum`, como se muestra en
el Listing 7-10.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-10/src/lib.rs}}
```

<span class="caption">Listing 7-10: Designar un enum como público hace que todas
sus variantes sean públicas</span>

Debido a que hicimos el enum `Appetizer` público, podemos usar las variantes
`Appetizer::Soup` y `Appetizer::Salad` en `eat_at_restaurant`.

Los Enums no son muy útiles a menos que sus variantes sean públicas; sería
molesto tener que anotar todas las variantes de enum con `pub` en todos los
casos, por lo que el valor predeterminado para las variantes de enum es ser
público. Los structs a menudo son útiles sin que sus campos sean públicos, por
lo que los campos de struct siguen la regla general de que todo es privado por
defecto a menos que se anote con `pub`.

Hay una situación más relacionada con `pub` que no hemos cubierto, y es
nuestra última característica del sistema de módulos: la palabra clave `use`.
Cubriremos `use` por sí solo primero, y luego mostraremos cómo combinar `pub` y
`use`.

[pub]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword
[api-guidelines]: https://rust-lang.github.io/api-guidelines/
[ch12]: ch12-00-an-io-project.html
