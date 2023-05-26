## Características de lenguajes orientados a objetos

No hay consenso en la comunidad de programación sobre qué características debe
tener un lenguaje para ser considerado orientado a objetos. Rust está
influenciado por muchos paradigmas de programación, incluido OOP; por ejemplo,
exploramos las características que provienen de la programación funcional en el
Capítulo 13. Es discutible que los lenguajes OOP compartan ciertas
características comunes, a saber, objetos, encapsulación y herencia. Veamos qué
significa cada una de esas características y si Rust la admite.

### Los objetos contienen datos y comportamiento

El libro *Design Patterns: Elements of Reusable Object-Oriented Software* de
Erich Gamma, Richard Helm, Ralph Johnson y John Vlissides (Addison-Wesley
Professional, 1994), coloquialmente conocido como el libro *Gang of Four*, es un
catálogo de patrones de diseño orientados a objetos. Define OOP de esta manera:

> Los programas orientados a objetos están compuestos por objetos. Un *objeto*
> empaqueta tanto datos como los procedimientos que operan en esos datos. Los
> procedimientos se denominan típicamente *métodos* u *operaciones*.

Usando esta definición, Rust es orientado a objetos: los structs y los
enums tienen datos, y los bloques `impl` proporcionan métodos en structs y 
enums. Aunque los structs y los enums con métodos no se llaman objetos, 
proporcionan la misma funcionalidad, según la definición de objetos del 
Gang of Four’s.

### Encapsulación que oculta los detalles de implementación

Otro aspecto comúnmente asociado con OOP es la idea de *encapsulación*, que
significa que los detalles de implementación de un objeto no son accesibles al
código que usa ese objeto. Por lo tanto, la única forma de interactuar con un
objeto es a través de su API pública; el código que usa el objeto no debería
poder acceder a los detalles internos del objeto y cambiar los datos o el
comportamiento directamente. Esto permite al programador cambiar y refactorizar
los detalles internos de un objeto sin necesidad de cambiar el código que usa
el objeto.

Hemos discutido cómo controlar la encapsulación en el Capítulo 7: podemos usar
la palabra clave `pub` para decidir qué módulos, tipos, funciones y métodos en
nuestro código deben ser públicos, y por defecto todo lo demás es privado. Por
ejemplo, podemos definir un struct `AveragedCollection` que tiene un campo que
contiene un vector de valores `i32`. El struct también puede tener un campo que
contiene el promedio de los valores en el vector, lo que significa que el
promedio no tiene que calcularse a pedido cada vez que alguien lo necesite. En
otras palabras, `AveragedCollection` almacenará en caché el promedio calculado
para nosotros. El Listado 17-1 tiene la definición del struct 
`AveragedCollection`:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-01/src/lib.rs}}
```

<span class="caption">Listing 17-1: Un struct `AveragedCollection` que
mantiene una lista de enteros y el promedio de los elementos en la colección
</span>

El struct está marcado como `pub` para que otro código pueda usarlo, pero los
campos dentro del struct permanecen privados. Esto es importante en este caso
porque queremos asegurarnos de que cada vez que se agrega o elimina un valor de
la lista, el promedio también se actualiza. Hacemos esto implementando los
métodos públicos `add`, `remove` y `average` en el struct, como se muestra en
el Listado 17-2:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-02/src/lib.rs:here}}
```

<span class="caption">Listing 17-2: Implementaciones de los métodos públicos
`add`, `remove`, y `average` en `AveragedCollection`</span>

Los métodos públicos `add`, `remove`, y `average` son las únicas formas de
acceder o modificar los datos en una instancia de `AveragedCollection`. Cuando
se agrega un elemento a `list` usando el método `add` o se elimina usando el
método `remove`, las implementaciones de cada uno llaman al método privado
`update_average` que maneja la actualización del campo `average` también.

Dejamos los campos `list` y `average` privados para que no haya forma de que el
código externo agregue o elimine elementos de `list` directamente; de lo
contrario, el campo `average` podría quedar fuera de sincronización cuando
`list` cambia. El método `average` devuelve el valor en el campo `average`,
permitiendo que el código externo lea el `average` pero no lo modifique.

