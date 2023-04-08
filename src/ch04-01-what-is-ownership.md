## ¿Qué es el Ownership?

El *ownership* es un conjunto de reglas que gobiernan cómo un programa de Rust
administra la memoria. Todos los programas tienen que administrar la forma en
que usan la memoria de un computador mientras se ejecutan. Algunos lenguajes
tienen recolección de basura que busca regularmente la memoria que ya no se
usa mientras el programa se ejecuta; en otros lenguajes, el programador debe
asignar y liberar la memoria explícitamente. Rust usa un tercer enfoque: la
memoria se administra a través de un sistema de ownership con un conjunto de
reglas que el compilador verifica. Si alguna de las reglas se viola, el
programa no se compilará. Ninguna de las características del ownership
ralentizará su programa mientras se ejecuta.

Porque el ownership es un concepto nuevo para muchos programadores, toma un
tiempo acostumbrarse. La buena noticia es que a medida que se vuelva más
experimentado con Rust y las reglas del sistema de ownership, más fácil le
resultará desarrollar naturalmente código que sea seguro y eficiente. ¡Sigue
intentándolo!

Cuando entienda el ownership, tendrá una base sólida para comprender las
características que hacen que Rust sea único. En este capítulo, aprenderá
ownership trabajando en algunos ejemplos que se centran en una estructura de
datos muy común: las cadenas de caracteres.

> Nota:
> La traducción de Ownership seria "Propiedad", la mayor parte de la comunidad
> habla de este sistema como Ownsership pero también es valido este termino.
> El motivo es que el sistema de ownership es solo una analogía.
>
> La analogía es que el ownership es como la propiedad de un objeto, por ejemplo
> si tienes un libro, el libro es tuyo. Si lo prestas a alguien, el libro sigue
> siendo tuyo, pero ahora el libro esta en posesión de otra persona. Cuando
> devuelves el libro, el libro regresa a tu posesión. 

> ### El Stack y el Heap
>
> Muchos lenguajes de programación no requieren que piense mucho en el stack y
> el heap. Pero en un lenguaje de programación de sistemas como Rust, si un
> valor está en el stack o en el heap afecta cómo el lenguaje se comporta y por
> qué debe tomar ciertas decisiones. Partes del ownership se describirán en
> relación con el stack y el heap más adelante en este capítulo, por lo que
> aquí hay una breve explicación en preparación.
>
> Tanto el stack como el heap son partes de la memoria disponible para su código
> para usar en tiempo de ejecución, pero están estructurados de formas
> diferentes. El stack almacena valores en el orden en que los recibe y elimina
> los valores en el orden opuesto. Esto se conoce como *último en, primero
> fuera*. Piense en una pila de platos: cuando agrega más platos, los coloca en
> la parte superior de la pila, y cuando necesita un plato, toma uno de la
> parte superior. Agregar o eliminar platos del medio o de la parte inferior no
> funcionaría tan bien! Agregar datos se llama *empujar en el stack*, y
> eliminar datos se llama *sacar del stack*. Todos los datos almacenados en el
> stack deben tener un tamaño conocido y fijo. Los datos con un tamaño
> desconocido en tiempo de compilación o un tamaño que puede cambiar deben
> almacenarse en el heap en su lugar.
>
> El heap es menos organizado: cuando coloca datos en el heap, solicita una
> cierta cantidad de espacio. El administrador de memoria encuentra un lugar
> vacío en el heap que sea lo suficientemente grande, lo marca como en uso y
> devuelve un *puntero*, que es la dirección de esa ubicación. Este proceso se
> llama *asignar en el heap* y a veces se abrevia como solo *asignar* (empujar
> valores en el stack no se considera asignar). Debido a que el puntero al heap
> es un tamaño conocido y fijo, puede almacenar el puntero en el stack, pero
> cuando desea los datos reales, debe seguir el puntero. Piense en estar sentado
> en un restaurante. Cuando ingresa, indica la cantidad de personas en su
> grupo, y el anfitrión encuentra una mesa vacía que quepa a todos y los lleva
> allí. Si alguien en su grupo llega tarde, puede preguntar dónde se ha
> sentado para encontrarlo.
>
> Empujar en el stack es más rápido que asignar en el heap porque el
> administrador de memoria nunca tiene que buscar un lugar para almacenar nuevos
> datos; esa ubicación siempre está en la parte superior de la pila. En
> comparación, asignar espacio en el heap requiere más trabajo porque el
> administrador de memoria debe encontrar primero un espacio lo suficientemente
> grande para contener los datos y luego realizar tareas administrativas para
> prepararse para la siguiente asignación.
>
> Acceder a los datos en el heap es más lento que acceder a los datos en el
> stack porque debe seguir un puntero para llegar allí. Los procesadores
> contemporáneos son más rápidos si saltan menos en la memoria. Continuando con
> la analogía, considere un servidor en un restaurante que toma pedidos de
> muchas mesas. Es más eficiente obtener todos los pedidos de una mesa antes de
> pasar a la siguiente mesa. Tomar un pedido de la mesa A, luego un pedido de la
> mesa B, luego uno de la A nuevamente y luego uno de la B nuevamente sería un
> proceso mucho más lento. Del mismo modo, un procesador puede hacer su trabajo
> mejor si trabaja con datos que están cerca de otros datos (como lo están en
> el stack) en lugar de más lejos (como pueden estar en el heap).
>
> Cuando su código llama a una función, los valores que se pasan a la función
> (incluidos, posiblemente, punteros a datos en el heap) y las variables locales
> de la función se empujan en el stack. Cuando la función termina, esos valores
> se sacan del stack.
>
> Mantener un registro de qué partes del código están utilizando qué datos en
> el heap, minimizar la cantidad de datos duplicados en el heap y limpiar los
> datos no utilizados en el heap para que no se quede sin espacio son todos
> problemas que ownership aborda. Una vez que comprenda ownership, no tendrá
> que pensar mucho en el stack y el heap, pero saber que el principal propósito
> de ownership es administrar datos en el heap puede ayudar a explicar por qué
> funciona de la manera en que lo hace.

