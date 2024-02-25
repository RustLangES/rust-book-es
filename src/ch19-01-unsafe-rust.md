## Unsafe Rust

Todo el código que hemos discutido hasta ahora ha tenido las garantías de
seguridad de memoria de Rust aplicadas en tiempo de compilación. Sin embargo,
Rust tiene un segundo lenguaje oculto dentro de él que no hace cumplir estas
garantías de seguridad de memoria: se llama _unsafe Rust_ y funciona como
Rust regular, pero nos da superpoderes adicionales.

Unsafe Rust existe porque, por naturaleza, el análisis estático es
conservador. Cuando el compilador intenta determinar si el código cumple o no
con las garantías, es mejor que rechace algunos programas válidos que aceptar
algunos programas no válidos. Aunque el código _podría_ estar bien, si el
compilador de Rust no tiene suficiente información para estar seguro, rechazará
el código. En estos casos, puede usar código inseguro para decirle al
compilador: "Confía en mí, sé lo que estoy haciendo". Sin embargo, debes tener
cuidado, ya que el uso de Unsafe Rust conlleva riesgos: si usas código inseguro 
de manera incorrecta, pueden ocurrir problemas debido a la inseguridad de la
memoria, como la desreferenciación de puntero nulo.

Otra razón por la que Rust tiene un alter ego inseguro es que el hardware
informático subyacente es inherentemente inseguro. Si Rust no le permitiera
realizar operaciones inseguras, no podría realizar ciertas tareas. Rust
necesita permitirle realizar programación de sistemas de bajo nivel, como
interactuar directamente con el sistema operativo o incluso escribir su propio
sistema operativo. Trabajar con programación de sistemas de bajo nivel es uno
de los objetivos del lenguaje. Veamos qué podemos hacer con Rust inseguro y
cómo hacerlo.

### Superpoderes Unsafe

Para cambiar a Unsafe Rust, use la palabra clave `unsafe` y luego comience un
nuevo bloque que contenga el código inseguro. Puede tomar cinco acciones en
Rust inseguro que no puede en Rust seguro, que llamamos _superpoderes
Unsafe_. Esos superpoderes incluyen la capacidad de:

- Desreferenciar un puntero crudo
- Llamar a una función o método inseguro
- Acceder o modificar una variable estática mutable
- Implementar un trait inseguro
- Acceder a los campos de `union`s

Es importante entender que `unsafe` no desactiva el borrow checker ni 
deshabilita ninguna otra de las comprobaciones de seguridad de Rust: si usa una
referencia en código inseguro, aún se verificará. La palabra clave `unsafe`
solo le da acceso a estas cinco funciones que luego no son verificadas por el
compilador para la seguridad de la memoria. Aún obtendrá cierto grado de
seguridad dentro de un bloque inseguro.

Además, `unsafe` no significa que el código dentro del bloque sea
necesariamente peligroso o que definitivamente tendrá problemas de seguridad de
memoria: la intención es que, como programador, se asegurará de que el código
dentro de un bloque `unsafe` acceda a la memoria de una manera válida.

Las personas son falibles y pueden cometer errores, pero al requerir que estas
cinco operaciones inseguras estén dentro de bloques anotados con `unsafe`,
sabrá que cualquier error relacionado con la seguridad de la memoria debe estar
dentro de un bloque `unsafe`. Mantenga los bloques `unsafe` pequeños; lo
agradecerá más tarde cuando investigue bugs de memoria.

Para aislar el código inseguro tanto como sea posible, es mejor encerrar el
código inseguro dentro de una abstracción segura y proporcionar una API segura,
que discutiremos más adelante en el capítulo cuando examinemos las funciones y
métodos inseguros. Partes de la biblioteca estándar se implementan como
abstracciones seguras sobre código inseguro que ha sido auditado. Envolver el
código inseguro en una abstracción segura evita que los usos de `unsafe` se
filtren en todos los lugares que usted o sus usuarios puedan querer usar la
funcionalidad implementada con código `unsafe`, porque usar una abstracción
segura es seguro.

