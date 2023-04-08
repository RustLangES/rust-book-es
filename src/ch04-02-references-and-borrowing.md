## Referencias y Prestamos

El problema con la tupla de código en el Listado 4-5 es que tenemos que devolver
el `String` a la función que lo llama para que podamos seguir usando el
`String` después de la llamada a `calcular_longitud`, porque el `String` se
movió a `calcular_longitud`. En lugar de eso, podemos proporcionar una
referencia al valor `String`. Una *referencia* es como un puntero en que es una
dirección que podemos seguir para acceder a los datos almacenados en esa
dirección; esos datos son propiedad de otra variable. A diferencia de un
puntero, una referencia garantiza que apunte a un valor válido de un tipo
particular para la vida de esa referencia.

Aquí está cómo definirías y usarías una función `calcular_longitud` que tiene
una referencia a un objeto como parámetro en lugar de tomar la propiedad del

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:all}}
```

Primero, ten en cuenta que todo el código de la tupla en la declaración de la
variable y el valor de retorno de la función ha desaparecido. En segundo
lugar, observe que pasamos `&s1` a `calcular_longitud` y, en su definición,
tomamos `&String` en lugar de `String`. Este signo ampersands (&) representa
*referencia*, y te permiten referirte a algún valor sin tomar la propiedad de
él. La Figura 4-5 representa este concepto.

<div class="center">
{{#include img/trpl04-05.svg }}
</div>

<span class="caption">Figura 4-5: Un diagrama de `&String s` apuntando a `String
s1`</span>

> Nota: Lo opuesto a la referencia usando `&` es *desreferenciar*, que se
> logra con el operador de desreferencia, `*`. Veremos algunos usos del
> operador de desreferencia en el Capítulo 8 y discutiremos detalles de la
> desreferenciación en el Capítulo 15.

Vamos a echar un vistazo más de cerca a la llamada de función aquí:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:here}}
```

La sintaxis `&s1` nos permite crear una referencia que *se refiere* al valor de
`s1` pero no la posee. Porque no la posee, el valor al que apunta no se
descartará cuando la referencia deje de usarse.

Del mismo modo, la firma de la función usa `&` para indicar que el tipo del
parámetro `s` es una referencia. Vamos a agregar algunas anotaciones

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-08-reference-with-annotations/src/main.rs:here}}
```

El ambito en el que la variable `s` es válida es el mismo que el ambito de
cualquier parámetro de función, pero el valor al que apunta la referencia no se
descarta cuando `s` deja de usarse, porque `s` no tiene la propiedad. Cuando
las funciones tienen referencias como parámetros en lugar de los valores
reales, no necesitaremos devolver los valores para devolver la propiedad,
porque nunca tuvimos la propiedad.

Llamamos a la acción de crear una referencia *prestar* (borrowing en ingles).
Como en la vida real, si una persona posee algo, puedes pedir prestado. 
Cuando termines, tienes que devolverlo. No lo posees.

Entonces, ¿qué pasa si intentamos modificar algo que estamos prestando? Prueba
el código en el Listado 4-6. Spoiler alert: ¡no funciona!

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-06/src/main.rs}}
```

<span class="caption">Listado 4-6: Intentando modificar un valor prestado</span>

Aquí está el error:

```console
{{#include ../listings/ch04-understanding-ownership/listing-04-06/output.txt}}
```

Al igual que las variables son inmutables por defecto, también lo son las
referencias. No se nos permite modificar algo al que tenemos una referencia.

### Referencias Mutables

Podemos arreglar el código del Listado 4-6 para permitirnos modificar un valor
prestado con solo unos pequeños cambios que usen, en su lugar, una
*referencia mutable*:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-09-fixes-listing-04-06/src/main.rs}}
```

Primero cambiamos `s` a `mut`. Luego creamos una referencia mutable con `&mut
s` donde llamamos a la función `modificar`, y actualizamos la firma de la función
para aceptar una referencia mutable con `un_string: &mut String`. Esto hace
muy claro que la función `modificar` mutará el valor que presta.

Las referencias mutables tienen una gran restricción: si tienes una referencia
mutable a un valor, no puedes tener otras referencias a ese valor. Este código
que intenta crear dos referencias mutables a `s` fallará:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/src/main.rs:here}}
```

