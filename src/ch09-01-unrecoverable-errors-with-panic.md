## Errores irrecuperables con `panic!`

A veces, sucede algo malo en su código y no hay nada que pueda hacer al
respecto. En estos casos, Rust tiene la macro `panic!`. Hay dos formas de causar
un panic en la práctica: tomando una acción que hace que nuestro código entre en
pánico (como acceder a una matriz más allá del final) o llamando explícitamente
a la macro `panic!`. En ambos casos, causamos un pánico en nuestro programa. De
forma predeterminada, estos pánicos imprimirán un mensaje de error, se desharán,
limpiarán la pila y se cerrarán. A través de una variable de entorno, también
puede hacer que Rust muestre la pila de llamadas cuando ocurre un pánico para
facilitar el seguimiento de la fuente del panic.

> ### Deshacer la pila o abortar en respuesta a un pánico
>
> Por defecto, cuando ocurre un panic, el programa comienza a _deshacerse_, lo
> que significa que Rust retrocede por la pila y limpia los datos de cada
> función que encuentra. Sin embargo, este retroceso y limpieza es mucho
> trabajo. Rust, por lo tanto, le permite elegir la alternativa de _abortar_
> inmediatamente, lo que termina el programa sin limpiar.
>
> La memoria que el programa estaba usando deberá ser limpiada por el sistema
> operativo. Si en su proyecto necesita hacer que el binario resultante sea lo
> más pequeño posible, puede cambiar de deshacer el programa a abortarlo al
> producir un pánico agregando `panic = 'abort'` a las secciones `[profile]`
> apropiadas en su archivo _Cargo.toml_. Por ejemplo, si desea abortar en caso
> de pánico en el modo de lanzamiento, agregue esto:
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

