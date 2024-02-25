## Organización de los Tests

Como se mencionó al comienzo del capítulo, el testing es una disciplina, y
diferentes personas usan diferentes terminologías y organización. La comunidad
de Rust piensa en los tests en términos de dos categorías principales: tests de
unidad e integración. Los _tests de unidad_ son pequeños y más enfocados,
probando un módulo a la vez en aislamiento, y pueden probar interfaces privadas.
Los _tests de integración_ son completamente externos a tu biblioteca y usan tu
código de la misma manera que cualquier otro código externo, usando solo la
interfaz pública y potencialmente ejercitando múltiples módulos por test.

Escribir ambos tipos de tests es importante para asegurar que las piezas de tu
biblioteca están haciendo lo que esperas, separada y conjuntamente.

### Tests Unitarios

El propósito de los tests unitarios es probar cada unidad de código en
aislamiento del resto del código para rápidamente identificar donde el código
está y no está funcionando como se espera. Pondrás los tests unitarios en el
directorio _src_ en cada archivo con el código que están testeando. La
convención es crear un módulo llamado `tests` en cada archivo para contener las
funciones de test y anotar el módulo con `cfg(test)`.

#### El módulo de tests y `#[cfg(test)]`

La anotación `#[cfg(test)]` en el módulo de tests le dice a Rust que compile y
ejecute el código de test solo cuando ejecutas `cargo test`, no cuando ejecutas
`cargo build`. Esto ahorra tiempo de compilación cuando solo quieres compilar
la biblioteca y ahorra espacio en el resultado compilado porque los tests no
están incluidos. Verás que porque los tests de integración van en un directorio
diferente, no necesitan la anotación `#[cfg(test)]`. Sin embargo, porque los
tests unitarios van en los mismos archivos que el código, usarás `#[cfg(test)]`
para especificar que no deberían ser incluidos en el resultado compilado.

Recuerda que cuando generamos el nuevo proyecto `adder` en la primera sección
de este capítulo, Cargo generó este código para nosotros:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

Este código es el módulo de tests generado automáticamente. El atributo `cfg`
significa _configuración_ y le dice a Rust que el siguiente item debería ser
incluido solo si una cierta opción de configuración está presente. En este
caso, la opción de configuración es `test`, la cual es provista por Rust para
compilar y ejecutar tests. Al usar el atributo `cfg`, Cargo compila nuestro
código de test solo si activamente ejecutamos los tests con `cargo test`. Esto
incluye cualquier función auxiliar que pueda estar dentro de este módulo, en
adición a las funciones anotadas con `#[test]`.

#### Testeando Funciones Privadas

Hay debate dentro de la comunidad de testing sobre si las funciones privadas
deberían ser testeables directamente, y otros lenguajes hacen difícil o
imposible testear funciones privadas. Independientemente de la ideología de
testing a la que te adhieras, las reglas de privacidad de Rust te permiten
testear funciones privadas. Considera el código en el Listado 11-12 con la
función privada `internal_adder`.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-12/src/lib.rs}}
```

<span class="caption">Listing 11-12: Testeando una función privada</span>

Nota que la función `internal_adder` no está marcada como `pub`. Los tests son
solo código Rust, y el módulo `tests` es solo otro módulo. Como discutimos en
la sección [“Paths for Referring to an Item in the Module Tree”][paths]<!--
ignore -->, items en módulos hijos pueden usar los items en sus ancestros. En
este test, traemos todos los items del padre del módulo `test` al alcance con
`use super::*`, y entonces el test puede llamar a `internal_adder`. Si no
piensas que las funciones privadas deberían ser testeables, no hay nada en Rust
que te obligue a hacerlo.

### Tests de Integración

En Rust, los tests de integración son completamente externos a tu biblioteca.
Usan tu biblioteca de la misma manera que cualquier otro código externo, lo
cual significa que solo pueden llamar a funciones que son parte de la API
pública. Su propósito es probar si muchas partes de tu biblioteca funcionan
correctamente juntas. Unidades de código que funcionan correctamente por su
cuenta podrían tener problemas cuando se integran, así que la cobertura de
tests del código integrado es importante también. Para crear tests de
integración, primero necesitas un directorio _tests_.

#### El directorio _tests_

Se crea un directorio llamado tests en el nivel superior del directorio de
nuestro proyecto, al lado de _src_. Cargo sabe buscar archivos de test de
integración en este directorio. Podemos crear tantos archivos de test como
queramos en este directorio, y Cargo compilará cada archivo como un crate
individual.

Creemos un test de integración. Con el código en el Listado 11-12 aún en el
archivo _src/lib.rs_, crea un directorio _tests_ y crea un nuevo archivo
llamado _tests/integration_test.rs_. Tu estructura de directorios debería
verse así:

```text
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

