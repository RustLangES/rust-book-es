## Publicando un Crate a Crates.io

Hasta ahora, hemos usado paquetes de [crates.io](https://crates.io/)<!-- ignore
--> como dependencias de nuestro proyecto, pero también puedes compartir tu
código con otras personas publicando tus propios paquetes. El registro de
paquetes en [crates.io](https://crates.io/)<!-- ignore --> distribuye el código
fuente de tus paquetes, por lo que aloja principalmente código que es de código
abierto.

Rust y Cargo tienen características que hacen que tu paquete publicado sea más
fácil de encontrar y usar. Hablaremos sobre algunas de estas características a
continuación y luego explicaremos cómo publicar un paquete.

### Haciendo comentarios de documentación útiles

Documentar adecuadamente tus paquetes ayudará a otros usuarios a saber cómo y
cuándo usarlos, por lo que vale la pena invertir el tiempo para escribir
documentación. En el Capítulo 3, discutimos cómo comentar el código Rust usando
dos barras diagonales, `//`. Rust también tiene un tipo particular de comentario
para la documentación, conocido convenientemente como un _comentario de
documentación_, que generará documentación HTML. El HTML muestra el contenido
de los comentarios de documentación para los elementos de API públicos
destinados a programadores interesados en saber cómo _usar_ tu paquete en
oposición a cómo se _implementa_ tu paquete.

Los comentarios de documentación usan tres barras diagonales, `///`, en lugar
de dos y admiten la notación Markdown para formatear el texto. Coloca los
comentarios de documentación justo antes del elemento que están documentando.
El Listado 14-1 muestra comentarios de documentación para una función `add_one`
en un crate llamado `my_crate`.

<Listing number="14-1" file-name="src/lib.rs" caption="Un comentario de documentación para una función">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-01/src/lib.rs}}
```

</Listing>

Aquí, damos una descripción de lo que hace la función `add_one`, comenzamos una
sección con el encabezado `Examples` y luego proporcionamos código que
demuestra cómo usar la función `add_one`. Podemos generar la documentación HTML
de este comentario de documentación ejecutando `cargo doc`. Este comando ejecuta
la herramienta `rustdoc` distribuida con Rust y coloca la documentación HTML
generada en el directorio _target/doc_.

Por conveniencia, ejecutar `cargo doc --open` generará el HTML para la
documentación de tu crate actual (así como la documentación para todas las
dependencias de tu crate) y abrirá el resultado en un navegador web. Navega
hasta la función `add_one` y verás cómo se renderiza el texto en los comentarios
de documentación, como se muestra en la Figura 14-1:

<img alt="Documentación HTML renderizada para la función `add_one` de `my_crate`" src="img/trpl14-01.png" class="center" />

<span class="caption">Figura 14-1: documentación en HTML para la función
`add_one`</span>

#### Secciones comúnmente usadas

Hemos usado el encabezado de Markdown `# Examples` en el Listado 14-1 para crear
una sección en el HTML con el título "Examples". Aquí hay algunas otras
secciones que los autores de crates comúnmente usan en su documentación:

- **Panics**: Los escenarios en los que la función documentada podría
  entrar en panic. Los llamadores de la función que no quieren que sus
  programas entren en panic deben asegurarse de no llamar a la función en
  estas situaciones.
- **Errores**: Si la función devuelve un `Result`, describir los tipos de
  errores que podrían ocurrir y qué condiciones podrían hacer que esos errores
  se devuelvan puede ser útil para los llamadores para que puedan escribir
  código para manejar los diferentes tipos de errores de diferentes maneras.
- **Seguridad**: Si la función es `unsafe` de llamar (discutimos unsafe en
  el Capítulo 20), debería haber una sección que explique por qué la función es
  insegura y cubra las invariantes que la función espera que los llamadores
  mantengan.

La mayoría de los comentarios de documentación no necesitan todas estas
secciones, pero esta es una buena lista de verificación para recordar los
aspectos del código que los usuarios estarán interesados en saber.

#### Comentarios de documentacion como Tests

