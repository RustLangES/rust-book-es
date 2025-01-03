## Mejorando nuestro proyecto I/O

Con este nuevo conocimiento sobre iteradores, podemos mejorar el proyecto I/O
en el Capítulo 12 usando iteradores para hacer que los lugares en el código
sean más claros y concisos. Veamos cómo los iterators pueden mejorar nuestra
implementación de la función `Config::build` y la función `search`.

### Removiendo un `clone` usando un iterator

En el Listado 12-6, agregamos código que tomó un slice de valores `String` y
creó una instancia del struct `Config` indexando en el slice y clonando
los valores, permitiendo que el struct `Config` posea esos valores. En el
Listado 13-17, hemos reproducido la implementación de la función `Config::build`
tal como estaba en el Listado 12-23:

<Listing number="13-17" file-name="src/lib.rs" caption="Reproducción de la función `Config::build` del Listing 12-23">

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-23-reproduced/src/lib.rs:ch13}}
```

</Listing>

En ese momento, dijimos que no nos preocupáramos por las llamadas ineficientes
a `clone` porque las eliminaríamos en el futuro. ¡Bueno, ese momento es ahora!

Necesitábamos `clone` aquí porque tenemos un slice con elementos `String` en el
parámetro `args`, pero la función `build` no posee `args`. Para retornar la
propiedad de una instancia de `Config`, tuvimos que clonar los valores de los
campos `query` y `file_path` de `Config` para que la instancia de `Config`
pueda poseer sus valores.

Con nuestro nuevo conocimiento sobre iteradores, podemos cambiar la función
`build` para tomar propiedad de un iterator como su argumento en lugar de
tomar prestado un slice. Usaremos la funcionalidad del iterator en lugar del
código que verifica la longitud del slice e indexa en ubicaciones específicas.
Esto aclarará lo que la función `Config::build` está haciendo porque el
iterator accederá a los valores.

Una vez que `Config::build` tome ownership del iterator y deje de usar
operaciones de indexación que toman borrowing, podemos mover los valores
`String` del iterator dentro de `Config` en lugar de llamar a `clone` y hacer
una nueva asignación.

#### Usando el iterator retornado directamente

Abre tu proyecto I/O en _src/main.rs_, el cual debería verse así:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-24-reproduced/src/main.rs:ch13}}
```

Primero cambiaremos el inicio de la función `main` que teníamos en el Listado
12-24 al código del Listado 13-18, el cual esta vez usa un iterator. Esto no
compilará hasta que actualicemos `Config::build` también.

<Listing number="13-18" file-name="src/main.rs" caption="Pasando el valor de retorno de `env::args` a `Config::build`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-18/src/main.rs:here}}
```

</Listing>

¡La función `env::args` retorna un iterator! En lugar de recolectar los valores
del iterator en un vector y luego pasar un slice a `Config::build`, ahora
estamos pasando ownership del iterator retornado por `env::args` directamente a
`Config::build`.

Luego, necesitamos actualizar la definición de `Config::build`. En el archivo
_src/lib.rs_ de tu proyecto I/O, cambiemos la firma de `Config::build` para que
se vea como el Listado 13-19. Esto aún no compilará porque necesitamos
actualizar el cuerpo de la función.

<Listing number="13-19" file-name="src/lib.rs" caption="Actualizando la firma de `Config::build` para esperar un iterator">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-19/src/lib.rs:here}}
```

</Listing>

La documentación de la biblioteca estándar para la función `env::args` muestra
que el tipo del iterator que retorna es `std::env::Args`, y que ese tipo
implementa el trait `Iterator` y retorna valores `String`.

Hemos actualizado la firma de la función `Config::build` para que el parámetro
`args` tenga un tipo genérico con los trait bounds
`impl Iterator<Item = String>` en lugar de `&[String]`. Este uso de la sintaxis
`impl Trait` que discutimos en la sección [“Traits como parámetros”][impl-trait]