Introducimos el código en el Listado 11-13 en el archivo
_tests/integration_test.rs_:

<span class="filename">Filename: tests/integration_test.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-13/tests/integration_test.rs}}
```

<span class="caption">Listing 11-13: Un test de integración de una función en el
crate `adder`</span>

Cada archivo en el directorio _tests_ es un crate separado, así que necesitamos
importar nuestra biblioteca en el scope de cada crate de test. Por esa razón,
agregamos `use adder` al inicio del código, lo cual no necesitamos en los tests
unitarios.

No es necesario anotar ningún código en _tests/integration_test.rs_ con
`#[cfg(test)]`. Cargo trata al directorio `tests` de manera especial y compila
los archivos en este directorio solo cuando ejecutamos `cargo test`. Ejecuta
`cargo test` ahora:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-13/output.txt}}
```

Las tres secciones de output incluyen los tests unitarios, el test de
integración y los tests de documentación. Nota que si algún test en una
sección falla, las siguientes secciones no serán ejecutadas. Por ejemplo, si
falla un test unitario, no habrá ningún output para los tests de integración y
de documentación porque esos tests solo serán ejecutados si todos los tests
unitarios pasan.

La primera sección es para los tests unitarios es la misma que hemos visto:
una línea para cada test unitario (uno llamado `internal` que agregamos en el
Listado 11-12) y luego una línea de resumen para los tests unitarios.

Los tests de integración comienzan con la línea
`Running tests/integration_test.rs`. Luego, hay una línea para cada función
de test en ese test de integración y una línea de resumen para los tests de
integración justo antes de que comience la sección `Doc-tests adder`.

Cada archivo de test de integración tiene su propia sección, así que si
agregamos más archivos en el directorio _tests_, habrá más secciones de tests
de integración.

Todavía podemos ejecutar una función de test de integración en particular
especificando el nombre de la función de test como argumento de `cargo test`.
Para ejecutar todos los tests en un archivo de test de integración en
particular, usa el argumento `--test` de `cargo test` seguido del nombre del
archivo:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-05-single-integration/output.txt}}
```

Este comando ejecuta solo los tests en el archivo _tests/integration_test.rs_.

#### Submódulos en Tests de Integración

En la medida en que se agregan más tests de integración, es posible que
quieras crear más archivos en el directorio tests para ayudar a organizarlas;
por ejemplo, puedes agrupar las funciones de test por la funcionalidad que
están probando. Como se mencionó anteriormente, cada archivo en el directorio
_tests_ es compilado como un crate separado, lo cual es útil para crear
scopes separados para imitar más de cerca la manera en que los usuarios finales
usarán tu crate. Sin embargo, esto significa que los archivos en el directorio
_tests_ no comparten el mismo comportamiento que los archivos en _src_, como
aprendiste en el Capítulo 7 sobre cómo separar el código en módulos y archivos.

La diferencia en el comportamiento de los archivos en _src_ y _tests_ es más
notable cuando tienes un conjunto de funciones de ayuda para usar en múltiples
archivos de test de integración y tratas de seguir los pasos en la sección

<!--ignore[“Separando Módulos en Diferentes 
Archivos”][separating-modules-into-files]--> del Capítulo 7 para extraerlas

