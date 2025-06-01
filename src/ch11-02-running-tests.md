## Controlando como los tests son ejecutados

Al igual que `cargo run` compila tu código y luego ejecuta el binario
resultante, `cargo test` compila tu código en modo de test y ejecuta el binario
resultante. El comportamiento por defecto del binario producido por `cargo test`
es ejecutar todos los tests en paralelo y capturar la salida generada durante la
ejecución de los tests, previniendo que la salida sea mostrada y haciendo más
fácil leer la salida relacionada con los resultados de los tests. Sin embargo,
puedes especificar opciones de línea de comandos para cambiar este
comportamiento por defecto.

Algunas opciones de línea de comandos van a `cargo test`, y otras van al binario
de test resultante. Para separar estos dos tipos de argumentos, debes listar los
argumentos que van a `cargo test` seguidos del separador `--` y luego los que
van al binario de test. Ejecutar `cargo test --help` muestra las opciones que
puedes usar con `cargo test`, y ejecutar `cargo test -- --help` muestra las
opciones que puedes usar después del separador. Esas opciones también están 
documentadas en la [sección "Tests"][tests] del [libro de rustc][rustc].

[tests]: https://doc.rust-lang.org/rustc/tests/index.html
[rustc]: https://doc.rust-lang.org/rustc/index.html

### Ejecutando tests en paralelo o consecutivamente

Cuando ejecutas múltiples tests, por defecto estos se ejecutan en paralelo
usando hilos, lo que significa que terminan de ejecutarse más rápido y obtienes
feedback más rápido. Debido a que los tests se ejecutan al mismo tiempo, debes
asegurarte que tus tests no dependan entre sí o de cualquier estado compartido,
incluyendo un entorno compartido, como el directorio de trabajo actual o las
variables de entorno.

Por ejemplo, digamos que cada uno de tus tests ejecuta código que crea un
archivo en disco llamado _test-output.txt_ y escribe algunos datos en ese
archivo. Luego cada test lee los datos en ese archivo y aserta que el archivo
contiene un valor particular, el cual es diferente en cada test. Debido a que
los tests se ejecutan al mismo tiempo, un test podría sobreescribir el archivo
en el tiempo entre que otro test escribe y lee el archivo. El segundo test
fallará, no porque el código sea incorrecto, sino porque los tests han
interferido entre sí mientras se ejecutaban en paralelo. Una solución es
asegurarte que cada test escriba en un archivo diferente; otra solución es
ejecutar los tests uno a la vez.

Si no deseas ejecutar los tests en paralelo o si deseas tener un control más
fino sobre el número de hilos usados, puedes enviar la bandera `--test-threads`
y el número de hilos que deseas usar al binario de test. Echa un vistazo al
siguiente ejemplo:

```console
$ cargo test -- --test-threads=1
```

Establecemos el número de hilos de test a `1`, indicando al programa que no use
ningún paralelismo. Ejecutar los tests usando un hilo tomará más tiempo que
ejecutarlos en paralelo, pero los tests no interferirán entre sí si comparten
estado.

### Mostrando el Output de las funciones

Por defecto, si un test pasa, la librería de tests de Rust captura cualquier
cosa impresa en la salida estándar. Por ejemplo, si llamamos a `println!` en un
test y el test pasa, no veremos la salida de `println!` en la terminal; solo
veremos la línea que indica que el test pasó. Si un test falla, veremos lo que
sea que se haya impreso en la salida estándar junto con el resto del mensaje de
falla.

Como ejemplo, el Listado 11-10 tiene una función tonta que imprime el valor de
su parámetro y retorna 10, así como un test que pasa y un test que falla.

<Listing number="11-10" file-name="src/lib.rs" caption="Tests para una función que llama a `println!`">

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-10/src/lib.rs}}
```

</Listing>

Cuando ejecutamos estos tests con `cargo test`, vemos el siguiente output:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-10/output.txt}}
```

