## Trabajando con Variables de Entorno

Mejoraremos `minigrep` agregando una característica extra: una opción para
búsqueda insensible a mayúsculas y minúsculas que el usuario puede activar
mediante una variable de entorno. Podríamos hacer esta característica una opción
de línea de comandos y requerir que los usuarios la ingresen cada vez que la
quieran aplicar, pero en lugar de eso, al hacerla una variable de entorno,
permitimos a nuestros usuarios establecer la variable de entorno una vez y
tener todas sus búsquedas insensibles a mayúsculas y minúsculas en esa sesión de
terminal.

### Escribiendo un Test Fallido para la Función `search` Insensible a Mayúsculas y Minúsculas

Primero agregaremos una nueva función `search_case_insensitive` que será
llamada cuando la variable de entorno tenga un valor. Continuaremos siguiendo el
proceso TDD, así que el primer paso es nuevamente escribir un test fallido.
Agregaremos un nuevo test para la nueva función `search_case_insensitive` y
renombraremos nuestro viejo test de `one_result` a `case_sensitive` para
clarificar las diferencias entre los dos tests, como se muestra en el Listado
12-20.

<Listing number="12-20" file-name="src/lib.rs" caption="Agregando un nuevo test fallido para la función insensible a mayúsculas y minúsculas que estamos a punto de agregar">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-20/src/lib.rs:here}}
```

</Listing>

Ten en cuenta que hemos editado el `contents` del viejo test también. Hemos
agregado una nueva línea con el texto `"Duct tape."` usando una *D* mayúscula 
que no debería coincidir con la consulta `"duct"` cuando estamos buscando de 
manera sensible a mayúsculas y minúsculas. Cambiar el viejo test de esta manera 
ayuda a asegurar que no rompamos accidentalmente la funcionalidad de búsqueda
sensible a mayúsculas y minúsculas que ya hemos implementado. Este test debería
pasar ahora y debería continuar pasando mientras trabajamos en la búsqueda
insensible a mayúsculas y minúsculas.

El nuevo test para la búsqueda insensible a mayúsculas y minúsculas usa `"rUsT"`
como su consulta. En la función `search_case_insensitive` que estamos a punto
de agregar, la consulta `"rUsT"` debería coincidir con la línea que contiene
`"Rust:"` con una *R* mayúscula y coincidir con la línea `"Trust me."` aunque
ambas tienen diferente capitalización que la consulta. Este es nuestro test
fallido, y fallará al compilar porque aún no hemos definido la función
`search_case_insensitive`. Siéntete libre de agregar una implementación
esqueleto que siempre devuelva un vector vacío, similar a la forma en que lo
hicimos para la función `search` en el Listado 12-16 para ver el test compilar
y fallar.

### Implementando la Función `search_case_insensitive`

La función `search_case_insensitive`, como se muestra en el Listado 12-21,
será casi la misma que la función `search`. La única diferencia es que
convertiremos a minúsculas la `query` y cada `line` para que no importe la
mayúscula o minúscula de los argumentos de entrada, serán la misma mayúscula o
minúscula cuando verifiquemos si la línea contiene la consulta.

<Listing number="12-21" file-name="src/lib.rs" caption="Definiendo la función `search_case_insensitive` para convertir a minúsculas tanto la consulta como la línea antes de compararlas">

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-21/src/lib.rs:here}}
```

</Listing>

Primero, convertimos el string `query` a minúsculas y lo almacenamos en una
variable sombreada con el mismo nombre. Llamar a `to_lowercase` en la consulta
es necesario para que no importe si la consulta del usuario es `"rust"`,
`"RUST"`, `"Rust"` o `"rUsT"`, trataremos la consulta como si fuera `"rust"` y
y seremos insensibles a la mayúscula o minúscula. Mientras que `to_lowercase`
manejará Unicode básico, no será 100% preciso. Si estuviéramos escribiendo una
aplicación real, querríamos hacer un poco más de trabajo aquí, pero esta
sección trata sobre variables de entorno, no Unicode, así que lo dejaremos así
aquí.

Nota que `query` ahora es un `String` en lugar de un string slice, porque
llamar a `to_lowercase` crea nuevos datos en lugar de referenciar datos
existentes. Digamos que la consulta es `"rUsT"`, como un ejemplo: ese string 
slice no contiene una `u` o `t` en minúscula para que podamos usar, así que
tenemos que asignar un nuevo `String` que contenga `"rust"`. Cuando pasamos
`query` como un argumento al método `contains` ahora, necesitamos agregar un
ampersand porque la firma de `contains` está definida para tomar un string 
slice.

A continuación, agregamos una llamada a `to_lowercase` en cada `line` para
convertir a minúsculas todos los caracteres. Ahora que hemos convertido `line`
y `query` a minúsculas, encontraremos coincidencias sin importar la mayúscula
o minúscula de la consulta.

Veamos si esta implementación pasa los tests:

```console
{{#include ../listings/ch12-an-io-project/listing-12-21/output.txt}}
```

