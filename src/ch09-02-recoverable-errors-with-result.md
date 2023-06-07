## Errores recuperables con `Result`

La mayoría de los errores no son lo suficientemente graves como para requerir
que el programa se detenga por completo. A veces, cuando una función falla, es
por una razón que puede interpretar y responder fácilmente. Por ejemplo, si
intenta abrir un archivo y esa operación falla porque el archivo no existe,
es posible que desee crear el archivo en lugar de terminar el proceso.

Recordemos el capítulo 
[“Manejo de fallas potenciales con `Result`”][handle_failure]<!--ignore --> 
en el Capítulo 2 que el enum `Result` se define como tener dos variantes,
`Ok` y `Err`, de la siguiente manera:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T` y `E` son parámetros de tipo generic: discutiremos los generics con más
detalle en el Capítulo 10. Lo que necesita saber ahora es que `T` representa el
tipo del valor que se devolverá en un caso de éxito dentro de la variante `Ok`,
y `E` representa el tipo del error que se devolverá en un caso de falla dentro
de la variante `Err`. Debido a que `Result` tiene estos parámetros de tipo
generic, podemos usar el tipo `Result` y las funciones definidas en él en
muchas situaciones diferentes donde el valor exitoso y el valor de error que
queremos devolver pueden diferir.

Llamemos a una función que devuelve un valor `Result` porque la función podría
fallar. En el Listado 9-3 intentamos abrir un archivo.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-03/src/main.rs}}
```

<span class="caption">Listing 9-3: Abriendo un archivo</span>

El tipo de retorno de `File::open` es un `Result<T, E>`. El parámetro genérico
`T` ha sido llenado por la implementación de `File::open` con el tipo del valor
de éxito, `std::fs::File`, que es un manejador de archivo. El tipo de `E`
utilizado en el valor de error es `std::io::Error`. Este tipo de retorno
significa que la llamada a `File::open` podría tener éxito y devolver un
manejador de archivo del que podemos leer o escribir. La llamada a la función
también podría fallar: por ejemplo, el archivo podría no existir, o podríamos
no tener permiso para acceder al archivo. La función `File::open` necesita tener
una forma de decirnos si tuvo éxito o falló y al mismo tiempo darnos el
manejador de archivo o la información de error. Esta información es exactamente
lo que transmite el enum `Result`.

En el caso en que `File::open` tenga éxito, el valor en la variable
`greeting_file_result` será una instancia de `Ok` que contiene un manejador de
archivo. En el caso en que falla, el valor en `greeting_file_result` será una
instancia de `Err` que contiene más información sobre el tipo de error que
ocurrió.