Intentemos llamar un `panic!` en un programa simple:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-01-panic/src/main.rs}}
```

Cuando ejecutes el programa, verás algo como esto:

```console
{{#include ../listings/ch09-error-handling/no-listing-01-panic/output.txt}}
```

La llamada a `panic!` causa el mensaje de error contenido en las dos últimas
líneas. La primera línea muestra nuestro mensaje de panic y el lugar en nuestro
código fuente donde ocurrió el panic: _src/main.rs:2:5_ indica que es la segunda
línea, quinto carácter de nuestro archivo _src/main.rs_.

En este caso, la línea indicada es parte de nuestro código, y si vamos a esa
línea, vemos la llamada a la macro `panic!`. En otros casos, la llamada a
`panic!` podría estar en el código que nuestro código llama, y el nombre de
archivo y el número de línea informados por el mensaje de error serán el código
de otra persona donde se llama a la macro `panic!`, no la línea de nuestro
código que finalmente condujo a la llamada a `panic!`. Podemos usar el backtrace
de las funciones de las que provino la llamada a `panic!` para determinar la
parte de nuestro código que está causando el problema. Discutiremos el backtrace
en más detalle a continuación.

### Usando el backtrace de `panic!`

Veamos otro ejemplo de cómo es cuando una llamada a `panic!` proviene de una
biblioteca debido a un error en nuestro código en lugar de que nuestro código
llame directamente a la macro. El listado 9-1 tiene algún código que intenta
acceder a un índice en un vector más allá del rango de índices válidos.

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-01/src/main.rs}}
```

<span class="caption">Listado 9-1: Intentando acceder a un elemento más allá del
fin de un vector, que provocará una llamada a `panic!`</span>

Aquí, estamos intentando acceder al elemento 100 de nuestro vector (que está en
el índice 99 porque el indexado comienza en cero), pero el vector solo tiene 3
elementos. En esta situación, Rust entrará en pánico. Usar `[]` se supone que
devuelve un elemento, pero si pasa un índice no válido, no hay ningún elemento
que Rust podría devolver aquí que sea correcto.

En C, intentar leer más allá del final de una estructura de datos es un
undefined. Podría obtener lo que está en la ubicación de memoria que
correspondería a ese elemento en la estructura de datos, aunque la memoria no
pertenece a esa estructura. Esto se llama _buffer overread_ y puede provocar
vulnerabilidades de seguridad si un atacante puede manipular el índice de tal
manera que lea datos que no debería estar permitido que se almacenen después de
la estructura de datos.

Para proteger su programa de este tipo de vulnerabilidad, si intenta leer un
elemento en un índice que no existe, Rust detendrá la ejecución y se negará a
continuar. Intentémoslo y veamos:

```console
{{#include ../listings/ch09-error-handling/listing-09-01/output.txt}}
```

Este error apunta a la línea 4 de nuestro `main.rs` donde intentamos acceder al
índice 99. La siguiente línea de nota nos dice que podemos establecer la
variable de entorno `RUST_BACKTRACE` para obtener el backtrace de exactamente lo
que sucedió para causar el error. El _Backtrace_ es una lista de todas las
funciones que se han llamado para llegar a este punto. El backtrace en Rust
funciona como lo hacen en otros lenguajes: la clave para leer el backtrace es
comenzar desde la parte superior y leer hasta que vea archivos que escribió. Ese
es el lugar donde se originó el problema. Las líneas por encima de ese punto son
el código que su código ha llamado; las líneas a continuación son el código que
llamó a su código. Estas líneas antes y después pueden incluir código de Rust
core, código de biblioteca estándar o crates que estés usando. Intentemos
obtener el backtrace estableciendo la variable de entorno `RUST_BACKTRACE` a
cualquier valor excepto 0. El listado 9-2 muestra una salida similar a la que
verás.

<!-- manual-regeneration
cd listings/ch09-error-handling/listing-09-01
RUST_BACKTRACE=1 cargo run
copy the backtrace output below
check the backtrace number mentioned in the text below the listing
-->

```console
$ export RUST_BACKTRACE=1; cargo run
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/panicking.rs:142:14
   2: core::panicking::panic_bounds_check
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/panicking.rs:84:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/slice/index.rs:242:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/slice/index.rs:18:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/alloc/src/vec/mod.rs:2591:9
   6: panic::main
             at ./src/main.rs:4:5
   7: core::ops::function::FnOnce::call_once
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/ops/function.rs:248:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

<span class="caption">Listado 9-2: El backtrace generado por una llamada a
`panic!` se muestra cuando la variable de entorno `RUST_BACKTRACE` está
configurada</span>

¡Eso es mucho resultado! La salida exacta que vea puede ser diferente según su
sistema operativo y la versión de Rust. Para obtener el backtrace con esta
información, deben estar habilitados los símbolos de depuración. Los símbolos de
depuración están habilitados de forma predeterminada cuando se usa `cargo build`
o `cargo run` sin el indicador `--release`, como tenemos aquí.

En la salida en el listado 9-2, la línea 6 del backtrace apunta a la línea en
nuestro proyecto que está causando el problema: la línea 4 de `src/main.rs`. Si
no queremos que nuestro programa entre en pánico, debemos comenzar nuestra
investigación en la ubicación señalada por la primera línea que menciona un
archivo que escribimos. En la listado 9-1, donde escribimos deliberadamente un
código que entraría en pánico, la forma de solucionar el pánico es no solicitar
un elemento más allá del rango de los índices del vector. Cuando su código entra
en pánico en el futuro, deberá averiguar qué acción está tomando el código con
qué valores para causar el pánico y qué debería hacer el código en su lugar.

¡Volveremos a `panic!` y cuándo deberíamos y no deberíamos usar `panic!` para
manejar las condiciones de error en la sección [“To `panic!` or Not to
`panic!`”][to-panic-or-not-to-panic]<!-- ignore --> más adelante en este
capítulo. A continuación, veremos cómo recuperarnos de un error usando `Result`.

[to-panic-or-not-to-panic]:
    ch09-03-to-panic-or-not-to-panic.html#panic-o-no-panic