¡Genial! Pasaron. Ahora, llamemos a la nueva función `search_case_insensitive`
desde la función `run`. Primero, agregaremos una opción de configuración a la
estructura `Config` para cambiar entre la búsqueda sensible a mayúsculas y
minúsculas y la búsqueda insensible a mayúsculas y minúsculas. Agregar este
campo causará errores del compilador porque aún no estamos inicializando este
campo en ningún lugar:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/lib.rs:here}}
```

Hemos agregado el campo `ignore_case` que contiene un booleano. A continuación,
necesitamos la función `run` para verificar el valor del campo `ignore_case` y
usar eso para decidir si llamar a la función `search` o la función
`search_case_insensitive`, como se muestra en el Listado 12-22. Esto aún no se
compilará.

<Listing number="12-22" file-name="src/lib.rs" caption="Llamando a `search` o `search_case_insensitive` en función del valor de `config.ignore_case`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/lib.rs:there}}
```

</Listing>

Finalmente, necesitamos verificar la variable de entorno. Las funciones para
trabajar con variables de entorno están en el módulo `env` en la biblioteca
estándar, así que traemos ese módulo al alcance en la parte superior de
*src/lib.rs*. Luego usaremos la función `var` del módulo `env` para verificar
si se ha establecido algún valor para una variable de entorno llamada
`IGNORE_CASE`, como se muestra en el Listado 12-23.

<Listing number="12-23" file-name="src/lib.rs" caption="Comprobando si existe algún valor en una variable de entorno llamada `IGNORE_CASE`">

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-23/src/lib.rs:here}}
```

</Listing>

Aquí, creamos una nueva variable, `ignore_case`. Para establecer su valor,
llamamos a la función `env::var` y le pasamos el nombre de la variable de
entorno `IGNORE_CASE`. La función `env::var` devuelve un `Result` que será la
variante `Ok` exitosa que contiene el valor de la variable de entorno si la
variable de entorno está configurada con algún valor. Devolverá la variante
`Err` si la variable de entorno no está configurada.

Usaremos el método `is_ok` en el `Result` para verificar si la variable de
entorno está configurada, lo que significa que el programa debería hacer una
búsqueda insensible a mayúsculas. Si la variable de entorno `IGNORE_CASE` no
está configurada en nada, `is_ok` devolverá `false` y el programa realizará
una búsqueda sensible a mayúsculas. No nos importa el *valor* de la variable
de entorno, solo si está configurada o no, así que estamos verificando
`is_ok` en lugar de usar `unwrap`, `expect` o cualquiera de los otros métodos
que hemos visto en `Result`.

Hemos pasado el valor en la variable `ignore_case` a la instancia `Config` para
que la función `run` pueda leer ese valor y decidir si llamar a la función
`search_case_insensitive` o `search`, como implementamos en el Listado 12-22.

¡Probémoslo! Primero, ejecutemos el programa sin la variable de entorno
establecida y con la consulta `to`, que debería coincidir con cualquier línea
que contenga la palabra *to* en minúsculas:

```console
{{#include ../listings/ch12-an-io-project/listing-12-23/output.txt}}
```

¡Parece que aún funciona! Ahora, ejecutemos el programa con `IGNORE_CASE`
establecido en `1` pero con la misma consulta *to*.

```console
$ export IGNORE_CASE=1; cargo run -- to poem.txt
```

Si estás usando PowerShell, deberás establecer la variable de entorno y
ejecutar el programa como comandos separados:

```console
PS> $Env:IGNORE_CASE=1; cargo run -- to poem.txt
```
Esto hará que `IGNORE_CASE` persista durante el resto de la sesión de tu
shell. Puede desestablecerse con el comando `Remove-Item`:

```console
PS> Remove-Item Env:IGNORE_CASE
```

Deberíamos obtener líneas que contengan *to* que podrían tener letras
mayúsculas:

<!-- manual-regeneration
cd listings/ch12-an-io-project/listing-12-23
IGNORE_CASE=1 cargo run -- to poem.txt
can't extract because of the environment variable
-->

```console
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

Excelente, ¡también obtuvimos líneas que contienen *To*! Nuestro programa
`minigrep` ahora puede hacer búsquedas insensibles a mayúsculas y minúsculas
controladas por una variable de entorno. Ahora sabes cómo administrar las
opciones establecidas mediante argumentos de línea de comandos o variables de
entorno.

Algunos programas permiten argumentos *y* variables de entorno para la misma
configuración. En esos casos, los programas deciden que uno u otro tiene
precedencia. Para otro ejercicio por tu cuenta, intenta controlar la
sensibilidad a mayúsculas y minúsculas a través de un argumento de línea de
comandos o una variable de entorno. Decide si el argumento de línea de comandos
o la variable de entorno deben tener prioridad si el programa se ejecuta con
uno configurado para ser sensible a mayúsculas y minúsculas y otro configurado
para ignorar mayúsculas y minúsculas.

El módulo `std::env` contiene muchas más funciones útiles para trabajar con
variables de entorno: consulta su documentación para ver qué está disponible.