### Reglas de Ownership

Primero, echemos un vistazo a las reglas de ownership. Mantenga estas reglas en
mente mientras trabajamos a través de los ejemplos que las ilustran:

* Cada valor en Rust tiene un *propietario*.
* Solo puede haber un propietario a la vez.
* Cuando el propietario sale del alcance, el valor se descartará.

### Ambito de las Variables

Ahora que hemos pasado la sintaxis básica de Rust, no incluiremos todo el código
`fn main() {` en los ejemplos, por lo que si está siguiendo, asegúrese de
colocar los siguientes ejemplos dentro de una función `main` manualmente. Como
resultado, nuestros ejemplos serán un poco más concisos, permitiéndonos
centrarnos en los detalles reales en lugar del código de la caldera.

Como primer ejemplo de ownership, veremos el *ambito* de algunas variables.
Un ámbito es el rango dentro de un programa para el que un elemento es válido.
Toma la siguiente variable:

```rust
let s = "hola";
```

La variable `s` se refiere a un literal de cadena, donde el valor de la cadena
está codificado en el texto de nuestro programa. La variable es válida desde el
punto en que se declara hasta el final del *ambito* actual. El listado 4-1
muestra un programa con comentarios que anotan dónde sería válida la variable
`s`.

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-01/src/main.rs:here}}
```

<span class="caption">Listado 4-1: Una variable y el ámbito en el que es
válida</span>

En otras palabras, hay dos puntos importantes en el tiempo aquí:

* Cuando `s` entra en *ambito*, es válido.
* Permanece válido hasta que sale de *ambito*.

En este punto, la relación entre los ámbitos y cuándo las variables son válidas
es similar a la de otros lenguajes de programación. Ahora construiremos sobre
este entendimiento al introducir el tipo `String`.

### El Tipo `String`

Para ilustrar las reglas de ownership, necesitamos un tipo de datos más complejo
que los que cubrimos en la sección [“Tipos de Datos”][data-types]<!-- ignore -->
del Capítulo 3. Los tipos cubiertos anteriormente son de un tamaño conocido,
pueden almacenarse en el stack y se pueden sacar del stack cuando su ámbito
termina, y se pueden copiar rápidamente y trivialmente para crear una nueva
instancia independiente si otra parte del código necesita usar el mismo valor
en un ámbito diferente. Pero queremos ver los datos que se almacenan en el heap
y explorar cómo Rust sabe cuándo limpiar esos datos, y el tipo `String` es un
gran ejemplo.

Nos centraremos en las partes de `String` que se relacionan con el ownership.
Estos aspectos también se aplican a otros tipos de datos complejos, ya sean
suministrados por la biblioteca estándar o creados por usted. Discutiremos
`String` con más profundidad en el [Capítulo 8][ch8]<!-- ignore -->.

Ya hemos visto literales de cadena, donde un valor de cadena está codificado en
nuestro programa. Los literales de cadena son convenientes, pero no son
adecuados para todas las situaciones en las que podríamos querer usar texto.
Una razón es que son inmutables. Otra es que no todos los valores de cadena se
pueden conocer cuando escribimos nuestro código: ¿y si queremos tomar la
entrada del usuario y almacenarla? Para estas situaciones, Rust tiene un segundo
tipo de cadena, `String`. Este tipo administra datos asignados en el heap y,
como tal, es capaz de almacenar una cantidad de texto que no conocemos en el
tiempo de compilación. Puede crear un `String` a partir de un literal de cadena
usando la función `from`, así:

```rust
let s = String::from("hola");
```

El operador doble dos puntos `::` nos permite usar el namespace (nombre
de espacio) de esta función `from` particular bajo el tipo `String` en lugar
de usar algún tipo de nombre como `string_from`. Discutiremos esta sintaxis
más en la sección [“Sintaxis de Método”][method-syntax]<!-- ignore --> del
Capítulo 5, y cuando hablamos sobre el uso de namespaces con módulos en
[“Rutas para Referir a un Elemento en el Árbol de Módulos”][paths-module-tree]<!-- ignore -->

Este tipo de cadena *puede* ser mutable:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-01-can-mutate-string/src/main.rs:here}}
```

