<!-- Old heading. Do not remove or links may break. -->


<a id="closures-anonymous-functions-that-can-capture-their-environment"></a>

## Closures: Funciones anónimas que capturan su entorno

Los closures de Rust son funciones anónimas que puede guardar en una variable o
pasar como argumentos a otras funciones. Puede crear el closure en un lugar y
luego llamar al closure en otro lugar para evaluarlo en un contexto diferente.
A diferencia de las funciones, los closures pueden capturar valores del scope en
el que se definen. Demostraremos cómo estas características de los closures
permiten la reutilización de código y la personalización del comportamiento.

<!-- Old headings. Do not remove or links may break. -->


<a id="creating-an-abstraction-of-behavior-with-closures"></a>
<a id="refactoring-using-functions"></a>
<a id="refactoring-with-closures-to-store-code"></a>
<a id="capturing-the-environment-with-closures"></a>

### Capturando el Entorno con Closures

Primero examinaremos cómo podemos usar closures para capturar valores del
entorno en el que están definidos para su uso posterior. Aquí está el escenario:
Cada cierto tiempo, nuestra compañía de camisetas regala una camiseta exclusiva
y de edición limitada a alguien en nuestra lista de correo como promoción. Las
personas en la lista de correo pueden agregar opcionalmente su color favorito a
su perfil. Si la persona elegida para una camiseta gratis tiene su color
favorito establecido, obtienen esa camiseta de color. Si la persona no ha
especificado un color favorito, obtienen el color que la compañía tiene
actualmente en mayor cantidad.

Hay muchas formas de implementar esto. Para este ejemplo, vamos a usar un enum
llamado `ShirtColor` que tiene las variantes `Red` y `Blue` (limitando el
número de colores disponibles para simplificar). Representamos el inventario de
la compañía con un struct `Inventory` que tiene un campo llamado `shirts` que
contiene un `Vec<ShirtColor>` que representa los colores de camisetas
actualmente en stock. El método `giveaway` definido en `Inventory` obtiene la
preferencia opcional de color de camiseta del ganador de la camiseta gratis, y
devuelve el color de camiseta que la persona obtendrá. Esta configuración se
muestra en el Listado 13-1:

<Listing number="13-1" file-name="src/main.rs" caption="Situación de giveaway">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-01/src/main.rs}}
```

</Listing>

La `store` definida en `main` tiene dos camisetas azules y una camiseta roja
restante para distribuir para esta promoción de edición limitada. Llamamos al
método `giveaway` para un usuario con preferencia por una camiseta roja y un
usuario sin ninguna preferencia.

Otra vez, este código podría implementarse de muchas maneras, y aquí, para
centrarnos en los closures, nos hemos adherido a los conceptos que ya has
aprendido, excepto por el cuerpo del método `giveaway` que usa un closure. En el
método `giveaway`, obtenemos la preferencia del usuario como un parámetro de
tipo `Option<ShirtColor>` y llamamos al método `unwrap_or_else` en
`user_preference`. El método [`unwrap_or_else` en `Option<T>`][unwrap-or-else]

<!-- ignore --> está definido por la biblioteca estándar. Toma un argumento: un

Closure sin ningún argumento que devuelve un valor `T` (el mismo tipo almacenado
en la variante `Some` de la `Option<T>`, en este caso `ShirtColor`). Si la
`Option<T>` es la variante `Some`, `unwrap_or_else` devuelve el valor de dentro
de `Some`. Si la `Option<T>` es la variante `None`, `unwrap_or_else` llama al
closure y devuelve el valor devuelto por el closure.

Especificamos el closure `|| self.most_stocked()` como argumento a
`unwrap_or_else`. Este es un closure que no toma parámetros en sí mismo (si el
closure tuviera parámetros, aparecerían entre las dos barras verticales). El
cuerpo del closure llama a `self.most_stocked()`. Estamos definiendo el closure
aquí, y la implementación de `unwrap_or_else` evaluará el closure más tarde si
se necesita el resultado.

Ejecutar este código imprime:

```console
{{#include ../listings/ch13-functional-features/listing-13-01/output.txt}}
```

Un aspecto interesante aquí es que hemos pasado un closure que llama a
`self.most_stocked()` en la instancia `Inventory` actual. La biblioteca estándar
no necesitaba saber nada sobre los tipos `Inventory` o `ShirtColor` que
definimos, o la lógica que queremos usar en este escenario. El closure captura
una referencia inmutable a la instancia `self` `Inventory` y la pasa con el
código que especificamos al método `unwrap_or_else`. Las funciones, por otro
lado, no pueden capturar su entorno de esta manera.

<a id="closure-type-inference-and-annotation"></a>

### Inferencia de tipo de Closure y anotación

Existen más diferencias entre funciones y closures. Los closures no suelen
requerir que anotes los tipos de los parámetros o el valor de retorno como lo
hacen las funciones `fn`. Las anotaciones de tipo son necesarias en las
funciones porque los tipos son parte de una interfaz explícita expuesta a tus
usuarios. Definir esta interfaz rígidamente es importante para garantizar que
todos estén de acuerdo en qué tipos de valores usa y devuelve una función. Los
closures, por otro lado, no se usan en una interfaz expuesta como esta: se
almacenan en variables y se usan sin nombrarlos y exponerlos a los usuarios de
nuestra biblioteca.

Los closures típicamente son cortos y relevantes solo dentro de un contexto
estrecho en lugar de en cualquier escenario arbitrario. Dentro de estos
contextos limitados, el compilador puede inferir los tipos de los parámetros y
el tipo de retorno, similar a cómo puede inferir los tipos de la mayoría de las
variables (hay casos raros en los que el compilador también necesita
anotaciones de tipo de closure).

Como con las variables, podemos agregar anotaciones de tipo opcionales si
queremos aumentar la explicitud y la claridad a costa de ser más verbosos de lo
estrictamente necesario. La anotación de tipos para un closure se vería como la
definición que se muestra en el Listado 13-2. En este ejemplo, estamos
definiendo un closure y almacenándolo en una variable en lugar de definir el
closure en el lugar donde lo pasamos como argumento como lo hicimos en el
Listado 13-1.

<Listing number="13-2" file-name="src/main.rs" caption="Agregando anotaciones de tipo opcionales para los tipos de parámetros y valor de retorno en el closure">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-02/src/main.rs:here}}
```

</Listing>

Con la anotación de tipo agregada, la sintaxis de los closures se parece más a
la sintaxis de las funciones. Aquí definimos una función que agrega 1 a su
parámetro y un closure que tiene el mismo comportamiento, para comparación.
Hemos agregado algunos espacios para alinear las partes relevantes. Esto
ilustra cómo la sintaxis de los closures es similar a la sintaxis de las
funciones, excepto por el uso de tuberías y la cantidad de sintaxis que es
opcional:

```rust,ignore
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

La primera línea muestra una definición de función, y la segunda línea muestra
una definición de closure completamente anotada. En la tercera línea, quitamos
las anotaciones de tipo de la definición de closure. En la cuarta línea,
quitamos los corchetes, que son opcionales porque el cuerpo del closure tiene
solo una expresión. Estas son todas definiciones válidas que producirán el mismo
comportamiento cuando se llamen. Las líneas `add_one_v3` y `add_one_v4`
requieren que los closures se evalúen para poder compilar porque los tipos se
inferirán a partir de su uso. Esto es similar a `let v = Vec::new();` que
necesita anotaciones de tipo o valores de algún tipo para insertar en el `Vec`
para que Rust pueda inferir el tipo.

Para las definiciones de closure, el compilador infiere un tipo concreto para
cada uno de sus parámetros y para su valor de retorno. Por ejemplo, el Listado
13-3 muestra la definición de un closure corto que solo devuelve el valor que
recibe como parámetro. Este closure no es muy útil, excepto para los propósitos
de este ejemplo. Tenga en cuenta que no hemos agregado ninguna anotación de
tipo a la definición. Debido a que no hay anotaciones de tipo, podemos llamar al
closure con cualquier tipo, lo que hemos hecho aquí con `String` la primera
vez. Si luego intentamos llamar a `example_closure` con un entero, obtendremos
un error.

<Listing number="13-3" file-name="src/main.rs" caption="Intentando llamar a un closure cuyos tipos se infieren con dos tipos diferentes">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-03/src/main.rs:here}}
```

</Listing>

El compilador nos da este error:

```console
{{#include ../listings/ch13-functional-features/listing-13-03/output.txt}}
```

La primera vez que llamamos a `example_closure` con el valor `String`, el
compilador infiere el tipo de `x` y el tipo de retorno del closure como
`String`. Esos tipos se bloquean en el closure en `example_closure`, y
obtenemos un error de tipo cuando intentamos usar un tipo diferente con el
mismo closure.

### Capturando referencias o moviendo el ownership

Los closures pueden capturar valores desde su entorno de tres maneras, que se 
mapean directamente a las tres formas en que una función puede tomar un 
parámetro: borrowing inmutable, borrowing mutable y tomando ownership. El 
closure decidirá cuál de estos usar en función de lo que haga el cuerpo de la 
función con los valores capturados.

En el Listado 13-4, definimos un closure que captura una referencia inmutable al
vector `list` ya que solo necesita una referencia inmutable para imprimir el
valor:

<Listing number="13-4" file-name="src/main.rs" caption="Definiendo y llamando a un closure que captura una referencia inmutable">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-04/src/main.rs}}
```

</Listing>

Este ejemplo también ilustra que una variable puede vincularse a una definición
de closure, y luego podemos llamar al closure usando el nombre de la variable y
paréntesis como si el nombre de la variable fuera un nombre de función.

Debido a que podemos tener múltiples referencias inmutables a `list` al mismo
tiempo, `list` sigue siendo accesible desde el código antes de la definición del
closure, después de la definición del closure, pero antes de que se llame al
closure, y después de que se llame al closure. Este código se compila, se
ejecuta e imprime:

```console
{{#include ../listings/ch13-functional-features/listing-13-04/output.txt}}
```

Luego, en el Listado 13-5, cambiamos el cuerpo del closure para que agregue un
elemento al vector `list`. El closure ahora captura una referencia mutable:

<Listing number="13-5" file-name="src/main.rs" caption="Definiendo y llamando a un closure que captura una referencia mutable">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-05/src/main.rs}}
```

</Listing>

Este código compila, se ejecuta e imprime:

```console
{{#include ../listings/ch13-functional-features/listing-13-05/output.txt}}
```

Nota que ya no hay un `println!` entre la definición y la llamada del closure
`borrows_mutably`: cuando se define `borrows_mutably`, captura una referencia
mutable a `list`. No usamos el closure nuevamente después de llamar al closure,
por lo que el préstamo mutable termina. Entre la definición del closure y la
llamada del closure, no se permite un préstamo inmutable para imprimir porque
no se permiten otros préstamos cuando hay un préstamo mutable. ¡Intente agregar
un `println!` allí para ver qué mensaje de error obtiene!

Si deseas forzar al closure para que tome ownership de los valores que usa en el
entorno, incluso cuando el cuerpo del closure no los necesite, puedes usar la
palabra clave `move` antes de la lista de parámetros.

Esta técnica es principalmente útil cuando se pasa un closure a un nuevo hilo
para mover los datos para que sean propiedad del nuevo hilo. Discutiremos los
hilos y por qué querrías usarlos en detalle en el Capítulo 16 cuando hablemos
sobre la concurrencia, pero por ahora, exploremos brevemente cómo generar un
nuevo hilo usando un closure que necesita la palabra clave `move`. El Listado
13-6 muestra el Listado 13-4 modificado para imprimir el vector en un nuevo
hilo en lugar de en el hilo principal:

<Listing number="13-6" file-name="src/main.rs" caption="Usando `move` para forzar que el closure del thread tome el ownership de `list`">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-06/src/main.rs}}
```

</Listing>

Iniciamos un nuevo hilo, dando al hilo un closure para ejecutar como argumento.
El cuerpo del closure imprime la lista. En el Listado 13-4, el closure solo
capturó `list` usando una referencia inmutable porque esa es la menor cantidad
de acceso a `list` necesaria para imprimirla. En este ejemplo, aunque el cuerpo
del closure todavía solo necesita una referencia inmutable, debemos especificar
que `list` debe moverse al closure poniendo la palabra clave `move` al comienzo
de la definición del closure. El nuevo hilo podría terminar antes de que el
resto del hilo principal termine, o el hilo principal podría terminar primero.
Si el hilo principal mantuviera la propiedad de `list` pero terminara antes de
que lo hiciera el nuevo hilo y dejara caer `list`, la referencia inmutable en
el hilo sería inválida. Por lo tanto, el compilador requiere que `list` se
mueva al closure dado al nuevo hilo para que la referencia sea válida. ¡Intente
eliminar la palabra clave `move` o usar `list` en el hilo principal después de
que se defina el closure para ver qué errores del compilador obtiene!

<!-- Old headings. Do not remove or links may break. -->


<a id="storing-closures-using-generic-parameters-and-the-fn-traits"></a>
<a id="limitations-of-the-cacher-implementation"></a>
<a id="moving-captured-values-out-of-the-closure-and-the-fn-traits"></a>

### Moviendo valores capturados fuera de los closures y los traits `Fn`

Una vez que un closure ha capturado una referencia o capturado el ownership de
un valor del entorno donde se define el closure (afectando así lo que, si
cualquier cosa, se mueve _dentro_ del closure), el código en el cuerpo del
closure define lo que sucede con las referencias o valores cuando el closure se
evalúa más tarde (afectando así lo que, si cualquier cosa, se mueve _fuera_ del
closure). El cuerpo de un closure puede hacer cualquiera de las siguientes
acciones: mover un valor capturado fuera del closure, mutar el valor capturado,
ni mover ni mutar el valor, o no capturar nada del entorno para comenzar.

La forma en que un closure captura y maneja los valores del entorno afecta qué
traits implementa el closure, y los traits son cómo las funciones y los
structs pueden especificar qué tipos de closures pueden usar. Los closures
implementarán automáticamente uno, dos o los tres de estos traits `Fn`, de
manera aditiva, dependiendo de cómo el cuerpo del closure maneje los valores:

1. `FnOnce` se aplica los closures que pueden ser llamados una vez. Todos los
   closures implementan al menos este trait, porque todos los closures pueden
   ser llamados. Un closure que mueve valores capturados fuera de su cuerpo
   solo implementará `FnOnce` y ninguno de los otros traits `Fn`, porque solo
   puede ser llamado una vez.
2. `FnMut` se aplica a los closures que no mueven valores capturados fuera de
   su cuerpo, pero que podrían mutar los valores capturados. Estos closures
   pueden ser llamados más de una vez.
3. `Fn` se aplica a los closures que no mueven valores capturados fuera de su
   cuerpo y que no mutan los valores capturados, así como los closures que no
   capturan nada de su entorno. Estos closures pueden ser llamados más de una
   vez sin mutar su entorno, lo cual es importante en casos como llamar a un
   closure múltiples veces concurrentemente.

Veamos la definición del método `unwrap_or_else` en `Option<T>` que utilizamos
en el Listado 13-1:

```rust,ignore
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