Nota que en ninguna parte de este output vemos `I got the value 4`, que es lo
que se imprime cuando el test que pasa se ejecuta. Ese output ha sido capturado.
El output del test que falla, `I got the value 8`, aparece en la sección del
resumen de tests, que también muestra la causa de la falla del test.

Si queremos ver los valores impresos por los tests que pasan también, podemos
decirle a Rust que muestre el output de los tests exitosos con `--show-output`.

```console
$ cargo test -- --show-output
```

Cuando ejecutamos los tests en el Listado 11-10 nuevamente con el flag
`--show-output`, vemos el siguiente output:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-01-show-output/output.txt}}
```

### Ejecutando un Subset de tests por nombre

A veces, ejecutar un conjunto completo de tests puede tomar mucho tiempo. Si
estás trabajando en código en un área particular, podrías querer ejecutar solo
los tests que pertenecen a ese código. Puedes elegir qué tests ejecutar
pasándole a `cargo test` el nombre o nombres del test(s) que quieres ejecutar
como argumento.

Para demostrar cómo ejecutar un subset de tests, primero crearemos tres tests
para nuestra función `add_two`, como se muestra en el Listado 11-11, y
elegiremos cuáles ejecutar.

<Listing number="11-11" file-name="src/lib.rs" caption="Tres tests con tres nombres diferentes">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-11/src/lib.rs}}
```

</Listing>

Si ejecutamos los tests sin pasar ningún argumento, como vimos anteriormente,
todos los tests se ejecutarán en paralelo:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-11/output.txt}}
```

#### Ejecutando un solo test

Podemos pasar el nombre de cualquier función de test a `cargo test` para
ejecutar solo ese test:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-02-single-test/output.txt}}
```

Solo se ejecutó el test con el nombre `one_hundred`; los otros dos tests no
coincidieron con ese nombre. El output de los tests nos indica que tenemos más
tests que no se ejecutaron al mostrar `2 filtered out` al final.

No podemos especificar los nombres de varios tests de esta manera; solo se usará
el primer valor dado a `cargo test`. Pero hay una manera de ejecutar varios
tests.

#### Filtrando para ejecutar múltiples tests

Podemos especificar parte de un nombre de test y cualquier test cuyo nombre
coincida con ese valor se ejecutará. Por ejemplo, como dos de nuestros tests
tienen `add` en el nombre, podemos ejecutar esos dos ejecutando `cargo test
add`:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-03-multiple-tests/output.txt}}
```

Este comando ejecutó todos los test con `add` en el nombre y filtró el test
con el nombre `one_hundred`. También nota que el módulo en el que aparece un
test se convierte en parte del nombre del test, por lo que podemos ejecutar
todos los tests en un módulo filtrando por el nombre del módulo.

### Ignorando algunos tests a menos que se soliciten especificamente

A veces, algunos tests específicos pueden ser muy lentos para ejecutarse, por lo
que puede que quieras excluirlos en la mayoría de las ejecuciones de
`cargo test`. En lugar de listar como argumentos todos los tests que quieres
ejecutar, puedes anotar los tests que consumen mucho tiempo usando el atributo
`ignore` para excluirlos, como se muestra aquí:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/src/lib.rs:here}}
```

Después de `#[test]` agregamos la línea `#[ignore]` al test que queremos
excluir. Ahora cuando ejecutamos nuestros tests, `it_works` se ejecuta, pero
`expensive_test` no:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/output.txt}}
```

Esta función `expensive_test` está listada como `ignored`. Si queremos ejecutar
solo los tests ignorados, podemos usar `cargo test -- --ignored`:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-04-running-ignored/output.txt}}
```

Controlando que tests se ejecutan, puedes asegurarte de que los resultados de
`cargo test` serán retornados rápidamente. Cuando estés en un punto en el que 
tenga sentido verificar los resultados de los tests ignorados y tengas tiempo 
para esperar los resultados, puedes ejecutar `cargo test -- --ignored` en su 
lugar. Si quieres ejecutar todos los tests, ignorados o no, puedes ejecutar 
`cargo test -- --include-ignored`.