Necesitamos agregar al código en el Listado 9-3 para tomar diferentes acciones
dependiendo del valor que `File::open` devuelve. El Listado 9-4 muestra una
forma de manejar él `Result` usando una herramienta básica, la expresión
`match` que discutimos en el Capítulo 6.

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-04/src/main.rs}}
```

<span class="caption">Listing 9-4: Usando una expresión `match` para manejar las
variantes `Result` que podrían devolverse</span>

Ten en cuenta que, al igual que el enum `Option`, el enum `Result` y sus
variantes se han traído al alcance por el preludio, por lo que no necesitamos
especificar `Result::` antes de las variantes `Ok` y `Err` en las opciones de
`match`.

Cuando el result es `Ok`, este código devolverá el valor interno `file` fuera
de la variante `Ok`, y luego asignaremos ese valor de manejador de archivo a la
variable `greeting_file`. Después del `match`, podemos usar el manejador de
archivo para leer o escribir.

La otra opción en el `match` es `Err`, que significa que el `File::open` ha
fallado y el valor interno `err` de la variante `Err` contendrá información
sobre cómo o por qué falló `File::open`. En este caso, llamamos a la función
`panic!` y pasamos el valor `err` al `panic!`. Esto causa que nuestro programa
se bloquee y muestre el mensaje de error que `panic!` proporciona. Si ejecutamos
este código, obtendremos el siguiente mensaje de error:

```console
{{#include ../listings/ch09-error-handling/listing-09-04/output.txt}}
```

Como de costumbre, esta salida nos dice exactamente qué ha salido mal.

### Haciendo coincidir diferentes errores

El código en el Listado 9-4 será `panic!` no importa por qué `File::open` falló.
Sin embargo, queremos tomar diferentes acciones para diferentes razones de
falla: si `File::open` falló porque el archivo no existe, queremos crear el
archivo y devolver el manejador del nuevo archivo. Si `File::open` falló por
cualquier otra razón, por ejemplo, porque no teníamos permiso para abrir el
archivo, todavía queremos que el código `panic!` de la misma manera que lo hizo
en el Listado 9-4. Para esto agregamos una expresión `match` interna, que se
muestra en el Listado 9-5.

<span class="filename">Filename: src/main.rs</span>

<!-- ignore this test because otherwise it creates hello.txt which causes other
tests to fail lol -->

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-05/src/main.rs}}
```

<span class="caption">Listing 9-5: Manejando diferentes tipos de errores de
diferentes formas</span>

El tipo de valor que `File::open` devuelve dentro de la variante `Err` es
`io::Error`, que es una estructura proporcionada por la biblioteca estándar.
Esta estructura tiene un método `kind` que podemos llamar para obtener un valor
`io::ErrorKind`. El enum `io::ErrorKind` es proporcionado por la biblioteca
estándar y tiene variantes que representan los diferentes tipos de errores que
podrían resultar de una operación `io`. La variante que queremos usar es
`ErrorKind::NotFound`, que indica que el archivo que estamos tratando de abrir
aún no existe. Así que hacemos coincidir en `greeting_file_result`, pero
también tenemos una coincidencia interna en `error.kind()`.

La condición que queremos verificar en la coincidencia interna es si el valor
devuelto por `error.kind()` es la variante `NotFound` del enum `ErrorKind`. Si
es así, intentamos crear el archivo con `File::create`. Sin embargo, debido a
que `File::create` también podría fallar, necesitamos una segunda opción en la
expresión `match` interna. Cuando no se puede crear el archivo, se imprime un
mensaje de error diferente. La segunda opción del `match` externo permanece
igual, por lo que el programa se bloquea en cualquier error además del error de
archivo faltante.

> ### Alternativas a usar `match` con `Result<T, E>`
>
> ¡Eso es mucho `match`! La expresión `match` es útil, pero también es bastante
> verbosa. En el Capítulo 13 aprenderás sobre los closures, que se usan con
> muchos de los métodos definidos en `Result<T, E>`. Estos métodos pueden ser
> más concisos que usar `match` al manejar valores `Result<T, E>` en tu código.
>
> Por ejemplo, aquí hay otra forma de escribir la misma lógica que se muestra en
> el Listado 9-5, esta vez usando closures y el método `unwrap_or_else`:
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust,ignore
> use std::fs::File;
> use std::io::ErrorKind;
>
> fn main() {
>     let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
>         if error.kind() == ErrorKind::NotFound {
>             File::create("hello.txt").unwrap_or_else(|error| {
>                 panic!("Problem creating the file: {:?}", error);
>             })
>         } else {
>             panic!("Problem opening the file: {:?}", error);
>         }
>     });
> }
> ```
>
> Aunque este código tiene el mismo comportamiento que el Listado 9-5, no
> contiene ninguna expresión `match` y es más fácil de leer. Vuelve a este
> ejemplo después de leer el Capítulo 13, y busca el método `unwrap_or_else` en
> la documentación de la biblioteca estándar. Muchos más de estos métodos pueden
> limpiar enormes expresiones `match` anidadas cuando se trata de errores.
> 
### Atajos para `panic` en caso de error: `unwrap` y `expect`

Usando `match` funciona bastante bien, pero puede ser un poco verboso y no
siempre comunica bien la intención. El tipo `Result<T, E>` tiene muchos métodos
auxiliares definidos en él para hacer varias tareas más específicas. El método
`unwrap` es un método de atajo implementado exactamente como la expresión
`match` que escribimos en el Listado 9-4. Si el valor `Result` es la variante
`Ok`, `unwrap` devolverá el valor dentro de `Ok`. Si el `Result` es la variante
`Err`, `unwrap` llamará a la macro `panic!` por nosotros. Aquí hay un ejemplo de
`unwrap` en acción:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-04-unwrap/src/main.rs}}
```

