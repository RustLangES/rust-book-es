## Desarrollando la funcionalidad de la biblioteca con el desarrollo Test-Driven

Ahora que hemos extraído la lógica en *src/lib.rs* y dejado la recolección de
argumentos y el manejo de errores en *src/main.rs*, es mucho más fácil escribir
pruebas para la funcionalidad principal de nuestro código. Podemos llamar a las
funciones directamente con varios argumentos y verificar los valores de
retorno sin tener que llamar a nuestro binario desde la línea de comandos.

En esta sección, agregaremos la lógica de búsqueda al programa `minigrep`
utilizando el proceso de desarrollo impulsado por pruebas (TDD) con los
siguientes pasos:

1. Escriba un test que falle y ejecútala para asegurarse de que falla por la
   razón que espera.
2. Escribe o modifica solo el código suficiente para que el nuevo test pase.
3. Refactoriza el código que acabas de agregar o cambiar y asegúrate de que los
   tests sigan pasando.
4. ¡Repite desde el paso 1!

Aunque es solo una de las muchas formas de escribir software, TDD puede ayudar
a impulsar el diseño del código. Escribir la prueba antes de escribir el código
que hace que la prueba pase ayuda a mantener una alta cobertura de prueba
durante todo el proceso.

Vamos a probar la implementación de la funcionalidad que realmente buscará el
string de consulta en el contenido del archivo y producirá una lista de líneas
que coincidan con la consulta. Agregaremos esta funcionalidad en una función
llamada `search`.

### Escribiendo un test fallido

Debido a que ya no los necesitamos, eliminemos las declaraciones `println!` de
*src/lib.rs* y *src/main.rs* que usamos para verificar el comportamiento del
programa. Luego, en *src/lib.rs*, agregue un módulo `tests` con una función de
prueba, como lo hicimos en [Capítulo 11][ch11-anatomy]<!-- ignore -->. La
función de prueba especifica el comportamiento que queremos que tenga la
función `search`: tomará una consulta y el texto a buscar, y devolverá solo las
líneas del texto que contengan la consulta. El listado 12-15 muestra esta
prueba, que aún no se compilará.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-15/src/lib.rs:here}}
```

<span class="caption">Listing 12-15: Creando un test fallido para la función
`search` que deseamos tener</span>

Este test busca el string `"duct"`. El texto que estamos buscando son tres
líneas, solo una de las cuales contiene `"duct"` (Tenga en cuenta que la barra
invertida después de la comilla doble de apertura le dice a Rust que no ponga
un carácter de nueva línea al comienzo del contenido de esta cadena literal).
Afirmamos que el valor devuelto de la función `search` contiene solo la línea
que esperamos.

Aún no podemos ejecutar este test y verlo fallar porque el test ni siquiera se
compila: ¡la función `search` aún no existe! De acuerdo con los principios de
TDD, agregaremos solo el código suficiente para que la prueba se compile y se
ejecute agregando una definición de la función `search` que siempre devuelve
un vector vacío, como se muestra en el listado 12-16. Luego, la prueba debería
compilar y fallar porque un vector vacío no coincide con un vector que
contiene la línea `"safe, fast, productive."`.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-16/src/lib.rs:here}}
```

<span class="caption">Listing 12-16: Definiendo solo lo necesario de la función
`search` para que nuestro test compile</span>

Observa que necesitamos definir un lifetime explícito `'a` en la firma de
`search` y usar ese lifetime con el argumento `contents` y el valor de retorno.
Recuerde en [Capítulo 10][ch10-lifetimes]<!-- ignore --> que los parámetros de
lifetime especifican qué lifetime de argumento está conectado al lifetime del
valor de retorno. En este caso, indicamos que el vector devuelto debe contener
string slices que hagan referencia a slices del argumento `contents` (en lugar
del argumento `query`).

En otras palabras, le decimos a Rust que los datos devueltos por la función
`search` vivirán tanto tiempo como los datos pasados a la función `search` en
el argumento `contents`. ¡Esto es importante! Los datos a los que hace
referencia un slice deben ser válidos para que la referencia sea válida; si el
compilador asume que estamos haciendo string slices de `query` en lugar de
`contents`, hará sus comprobaciones de seguridad incorrectamente.

Si olvidamos las anotaciones de lifetime y tratamos de compilar esta función,
obtendremos este error:

```console
{{#include ../listings/ch12-an-io-project/output-only-02-missing-lifetimes/output.txt}}
```

Rust no puede saber qué argumento de los dos necesitamos, por lo que debemos
decirle explícitamente. Debido a que `contents` es el argumento que contiene
todo nuestro texto y queremos devolver las partes de ese texto que coincidan,
sabemos que `contents` es el argumento que debe estar conectado al valor de
retorno usando la sintaxis de lifetime.

Otros lenguajes de programación no requieren que conectes argumentos a valores
de retorno en la firma, pero esta práctica será más fácil con el tiempo. Quizás
quiera comparar este ejemplo con la sección ["Validando referencias con
lifetimes"][validating-references-with-lifetimes]<!-- ignore --> en el 
Capítulo 10.

Ahora ejecutemos el test:

```console
{{#include ../listings/ch12-an-io-project/listing-12-16/output.txt}}
```

¡Genial, el test falla, exactamente como esperábamos! ¡Vamos a hacer que el
test pase!

### Escribiendo código para pasar el test

Actualmente, nuestro test falla porque siempre devolvemos un vector vacío. Para
solucionar eso e implementar `search`, nuestro programa debe seguir estos
pasos:

* Iterar a través de cada línea del contenido.
* Compruebe si la línea contiene nuestro string de consulta.
* Si es así, agréguelo a la lista de valores que estamos devolviendo.
* Si no lo hace, no haga nada.
* Devuelve la lista de resultados que coinciden.

Trabajaremos en cada paso, comenzando por iterar a través de las líneas.

#### Iterando a través de las líneas con el método `lines`

Rust tiene un método útil para manejar la iteración línea por línea de strings,
convenientemente llamado `lines`, que funciona como se muestra en el listado
12-17. Tenga en cuenta que esto aún no se compilará.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-17/src/lib.rs:here}}
```

<span class="caption">Listing 12-17: Iterando a través de cada línea en
`contents`</span>

El método `lines` devuelve un iterador. Hablaremos sobre los iteradores en
profundidad en [Capítulo 13][ch13-iterators]<!-- ignore -->, pero recuerde que
vio esta forma de usar un iterador en [Listado 3-5][ch3-iter]<!-- ignore -->,
donde usamos un bucle `for` con un iterador para ejecutar algún código en cada
elemento de una colección.

#### Buscando cada línea para la consulta

A continuación, necesitamos verificar si la línea contiene el string de
consulta. Afortunadamente, los strings tienen un método útil llamado `contains`
que hace esto por nosotros. Agregue una llamada al método `contains` en la
función `search`, como se muestra en el listado 12-18. Tenga en cuenta que esto
aún no se compilará.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-18/src/lib.rs:here}}
```

<span class="caption">Listing 12-18: Agregando funcionalidad para verificar si
la línea contiene el string en `query`</span>

En este punto, estamos construyendo funcionalidad. Para que compile, debemos
devolver un valor del cuerpo como indicamos en la firma de la función.

#### Almacenando líneas coincidentes

Para terminar esta función, necesitamos una forma de almacenar las líneas
coincidentes que queremos devolver. Para eso, podemos hacer un vector mutable
antes del bucle `for` y llamar al método `push` para almacenar una `line` en el
vector. Después del bucle `for`, devolvemos el vector, como se muestra en el
listado 12-19.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:here}}
```

<span class="caption">Listing 12-19: Almacenando las líneas que coinciden para
poder devolverlas</span>

Ahora la función `search` debería devolver solo las líneas que contienen
`query`, y nuestro test debería pasar. Ejecutemos el test:

```console
{{#include ../listings/ch12-an-io-project/listing-12-19/output.txt}}
```

Nuestro test pasó, así que sabemos que funciona. ¡Genial!

En este punto, podríamos considerar oportunidades para refactorizar la
implementación de la función `search` mientras mantenemos las pruebas para
mantener la misma funcionalidad. El código en la función `search` no es tan
malo, pero no aprovecha algunas características útiles de los iteradores.
Volveremos a este ejemplo en [Capítulo 13][ch13-iterators]<!-- ignore -->, donde
exploraremos los iteradores en detalle y veremos cómo mejorarlo.

#### Usando la función `search` en la función `run`

Ahora que la función `search` funciona y está probada, necesitamos llamar a
`search` desde nuestra función `run`. Necesitamos pasar el valor de 
`config.query` y el `contents` que `run` lee del archivo a la función `search`. 
Luego, `run` imprimirá cada línea devuelta por `search`:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/src/lib.rs:here}}
```

Todavía estamos usando un bucle `for` para devolver cada línea de `search` e
imprimirla.

Ahora todo el programa debería funcionar. Probémoslo con una palabra que
debería devolver exactamente una línea del poema de Emily Dickinson, "frog":

```console
{{#include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/output.txt}}
```

¡Funciona! Ahora intentemos que coincida con varias líneas, como "body":

```console
{{#include ../listings/ch12-an-io-project/output-only-03-multiple-matches/output.txt}}
```

Y finalmente, asegurémonos de que no obtengamos ninguna línea cuando buscamos
una palabra que no está en el poema, como "monomorphization":

```console
{{#include ../listings/ch12-an-io-project/output-only-04-no-matches/output.txt}}
```

¡Excelente! Hemos construido nuestra propia versión de una herramienta clásica
y hemos aprendido mucho sobre cómo estructurar aplicaciones. También hemos
aprendido un poco sobre input y output de archivos, lifetimes, testing y
análisis de líneas de comandos.

Para completar nuestro proyecto, demostraremos brevemente cómo trabajar con
variables de entorno y cómo imprimir en el error estándar, ambas son útiles
cuando se escriben programas de línea de comandos.

[validating-references-with-lifetimes]:
ch10-03-lifetime-syntax.html#validating-references-with-lifetimes
[ch11-anatomy]: ch11-01-writing-tests.html#the-anatomy-of-a-test-function
[ch10-lifetimes]: ch10-03-lifetime-syntax.html
[ch3-iter]: ch03-05-control-flow.html#looping-through-a-collection-with-for
[ch13-iterators]: ch13-02-iterators.html