Entonces, ¿cuál es la diferencia aquí? ¿Por qué `String` puede ser mutable pero
los literales no pueden? La diferencia está en cómo estos dos tipos manejan la
memoria.

### Memoria y Asignación

En el caso de un literal de cadena, conocemos los contenidos en tiempo de
compilación, por lo que el texto está codificado directamente en el ejecutable
final. Es por eso que los literales de cadena son rápidos y eficientes. Pero
estas propiedades solo vienen de la inmutabilidad del literal de cadena.
Desafortunadamente, no podemos poner un blob de memoria en el binario para
cada pieza de texto cuyo tamaño es desconocido en tiempo de compilación y cuyo
tamaño puede cambiar mientras se ejecuta el programa.

Con el tipo `String`, para poder soportar una pieza mutable y extensible de
texto, necesitamos asignar una cantidad de memoria en el heap, desconocida en
tiempo de compilación, para contener el contenido. Esto significa:

* La memoria debe solicitarse al administrador de memoria en tiempo de ejecución.
* Necesitamos una forma de devolver esta memoria al administrador cuando
  terminemos con nuestro `String`.

Esa primera parte la hacemos nosotros: cuando llamamos a `String::from`, su
implementación solicita la memoria que necesita. Esto es prácticamente
universal en los lenguajes de programación.

Sin embargo, la segunda parte es diferente. En los lenguajes con un *recolector
de basura (Garbage Collector)*, el recolector de basura rastrea y limpia la
memoria que ya no se está usando y no necesitamos pensar en ello. En la mayoría
de los lenguajes sin un GC, es nuestra responsabilidad identificar cuándo la
memoria ya no se está usando y llamar al código para liberarla explícitamente,
tal como lo hicimos para solicitarla. Hacer esto correctamente ha sido
históricamente un problema difícil de programación. Si lo olvidamos,
desperdiciaremos memoria. Si lo hacemos demasiado pronto, tendremos una variable
inválida. Si lo hacemos dos veces, eso también es un error. Necesitamos
emparejar exactamente una `asignación` con exactamente una `liberación`.