Si ejecutamos este código sin un archivo *hello.txt*, veremos un mensaje de
error de la llamada `panic!` que el método `unwrap` hace:

<!-- manual-regeneration
cd listings/ch09-error-handling/no-listing-04-unwrap
cargo run
copy and paste relevant text
-->

```text
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os {
code: 2, kind: NotFound, message: "No such file or directory" }',
src/main.rs:4:49
```

Del mismo modo, el método `expect` nos permite elegir el mensaje de error de
`panic!`. Usando `expect` en lugar de `unwrap` y proporcionando buenos mensajes
de error puede transmitir tu intención y facilitar el seguimiento de la fuente
de un pánico. La sintaxis de `expect` se ve así:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-05-expect/src/main.rs}}
```

Nosotros usamos `expect` de la misma manera que `unwrap`: para devolver el
manejo de archivo o llamar a la macro `panic!`. El mensaje de error utilizado
por `expect` en su llamada a `panic!` será el parámetro que pasamos a `expect`,
en lugar del mensaje predeterminado de `panic!` que usa `unwrap`. Así es como
se ve:

<!-- manual-regeneration
cd listings/ch09-error-handling/no-listing-05-expect
cargo run
copy and paste relevant text
-->

```text
thread 'main' panicked at 'hello.txt should be included in this project: Os {
code: 2, kind: NotFound, message: "No such file or directory" }',
src/main.rs:5:10
```

En producción, la mayoría de los Rustaceans eligen `expect` en lugar de
`unwrap` y dan más contexto sobre por qué se espera que la operación siempre
tenga éxito. De esa manera, si tus suposiciones se demuestran incorrectas,
tienes más información para usar en la depuración.

### Propagación de errores

Cuando escribes una función cuyo cuerpo puede generar un error, en lugar de
manejar el error dentro de la función, puedes devolver el error al código que
llamó la función. Esto se conoce como *propagación* del error y le da más
control al código que llama, donde puede haber más información o lógica que
dicte cómo se debe manejar el error que la que tienes disponible en el contexto
de tu código.

Por ejemplo, El Listado 9-6 muestra una función que lee un nombre de usuario de
un archivo. Si el archivo no existe o no se puede leer, esta función devolverá
esos errores al código que llamó a la función.

<span class="filename">Filename: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-06/src/main.rs:here}}
```

<span class="caption">Listing 9-6: Una función que devuelve errores al código
llamado usando `match`</span>

Esta función se puede escribir de una manera mucho más corta, pero vamos a
empezar por hacer mucho de ella manualmente para explorar el manejo de errores;
al final, mostraremos la forma más corta. Veamos primero el tipo de retorno de
la función: `Result<String, io::Error>`. Esto significa que la función está
devolviendo un valor del tipo `Result<T, E>` donde el parámetro genérico `T` se
ha rellenado con el tipo concreto `String`, y el tipo genérico `E` se ha
rellenado con el tipo concreto `io::Error`.

Si esta función tiene éxito sin ningún problema, el código que llama a esta
función recibirá un valor `Ok` que contiene una `String` - el nombre de usuario
que esta función leyó del archivo. Si esta función encuentra algún problema, el
código que llama recibirá un valor `Err` que contiene una instancia de
`io::Error` que contiene más información sobre cuáles fueron los problemas. 
Elegimos `io::Error` como el tipo de retorno de esta función porque eso sucede 
que es el tipo del valor de error devuelto de ambas operaciones que estamos 
llamando en el cuerpo de esta función que podrían fallar: la función
`File::open` y el método `read_to_string`.

El cuerpo de la función comienza llamando a la función `File::open`. Entonces
manejamos el valor `Result` con una expresión `match` similar a la del Listado
9-4. Si `File::open` tiene éxito, el archivo manejador en el patrón de variable
`file` se convierte en el valor en la variable de patrón mutable `username_file`
y la función continúa. En el caso de `Err`, en lugar de llamar a `panic!`, 
usamos la palabra clave `return` para devolver temprano la función por 
completo y pasar el valor de error de `File::open`, ahora en la variable de 
patrón `e`, de vuelta al código que llama a esta función como el valor de error 
de esta función.