Veamos cada uno de los cinco superpoderes unsafe a su vez. También
veremos algunas abstracciones que proporcionan una interfaz segura al código
inseguro.

### Desreferenciación de un puntero crudo

En el Capítulo 4, en la sección [`Referencias y punteros`][referencias-colgantes]

<!-- ignore--> mencionamos que el compilador garantiza que las referencias siempre son válidas.

Unsafe Rust tiene dos nuevos tipos llamados _punteros crudos_ que son similares
a las referencias. Al igual que con las referencias, los punteros crudos pueden
ser inmutables o mutables y
se escriben como `*const T` y `*mut T`, respectivamente. El asterisco no es el
operador de desreferencia; es parte del nombre del tipo. En el contexto de los
punteros crudos, _inmutable_ significa que el puntero no se puede asignar
directamente después de ser desreferenciado.

A Diferencia de las referencias y los smart pointers, los punteros crudos:

- Son permitidos ignorar las reglas de borrowing al tener tanto punteros
  inmutables como mutables o múltiples punteros mutables al mismo lugar
- No se garantiza que apunten a una memoria válida
- Se les permite ser nulos
- No implementan ninguna limpieza automática

Al optar por no hacer que Rust haga cumplir estas garantías, puede renunciar a
la seguridad garantizada a cambio de un mayor rendimiento o la capacidad de
interactuar con otro lenguaje o hardware donde las garantías de Rust no se
aplican.

El Listing 19-1 muestra cómo crear un puntero crudo inmutable y mutable a 
partir de referencias.

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-01/src/main.rs:here}}
```

<span class="caption">Listing 19-1: Creando punteros crudos a partir de 
referencias</span>

Observa que no incluimos la palabra clave `unsafe` en este código. Podemos
crear punteros crudos en código seguro; simplemente no podemos desreferenciar
punteros crudos fuera de un bloque `unsafe`, como verás en un momento.

Hemos creado punteros crudos utilizando `as` para convertir una referencia
inmutable y una mutable en sus tipos de puntero crudo correspondientes. Como
los creamos directamente a partir de referencias garantizadas como válidas,
sabemos que estos punteros crudos particulares son válidos, pero no podemos
hacer esa suposición sobre cualquier puntero crudo.

Para demostrar esto, a continuación crearemos un puntero crudo cuya validez
no podemos estar tan seguros. El Listado 19-2 muestra cómo crear un puntero
crudo a una ubicación arbitraria en la memoria. Intentar usar memoria arbitraria
es indefinido: puede haber datos en esa dirección o no, el compilador puede
optimizar el código para que no haya acceso a la memoria, o el programa puede
generar un error con un fallo de segmentación. Por lo general, no hay una buena
razón para escribir código como este, pero es posible.

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-02/src/main.rs:here}}
```

<span class="caption">Listing 19-2: Creando un puntero crudo a una dirección de
memoria arbitraria</span>