Recuerda que `T` es el tipo generic que representa el tipo del valor en la
variante `Some` de un `Option`. Ese tipo `T` también es el tipo de retorno de
la función `unwrap_or_else`: el código que llama a `unwrap_or_else` en un
`Option<String>`, por ejemplo, obtendrá un `String`.

Luego, observe que el método `unwrap_or_else` tiene el parámetro de tipo
generic adicional `F`. El tipo `F` es el tipo del parámetro llamado `f`, que
es el closure que proporcionamos al llamar a `unwrap_or_else`.

El trait bound especificado en el tipo generic `F` es `FnOnce() -> T`, lo que
significa que `F` debe poder ser llamado una vez para producir un valor del
tipo `T`. Usar `FnOnce` en el trait bound expresa la restricción de que
`unwrap_or_else` solo va a llamar a `f` como máximo una vez. En el cuerpo de
`unwrap_or_else`, podemos ver que si el `Option` es `Some`, `f` no se llamará.
Si el `Option` es `None`, `f` se llamará una vez. Debido a que todos los
closures implementan `FnOnce`, `unwrap_or_else` acepta todos esos tipos de 
closures y es tan flexible como puede ser.

> Nota: Las funciones también pueden implementar los tres traits `Fn`, `FnMut`
> y `FnOnce`. Si lo que queremos hacer no requiere capturar un valor del
> entorno, podemos usar el nombre de una función en lugar de un closure donde
> necesitamos algo que implemente uno de los traits `Fn`. Por ejemplo, en un
> valor `Option<Vec<T>>`, podríamos llamar a `unwrap_or_else(Vec::new)` para
> obtener un nuevo vector vacío si el valor es `None`. El compilador implementa 
> automáticamente el trait Fn (o el que sea aplicable) para las definiciones de 
> funciones.