Entonces, si tenemos un manejador de archivo en `username_file`, la función
crea un nuevo `String` en la variable `username` y llama al método
`read_to_string` en el manejador de archivo en `username_file` para leer el
contenido del archivo en `username`. El método `read_to_string` también
devuelve un `Result` porque podría fallar, incluso si `File::open` tuvo éxito.
Así que necesitamos otro `match` para manejar ese `Result`: si `read_to_string`
tiene éxito, entonces nuestra función ha tenido éxito, y devolvemos el nombre de
usuario del archivo que ahora está en `username` envuelto en un `Ok`. Si
`read_to_string` falla, devolvemos el valor de error de la misma manera que
devolvimos el valor de error en el `match` que manejó el valor de retorno de
`File::open`. Sin embargo, no necesitamos decir explícitamente `return`, porque
esta es la última expresión de la función.

El código que llama a este código se encargará de obtener un valor `Ok` que
contiene un nombre de usuario o un valor `Err` que contiene un `io::Error`. Es
responsabilidad del código que llama decidir qué hacer con esos valores. Si el
código que llama obtiene un valor `Err`, podría llamar a `panic!` y bloquear el
programa, usar un nombre de usuario predeterminado o buscar el nombre de
usuario en algún lugar que no sea un archivo, por ejemplo. No tenemos
suficiente información sobre lo que el código que llama realmente está tratando
de hacer, por lo que propagamos toda la información de éxito o error hacia
arriba para que la maneje apropiadamente.

Este patrón de propagación de errores es tan común en Rust que Rust proporciona
el operador de interrogación `?` para hacer esto más fácil.

#### Un atajo para propagar errores: el operador `?`

El Listado 9-7 muestra una implementación de `read_username_from_file` que tiene
la misma funcionalidad que en el Listado 9-6, pero esta implementación utiliza
el operador `?`.

<span class="filename">Filename: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-07/src/main.rs:here}}
```

<span class="caption">Listing 9-7: Una función que devuelve errores al código
llamado usando el operador `?`</span>

El `?` colocado después de un valor `Result` se define para funcionar de casi
la misma manera que las expresiones `match` que definimos para manejar los
valores `Result` en el Listado 9-6. Si el valor de `Result` es un `Ok`, el
valor dentro del `Ok` se devolverá de esta expresión, y el programa continuará.
Si el valor es un `Err`, él `Err` se devolverá de toda la función como si
hubiéramos usado la palabra clave `return` para que el valor de error se
propague al código que llama.

Hay una diferencia entre lo que hace la expresión `match` del Listado 9-6 y lo
que hace el operador `?`: los valores de error que tienen el operador `?`
llamado en ellos pasan a través de la función `from`, definida en el trait
`From` en la biblioteca estándar, que se usa para convertir valores de un tipo
a otro. Cuando el operador `?` llama a la función `from`, el tipo de error
recibido se convierte en el tipo de error definido en el tipo de retorno de la
función actual. Esto es útil cuando una función devuelve un tipo de error para
representar todas las formas en que una función podría fallar, incluso si las
partes podrían fallar por muchas razones diferentes.

Por ejemplo, podríamos cambiar la función `read_username_from_file` en el
Listado 9-7 para devolver un tipo de error personalizado llamado `OurError` que
definimos. Si también definimos `impl From<io::Error> for OurError` para
construir una instancia de `OurError` a partir de un `io::Error`, entonces el
operador `?` llama en el cuerpo de `read_username_from_file` llamará a `from`
y convertirá los tipos de error sin necesidad de agregar más código a la
función.

En el contexto del Listado 9-7, el `?` al final de la llamada a `File::open`
devolverá el valor dentro de un `Ok` a la variable `username_file`. Si ocurre
un error, el `?` operador devolverá temprano toda la función y dará cualquier
valor `Err` al código que llama. Lo mismo se aplica al `?` al final de la
llamada a `read_to_string`.

El operador `?` elimina mucho código repetitivo y realiza esta función de
implementación más simple. Incluso podríamos acortar aún más este código
encadenando llamadas de método inmediatamente después del `?`, como se muestra
en el Listado 9-8.

<span class="filename">Filename: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-08/src/main.rs:here}}
```

<span class="caption">Listing 9-8: Método de encadenamiento 
llamado después del operador `?`</span>