Agregar bloques de código de ejemplo en tus comentarios de documentación puede
ayudar a demostrar cómo usar tu biblioteca, y hacerlo tiene una ventaja
adicional: ¡ejecutar `cargo test` ejecutará los ejemplos de código en tu
documentación como pruebas! Nada es mejor que la documentación con ejemplos.
Pero nada es peor que los ejemplos que no funcionan porque el código ha cambiado
desde que se escribió la documentación. Si ejecutamos `cargo test` con la
documentación para la función `add_one` del Listado 14-1, veremos una sección en
los resultados de la prueba como esta:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-01/
cargo test
copy just the doc-tests section below
-->

```text
   Doc-tests my_crate

running 1 test
test src/lib.rs - add_one (line 5) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.27s
```

¡Ahora si cambiamos la función o el ejemplo para que el `assert_eq!` en el
ejemplo entre en pánico y ejecutamos `cargo test` nuevamente, veremos que los
doc tests capturan que el ejemplo y el código están fuera de sincronización
entre sí!

#### Comentando items contenidos

El estilo de comentario de doc `//!` agrega documentación al item que contiene
los comentarios en lugar de a los items que siguen a los comentarios.
Normalmente, usamos estos comentarios de documentación dentro del archivo raíz
del crate (_src/lib.rs_ por convención) o dentro de un módulo para documentar el
crate o el módulo en su conjunto.

Por ejemplo, para agregar documentación que describe el propósito del crate
`my_crate` que contiene la función `add_one`, agregamos comentarios de
documentación que comienzan con `//!` al principio del archivo _src/lib.rs_,
como se muestra en el Listado 14-2:

<Listing number="14-2" file-name="src/lib.rs" caption="Documentación para el crate `my_crate` como un todo">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-02/src/lib.rs:here}}
```

</Listing>

Observa que no hay ningún código después de la última línea que comienza con
`//!`. Debido a que comenzamos los comentarios con `//!` en lugar de `///`,
estamos documentando el item que contiene este comentario en lugar de un item
que sigue a este comentario. En este caso, ese item es el archivo _src/lib.rs_,
que es el crate root. Estos comentarios describen todo el crate.

Cuando ejecutamos `cargo doc --open` ahora, veremos la documentación para el
crate `my_crate` en lugar de la documentación para la función `add_one`, como
se muestra en la Figura 14-2:

<img alt="Documentación HTML renderizada con un comentario para el crate como un todo" src="img/trpl14-02.png" class="center" />

<span class="caption">Figura 14-2: Documentación renderizada para `my_crate`,
incluido el comentario que describe el crate como un todo</span>

Los comentarios de documentación dentro de los items son útiles para describir
crates y módulos en particular. Úsalos para explicar el propósito general del
contenedor para ayudar a tus usuarios a comprender la organización del crate.

### Exportando una API publica conveniente con `pub use`

La estructura de tu API pública es una consideración importante al publicar un
crate. Las personas que usan tu crate están menos familiarizadas con la
estructura que tú y podrían tener dificultades para encontrar las piezas que
desean usar si tu crate tiene una gran jerarquía de módulos.

En el Capítulo 7, cubrimos cómo hacer que los items sean públicos usando la
palabra clave `pub` y traer items a un scope con la palabra clave `use`.
Sin embargo, la estructura que tiene sentido para ti mientras desarrollas un
crate puede que no sea muy conveniente para tus usuarios. Es posible que desees
organizar tus structs en una jerarquía que contenga varios niveles, pero luego
las personas que desean usar un tipo que has definido profundamente en la
jerarquía podrían tener problemas para descubrir que ese tipo existe. También
podrían estar molestos por tener que ingresar `use`
`my_crate::some_module::another_module::UsefulType;` en lugar de `use`
`my_crate::UsefulType;`.

Las buenas noticias son que si la estructura _no_ es conveniente para que otros
la usen desde otra biblioteca, no tienes que reorganizar tu organización
interna: en su lugar, puedes reexportar items para hacer una estructura pública
que sea diferente de tu estructura privada usando `pub use`. Reexportar toma un
item público en una ubicación y lo hace público en otra ubicación, como si se
definiera en la otra ubicación en su lugar.

Por ejemplo, supongamos que creamos una biblioteca llamada `art` para modelar
conceptos artísticos. Dentro de esta biblioteca hay dos módulos: un módulo
`kinds` que contiene dos enums llamados `PrimaryColor` y `SecondaryColor` y un
módulo `utils` que contiene una función llamada `mix`, como se muestra en el
Listado 14-3:

<Listing number="14-3" file-name="src/lib.rs" caption="Una biblioteca llamada `art` con items organizados en los módulos `kinds` y `utils">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-03/src/lib.rs:here}}
```

</Listing>

La Figura 14-3 muestra cómo se vería la página frontal de la documentación para
este crate generada por `cargo doc`:

<img alt="Rendered documentation for the `art` crate that lists the `kinds` and `utils` modules" src="img/trpl14-03.png" class="center" />

<span class="caption">Figure 14-3: Página principal de la documentación de `art`
que enumera los módulos `kinds` y `utils`</span>

Nota que los tipos `PrimaryColor` y `SecondaryColor` no están listados en la
página principal. Tampoco lo está la función `mix`. Para verlos, tendríamos que
hacer clic en `kinds` y `utils`.

Otro crate que depende de esta biblioteca necesitaría declarar un `use` que
traigan los items de `art` al scope, especificando la estructura de módulos
actualmente definida. El Listado 14-4 muestra un ejemplo de un crate que usa
los items `PrimaryColor` y `mix` del crate `art`:

<Listing number="14-4" file-name="src/main.rs" caption="Un crate que utiliza los items del crate `art` con su estructura interna exportada">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-04/src/main.rs}}
```

</Listing>

El autor del código en el Listado 14-4, que usa el crate `art`, tuvo que
averiguar que `PrimaryColor` está en el módulo `kinds` y `mix` está en el
módulo `utils`. La estructura de módulos del crate `art` es más relevante para
los desarrolladores que trabajan en el crate `art` que para aquellos que lo
usan. La estructura interna no contiene ninguna información útil para alguien
que intenta comprender cómo usar el crate `art`, sino que causa confusión
porque los desarrolladores que lo usan tienen que averiguar dónde buscar y
deben especificar los nombres de módulo en las declaraciones `use`.

Para remover la estructura interna de la API pública, podemos modificar el
código del crate `art` en el Listado 14-3 para agregar declaraciones `pub use`
para reexportar los items en el nivel superior, como se muestra en el Listado
14-5:

<Listing number="14-5" file-name="src/lib.rs" caption="Agregando declaraciones `pub use` para re-exportar items">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-05/src/lib.rs:here}}
```

</Listing>

La documentación de la API que `cargo doc` genera para este crate ahora
listará y enlazará los reexports en la página principal, como se muestra en la
Figura 14-4, haciendo que los tipos `PrimaryColor` y `SecondaryColor` y la
función `mix` sean más fáciles de encontrar.

<img alt="Documentación renderizada para el crate `art` con las re-exportaciones en la página principal" src="img/trpl14-04.png" class="center" />

<span class="caption">Figura 14-4: La página principal de la documentación para
`art` que lista las re-exportaciones</span>

Los usuarios del crate `art` aún pueden ver y usar la estructura interna del
Listado 14-3 como se demuestra en el Listado 14-4, o pueden usar la estructura
más conveniente del Listado 14-5, como se muestra en el Listado 14-6:

<Listing number="14-6" file-name="src/main.rs" caption="Un programa que utiliza los items reexportados del crate `art`">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-06/src/main.rs:here}}
```

</Listing>

En casos donde hay muchos módulos anidados, reexportar los tipos en el nivel
superior con `pub use` puede hacer una diferencia significativa en la
experiencia de las personas que usan el crate. Otro uso común de `pub use` es
reexportar definiciones de una dependencia en el crate actual para hacer que
las definiciones de ese crate sean parte de la API pública de su crate.

Crear una estructura de API pública es más un arte que una ciencia, y puedes
iterar para encontrar la API que funcione mejor para tus usuarios. Elegir `pub
use` te da flexibilidad en cómo estructuras tu crate internamente y desacopla
esa estructura interna de lo que presentas a tus usuarios. Mira algo del código
de los crates que has instalado para ver si su estructura interna difiere de su
API pública.

### Configurando una cuenta de Crates.io