en un módulo común. Por ejemplo, si creamos _tests/common.rs_ y colocamos una
función llamada `setup` en él, podemos agregar algo de código a `setup` que
queremos llamar desde múltiples funciones de test en múltiples archivos de
test:

<span class="filename">Filename: tests/common.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/tests/common.rs}}
```

Cuando volvemos a ejecutar los tests, veremos una sección en el output de los
tests para el archivo _common.rs_, aunque este archivo no contiene ninguna
función de test ni hemos llamada a la función `setup` desde ningún lado:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/output.txt}}
```

Tener `common` apareciendo en los resultados de los tests con `running 0 tests`
mostrado para él no es lo que queríamos. Solo queríamos compartir algo de código
con los otros archivos de test de integración.

Para evitar que `common` aparezca en el output de los tests, en lugar de crear
_tests/common.rs_, crearemos _tests/common/mod.rs_. El directorio del proyecto
ahora se ve así:

```text
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```

Esta es la convención de nomenclatura anterior que Rust también entiende y que
mencionamos en la sección [“Rutas de Archivos Alternativas”][alt-paths] del
Capítulo 7. Nombrar el archivo de esta manera le dice a Rust que no trate al
módulo `common` como un archivo de test de integración. Cuando movemos el código
de la función `setup` a _tests/common/mod.rs_ y borramos el archivo
_tests/common.rs_, la sección en el output de los tests ya no aparecerá. Los
archivos en subdirectorios del directorio _tests_ no son compilados como crates
separados ni tienen secciones en el output de los tests.

Después de haber creado _tests/common/mod.rs_, podemos usarlo desde cualquier
archivo de test de integración como un módulo. Aquí hay un ejemplo de llamar a
la función `setup` desde el test `it_adds_two` en _tests/integration_test.rs_:

<span class="filename">Filename: tests/integration_test.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-13-fix-shared-test-code-problem/tests/integration_test.rs}}
```

Nota que la declaración `mod common;` es la misma que la declaración de módulo
que demostramos en el Listado 7-21. Luego, en la función de test, podemos llamar
a la función `common::setup()`.

#### Tests de Integración para Crates Binarios

Si nuestro proyecto es un crate binario que solo contiene un archivo
_src/main.rs_ y no tiene un archivo _src/lib.rs_, no podemos crear tests de
integración en el directorio _tests_ y traer funciones definidas en el archivo
_src/main.rs_ al scope con una declaración `use`. Solo los crates de librería
exponen funciones que otros crates pueden usar; los crates binarios están
destinados a ser ejecutados por sí mismos.

Esta es una de las razones por las que los proyectos Rust que proveen un binario
tienen un archivo _src/main.rs_ que llama a la lógica que vive en el archivo
_src/lib.rs_. Usando esa estructura, los tests de integración _pueden_ probar el
crate de la librería con `use` para hacer que la funcionalidad importante esté
disponible. Si la funcionalidad importante funciona, la pequeña cantidad de
código en el archivo _src/main.rs_ también funcionará, y ese pequeño código no
necesita ser testeado.

## Resumen

Las características de testing de Rust proveen una manera de especificar cómo el
código debería funcionar para asegurarse de que continúe funcionando como
esperas, incluso mientras haces cambios. Los tests unitarios ejercitan
diferentes partes de una librería por separado y pueden testear detalles de
implementación privados. Los tests de integración chequean que muchas partes de
la librería funcionen juntas correctamente, y usan la API pública de la
librería para testear el código de la misma manera que el código externo lo
usará. Aunque el sistema de tipos y las reglas de ownership de Rust ayudan a
prevenir algunos tipos de bugs, los tests son todavía importantes para reducir
bugs de lógica que tienen que ver con cómo se espera que tu código se comporte.

¡Combinemos el conocimiento que aprendiste en este capítulo y en capítulos
anteriores para trabajar en un proyecto!

[paths]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
[separating-modules-into-files]: ch07-05-separating-modules-into-different-files.html
[alt-paths]: ch07-05-separating-modules-into-different-files.html#rutas-alternativas-de-archivos