Hemos movido la creación del nuevo `String` en `username` al principio de la
función; esa parte no ha cambiado. En lugar de crear una variable
`username_file`, hemos encadenado la llamada a `read_to_string` directamente
sobre el resultado de `File::open("hello.txt")?`. Todavía tenemos un `?` al
final de la llamada a `read_to_string`, y todavía devolvemos un valor `Ok`
que contiene `username` cuando tanto `File::open` como `read_to_string`
tienen éxito en lugar de devolver errores. La funcionalidad es nuevamente la
misma que en el Listado 9-6 y el Listado 9-7; esta es solo una forma diferente
y más ergonómica de escribirla.

El Listado 9-9 muestra una forma de hacer esto aún más conciso usando
`fs::read_to_string`.

<span class="filename">Filename: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-09/src/main.rs:here}}
```

<span class="caption">Listing 9-9: Usando `fs::read_to_string` en lugar de
abrir y luego leer el archivo</span>

Leer un archivo en un `String` es una operación bastante común, por lo que la
biblioteca estándar proporciona la conveniente función `fs::read_to_string`
que abre el archivo, crea un nuevo `String`, lee el contenido del archivo,
coloca el contenido en ese `String` y lo devuelve. Por supuesto, usar
`fs::read_to_string` no nos da la oportunidad de explicar todo el manejo de
errores, por lo que lo hicimos de la manera más larga primero.

#### Donde se puede usar el operador `?`

El operador `?` solo puede usarse en funciones cuyo tipo de retorno sea
compatible con el valor que se usa con el operador `?`. Porque el operador `?`
está definido para realizar una devolución temprana de un valor de la función,
de la misma manera que la expresión `match` que definimos en el Listado 9-6.
En el Listado 9-6, el `match` estaba usando un valor `Result`, y el brazo de
devolución temprana devolvió un valor `Err(e)`. El tipo de retorno de la
función debe ser un `Result` para que sea compatible con este `return`.

En el Listado 9-10, veamos el error que obtendremos si usamos el operador `?`
en una función `main` con un tipo de retorno incompatible con el tipo de valor
que usamos `?`:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-10/src/main.rs}}
```

<span class="caption">Listing 9-10: Intentando usar el `?` en la función `main`
que devuelve `()` no se compilará</span>

Este código abre un archivo, que puede fallar. El operador `?` sigue el valor
`Result` devuelto por `File::open`, pero esta función `main` tiene el tipo de
retorno de `()`, no `Result`. Cuando compilamos este código, obtenemos el
siguiente mensaje de error:

```console
{{#include ../listings/ch09-error-handling/listing-09-10/output.txt}}
```

Este error señala que solo podemos usar el operador `?` en una función que
devuelve `Result` o `Option` o en cualquier otro tipo que implemente
`FromResidual`.

Para corregir el error, tienes dos opciones. Una opción es cambiar el tipo de
retorno de tu función para que sea compatible con el valor que estás usando el
operador `?` mientras no tengas restricciones que lo impidan. La otra técnica
es usar un `match` o uno de los métodos `Result<T, E>` para manejar el 
`Result<T, E>` de la manera que sea apropiada.