Rust toma un camino diferente: la memoria se devuelve automáticamente una vez
que la variable que la posee sale del alcance. Aquí hay una versión de nuestro
ejemplo de alcance de la Lista 4-1 usando un `String` en lugar de un literal
de cadena:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-02-string-scope/src/main.rs:here}}
```

Hay un punto natural en el que podemos devolver la memoria que necesita nuestro
`String` al administrador: cuando `s` sale del alcance. Cuando una variable
sale del alcance, Rust llama a una función especial para nosotros. Esta
función se llama [`drop`][drop]<!-- ignore -->, y es donde el autor de `String`
puede poner el código para devolver la memoria. Rust llama a `drop`
automáticamente en la llave de cierre.

> Nota: En C++, este patrón de desasignación de recursos al final de la vida
> útil de un elemento a veces se denomina *Resource Acquisition Is
> Initialization (RAII)*. La función `drop` en Rust será familiar para usted si
> ha utilizado patrones RAII.

Este patrón tiene un profundo impacto en la forma en que se escribe el código
Rust. Puede parecer simple ahora, pero el comportamiento del código puede ser
inesperado en situaciones más complejas cuando queremos que varias variables
usen los datos que hemos asignado en el heap. Exploremos algunas de esas
situaciones ahora.

<!-- Old heading. Do not remove or links may break. -->
<a id="ways-variables-and-data-interact-move"></a>

#### Variables y datos interactuando con Move

Varias variables pueden interactuar con los mismos datos de diferentes formas
en Rust. Veamos un ejemplo usando un entero en la Lista 4-2.

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-02/src/main.rs:here}}
```

<span class="caption">Lista 4-2: Asignando el valor entero de la variable `x`
a `y`</span>

Podemos adivinar lo que está haciendo: "vincular el valor `5` a `x`; luego
hacer una copia del valor en `x` y vincularlo a `y`". Ahora tenemos dos
variables, `x` y `y`, y ambos son `5`. Esto es lo que está sucediendo, porque
los enteros son valores simples con un tamaño conocido y fijo, y estos dos
valores `5` se empujan en la pila.

Ahora veamos la versión `String`:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-03-string-move/src/main.rs:here}}
```

Esto se ve muy similar, por lo que podríamos suponer que la forma en que
funciona sería la misma: es decir, la segunda línea haría una copia del valor en
`s1` y lo vincularía a `s2`. Pero esto no es exactamente lo que sucede.

Mire la Figura 4-1 para ver lo que está sucediendo con `String` bajo las
coberturas.
Un `String` está compuesto por tres partes, mostradas a la izquierda:
un puntero a la memoria que contiene el contenido de la cadena, una longitud y
una capacidad. Este grupo de datos se almacena en la pila. A la derecha está la
memoria en el heap que contiene el contenido.

<div style="width:50%; max-width: 100%;">
{{#include img/trpl04-01.svg}}
</div>

<span class="caption">Figura 4-1: Representación en memoria de un `String`
que contiene el valor `"hola"` vinculado a `s1`</span>

La longitud es cuánta memoria, en bytes, los contenidos del `String` están
utilizando actualmente. La capacidad es la cantidad total de memoria, en bytes,
que el `String` ha recibido del administrador. La diferencia entre longitud y
capacidad importa, pero no en este contexto, por lo que por ahora está bien
ignorar la capacidad.

Cuando asignamos `s1` a `s2`, los datos de `String` se copian, lo que significa
que copiamos el puntero, la longitud y la capacidad que están en la pila. No
copiamos los datos en el heap al que hace referencia el puntero. En otras
palabras, la representación de datos en memoria se ve como la Figura 4-2.

<div style="width:50%; max-width: 100%;">
{{#include img/trpl04-02.svg}}
</div>

<span class="caption">Figura 4-2: Representación en memoria de la variable
`s2` que tiene una copia del puntero, la longitud y la capacidad de `s1`.</span>

La representación *no* se ve como la Figura 4-3, que es lo que la memoria
parecería si Rust copiara además los datos del heap. Si Rust hiciera esto, la
operación `s2 = s1` podría ser muy costosa en términos de rendimiento de tiempo
de ejecución si los datos en el heap fueran grandes.

<div style="width:50%; max-width: 100%;">
{{#include img/trpl04-03.svg}}
</div>

<span class="caption">Figura 4-3: Otra posibilidad de lo que `s2 = s1` podría
hacer si Rust copiara también los datos del heap</span>

Anteriormente, dijimos que cuando una variable sale de ámbito, Rust llama
automáticamente a la función `drop` y limpia la memoria del heap para esa
variable. Pero la Figura 4-2 muestra que ambos punteros de datos apuntan al
mismo lugar. Esto es un problema: cuando `s2` y `s1` salen de ámbito, ambos
intentarán liberar la misma memoria. Esto se conoce como un error de *doble
liberación* y es uno de los errores de seguridad de la memoria que mencionamos
anteriormente. Liberar la memoria dos veces puede conducir a la corrupción de
memoria, lo que puede conducir a vulnerabilidades de seguridad.

Para garantizar la seguridad de la memoria, después de la línea `let s2 = s1;`,
Rust considera a `s1` como no válida. Por lo tanto, Rust no necesita liberar
nada cuando `s1` sale de ámbito. Echa un vistazo a lo que sucede cuando intentas
usar `s1` después de que se crea `s2`; no funcionará:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-04-cant-use-after-move/src/main.rs:here}}
```

