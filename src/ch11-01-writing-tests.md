## Como escribir tests

Los tests son funciones en Rust que verifican que el código no-test funciona de
la manera esperada. Los cuerpos de las funciones de test típicamente realizan
estas tres acciones:

1. Configurar cualquier dato o estado necesario.
2. Ejecutar el código que se quiere testear.
3. Verificar que los resultados son los esperados.

Veamos las características que Rust provee específicamente para escribir tests
que incluyen el atributo `test`, algunas macros, y el atributo `should_panic`.

### La anatomia de una funcion de test

En su forma más simple, un test en Rust es una función que está anotada con el
atributo `test`. Los atributos son metadatos sobre piezas de código Rust; un
ejemplo es el atributo `derive` que usamos con structs en el Capítulo 5. Para
cambiar una función en una función de test, agrega `#[test]` en la línea antes
de `fn`. Cuando ejecutas tus tests con el comando `cargo test`, Rust construye
un binario que corre las funciones anotadas y reporta si cada función de test
pasa o falla.

Cuando creamos un nuevo proyecto de librería con Cargo, se genera
automáticamente un módulo de test con una función de test. Este módulo te da
una plantilla para escribir tus tests para que no tengas que buscar la
estructura y sintaxis exacta cada vez que comiences un nuevo proyecto. ¡Puedes
agregar tantas funciones de test adicionales y tantos módulos de test como
quieras!

Exploraremos algunos aspectos de cómo funcionan los tests experimentando con la
plantilla de test antes de testear cualquier código. Luego escribiremos algunos
tests del mundo real que llaman a algún código que hemos escrito y verifican
que su comportamiento es correcto.

Creemos un nuevo proyecto de librería llamado `adder` que sume dos números:

```console
$ cargo new adder --lib
     Created library `adder` project
$ cd adder
```

El contenido del archivo `src/lib.rs` en tu librería `adder` debería verse como
el Listado 11-1.

<Listing number="11-1" file-name="src/lib.rs" caption="El módulo test y la función generada automáticamente por `cargo new`">

<!-- manual-regeneration
cd listings/ch11-writing-automated-tests
rm -rf listing-11-01
cargo new listing-11-01 --lib --name adder
cd listing-11-01
echo "$ cargo test" > output.txt
RUSTFLAGS="-A unused_variables -A dead_code" RUST_TEST_THREADS=1 cargo test >> output.txt 2>&1
git diff output.txt # commit any relevant changes; discard irrelevant ones
cd ../../..
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

</Listing>

The file starts with an example `add` function, so that we have something
to test.

Por ahora, ignoremos las dos primeras líneas y nos enfoquemos solamente en la 
función `it_works()`.
Nota la anotación `#[test]`: este atributo indica que esta es una función de
test, así que el test runner sabe que tratar esta función como un test. También
podríamos tener funciones no-test en el módulo `tests` para ayudar a configurar
escenarios comunes o realizar operaciones comunes, así que siempre necesitamos
indicar qué funciones son tests.

El cuerpo de la función de test llama a la macro `assert_eq!`, que verifica que
dos valores sean iguales. Si los valores no son iguales, `assert_eq!` falla y
el test falla. Si son iguales, no pasa nada y el test pasa.

El comando `cargo test` ejecuta todos los tests en tu proyecto de librería, Como
puedes ver en el Listado 11-2.

<Listing number="11-2" caption="El resultado de ejecutar el test generado automáticamente">

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-01/output.txt}}
```

</Listing>

Cargo compila y ejecuta el test. Vemos la línea `running 1 test`. La siguiente
línea muestra el nombre de la función de test generada, llamada 
`tests::it_works`, y que el resultado de ejecutar ese test es `ok`. El resumen 
general `test result: ok.` significa que todos los tests pasaron, y la porción 
que lee `1 passed; 0 failed` totaliza el número de tests que pasaron o fallaron.

Es posible marcar un test como ignorado para que no se ejecute en una
particular instancia; cubriremos eso en la sección [“Ignorando algunos tests a
menos que sean específicamente requeridos”][ignoring]<!-- ignore --> más tarde
en este capítulo. Porque no hemos hecho eso aquí, el resumen muestra `0
ignored`. También podemos pasar un argumento al comando `cargo test` para
ejecutar solo tests cuyo nombre coincida con un string; esto se llama
_filtrado_ y lo cubriremos en la sección [“Ejecutando un subconjunto de tests
por nombre”][subset]<!-- ignore -->. Tampoco hemos filtrado los tests que se
ejecutan, así que el final del resumen muestra `0 filtered out`.

La estadística `0 measured` es para tests de benchmark que miden performance.
Los tests de benchmark, al momento de escribir esto, solo están disponibles en
Rust nightly. Ver [la documentación sobre tests de benchmark][bench] para
aprender más.

Nosotros podemos pasar un argumento al comando `cargo test` para ejecutar solo
los tests que coincidan con el nombre en un string; esto es conocido como 
*filtering* y lo cubriremos en la sección 
[“Ejecutando un sub conjunto de Tests por Nombre”][subset]<!-- ignore -->. Aquí
no filtraremos los tests para ser ejecutados, al final de todo mostrara 
`0 filtered out`.

La siguiente parte del output de test, comenzando con `Doc-tests adder`, es
para los resultados de cualquier test de documentación. No tenemos tests de
documentación aún, pero Rust puede compilar cualquier ejemplo de código que
aparezca en nuestra documentación de API. ¡Esta característica ayuda a mantener
tus docs y tu código en sincronía! Discutiremos cómo escribir tests de
documentación en la sección [“Documentación como tests”][doc-comments]<!--ignore --> 
del Capítulo 14. Por ahora, ignoraremos el output `Doc-tests`.

Comenzaremos a personalizar el test para nuestras propias necesidades. Primero
cambiaremos el nombre de la función `it_works` a un nombre diferente, como
`exploration`, así:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/src/lib.rs}}
```

Entonces ejecutamos `cargo test` de nuevo. El output ahora muestra `exploration`
en lugar de `it_works`:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/output.txt}}
```

Ahora agregaremos otro test, ¡pero esta vez haremos un test que falle! Los
tests fallan cuando algo en la función de test hace panic. Cada test se ejecuta
en un nuevo thread, y cuando el thread principal ve que un thread de test ha
muerto, el test se marca como fallido. En el Capítulo 9, hablamos sobre cómo la
forma más simple de hacer panic es llamar a la macro `panic!`. Ingresa el nuevo
test como una función llamada `another`, así que tu archivo _src/lib.rs_ se ve
como el Listado 11-3.

<Listing number="11-3" file-name="src/lib.rs" caption="Agregando un segundo test que fallará porque llamamos a la macro `panic!`">

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-03/src/lib.rs}}
```

</Listing>

Volvemos a ejecutar los tests usando `cargo test`. El output debería verse como
el Listado 11-4, que muestra que nuestro test `exploration` pasó y `another`
falló.

<Listing number="11-4" caption="Resultados del test cuando un test pasa y el otro falla">

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-03/output.txt}}
```

</Listing>

<!-- manual-regeneration
rg panicked listings/ch11-writing-automated-tests/listing-11-03/output.txt
check the line number of the panic matches the line number in the following paragraph
 -->

En lugar de `ok`, la línea `test tests::another` muestra `FAILED`. Dos nuevas
secciones aparecen entre los resultados individuales y el resumen: la primera
muestra la razón detallada de cada falla de test. En este caso, obtenemos los
detalles de que `another` falló porque `panicked at 'Make this test fail'` en la
línea 17 del archivo _src/lib.rs_. La siguiente sección lista solo los nombres
de todos los tests que fallaron, lo cual es útil cuando hay muchos tests y
mucho output detallado de tests fallidos. Podemos usar el nombre de un test
fallido para ejecutar solo ese test y depurarlo más fácilmente; hablaremos más
sobre formas de ejecutar tests en la sección [“Controlando cómo se ejecutan los
tests”][controlando-como-los-tests-son-ejecutados]<!-- ignore -->.

La línea de resumen se muestra al final: en general, nuestro resultado de test
es `FAILED`. Tenemos un test que pasó y uno que falló.

Ahora que has visto cómo se ven los resultados de tests en diferentes
escenarios, veamos algunas macros que son útiles en tests que no sean `panic!`.

### Comprobando resultados con la macro `assert!`

La macro `assert!`, proporcionada por la biblioteca estándar, es útil cuando
quieres asegurarte de que alguna condición en un test se evalúe como `true`. Le
damos a la macro `assert!` un argumento que se evalúa a un booleano. Si el valor
es `true`, no pasa nada y el test pasa. Si el valor es `false`, la macro
`assert!` llama a `panic!` para hacer que el test falle. Usar la macro `assert!`
nos ayuda a verificar que nuestro código esté funcionando de la forma que
queremos.

En el capítulo 5, en el Listado 5-15, usamos un struct `Rectangle` y un método
`can_hold`, que se repiten aquí en el Listado 11-5. Pondremos este código en el
archivo _src/lib.rs_, luego escribiremos algunos tests para él usando la macro
`assert!`.

<Listing number="11-5" file-name="src/lib.rs" caption="Usando el struct `Rectangle` y su método `can_hold` del Capítulo 5">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-05/src/lib.rs}}
```

</Listing>

El método `can_hold` devuelve un valor booleano, lo que significa que es un caso
de uso perfecto para la macro `assert!`. En el Listado 11-6, escribimos un test
que ejercita el método `can_hold` creando una instancia de `Rectangle` que tiene
un ancho de 8 y una altura de 7 y afirmando que puede contener otra instancia
de `Rectangle` que tiene un ancho de 5 y una altura de 1.

<Listing number="11-6" file-name="src/lib.rs" caption="Un test para `can_hold` que verifica si un rectángulo más grande puede contener un rectángulo más pequeño">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-06/src/lib.rs:here}}
```

</Listing>

Observa que hemos agregado una nueva línea dentro del módulo `tests`: `use
super::*;`. El módulo `tests` es un módulo regular que sigue las reglas de
visibilidad habituales que cubrimos en el Capítulo 7 en la sección
[“Paths para referirse a un item en el árbol de
módulos”][paths-for-referring-to-an-item-in-the-module-tree]<!-- ignore -->.
Como el módulo `tests` es un módulo interno, necesitamos traer el código bajo
test en el módulo externo al alcance del módulo interno. Usamos un asterisco aquí
para que cualquier cosa que definamos en el módulo externo esté disponible para
este módulo `tests`.

Hemos llamado a nuestro test `larger_can_hold_smaller`, y hemos creado dos
instancias de `Rectangle` que necesitamos. Luego llamamos a la macro `assert!`
y le pasamos el resultado de llamar a `larger.can_hold(&smaller)`. Esta
expresión debería devolver `true`, por lo que nuestro test debería pasar.
¡Veámoslo!

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-06/output.txt}}
```

¡Pasó! Ahora agreguemos otro test, esta vez afirmando que un rectángulo más
pequeño no puede contener un rectángulo más grande:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/src/lib.rs:here}}
```

Porque el resultado correcto de la función `can_hold` en este caso es `false`,
necesitamos negar ese resultado antes de pasarlo a la macro `assert!`. Como
resultado, nuestro test pasará si `can_hold` devuelve `false`:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/output.txt}}
```

¡Dos tests que pasan! Ahora veamos qué sucede con nuestros resultados de test
cuando introducimos un bug en nuestro código. Cambiaremos la implementación del
método `can_hold` reemplazando el signo mayor que con un signo menor que cuando
compara los anchos:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/src/lib.rs:here}}
```

Ejecutar los tests ahora produce lo siguiente:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/output.txt}}
```

¡Nuestros tests atraparon el bug! Debido a que `larger.width` es `8` y
`smaller.width` es `5`, la comparación de los anchos en `can_hold` ahora devuelve
`false`: 8 no es menor que 5.

### Testeando la igualdad con las macros `assert_eq!` y `assert_ne!`

Una manera común de verificar la funcionalidad es probar la igualdad entre el
resultado del código bajo test y el valor que esperamos que el código devuelva.
Podrías hacer esto usando la macro `assert!` y pasándole una expresión usando
el operador `==`. Sin embargo, este es un test tan común que la biblioteca
estándar provee un par de macros —`assert_eq!` y `assert_ne!`— para realizar
este test de manera más conveniente. Estas macros comparan dos argumentos por
igualdad o desigualdad, respectivamente. También imprimirán los dos valores si
la aserción falla, lo que hace más fácil ver _por qué_ falló el test;
conversamente, la macro `assert!` solo indica que obtuvo un valor `false` para
la expresión `==`, sin imprimir los valores que llevaron al valor `false`.

En el Listado 11-7, escribimos una función llamada `add_two` que suma `2` a su
parámetro, luego testeamos esta función usando la macro `assert_eq!`.

<Listing number="11-7" file-name="src/lib.rs" caption="Testeando la función `add_two` usando la macro `assert_eq!`">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-07/src/lib.rs}}
```

</Listing>

¡Veamos que pasa!

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-07/output.txt}}
```

Creamos una variable llamada `result`que contiene el resultado de la llamada 
`add_two(2)`. Luego hemos pasado `4` como argumento a `assert_eq!`, que es igual 
al resultado de llamar a `add_two(2)`. La línea para este test es 
`test tests::it_adds_two ... ok`, y el texto `ok` indica que nuestro test pasó!

Vamos a introducir un error en nuestro código para ver cómo se ve `assert_eq!`
cuando falla. Cambiaremos la implementación de la función `add_two` para que
en su lugar añada `3`:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/src/lib.rs:here}}
```

Ejecutemos los tests nuevamente:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/output.txt}}
```

¡Nuestro test atrapó el bug! El test `it_adds_two` falló, y el mensaje nos dice
que la aserción que falló fue `` assertion `left == right` failed`` y
cuáles son los valores de `left` y `right`. Este mensaje nos ayuda a comenzar a
debuggear: el argumento `left` fue `4` pero el argumento `right`, donde
llamamos a `add_two(2)`, fue `5`. Puedes imaginar que esto sería especialmente
útil cuando tenemos muchos tests en marcha.

Cabe señalar que en algunos lenguajes y frameworks de test, los parámetros de
las funciones de aserción de igualdad se llaman `expected` y `actual`, y el
orden en que especificamos los argumentos importa. Sin embargo, en Rust, se
llaman `left` y `right`, y el orden en que especificamos el valor que esperamos
y el valor que el código produce no importa. Podríamos escribir la aserción en
este test como `assert_eq!(add_two(2), result)`, lo que resultaría en el mismo
mensaje de error que muestra `` assertion failed: `(left == right)` ``.

La macro `assert_ne!` pasará si los dos valores que le proporcionamos no son
iguales. Esta macro es más útil en casos en los que no estamos seguros de cuál
será el valor, pero sabemos que el valor definitivamente _no debería_ ser. Por
ejemplo, si estamos testeando una función que está garantizada de cambiar su
entrada de alguna manera, pero la forma en que la entrada cambia depende del
día de la semana en que ejecutamos nuestros tests, lo mejor sería afirmar que
el output de la función no es igual al input.

En la base, las macros `assert_eq!` y `assert_ne!` usan los operadores `==` y
`!=`, respectivamente. Cuando las aserciones fallan, estas macros imprimen sus
argumentos usando el formato de debug, lo que significa que los valores que se
comparan deben implementar los traits `PartialEq` y `Debug`. Todos los tipos
primitivos y la mayoría de los tipos de la biblioteca estándar implementan
estos traits. Para las estructuras y enumeraciones que definas, deberás
implementar `PartialEq` para afirmar la igualdad de esos tipos. También
necesitarás implementar `Debug` para imprimir los valores cuando la aserción
falla. Debido a que ambos traits son derivables, como se mencionó en el
Listado 5-12 en el Capítulo 5, esto suele ser tan sencillo como agregar la
anotación `#[derive(PartialEq, Debug)]` a la definición de tu estructura o
enumeración. Consulta el Apéndice C,
[“Traits derivables,”][derivable-traits]<!-- ignore -->
para obtener más detalles sobre estos y otros traits derivables.

### Agregando mensajes de fallo personalizados

También puedes agregar un mensaje personalizado a ser impreso con el mensaje de
fallo como argumentos opcionales a las macros `assert!`, `assert_eq!` y
`assert_ne!`. Cualquier argumento especificado después de los argumentos
requeridos se pasa a la macro `format!` (discutida en el Capítulo 8 en la
sección [“Concatenación con el operador `+` o la macro
`format!`”][concatenacion-con-el-operador--o-la-macro-format]<!-- ignore
-->), por lo que puedes pasar una format string que contenga marcadores de
posición `{}` y valores para ir en esos marcadores de posición. Los mensajes
personalizados son útiles para documentar lo que significa una aserción; cuando
un test falla, tendrás una mejor idea de cuál es el problema con el código.

Por ejemplo, supongamos que tenemos una función que saluda a las personas por
nombre y queremos probar que el nombre que pasamos a la función aparece en el
output:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-05-greeter/src/lib.rs}}
```

Las especificaciones para este programa aún no se han acordado, y estamos
bastante seguros de que el texto `Hello` al comienzo del saludo cambiará.
Decidimos que no queremos tener que actualizar el test cuando cambien los
requisitos, por lo que en lugar de verificar la igualdad exacta con el valor
devuelto de la función `greeting`, solo afirmaremos que el output contiene el
texto del parámetro de entrada.

Ahora introduciremos un bug en este código cambiando `greeting` para excluir el
`name` y veremos cómo se ve el fallo de test predeterminado:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/src/lib.rs:here}}
```

Ejecutando este test produce lo siguiente:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/output.txt}}
```

El resultado indica simplemente que la aserción falló y en qué línea se
encuentra. Un mensaje de fallo más útil imprimiría el valor de la función
`greeting`. Agreguemos un mensaje de fallo personalizado compuesto por un
format string con un marcador de posición reemplazado por el valor real que
obtuvimos de la función `greeting`:

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/src/lib.rs:here}}
```

Ahora, cuando ejecutemos el test, obtendremos un mensaje de error más
informativo:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/output.txt}}
```

Podemos ver el valor que realmente obtuvimos en el output del test, lo que
nos ayudaría a debuggear lo que sucedió en lugar de lo que esperábamos que
sucediera.

### Comprobando panics con `should_panic`

Además de verificar los valores de retorno, es importante verificar que nuestro
código maneje las condiciones de error como esperamos. Por ejemplo, considera
el tipo `Guess` que creamos en el Listado 9-13 del Capítulo 9. Otro código que
usa `Guess` depende de la garantía de que las instancias de `Guess` contendrán
solo valores entre 1 y 100. Podemos escribir un test que asegure que al
intentar crear una instancia de `Guess` con un valor fuera de ese rango, se
produzca un panic.

Lo hacemos agregando el atributo `should_panic` a nuestra función de test. El
test pasa si el código dentro de la función hace un panic; el test falla si el
código dentro de la función no hace un panic.

El Listado 11-8 muestra un test que verifica que las condiciones de error de
`Guess::new` sucedan cuando esperamos que sucedan.

<Listing number="11-8" file-name="src/lib.rs" caption="Testeando que una condición causará un `panic!`">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-08/src/lib.rs}}
```

</Listing>

Colocamos el atributo `#[should_panic]` después del atributo `#[test]` y antes
de la función de test a la que se aplica. Veamos el resultado cuando pase este
test:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-08/output.txt}}
```

¡Se ve bien! Ahora introduzcamos un bug en nuestro código eliminando la
condición de que la función `new` hará un panic si el valor es mayor que 100:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/src/lib.rs:here}}
```

Cuando ejecutemos el test del Listado 11-8, veremos que fallará:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/output.txt}}
```

No obtenemos un mensaje muy útil en este caso, pero cuando miramos la función
de test, vemos que está anotada con `#[should_panic]`. El fallo que obtuvimos
significa que el código en la función de test no causó un panic.

Los tests que usan `should_panic` pueden ser imprecisos. Un test `should_panic`
pasaría incluso si el test hace un panic por una razón diferente a la que
esperábamos. Para hacer que los tests `should_panic` sean más precisos,
podemos agregar un parámetro opcional `expected` al atributo `should_panic`.
El test harness se asegurará de que el mensaje de error contenga el texto
proporcionado. Por ejemplo, considera el código modificado para `Guess` en el
Listado 11-9 donde la función `new` hace un panic con mensajes diferentes
dependiendo de si el valor es demasiado pequeño o demasiado grande.

<Listing number="11-9" file-name="src/lib.rs" caption="Testeando un `panic!` con un mensaje panic que contiene un substring específico">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-09/src/lib.rs:here}}
```

</Listing>

Este test fallará porque el valor que pusimos en el parámetro `expected` del
atributo `should_panic` es un substring del mensaje que genera la función
`Guess::new`. Podríamos haber especificado todo el mensaje de excepción que
esperamos, que en este caso sería `Guess value must be less than or equal to
100, got 200`. Lo que elijas especificar depende de cuánto del mensaje de
excepción es único o dinámico y de cuán preciso quieras que sea tu test. En
este caso, un substring del mensaje de excepción es suficiente para asegurar
que el código en la función de test ejecuta el caso `else if value > 100`.

Para ver que sucede cuando un test `should_panic` con un mensaje `expected`
falla, introduzcamos un bug en nuestro código al intercambiar los cuerpos de
los bloques `if value < 1` y `else if value > 100`:

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/src/lib.rs:here}}
```

Esta vez, cuando ejecutemos el test `should_panic`, fallará:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/output.txt}}
```

El mensaje de error indica que el test falló con un error como esperábamos, pero
el mensaje de panic no incluyó el string esperado `less than or equal
to 100`. El mensaje de panic que obtuvimos en este caso fue
`Guess value must be greater than or equal to 1, got 200.`. ¡Ahora podemos
empezar a descubrir dónde está nuestro bug!

### Usando `Result<T, E>` en Tests

Hasta ahora, todos nuestros tests entran en panic cuando fallan. ¡También podemos
escribir tests que usen `Result<T, E>`! Aquí está el test del Listado 11-1,
reescrito para usar `Result<T, E>` y devolver un `Err` en lugar de hacer un
panic:

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-10-result-in-tests/src/lib.rs:here}}
```

La función `it_works` ahora tiene el tipo de retorno `Result<(), String>`. En
el cuerpo de la función, en lugar de llamar al macro `assert_eq!`, devolvemos
`Ok(())` cuando el test pasa y un `Err` con un `String` dentro cuando el test
falla.

Escribir tests que devuelvan un `Result<T, E>` te permite usar el operador
`?` en el cuerpo de los tests, lo que puede ser una forma conveniente de
escribir tests que fallarán si cualquier operación dentro de ellos devuelve
una variante `Err`.

No puedes utilizar la anotación `#[should_panic]` en tests que usen `Result<T,
E>`. Para asegurar que una operación devuelve una variante `Err`, _no_ uses el
operador `?` en el valor `Result<T, E>`. En su lugar, usa
`assert!(value.is_err())`.

Ahora que conoces varias formas de escribir tests, veamos qué sucede cuando
ejecutamos nuestros tests y exploremos las diferentes opciones que podemos usar
con `cargo test`.

[concatenacion-con-el-operador--o-la-macro-format]: ch08-02-strings.html#concatenacion-con-el-operador--o-la-macro-format
[bench]: https://doc.rust-lang.org/unstable-book/library-features/test.html
[ignoring]: ch11-02-running-tests.html#ignorando-algunos-tests-a-menos-que-se-soliciten-especificamente
[subset]: ch11-02-running-tests.html#ejecutando-un-subset-de-tests-por-nombre
[controlando-como-los-tests-son-ejecutados]: ch11-02-running-tests.html#controlando-como-los-tests-son-ejecutados
[derivable-traits]: appendix-03-derivable-traits.html
[doc-comments]: ch14-02-publishing-to-crates-io.html#comentarios-de-documentacion-como-tests
[paths-for-referring-to-an-item-in-the-module-tree]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