<!-- ignore --> del Capítulo 10 significa que `args` puede ser cualquier tipo

que implemente el trait `Iterator` y retorne items `String`.

Debido a que estamos tomando ownership de `args` y estaremos mutando `args`
por iterarlo, podemos agregar la palabra clave `mut` en la especificación del
parámetro `args` para hacerlo mutable.

#### Usando los métodos del trait `Iterator` en lugar de indexar

Luego, necesitamos actualizar el cuerpo de `Config::build` para usar los
métodos del trait `Iterator` en lugar de indexar en el slice. En el Listado
13-20 hemos actualizado el código del Listado 12-23 para usar el método `next`:

<Listing number="13-20" file-name="src/lib.rs" caption="Changing the body of `Config::build` to use iterator methods">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-20/src/lib.rs:here}}
```

</Listing>

Recuerda que el primer valor en el valor de retorno de `env::args` es el nombre
del programa. Queremos ignorar eso y llegar al siguiente valor, así que
primero llamamos a `next` y no hacemos nada con el valor de retorno. Segundo,
llamamos a `next` para obtener el valor que queremos poner en el campo `query`
de `Config`. Si `next` retorna un `Some`, usamos un `match` para extraer el
valor. Si retorna `None`, significa que no se dieron suficientes argumentos y
retornamos temprano con un valor `Err`. Hacemos lo mismo para el valor
`file_path`.

### Haciendo el código más claro con iterator adapters

También podemos aprovechar los iterators en la función `search` de nuestro
proyecto I/O, el cual se reproduce aquí en el Listado 13-21 como estaba en el
Listado 12-19:

<Listing number="13-21" file-name="src/lib.rs" caption="La implementación de la función `search` del Listing 12-19">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:ch13}}
```

</Listing>

Podemos escribir este código de una manera más concisa usando los métodos
adaptor del iterator. Hacerlo también nos permite evitar tener un vector
intermedio mutable `results`. El estilo de programación funcional prefiere
minimizar la cantidad de estado mutable para hacer el código más claro. Remover
el estado mutable podría permitir una mejora futura para hacer que la búsqueda
ocurra en paralelo, porque no tendríamos que manejar el acceso concurrente al
vector `results`. El Listado 13-22 muestra este cambio:

<Listing number="13-22" file-name="src/lib.rs" caption="Utilizando método iterator adaptor en la implementación de la función `search`">

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-22/src/lib.rs:here}}
```

</Listing>

Recuerda que el propósito de la función `search` es retornar todas las líneas
en `contents` que contengan `query`. Similar al ejemplo de `filter` en el
Listado 13-16, este código usa el adaptador `filter` para mantener solo las
líneas que retornan `true` para `line.contains(query)`. Luego recolectamos las
líneas que coinciden en otro vector con `collect`. ¡Mucho más simple! Siéntete
libre de hacer el mismo cambio para usar los métodos del iterator en la función
`search_case_insensitive` también.

### Escogiendo entre loops o iterators

La siguiente pregunta lógica es qué estilo deberías escoger en tu propio código
y por qué: la implementación original en el Listado 13-21 o la versión usando
iterators en el Listado 13-22. La mayoría de los programadores Rust prefieren
usar el estilo de iterators. Es un poco más difícil de entender al principio,
pero una vez que obtienes una idea de los varios adaptadores de iterators y lo
que hacen, los iterators pueden ser más fáciles de entender. En lugar de
manipular los varios bits de los loops y construir nuevos vectores, el código
se enfoca en el objetivo de alto nivel del loop. Esto abstrae un poco del
código común para que sea más fácil ver los conceptos que son únicos a este
código, como la condición de filtrado que cada elemento en el iterator debe
pasar.

¿Pero son las dos implementaciones realmente equivalentes? La suposición
intuitiva podría ser que el loop más bajo nivel será más rápido. Hablemos de
performance.

[impl-trait]: ch10-02-traits.html#traits-como-parametros