Antes de que puedas publicar cualquier crate, necesitas crear una cuenta en
[crates.io](https://crates.io/)<!-- ignore --> y obtener un token de API. Para
hacerlo, visita la página de inicio en
[crates.io](https://crates.io/)<!-- ignore --> e inicia sesión a través de una
cuenta de GitHub. (La cuenta de GitHub es actualmente un requisito, pero el
sitio podría admitir otras formas de crear una cuenta en el futuro). Una vez
que hayas iniciado sesión, visita la configuración de tu cuenta en
[https://crates.io/me/](https://crates.io/me/)<!-- ignore --> y recupera tu
clave de API. Luego ejecuta el comando `cargo login` y pega tu clave de la API, 
cuando se solicitad, como se muestra a continuación:

```console
$ cargo login
abcdefghijklmnopqrstuvwxyz012345
```

Este comando informará a Cargo de tu token de API y lo almacenará localmente en
_~/.cargo/credentials_. Ten en cuenta que este token es un _secreto_: no lo
compartas con nadie. Si lo compartes con alguien por cualquier motivo, debes
revocarlo y generar un nuevo token en
[crates.io](https://crates.io/)<!-- ignore -->.

### Agregando metadata a un nuevo crate

Supongamos que tienes un crate que deseas publicar. Antes de publicarlo,
necesitarás agregar algunos metadatos en la sección `[package]` del archivo
_Cargo.toml_ del crate.

Tu crate necesitará un nombre único. Mientras trabajas en un crate localmente,
puedes nombrar un crate como quieras. Sin embargo, los nombres de los crates en
[crates.io](https://crates.io/)<!-- ignore --> se asignan por orden de llegada.
Una vez que se toma un nombre de crate, nadie más puede publicar un crate con
ese nombre. Antes de intentar publicar un crate, busca el nombre que deseas
usar. Si el nombre ha sido usado, deberás encontrar otro nombre y editar el
campo `name` en el archivo _Cargo.toml_ bajo la sección `[package]` para usar
el nuevo nombre para publicar, como se muestra a continuación:

<span class="filename">Nombre de archivo: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
```

Incluso si has elegido un nombre único, cuando ejecutes `cargo publish` para
publicar el crate en este punto, obtendrás una advertencia y luego un error:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-01/
cargo publish
copy just the relevant lines below
-->

```console
$ cargo publish
    Updating crates.io index
warning: manifest has no description, license, license-file, documentation, homepage or repository.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
--snip--
error: failed to publish to registry at https://crates.io

Caused by:
  the remote server responded with an error (status 400 Bad Request): missing or empty metadata fields: description, license. Please see https://doc.rust-lang.org/cargo/reference/manifest.html for more information on configuring these field
```

Estos errores se deben a que te faltan algunos datos cruciales: se requiere una
descripción y una licencia para que las personas sepan qué hace tu crate y
bajo qué términos pueden usarlo. En _Cargo.toml_, agrega una descripción que
sea solo una o dos oraciones, porque aparecerá con tu crate en los resultados
de búsqueda. Para el campo `license`, debes dar un _valor de identificador de
licencia_. La [Linux Foundation’s Software Package Data Exchange (SPDX)][spdx]
enumera los identificadores que puedes usar para este valor. Por ejemplo, para
especificar que has licenciado tu crate usando la Licencia MIT, agrega el
identificador `MIT`:

<span class="filename">Nombre de archivo: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
license = "MIT"
```

Si tu deseas especificar una licencia que no aparece en el SPDX, necesitas
colocar el texto de esa licencia en un archivo, incluir el archivo en tu
proyecto y luego usar `license-file` para especificar el nombre de ese archivo
en lugar de usar la key `license`.

La orientación sobre qué licencia es apropiada para tu proyecto está fuera del
alcance de este libro. Muchas personas en la comunidad de Rust licencian sus
proyectos de la misma manera que Rust, usando una licencia dual de `MIT OR
Apache-2.0`. Esta práctica demuestra que también puedes especificar múltiples
identificadores de licencia separados por `OR` para tener múltiples licencias
para tu proyecto.

Con un nombre único, la versión, una descripción y una licencia agregados, el
archivo _Cargo.toml_ para un proyecto que está listo para publicar podría
verse así:

<span class="filename">Nombre de archivo: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]
```

[La documentación de Cargo](https://doc.rust-lang.org/cargo/) describe otros
metadatos que puedes especificar para asegurarte de que otros puedan descubrir
y usar tu crate más fácilmente.

### Publicando en Crates.io

Ahora que has creado una cuenta, guardado tu token de API, elegido un nombre
para tu crate y especificado los metadatos requeridos, ¡estás listo para
publicar! Publicar un crate carga una versión específica en
[crates.io](https://crates.io/)<!-- ignore --> para que otros la usen.

Ten cuidado, porque una publicación es _permanente_. La versión nunca se puede
sobrescribir y el código no se puede eliminar. Uno de los principales objetivos
de [crates.io](https://crates.io/)<!-- ignore --> es actuar como un archivo
permanente de código para que las compilaciones de todos los proyectos que
dependen de crates de [crates.io](https://crates.io/)<!-- ignore --> sigan
funcionando. Permitir la eliminación de versiones haría imposible cumplir ese
objetivo. Sin embargo, no hay límite en la cantidad de versiones de crate que
puedes publicar.

Ejecuta el comando `cargo publish` otra vez. Esta vez, debería tener éxito:

<!-- manual-regeneration
go to some valid crate, publish a new version
cargo publish
copy just the relevant lines below
-->

```console
$ cargo publish
    Updating crates.io index
   Packaging guessing_game v0.1.0 (file:///projects/guessing_game)
   Verifying guessing_game v0.1.0 (file:///projects/guessing_game)
   Compiling guessing_game v0.1.0
(file:///projects/guessing_game/target/package/guessing_game-0.1.0)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
   Uploading guessing_game v0.1.0 (file:///projects/guessing_game)
```

¡Felicidades! Ahora has compartido tu código con la comunidad de Rust y
cualquiera puede agregar tu crate como una dependencia de su proyecto.

### Publicando una Nueva Versión de un Crate Existente

Cuando hayas realizado cambios en tu crate y estés listo para publicar una
nueva versión, cambia el valor `version` especificado en tu archivo
_Cargo.toml_ y vuelve a publicar. Usa las
[reglas de versionado semántico][semver] para decidir cuál es el siguiente
número de versión apropiado en función de los tipos de cambios que hayas
realizado. Luego, ejecuta `cargo publish` para cargar la nueva versión.

<!-- Old link, do not remove -->

<a id="removing-versions-from-cratesio-with-cargo-yank"></a>

### Deprecando Versiones de Crates.io con `cargo yank`

Aunque no puedes eliminar versiones anteriores de un crate, puedes evitar que
cualquier proyecto futuro las agregue como una nueva dependencia. Esto es útil
cuando una versión de crate está rota por una razón u otra. En tales
situaciones, Cargo admite _yanking_ una versión de crate.

Hacer un _yank_ a una versión impide que nuevos proyectos dependan de esa
versión, pero permite que todos los proyectos existentes que dependen de ella
continúen. Esencialmente, un _yank_ significa que todos los proyectos con un
_Cargo.lock_ no se romperán y que cualquier _Cargo.lock_ futuro generado no
usará la versión _yanked_.

Para hacer un _yank_ de una versión de un crate, en el directorio del crate que
has publicado previamente, ejecuta `cargo yank` y especifica qué versión
deseas _yank_. Por ejemplo, si hemos publicado un crate llamado
`guessing_game` versión 1.0.1 y queremos _yank_ la versión, en el directorio
del proyecto para `guessing_game` ejecutaríamos:

<!-- manual-regeneration:
cargo yank carol-test --version 2.1.0
cargo yank carol-test --version 2.1.0 --undo
-->

```console
$ cargo yank --vers 1.0.1
    Updating crates.io index
        Yank guessing_game@1.0.1
```

Al agregar `--undo` al comando, también puedes deshacer un _yank_ y permitir
que los proyectos vuelvan a depender de una versión:

```console
$ cargo yank --vers 1.0.1 --undo
    Updating crates.io index
      Unyank guessing_game@1.0.1
```

Un _yank_ no borra ningún código. No puede, por ejemplo, eliminar secretos
cargados accidentalmente. Si eso sucede, debes restablecer esos secretos
inmediatamente.

[spdx]: http://spdx.org/licenses/
[semver]: http://semver.org/