Recuerda que podemos crear punteros crudos en código seguro, pero no podemos
_desreferenciar_ punteros crudos y leer la memoria a la que apuntan fuera de un
bloque `unsafe`.

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-03/src/main.rs:here}}
```

<span class="caption">Listing 19-3: Desreferenciando punteros crudos dentro de
un bloque `unsafe`</span>

Crear un puntero no causa daño; solo cuando intentamos acceder al valor al que
apunta que podríamos terminar tratando con un valor no válido.

También ten en cuenta que en los Listados 19-1 y 19-3, creamos `*const i32` y
`*mut i32` punteros crudos que apuntaban a la misma ubicación de memoria, donde
se almacena `num`. Si en su lugar intentáramos crear una referencia inmutable y
mutable a `num`, el código no se compilaría porque las reglas de ownership de
Rust no permiten una referencia mutable al mismo tiempo que cualquier referencia
inmutable. Con punteros crudos, podemos crear un puntero mutable y un puntero
inmutable a la misma ubicación y cambiar los datos a través del puntero mutable,
potencialmente creando una carrera de datos. ¡Ten cuidado!

Con todos estos peligros, ¿por qué usarías punteros crudos? Un caso de uso
importante es cuando se interactúa con código C, como verás en la siguiente
sección, [“Llamando a una función o método 
inseguro”](#llamando-a-una-funcion-o-metodo-inseguro).<!-- ignore --> Otro caso 
es cuando se construyen abstracciones seguras que el borrow checker no entiende. 
Presentaremos funciones inseguras y luego veremos un ejemplo de una abstracción
segura que usa código inseguro.

### Llamando a una funcion o metodo inseguro

El segundo tipo de operación que solo se puede realizar en un bloque unsafe es
llamar a una función o método inseguro. Podemos crear funciones inseguras y
métodos inseguros que se ven exactamente como funciones y métodos regulares,
pero tienen un `unsafe` adicional antes del resto de la definición. La palabra
clave `unsafe` en este contexto indica que la función tiene requisitos que
debemos cumplir cuando llamamos a esta función porque Rust no puede garantizar
que hayamos cumplido con estos requisitos. Al llamar a una función insegura
dentro de un bloque `unsafe`, estamos diciendo que hemos leído la documentación
de esta función y asumimos la responsabilidad de cumplir con los contratos de
la función.

Aquí hay un ejemplo de una función insegura llamada `dangerous` que no hace
nada en su cuerpo:

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-01-unsafe-fn/src/main.rs:here}}
```

Debemos llamar a la función `dangerous` dentro de un bloque `unsafe` separado.
Si intentamos llamar a esta función sin un bloque `unsafe`, obtendremos un  
error:

```console
{{#include ../listings/ch19-advanced-features/output-only-01-missing-unsafe/output.txt}}
```

Con el bloque `unsafe`, le estamos indicando a Rust que hemos leído la
documentación de la función, entendemos cómo usarla correctamente y hemos
verificado que estamos cumpliendo con el contrato de la función.

Los cuerpos de las funciones `unsafe` son similares a los bloques `unsafe`,
por lo que para realizar otras operaciones `unsafe` dentro de una función
`unsafe`, no necesitamos agregar otro bloque `unsafe`.

#### Creando una abstracción segura sobre código inseguro

Solo porque una función contiene código inseguro no significa que debamos
marcar toda la función como insegura. De hecho, envolver el código inseguro en
una función segura es una abstracción común. Como ejemplo, estudiemos la
función `split_at_mut` de la biblioteca estándar, que requiere algo de código
inseguro. Exploraremos cómo podríamos implementarlo. Este método seguro está
definido en slices mutables: toma un slice y lo divide en dos al dividir
el slice en el índice dado como argumento. El Listado 19-4 muestra cómo usar
`split_at_mut`.

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-04/src/main.rs:here}}
```

<span class="caption">Listing 19-4: Usando la función segura 
`split_at_mut`</span>

No podemos implementar esta función utilizando solo Rust seguro. Un intento
podría ser algo como el Listado 19-5, que no se compilará. Para simplificar,
implementaremos `split_at_mut` como una función en lugar de un método y solo
para slices de valores `i32` en lugar de para un tipo genérico `T`.

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-05/src/main.rs:here}}
```

<span class="caption">Listing 19-5: Un intento de implementación de
`split_at_mut` usando solo Rust seguro</span>

Esta función primero obtiene la longitud total del slice. Luego verifica si el
índice dado como parámetro está dentro del slice al verificar si es menor o
igual a la longitud. La aserción significa que si pasamos un índice que es
mayor que la longitud para dividir el slice, la función entrará en panic
antes de intentar usar ese índice.

Luego, devolvemos dos slices mutables en una tupla: uno desde el inicio del
slice original hasta el índice `mid` y otro desde `mid` hasta el final del
slice.

Cuando intentamos compilar el código en el Listado 19-5, obtendremos un error:

```console
{{#include ../listings/ch19-advanced-features/listing-19-05/output.txt}}
```

El borrow checker de Rust no puede entender que estamos tomando prestado
diferentes partes del slice; solo sabe que estamos tomando prestado el mismo
slice dos veces. Tomar prestadas diferentes partes de un slice es
fundamentalmente correcto porque los dos slices no se superponen, pero Rust no
es lo suficientemente inteligente como para saber esto. Cuando sabemos que el
código está bien, pero Rust no lo sabe, es hora de recurrir al código inseguro.

El Listado 19-6 muestra cómo usar un bloque `unsafe`, un puntero sin procesar
y algunas llamadas a funciones inseguras para hacer que la implementación de
`split_at_mut` funcione.

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-06/src/main.rs:here}}
```

<span class="caption">Listing 19-6: Usando código inseguro en la implementación 
de la función `split_at_mut`</span>

Recordemos la sección [“The Slice Type”][el-tipo-slice]<!-- ignore --> del
Capítulo 4 que los slices son un puntero a algunos datos y la longitud del
slice. Usamos el método `len` para obtener la longitud del slice y el método
`as_mut_ptr` para acceder al puntero sin procesar de un slice. En este caso,
porque tenemos un slice mutable a valores `i32`, `as_mut_ptr` devuelve un
puntero sin procesar con el tipo `*mut i32`, que hemos almacenado en la
variable `ptr`.

Mantenemos la afirmación de que el índice `mid` está dentro del slice. Luego
llegamos al código inseguro: la función `slice::from_raw_parts_mut` toma un
puntero sin procesar y una longitud, y crea un slice. Usamos esta función para
crear un slice que comienza desde `ptr` y es `mid` elementos de largo. Luego
llamamos al método `add` en `ptr` con `mid` como argumento para obtener un
puntero sin procesar que comienza en `mid`, y creamos un slice usando ese
puntero y el número restante de elementos después de `mid` como la longitud.

La función `slice::from_raw_parts_mut` es insegura porque toma un puntero sin
procesar y debe confiar en que este puntero es válido. El método `add` en
punteros sin procesar también es inseguro porque debe confiar en que la
ubicación del desplazamiento también es un puntero válido. Por lo tanto,
tuvimos que poner un bloque `unsafe` alrededor de nuestras llamadas a
`slice::from_raw_parts_mut` y `add` para poder llamarlas. Al mirar el código y
al agregar la afirmación de que `mid` debe ser menor o igual a `len`, podemos
decir que todos los punteros sin procesar utilizados dentro del bloque
`unsafe` serán punteros válidos a datos dentro del slice. Este es un uso
aceptable y apropiado de `unsafe`.

Tenga en cuenta que no necesitamos marcar la función resultante `split_at_mut`
como `unsafe`, y podemos llamar a esta función desde Rust seguro. Hemos creado
una abstracción segura para el código inseguro con una implementación de la
función que usa código `unsafe` de manera segura, porque crea solo punteros
válidos a partir de los datos a los que esta función tiene acceso.

Por el contrario, el uso de `slice::from_raw_parts_mut` en el Listado 19-7
probablemente se bloqueará cuando se use el slice. Este código toma una
ubicación de memoria arbitraria y crea un slice de 10,000 elementos.

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-07/src/main.rs:here}}
```

<span class="caption">Listing 19-7: Creando un slice a partir de una ubicación 
de memory arbitraria</span>

No somos propietarios de la memoria en esta ubicación arbitraria, y no hay
garantía de que el slice que este código crea contenga valores `i32` válidos.
Intentar usar `values` como si fuera un slice válido da como resultado un
comportamiento indefinido.

#### Usando funciones `extern` para llamar código externo

A veces, tu código en Rust necesita interactuar con código escrito en otro
lenguaje. Para esto, Rust tiene la palabra clave `extern` que facilita la
creación y el uso de una _Foreign Function Interface (FFI)_. Una FFI es una
forma para que un lenguaje de programación defina funciones y permita que un
lenguaje de programación diferente (extranjero) llame a esas funciones.

