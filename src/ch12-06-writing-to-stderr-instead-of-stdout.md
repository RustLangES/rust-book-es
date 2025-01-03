## Escribiendo mensajes de error estándar en lugar del output estándar

En este momento, estamos escribiendo toda nuestro output en la terminal usando
la macro `println!`. En la mayoría de las terminales, hay dos tipos de output:
*output estándar* (`stdout`) para información general y *error estándar*
(`stderr`) para mensajes de error. Esta distinción permite a los usuarios
elegir dirigir el output exitoso de un programa a un archivo pero aun así
imprimir mensajes de error en la pantalla.

La macro `println!` solo es capaz de imprimir en el output estándar, así que
tenemos que usar algo más para imprimir en el error estándar.

### Revisando donde se escriben los errores

Primero, observemos como el contenido impreso por `minigrep` está siendo
escrito en el output estándar, incluyendo cualquier mensaje de error que
queramos escribir en el error estándar. Haremos eso redirigiendo el output
estándar a un archivo mientras causamos un error intencionalmente. No
redirigiremos el error estándar, así que cualquier contenido enviado al error
estándar continuará mostrándose en la pantalla.

Los programas de línea de comandos se espera que envíen mensajes de error al
error estándar así que podemos ver los mensajes de error en la pantalla incluso
si redirigimos el output estándar a un archivo. Nuestro programa no se está
comportando bien: estamos a punto de ver que guarda los mensajes de error en un
archivo en su lugar!

Para demostrar este comportamiento, ejecutaremos el programa con `>` y la ruta
del archivo, *output.txt*, al que queremos redirigir el output estándar. No
pasaremos ningún argumento, lo que debería causar un error:

```console
$ cargo run > output.txt
```

La sintaxis `>` le dice a la shell que escriba el contenido del output estándar
en *output.txt* en lugar de la pantalla. No vimos el mensaje de error que
esperábamos impreso en la pantalla, así que eso significa que debe haber
terminado en el archivo. Esto es lo que contiene *output.txt*:

```text
Problem parsing arguments: not enough arguments
```

Sí, nuestro mensaje de error está siendo impreso en el output estándar. Es mucho
más útil para mensajes de error como este ser impresos en el error estándar así
que solo los datos de una ejecución exitosa terminen en el archivo. Cambiaremos
eso.

### Imprimiendo errores en el error estándar

Usaremos el código en el Listado 12-24 para cambiar como los mensajes de error
son impresos. Debido al refactor que hicimos anteriormente en este capítulo,
todo el código que imprime mensajes de error está en una función, `main`. La
librería estándar provee la macro `eprintln!` que imprime en el flujo de error
estándar, así que cambiaremos los dos lugares donde estábamos llamando
`println!` para imprimir errores usando `eprintln!` en su lugar.

<Listing number="12-24" file-name="src/main.rs" caption="Escribiendo mensajes de error en el error estándar en lugar del output estándar utilizando `eprintln!`">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-24/src/main.rs:here}}
```

</Listing>

Ahora, ejecutaremos el programa de la misma manera que antes, sin pasar ningún
argumento y redirigiendo el output estándar con `>`:

```console
$ cargo run > output.txt
Problem parsing arguments: not enough arguments
```

Ahora podemos ver el mensaje de error en la pantalla y *output.txt* no contiene
nada, que es el comportamiento que esperamos de los programas de línea de
comandos.

Ejecutemos el programa otra vez con argumentos que no causen un error pero aun
así redirigiendo el output estándar a un archivo, como así:

```console
$ cargo run -- to poem.txt > output.txt
```

No veremos ningún output en la terminal, y *output.txt* contendrá nuestros
resultados:

<span class="filename">Filename: output.txt</span>

```text
Are you nobody, too?
How dreary to be somebody!
```

Esto demuestra que ahora estamos usando el output estándar para output exitoso
y el error estándar para output de error como es apropiado.

## Resumen

En este capítulo repasó algunos de los conceptos principales que has aprendido
hasta ahora y cubrió como realizar operaciones de I/O comunes en Rust. Al usar
argumentos de línea de comandos, archivos, variables de ambiente, y la macro
`eprintln!` para imprimir errores, ahora estás preparado para escribir
aplicaciones de línea de comandos. Combinado con los conceptos de capítulos
anteriores, tu código estará bien organizado, almacenará datos efectivamente en
las estructuras de datos apropiadas, manejará errores de manera agradable, y
estará bien testeado.

A continuación, exploraremos algunas características de Rust que fueron
influenciadas por lenguajes funcionales: closures e iterators.