Obtendrás un error como este porque Rust te impide usar la referencia
invalidada:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-04-cant-use-after-move/output.txt}}
```

Si has escuchado los términos *copia superficial* y *copia profunda* mientras
trabajabas con otros lenguajes, el concepto de copiar el puntero, la longitud y
la capacidad sin copiar los datos probablemente suene a hacer una copia
superficial. Pero debido a que Rust también invalida la primera variable, en
vez de llamarse una copia superficial, se conoce como un *movimiento*. En este
ejemplo, diríamos que `s1` fue *movido* a `s2`. Entonces, lo que realmente
sucede se muestra en la Figura 4-4.

<div style="width:50%; max-width: 100%;">
{{#include img/trpl04-04.svg}}
</div>

<span class="caption">Figura 4-4: Representación en memoria después de que
`s1` se haya invalidado</span>

¡Eso resuelve nuestro problema! Con solo `s2` válido, cuando sale de ámbito
solo él liberará la memoria, y ya está.

Además, hay una elección de diseño que se infiere de esto: Rust nunca
creará automáticamente "copias profundas" de tus datos. Por lo tanto, cualquier
copia *automática* se puede asumir que es económica en términos de rendimiento
en tiempo de ejecución.

<!-- Old heading. Do not remove or links may break. -->
<a id="ways-variables-and-data-interact-clone"></a>

#### Variables y datos interactuando con Clone

Si *queremos* copiar profundamente los datos del heap de la `String`, no solo
los datos de la pila, podemos usar un método común llamado `clone`. Discutiremos
la sintaxis del método en el Capítulo 5, pero debido a que los métodos son una
característica común en muchos lenguajes de programación, probablemente los
hayas visto antes.

Aquí hay un ejemplo del método `clone` en acción:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-05-clone/src/main.rs:here}}
```

Esto funciona bien y produce explícitamente el comportamiento mostrado en la
Figura 4-3, donde los datos del heap *se copian*.

Cuando veas una llamada a `clone`, sabrás que se está ejecutando algún código
arbitrario y que ese código puede ser costoso. Es un indicador visual de que
algo diferente está sucediendo.

#### Solo datos del stack: Copiar