El Listado 19-8 demuestra cómo configurar una integración con la función `abs`
de la biblioteca estándar de C. Las funciones declaradas dentro de bloques
`extern` siempre son inseguras de llamar desde el código Rust. La razón es que
otros lenguajes no hacen cumplir las reglas y garantías de Rust, y Rust no
puede verificarlas, por lo que la responsabilidad recae en el programador para
garantizar la seguridad.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-08/src/main.rs}}
```

<span class="caption">Listing 19-8: Declarando y llamando a una función `extern`
definida en otro lenguaje</span>

Dentro del bloque `extern "C"` en el Listado 19-8, enumeramos los nombres y
las firmas de las funciones externas que queremos llamar. El nombre y la firma
de la función `abs` se definen en el estándar C y son parte de la biblioteca
estándar de C. La firma de la función `abs` es `int abs(int)`, lo que significa
que toma un argumento `int` y devuelve un `int`. La función `abs` devuelve el
valor absoluto de su argumento.

> #### Llamando a funciones Rust desde otros lenguajes
>
> También podemos usar `extern` para crear una interfaz que permita que otros
> lenguajes llamen funciones Rust. En lugar de crear un bloque `extern`, podemos
> agregar la palabra clave `extern` y especificar la ABI a usar justo antes de
> la palabra clave `fn` para la función relevante. También necesitamos agregar
> una anotación `#[no_mangle]` para decirle al compilador de Rust que no
> cambie el nombre de esta función. _Mangling_ es cuando un compilador cambia
> el nombre que le hemos dado a una función a un nombre diferente que contiene
> más información para otras partes del proceso de compilación para consumir,
> , pero es menos legible para los humanos. Cada compilador de lenguaje de
> programación mangla los nombres de manera ligeramente diferente, por lo que
> para que una función Rust sea nombrable por otros lenguajes, debemos
> deshabilitar el mangling del compilador de Rust.
> 
> En el siguiente ejemplo, hacemos que la función `call_from_c` sea accesible
> desde el código C, después de que se compile a una biblioteca compartida y
> se vincule desde C:
>
> ```rust
> #[no_mangle]
> pub extern "C" fn call_from_c() {
>     println!("Just called a Rust function from C!");
> }
> ```
> Este uso de `extern` no requiere `unsafe`.

### Acceder o modificar una variable estática mutable

En este libro, aún no hemos hablado de _variables globales_, las cuales Rust
admite, pero pueden ser problemáticas con las reglas de ownership de Rust. Si
dos hilos acceden a la misma variable global mutable, puede causar una 
condición de carrera.

En Rust, las variables globales son llamadas variables _static_. El Listado
19-9 muestra un ejemplo de declaración y uso de una variable static con un
string slice como valor.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-09/src/main.rs}}
```

<span class="caption">Listing 19-9: Definición y uso de una variable static
inmutable</span>

Las static variables son similares a las constantes, que discutimos en la
sección ["Diferencias entre variables y 
constantes"][differences-between-variables-and-constants] en el Capítulo 3. Los
nombres de las variables static están en `SCREAMING_SNAKE_CASE` por convención.
Las variables static solo pueden almacenar referencias con el lifetime
`'static`, lo que significa que el compilador de Rust puede calcular el 
lifetime y no estamos obligados a anotarlo explícitamente. Acceder a una 
variable static inmutable es seguro.