El mensaje de error también menciona que el operador `?` también se puede usar 
con valores `Option<T>`. Al igual que con el uso de `?` en `Result`, solo
puedes usar `?` en `Option` en una función que devuelve `Option`. El
comportamiento del operador `?` cuando se llama en un `Option<T>` es similar a
su comportamiento cuando se llama en un `Result<T, E>`: si el valor es `None`,
el `None` se devolverá temprano desde la función en ese punto. Si el valor es
`Some`, el valor dentro de `Some` es el valor resultante de la expresión y la
función continúa. El Listado 9-11 tiene un ejemplo de una función que encuentra
el último carácter de la primera línea en el texto dado:

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-11/src/main.rs:here}}
```

<span class="caption">Listing 9-11: Using the `?` operator on an `Option<T>`
value</span>

Esta función devuelve `Option<char>` porque es posible que haya un carácter
allí, pero también es posible que no lo haya. Este código toma el argumento de
string slice `text` y llama al método `lines` en él, que devuelve un
iterador sobre las líneas en el string. Debido a que esta función quiere
examinar la primera línea, llama a `next` en el iterador para obtener el primer
valor del iterador. Si `text` es un string vacío, esta llamada a `next`
devolverá `None`, en cuyo caso usamos `?` para detener y devolver `None` desde
`last_char_of_first_line`. Si `text` no es un string vacío, `next` devolverá
un valor `Some` que contiene un string slice de la primera línea en `text`.

El `?` extrae el string slice, y podemos llamar a `chars` en ese string slice
para obtener un iterador de sus caracteres. Estamos interesados en el último
carácter en esta primera línea, por lo que llamamos a `last` para devolver el
último elemento en el iterador. Esto es un `Option` porque es posible que la
primera línea sea el string vacío, por ejemplo, si `text` comienza con una
línea en blanco pero tiene caracteres en otras líneas, como en `"\nhi"`. Sin
embargo, si hay un último carácter en la primera línea, se devolverá en la
variante `Some`. El operador `?` en el medio nos da una forma concisa de
expresar esta lógica, lo que nos permite implementar la función en una línea.
Si no pudiéramos usar el operador `?` en `Option`, tendríamos que implementar
esta lógica usando más llamadas de método o una expresión `match`.

Ten en cuenta que puedes usar el operador `?` en una función que devuelve
`Result` y puedes usar el operador `?` en una función que devuelve `Option`,
pero no puedes mezclar y combinar. El operador `?` no convertirá
automáticamente un `Result` en un `Option` o viceversa; en esos casos, puedes
usar métodos como el método `ok` en `Result` o el método `ok_or` en `Option`
para hacer la conversión explícitamente.

Hasta ahora, todas las funciones `main` que hemos usado devuelven `()`. La
función `main` es especial porque es el punto de entrada y salida de los
programas ejecutables, y hay restricciones sobre cuál puede ser su tipo de
retorno para que los programas se comporten como se espera.

Por suerte, `main` también puede devolver un `Result<(), E>`. El Listado 9-12
tiene el código del Listado 9-10, pero hemos cambiado el tipo de retorno de
`main` para que sea `Result<(), Box<dyn Error>>` y hemos agregado un valor de
retorno `Ok(())` al final. Este código ahora se compilará:

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-12/src/main.rs}}
```

<span class="caption">Listing 9-12: Cambiando `main` devuelve `Result<(), E>`
permitiendo el uso del operador `?` en valores `Result`</span>

El `Box<dyn Error>` tipo es un *trait object*, que hablaremos en la sección
[“Usando Trait Objects que permiten valores de diferentes
tipos”][trait-objects]<!-- ignore --> en el Capítulo 17. Por ahora, puedes leer
`Box<dyn Error>` para significar “cualquier tipo de error”. Usar `?` en un
valor `Result` en una función `main` con el tipo de error `Box<dyn Error>` está
permitido, porque permite que cualquier valor `Err` se devuelva temprano. A
pesar de que el cuerpo de esta función `main` solo devolverá errores de tipo
`std::io::Error`, al especificar `Box<dyn Error>`, esta firma seguirá siendo
correcta incluso si se agrega más código que devuelva otros errores al cuerpo
de `main`.

Cuando una función `main` devuelve un `Result`, el ejecutable puede
salir con un valor de `0` si `main` devuelve `Ok(())` y saldrá con un valor
distinto de `0` si `main` devuelve un `Err`. Los ejecutables escritos en C
devuelven enteros cuando salen: los programas que salen con éxito devuelven el
entero `0`, y los programas que devuelven un error devuelven algún entero
distinto de `0`. Rust también devuelve enteros de ejecutables para ser
compatibles con esta convención.

La función `main` puede devolver cualquier tipo que implemente el trait
[`std::process::Termination`][termination]<!-- ignore -->, que incluye una
función `report` que devuelve un `ExitCode`. Consulta la documentación de la
biblioteca estándar para obtener más información sobre la implementación del
trait `Termination` para tus propios tipos.

Ahora que hemos discutido los detalles de llamar a `panic!` o devolver
`Result`, volvamos al tema de cómo decidir cuál es apropiado usar en qué casos.

[handle_failure]: ch02-00-guessing-game-tutorial.html#handling-potential-failure-with-result
[trait-objects]: ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
[termination]: https://doc.rust-lang.org/std/process/trait.Termination.html