Ahora veamos el método de la biblioteca estándar `sort_by_key` definido en
slices, para ver cómo difiere de `unwrap_or_else` y por qué `sort_by_key`
utiliza `FnMut` en lugar de `FnOnce` para el trait bound. El closure recibe un
argumento en forma de referencia al elemento actual en el slice que se está
considerando, y devuelve un valor de tipo `K` que se puede ordenar. Esta
función es útil cuando desea ordenar un slice por un atributo particular de
cada elemento. En el Listado 13-7, tenemos una lista de instancias de
`Rectangle` y usamos `sort_by_key` para ordenarlas por su atributo `width`
de menor a mayor:

<Listing number="13-7" file-name="src/main.rs" caption="Usando `sort_by_key` para ordenar rectángulos por ancho">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-07/src/main.rs}}
```

</Listing>

Este código imprime:

```console
{{#include ../listings/ch13-functional-features/listing-13-07/output.txt}}
```

La razón por la que `sort_by_key` está definido para tomar un closure `FnMut`
es que llama al closure varias veces: una vez por cada elemento en el slice.
El closure `|r| r.width` no captura, muta ni mueve nada de su entorno, por lo
que cumple con los requisitos de los trait bound.

En contraste, El Listado 13-8 muestra un ejemplo de un closure que implementa
solo el trait `FnOnce`, porque mueve un valor fuera del entorno. El
compilador no nos permitirá usar este closure con `sort_by_key`:

<Listing number="13-8" file-name="src/main.rs" caption="Intentando usar un closure `FnOnce` con `sort_by_key`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-08/src/main.rs}}
```

</Listing>

Esto es un ejemplo artificial y complicado (que no funciona) para tratar de
contar la cantidad de veces que se llama a `sort_by_key` llama a la closure al 
ordenar `list`.
Este código intenta hacer este conteo empujando `value`—un `String` del
entorno del closure—en el vector `sort_operations`. El closure captura `value`
y luego mueve `value` fuera del closure transfiriendo la propiedad de `value`
al vector `sort_operations`. Este closure puede ser llamado una vez; tratar de
llamarlo una segunda vez no funcionaría porque `value` ya no estaría en el
entorno para ser empujado a `sort_operations` nuevamente. Por lo tanto, este
closure solo implementa `FnOnce`. Cuando intentamos compilar este código,
obtenemos este error de que `value` no se puede mover fuera del closure porque
el closure debe implementar `FnMut`:

```console
{{#include ../listings/ch13-functional-features/listing-13-08/output.txt}}
```

El error señala la línea en el cuerpo del closure que mueve `value` fuera del
entorno. Para solucionar esto, debemos cambiar el cuerpo del closure para que
no mueva valores fuera del entorno. Para contar la cantidad de veces que se
llama a la closure, mantener un contador en el entorno e incrementar su
valor en el cuerpo del closure es una forma más directa de calcular eso. El
closure en el Listado 13-9 funciona con `sort_by_key` porque solo está
capturando una referencia mutable al contador `num_sort_operations` y, por lo
tanto, puede ser llamado más de una vez:

<Listing number="13-9" file-name="src/main.rs" caption="Usando un closure `FnMut` con `sort_by_key` está permitido">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-09/src/main.rs}}
```

</Listing>

Los `Fn` traits son importantes al definir o usar funciones o tipos que
hacen uso de closures. En la siguiente sección, discutiremos los iteradores.
Muchos métodos de iteradores toman argumentos de closure, ¡así que tenga en
cuenta estos detalles de closure a medida que continuamos!

[unwrap-or-else]: https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_else