Hay otro problema que aún no hemos hablado. Este código usando enteros - parte
de lo que se mostró en el Listado 4-2 - funciona y es válido:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-06-copy/src/main.rs:here}}
```

Pero este código parece contradecir lo que acabamos de aprender: no tenemos una
llamada a `clone`, pero `x` sigue siendo válido y no se movió a `y`.

La razón es que los tipos como los enteros que tienen un tamaño conocido en el
momento de la compilación se almacenan completamente en la pila, por lo que
copiar los valores reales es rápido. Eso significa que no hay razón para que
queramos evitar que `x` sea válido después de crear la variable `y`. En otras
palabras, no hay diferencia entre copiar superficial y profunda aquí, por lo que
llamar a `clone` no haría nada diferente de la copia superficial habitual, y
podemos dejarlo fuera.

Rust tiene una anotación especial llamada `Copy` que podemos colocar en tipos
que se almacenan en la pila, como los enteros (hablaremos más sobre los
*traits* en el [Capítulo 10][traits]<!-- ignore -->). Si un tipo implementa el
`Copy` *trait*, las variables que lo usan no se mueven, sino que se copian
trivialmente, haciendo que sigan siendo válidas después de asignarlas a otra
variable.

Rust no nos permitirá anotar un tipo con `Copy` si el tipo, o cualquiera de sus
partes, ha implementado el *trait* `Drop`. Si el tipo necesita que algo
especial suceda cuando el valor sale del alcance y agregamos la anotación `Copy`
a ese tipo, obtendremos un error de tiempo de compilación. Para aprender cómo
agregar la anotación `Copy` a tu tipo para implementar el *trait*, consulta
[“Traits derivables”][derivable-traits]<!-- ignore --> en el Apéndice C.

Entonces, ¿qué tipos implementan el *trait* `Copy`? Puedes consultar la
documentación del tipo dado para asegurarte, pero como regla general, cualquier
grupo de valores escalares simples puede implementar `Copy`, y nada que
requiera asignación o sea alguna forma de recurso puede implementar `Copy`.
Aquí hay algunos de los tipos que implementan `Copy`:

* Todos los tipos enteros, como `u32`.
* El tipo booleano, `bool`, con valores `true` y `false`.
* Todos los tipos de punto flotante, como `f64`.
* El tipo de carácter, `char`.
* Tuplas, si solo contienen tipos que también implementan `Copy`. Por ejemplo,
  `(i32, i32)` implementa `Copy`, pero `(i32, String)` no lo hace.

### Propiedad y funciones

Las mecánicas de pasar un valor a una función son similares a las de asignar un
valor a una variable. Pasar una variable a una función moverá o copiará, como
hace la asignación. La Lista 4-3 tiene un ejemplo con algunas anotaciones que
muestran dónde entran y salen las variables del alcance.

<span class="filename">Nombre del archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-03/src/main.rs}}
```

<span class="caption">Lista 4-3: Funciones con propiedad y alcance
anotados</span>

Si intentamos usar `s` después de llamar a `tomar_ownership`, Rust lanzaría un
error de tiempo de compilación. Estas comprobaciones estáticas nos protegen de
errores. Intenta agregar código a `main` que use `s` y `x` para ver dónde puedes
usarlos y dónde las reglas de propiedad te impiden hacerlo.

### Valores de retorno y alcance

Los valores de retorno también pueden transferir la propiedad. La Lista 4-4
muestra un ejemplo de una función que devuelve algún valor, con anotaciones
similares a las de la Lista 4-3.

<span class="filename">Nombre del archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-04/src/main.rs}}
```

<span class="caption">Lista 4-4: Transferencia de propiedad de los valores
de retorno</span>

La propiedad (ownership) de una variable sigue el mismo patrón cada vez: 
asignar un valor a otra variable lo mueve. Cuando una variable que incluye datos
en el heap sale del alcance, el valor se limpiará por `drop` a menos que la
propiedad de los datos se haya movido a otra variable.

Aunque esto funciona, tomar la propiedad y luego devolver la propiedad con cada
función es un poco tedioso. ¿Qué pasa si queremos que una función use un valor
pero no tome la propiedad? Es bastante molesto que todo lo que pasamos también
necesite volver a pasar si queremos usarlo de nuevo, además de cualquier dato
que resulte del cuerpo de la función que también podríamos querer devolver.

Rust nos permite devolver múltiples valores usando una tupla, como se muestra
en la Lista 4-5.

<span class="filename">Nombre del archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-05/src/main.rs}}
```

<span class="caption">Lista 4-5: Devolución de la propiedad de los
parámetros</span>

Pero esto es demasiado ceremonioso y mucho trabajo para un concepto que debería
ser común. Afortunadamente para nosotros, Rust tiene una característica para
usar un valor sin transferir la propiedad, llamada *referencias*.

[data-types]: ch03-02-data-types.html#tipos-de-datos
[ch8]: ch08-02-strings.html
[traits]: ch10-02-traits.html
[derivable-traits]: appendix-03-derivable-traits.html
[method-syntax]: ch05-03-method-syntax.html#method-syntax
[paths-module-tree]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
[drop]: https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop
