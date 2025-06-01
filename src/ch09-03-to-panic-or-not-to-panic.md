## `panic!` o no `panic!`

Entonces, ¿cómo decides cuándo debes llamar a `panic!` y cuándo debes devolver
`Result`? Cuando el código entra en panic, no hay forma de recuperarse. Podrías
llamar a `panic!` para cualquier situación de error, ya sea que haya una forma
posible de recuperarse o no, pero entonces estás tomando la decisión de que una
situación es irreparable en nombre del código que llama. Cuando eliges devolver
un valor `Result`, le das al código que llama opciones. El código que llama
podría elegir intentar recuperarse de una manera que sea apropiada para su
situación, o podría decidir que un valor `Err` en este caso es irreparable, por
lo que puede llamar a `panic!` y convertir su error recuperable en uno
irreparable. Por lo tanto, devolver `Result` es una buena opción predeterminada
cuando estás definiendo una función que podría fallar.

En situaciones como ejemplos, código de prototipo y pruebas, es más apropiado
escribir código que entre en panic en lugar de devolver un `Result`. Veamos
por qué, luego discutiremos situaciones en las que el compilador no puede
darse cuenta de que la falla es imposible, pero tú como humano puedes. El
capítulo concluirá con algunas pautas generales sobre cómo decidir si entrar en
panic en el código de la biblioteca.

### Ejemplos, código de prototipo y test

Cuando estás escribiendo un ejemplo para ilustrar algún concepto, también
incluir código de manejo de errores robusto puede hacer que el ejemplo sea
menos claro. En los ejemplos, se entiende que una llamada a un método como
`unwrap` que podría entrar en panic se entiende como un marcador de posición
para la forma en que desea que su aplicación maneje los errores, que puede
diferir según lo que el resto de su código está haciendo.

De manera similar, los métodos `unwrap` y `expect` son muy útiles cuando se
prototipa, antes de que estés listo para decidir cómo manejar los errores.
Dejan marcadores claros en tu código para cuando estés listo para hacer que tu
programa sea más robusto.

Si una llamada a un método falla en una prueba, querrás que toda la prueba
falle, incluso si ese método no es la funcionalidad en prueba. Debido a que
`panic!` es la forma en que una prueba se marca como fallida, llamar a
`unwrap` o `expect` es exactamente lo que debería suceder.

### Casos en los que tienes mas informacion que el compilador

También sería apropiado llamar a `unwrap` o `expect` cuando tienes alguna otra
lógica que garantiza que el `Result` tendrá un valor `Ok`, pero la lógica no
es algo que el compilador entiende. Aún tendrás un valor `Result` que debes
manejar: la operación que estás llamando aún tiene la posibilidad de fallar en
general, incluso si es lógicamente imposible en tu situación particular. Si
puedes asegurar inspeccionando manualmente el código que nunca tendrás una
variante `Err`, es perfectamente aceptable llamar a `unwrap`, e incluso mejor
documentar la razón por la que crees que nunca tendrás una variante `Err` en el
texto de `expect`. Aquí hay un ejemplo:

```rust
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-08-unwrap-that-cant-fail/src/main.rs:here}}
```

Aquí estamos creando una instancia `IpAddr` analizando una cadena codificada.
Podemos ver que `127.0.0.1` es una dirección IP válida, por lo que es aceptable
usar `expect` aquí. Sin embargo, tener una cadena válida codificada no cambia
el tipo de retorno del método `parse`: aún obtenemos un valor `Result`, y el
compilador aún nos hará manejar el `Result` como si la variante `Err` fuera una
posibilidad porque el compilador no es lo suficientemente inteligente como
para ver que esta cadena es siempre una dirección IP válida. Si la cadena de
dirección IP proviniera de un usuario en lugar de estar codificada en el
programa y, por lo tanto, _tuviera_ una posibilidad de falla, definitivamente
querríamos manejar el `Result` de una manera más robusta en su lugar. Mencionar
la suposición de que esta dirección IP está codificada nos indicará que
cambiemos `expect` a un mejor código de manejo de errores si en el futuro
necesitamos obtener la dirección IP de otra fuente.

### Pautas para el manejo de errores

Es aconsejable que tu código entre en panic cuando sea posible que tu código
termine en un estado incorrecto. En este contexto, un _estado incorrecto_ es
cuando se ha roto alguna suposición, garantía, contrato o invariante, como
cuando se pasan valores no válidos, valores contradictorios o valores
faltantes a tu código, más uno o más de los siguientes:

- El mal estado es algo inesperado, a diferencia de algo que probablemente
  suceda ocasionalmente, como un usuario que ingresa datos en el formato
  incorrecto.
- Tu código después de este punto debe confiar en no estar en este mal estado,
  en lugar de verificar el problema en cada paso.
- No hay una buena manera de codificar esta información en los tipos que
  usas. Trabajaremos a través de un ejemplo de lo que queremos decir en la
  sección [“Codificación de estados y comportamientos como tipos”][encoding]
  <!-- ignore --> del Capítulo 18.

Si alguien llama a tu código y pasa valores que no tienen sentido, es mejor
devolver un error si puedes para que el usuario de la biblioteca pueda decidir
qué hacer en ese caso. Sin embargo, en los casos en que continuar podría ser
inseguro o dañino, la mejor opción podría ser llamar a `panic!` y alertar a la
persona que usa tu biblioteca sobre el error en su código para que puedan
solucionarlo durante el desarrollo. De manera similar, `panic!` a menudo es
apropiado si estás llamando a un código externo que está fuera de tu control y
devuelve un estado no válido que no tienes forma de solucionar.

Sin embargo, cuando se espera que falle, es más apropiado devolver un `Result`
que hacer una llamada a `panic!`. Los ejemplos incluyen un analizador que
recibe datos con formato incorrecto o una solicitud HTTP que devuelve un estado
que indica que has alcanzado un límite de velocidad. En estos casos, devolver
un `Result` indica que el fallo es una posibilidad esperada que el código
llamado decidirá cómo manejarlo.

Cuando tu código realiza una operación que podría poner a un usuario en riesgo
si se llama con valores no válidos, tu código debe verificar primero que los
valores sean válidos y entrar en panic si los valores no son válidos. Esto es
principalmente por razones de seguridad: intentar operar con datos no válidos
puede exponer tu código a vulnerabilidades. Esta es la razón principal por la
que la biblioteca estándar llamará a `panic!` si intentas un acceso a memoria
fuera de los límites: intentar acceder a la memoria que no pertenece a la
estructura de datos actual es un problema de seguridad común. Las funciones
suelen tener _contratos_: su comportamiento solo está garantizado si las
entradas cumplen con requisitos particulares. Entrar en panic cuando se viola
el contrato tiene sentido porque una violación del contrato siempre indica un
error del lado del llamador y no es un tipo de error que deseas que el código
llamado tenga que manejar explícitamente. De hecho, no hay una manera
razonable para que el código de llamada se recupere; los programadores que
llaman deben corregir el código. Los contratos para una función, especialmente
cuando una violación causará un panic, deben explicarse en la documentación
de la API de la función.

Sin embargo, tener muchas comprobaciones de errores en todas tus funciones
sería verboso y molesto. Afortunadamente, puedes usar el sistema de tipos de
Rust (y, por lo tanto, la comprobación de tipos realizada por el compilador)
para hacer muchas de las comprobaciones por ti. Si tu función tiene un tipo
particular como parámetro, puedes proceder con la lógica de tu código sabiendo
que el compilador ya se ha asegurado de que tengas un valor válido. Por
ejemplo, si tienes un tipo en lugar de un `Option`, tu programa espera tener
_algo_ en lugar de _nada_. Tu código entonces no tiene que manejar dos casos
para las variantes `Some` y `None`: solo tendrá un caso para tener
definitivamente un valor. El código que intenta pasar nada a tu función ni
siquiera se compilará, por lo que tu función no tiene que verificar ese caso
en tiempo de ejecución. Otro ejemplo es usar un tipo de entero sin signo como
`u32`, que garantiza que el parámetro nunca sea negativo.

### Creacion de tipos personalizados para validacion

Tomemos la idea de usar el sistema de tipos de Rust para garantizar que
tengamos un valor válido un paso más allá y veamos cómo crear un tipo
personalizado para validación. Recuerda el juego de adivinanzas en el Capítulo
2 en el que nuestro código le pidió al usuario que adivinara un número entre 1
y 100. Nunca validamos que la suposición del usuario estuviera entre esos
números antes de verificarla con nuestro número secreto; solo validamos que la
suposición fuera positiva. En este caso, las consecuencias no fueron muy
graves: nuestra salida de “Demasiado alto” o “Demasiado bajo” seguiría siendo
correcta. Pero sería una mejora útil guiar al usuario hacia suposiciones
válidas y tener un comportamiento diferente cuando un usuario adivina un
número que está fuera del rango en comparación con cuando un usuario escribe,
por ejemplo, letras en su lugar.