Una diferencia sutil entre constantes y variables static inmutables es que los
valores en una variable static tienen una dirección fija en la memoria. Usar el
valor siempre accederá a los mismos datos. Las constantes, por otro lado,
pueden duplicar sus datos cada vez que se usan. Otra diferencia es que las
variables static pueden ser mutables. Acceder y modificar variables static
mutables es _inseguro_. El Listado 19-10 muestra cómo declarar, acceder y
modificar una variable static mutable llamada `COUNTER`.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-10/src/main.rs}}
```

<span class="caption">Listing 19-10: Leer o escribir en una variable static
mutable es inseguro</span>

Como con las variables regulares, especificamos la mutabilidad usando la
palabra clave `mut`. Cualquier código que lea o escriba desde `COUNTER` debe
estar dentro de un bloque `unsafe`. Este código se compila e imprime `COUNTER:
3` como esperaríamos porque es de un solo hilo. Tener múltiples hilos accediendo
a `COUNTER`, probablemente habría condiciones de carrera.

Con datos mutables que son accesibles globalmente, es difícil asegurarse de que
no haya carreras de datos, por lo que Rust considera que las variables static
mutables son inseguras. Cuando sea posible, es preferible usar las técnicas de
concurrencia y los smart pointers seguros para los hilos que discutimos en el
Capítulo 16, para que el compilador verifique que los datos a los que se accede
desde diferentes hilos se hagan de manera segura.

### Implementando un trait inseguro

Podemos usar `unsafe` para implementar un trait inseguro. Un trait se considera
inseguro cuando al menos uno de sus métodos tiene algún invariante que el
compilador no puede verificar. Declaramos que un trait es `unsafe` agregando la
palabra clave `unsafe` antes de `trait` y marcando la implementación del trait
como `unsafe` también, como se muestra en el Listado 19-11.

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-11/src/main.rs}}
```

<span class="caption">Listing 19-11: Definiendo e implementando un trait 
inseguro</span>

Al utilizar `unsafe impl`, estamos prometiendo que mantendremos las invariantes
que el compilador no puede verificar.

Como ejemplo, recordemos los marcadores de traits `Sync` y `Send` que
discutimos en la sección ["Concurrencia extensible con los traits `Sync` y
`Send`"][concurrencia-extensible-con-los-traits-sync-y-send] en el Capítulo
16: el compilador implementa estos traits automáticamente si nuestros tipos se
componen únicamente de tipos `Send` y `Sync`. Si implementamos un tipo que
contiene un tipo que no es `Send` o `Sync`, como punteros crudos, y queremos
marcar ese tipo como `Send` o `Sync`, debemos usar `unsafe`. Rust no puede
verificar que nuestro tipo cumpla con las garantías de que se puede enviar
seguramente a través de hilos o acceder desde múltiples hilos; por lo tanto,
debemos hacer esas comprobaciones manualmente e indicarlo con `unsafe`.

### Acceder a los campos de una union

La última acción que solo se puede realizar con `unsafe` es acceder a los
campos de una _union_. Una `union` es similar a una `struct`, pero solo un
campo declarado se usa en una instancia particular en un momento dado. Las
unions se usan principalmente para interactuar con unions en código C. Acceder
a los campos de la union es inseguro porque Rust no puede garantizar el tipo de
los datos que se almacenan actualmente en la instancia de la union. Puedes
aprender más sobre las uniones en [la Referencia de Rust][reference].

### Cuándo usar código inseguro

Utilizar `unsafe` para llevar a cabo una de las cinco acciones (superpoderes)
que se acaban de mencionar no está mal ni se desaconseja. Sin embargo, es más
difícil obtener código `unsafe` correcto porque el compilador no puede ayudar a
mantener la seguridad de la memoria. Cuando tengas una razón para usar código
`unsafe`, puedes hacerlo, y tener la anotación `unsafe` explícita hace que sea
más fácil rastrear la fuente de los problemas cuando ocurren.

[referencias-colgantes]: ch04-02-references-and-borrowing.html#referencias-colgantes
[differences-between-variables-and-constants]: ch03-01-variables-and-mutability.html#constantes
[concurrencia-extensible-con-los-traits-sync-y-send]: ch16-04-extensible-concurrency-sync-and-send.html#concurrencia-extensible-con-los-traits-sync-y-send
[el-tipo-slice]: ch04-03-slices.html#el-tipo-slice
[reference]: https://doc.rust-lang.org/reference/items/unions.html