Aquí está el error:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/output.txt}}
```

Este error dice que este código es inválido porque no podemos prestar `s` como
mutable más de una vez a la vez. El primer préstamo mutable está en `r1` y debe
durar hasta que se use en el `println!`, pero entre la creación de esa
referencia mutable y su uso, intentamos crear otra referencia mutable en `r2`
que presta los mismos datos que `r1`.

La restricción que impide múltiples referencias mutables a los mismos datos al
mismo tiempo permite la mutación pero de una manera muy controlada. Es algo
con lo que los nuevos Rustaceans luchan porque la mayoría de los lenguajes te
permiten mutar cuando quieras. El beneficio de tener esta restricción es que
Rust puede prevenir las carreras de datos en tiempo de compilación. Una
*carrera de datos* es similar a una condición de carrera y ocurre cuando
ocurren estos tres comportamientos:

* Dos o más punteros acceden a los mismos datos al mismo tiempo.
* Al menos uno de los punteros se está utilizando para escribir en los datos.
* No hay ningún mecanismo que se esté utilizando para sincronizar el acceso a
  los datos.

Data races cause undefined behavior and can be difficult to diagnose and fix
when you’re trying to track them down at runtime; Rust prevents this problem by
refusing to compile code with data races!

Las carreras de datos causan un comportamiento indefinido y pueden ser
difíciles de diagnosticar y corregir cuando intentas rastrearlas en tiempo de
ejecución; ¡Rust evita este problema al negarse a compilar código con carreras
de datos!

Como siempre, podemos usar llaves para crear un nuevo ámbito, permitiendo
múltiples referencias mutables, solo no *simultáneas*:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-11-muts-in-separate-scopes/src/main.rs:here}}
```

Rust impone una regla similar para combinar referencias mutables e inmutables.
Este código da como resultado un error:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/src/main.rs:here}}
```

Aquí está el error:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/output.txt}}
```

¡Uf! *También* no podemos tener una referencia mutable mientras tenemos una
inmutable al mismo valor.

¡Los usuarios de una referencia inmutable no esperan que el valor cambie
repentinamente debajo de ellos! Sin embargo, se permiten múltiples referencias
inmutables porque nadie que solo está leyendo los datos tiene la capacidad de
afectar la lectura de los datos de nadie más.

Tenga en cuenta que el ámbito de una referencia comienza desde donde se
introduce y continúa hasta la última vez que se usa la referencia. Por
ejemplo, este código se compilará porque el último uso de las referencias
inmutables, el `println!`, ocurre antes de que se introduzca la referencia
mutable:

```rust,edition2021
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-13-reference-scope-ends/src/main.rs:here}}
```

Los ámbitos de las referencias inmutables `r1` y `r2` terminan después del
`println!` donde se usan por última vez, que es antes de que se cree la
referencia mutable `r3`. Estos ámbitos no se superponen, por lo que este código
está permitido: ¡el compilador puede decir que la referencia ya no se está
utilizando en un punto antes del final del ámbito!

Aunque los errores de préstamo a veces pueden ser frustrantes, recuerda que
es el compilador de Rust que señala un error potencial temprano (en tiempo de
compilación en lugar de en tiempo de ejecución) y te muestra exactamente dónde
está el problema. Entonces no tienes que rastrear por qué tus datos no son lo
que pensabas que eran.

### Referencias colgantes

En lenguajes con punteros, es fácil crear accidentalmente un *puntero colgante*:
un puntero que hace referencia a una ubicación en la memoria que puede haber
sido otorgada a otra persona, al liberar algo de memoria mientras se preserva
un puntero a esa memoria. En Rust, por el contrario, el compilador garantiza
que las referencias nunca serán referencias colgantes: si tiene una referencia
a algún dato, el compilador asegurará que los datos no salgan de ámbito antes
de que la referencia a los datos lo haga.

Intentemos crear una referencia colgante para ver cómo Rust los previene con un
error de tiempo de compilación:

<span class="filename">Filename: src/main.rs</span>

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/src/main.rs}}
```

Aquí está el error:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/output.txt}}
```

Este mensaje de error se refiere a una característica que aún no hemos cubierto:
los tiempos de vida. Discutiremos los tiempos de vida en detalle en el Capítulo
10. Pero, si ignora las partes sobre los tiempos de vida, el mensaje contiene la
clave para saber por qué este código es un problema:

```text
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from
```

Se traduciria algo así como:

```text
el tipo de retorno de la función contiene un valor prestado, pero no hay valor
para que se preste
```

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-15-dangling-reference-annotated/src/main.rs:here}}
```

Porque `s` se crea dentro de `colgar`, cuando el código de `colgar` finaliza,
`s` se desalocará. Pero intentamos devolver una referencia a él. Eso significa
que esta referencia estaría apuntando a una `String` inválida. ¡Eso no está
bien! Rust no nos dejará hacer esto.

La solución aquí es devolver la `String` directamente:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-16-no-dangle/src/main.rs:here}}
```

Esto funciona sin problemas. La propiedad se mueve fuera y nada se desaloca.

### Las reglas de las referencias

Repasemos lo que hemos discutido sobre las referencias:

* En cualquier momento dado, puedes tener *o bien* una referencia mutable *o*
  cualquier número de referencias inmutables.
* Las referencias deben ser siempre válidas.

A continuación, veremos un tipo diferente de referencia: los slices.