Debido a que hemos encapsulado la implementación de `AveragedCollection`, podemos
cambiar fácilmente los aspectos, como la estructura de datos, en el futuro. Por
ejemplo, podríamos usar un `HashSet<i32>` en lugar de un `Vec<i32>` para el
campo `list`. Mientras las firmas de los métodos públicos `add`, `remove`, y
`average` permanezcan iguales, el código que usa `AveragedCollection` no
necesitaría cambiar. Si hicimos `list` pública en su lugar, esto no sería
necesariamente cierto: `HashSet<i32>` y `Vec<i32>` tienen diferentes métodos
para agregar y eliminar elementos, por lo que el código externo probablemente
tendría que cambiar si estuviera modificando `list` directamente.

Si la encapsulación es un aspecto requerido para que un lenguaje se considere
orientado a objetos, entonces Rust cumple con ese requisito. La opción de usar
`pub` o no para diferentes partes del código permite la encapsulación de los
detalles de implementación.

### Herencia como un sistema de tipos y como Code Sharing

*Herencia* es un mecanismo mediante el cual un objeto puede heredar elementos de
la definición de otro objeto, obteniendo así los datos y el comportamiento del
objeto padre sin tener que definirlos nuevamente.

Si se considera que un lenguaje debe tener herencia para ser un lenguaje
orientado a objetos, entonces Rust no cumple con esta definición. No existe
una forma de definir un struct que herede los campos y las implementaciones de
métodos de un struct padre sin usar una macro.

Sin embargo, si estás acostumbrado a tener la herencia en tu caja de
programación, puedes usar otras soluciones en Rust, dependiendo de tu razón
para recurrir a la herencia en primer lugar.

Elegirías la herencia por dos razones principales. Una es reutilizar el código:
puedes implementar un comportamiento particular para un tipo, y la herencia te
permite reutilizar esa implementación para un tipo diferente. Puedes hacer esto
de una manera limitada en el código Rust usando implementaciones de métodos
predeterminados de un trait, que viste en el Listado 10-14 cuando agregamos una
implementación predeterminada del método `summarize` en el trait `Summary`. 
Cualquier tipo que implemente el trait `Summary` tendría el método `summarize` 
disponible sin ningún código adicional. Esto es similar a una clase padre que 
tiene una implementación de un método y una clase hija heredada que también 
tiene la implementación del método. También podemos anular la implementación
predeterminada del método `summarize` cuando implementamos el trait `Summary`,
lo que es similar a una clase hija anulando la implementación de un método
heredado de una clase padre.

La otra razón para usar la herencia está relacionada con el sistema de tipos:
permitir que un tipo hijo se use en los mismos lugares que el tipo padre. Esto
es también llamado *polimorfismo*, lo que significa que puedes sustituir
múltiples objetos entre sí en tiempo de ejecución si comparten ciertas
características.

> ### Polimorfismo
>
> Para muchas personas, el polimorfismo es sinónimo de herencia. Pero en
> realidad es un concepto más general que se refiere al código que puede
> trabajar con datos de múltiples tipos. Para la herencia, esos tipos son
> generalmente subclases.
>
> En cambio, Rust utiliza generics para abstraerse sobre diferentes tipos
> posibles y los trait bounds para imponer restricciones sobre lo que
> esos tipos deben proporcionar. Esto se llama a veces *polimorfismo paramétrico
> acotado*.

En los últimos tiempos, la herencia ha perdido popularidad como solución de
diseño de programas en muchos lenguajes de programación porque a menudo está en
riesgo de compartir más código del necesario. Las subclases no siempre deben
compartir todas las características de su clase padre, pero lo harán con la
herencia. Esto puede hacer que el diseño de un programa sea menos flexible.
También introduce la posibilidad de llamar a métodos en subclases que no tienen
sentido o que causan errores porque los métodos no se aplican a la subclase.
Además, algunos lenguajes solo permitirán una herencia única (lo que significa
que una subclase solo puede heredar de una clase), lo que restringe aún más la
flexibilidad del diseño de un programa.

Por estas razones, Rust toma un enfoque diferente utilizando trait objects en
lugar de herencia. Veamos cómo los trait objects permiten el polimorfismo en
Rust.