Una forma de hacer esto sería analizar la suposición como un `i32` en lugar de
solo un `u32` para permitir números potencialmente negativos, y luego agregar
una verificación de que el número esté en el rango, de esta manera:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-09-guess-out-of-range/src/main.rs:here}}
```

</Listing>

La expresión `if` verifica si nuestro valor está fuera del rango, le dice al
usuario sobre el problema y llama a `continue` para iniciar la siguiente
iteración del ciclo y pedir otra suposición. Después de la expresión `if`,
podemos continuar con las comparaciones entre `guess` y el número secreto
sabiendo que `guess` está entre 1 y 100.

Sin embargo, esta no es una solución ideal: si fuera absolutamente crítico que
el programa solo operara en valores entre 1 y 100, y tuviera muchas funciones
con este requisito, tener una verificación como esa en cada función sería
tedioso (y podría afectar el rendimiento).

En su lugar, podemos crear un nuevo tipo y poner las verificaciones en una
función para crear una instancia del tipo en lugar de repetir las
verificaciones en cada función. De esa manera, es seguro que las funciones
utilicen el nuevo tipo en sus firmas y utilicen los valores que reciben con
confianza. El Listado 9-13 muestra una forma de definir un tipo `Guess` que
solo creará una instancia de `Guess` si la función `new` recibe un valor entre
1 y 100.

<Listing number="9-13" caption="Un tipo `Guess` que solo continuará con valores entre 1 y 100" file-name="src/guessing_game.rs">

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-13/src/guessing_game.rs}}
```

</Listing>

Primero, creamos un nuevo modulo llamado `guessing_game`. Luego definimos un 
struct en este modulo llamado `Guess` que tiene un campo llamado `value` que 
contiene un `i32`. 
Aquí es donde se almacenará el número.

Luego implementamos una función asociada llamada `new` en `Guess` que crea
instancias de valores `Guess`. La función `new` está definida para tener un
parámetro llamado `value` de tipo `i32` y para devolver un `Guess`. El código
en el cuerpo de la función `new` prueba `value` para asegurarse de que esté
entre 1 y 100. Si `value` no pasa esta prueba, hacemos una llamada `panic!`,
que alertará al programador que está escribiendo el código de llamada que
tiene un error que debe corregir, porque crear un `Guess` con un `value` fuera
de este rango violaría el contrato en el que `Guess::new` se basa. Las
condiciones en las que `Guess::new` podría entrar en pánico deben discutirse
en la documentación de la API de cara al público; cubriremos las convenciones
de documentación que indican la posibilidad de un `panic!` en la documentación
de la API que creas en el Capítulo 14. Si `value` pasa la prueba, creamos un
nuevo `Guess` con su campo `value` establecido en el `value` y devolvemos el
`Guess`.

A continuación, implementamos un método llamado `value` que toma prestado
`self`, no tiene otros parámetros y devuelve un `i32`. Este tipo de método se
llama a veces _getter_, porque su propósito es obtener algunos datos de sus
campos y devolverlos. Este método público es necesario porque el campo `value`
del struct `Guess` es privado. Es importante que el campo `value` sea privado
para que el código que usa el struct `Guess` no pueda establecer `value`
directamente: el código fuera del módulo `guessing_game` _debe_ usar la función 
`Guess::new` para crear una instancia de `Guess`, lo que garantiza que no hay 
forma de que un `Guess` tenga un `value` que no haya sido verificado por las 
condiciones en la función `Guess::new`.

Una función que tiene un parámetro o devuelve solo números entre 1 y 100 podría
entonces declarar en su firma que toma o devuelve un `Guess` en lugar de un
`i32` y no necesitaría hacer ninguna verificación adicional en su cuerpo.

## Resumen

Las características de manejo de errores de Rust están diseñadas para ayudarte
a escribir un código más robusto. La macro `panic!` indica que tu programa
está en un estado que no puede manejar y te permite indicarle al proceso que
se detenga en lugar de intentar continuar con valores no válidos o incorrectos.
El enum `Result` usa el sistema de tipos de Rust para indicar que las
operaciones pueden fallar de una manera que tu código podría recuperar. Puedes
usar `Result` para decirle al código que llama a tu código que necesita manejar
el éxito o el error de manera potencial. Usar `panic!` y `Result` en las
situaciones apropiadas hará que tu código sea más confiable ante los problemas
inevitables.

Ahora que has visto formas útiles en que la biblioteca estándar usa generics
con los enums `Option` y `Result`, hablaremos sobre cómo funcionan los
generics y cómo puedes usarlos en tu código.

[encoding]: ch18-03-oo-design-patterns.html#codificando-estados-y-comportamiento-como-tipos
