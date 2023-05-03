## Definiendo módulos para controlar el scope y la privacidad

En esta sección, hablaremos sobre módulos y otras partes del sistema de módulos,
es decir, *paths* que te permiten nombrar elementos; la palabra clave `use` que
trae un path dentro del alcance; y la palabra clave `pub` para hacer elementos
públicos. También discutiremos la palabra clave `as`, los paquetes externos y el
operador glob.

Primero, vamos a empezar con una lista de reglas para tener a mano cuando
estés organizando tu código en el futuro. Luego explicaremos cada una de las
reglas en detalle.

### Hoja de referencia de módulos

Aquí te proporcionamos una referencia rápida sobre cómo funcionan los módulos,
los paths, la palabra clave `use` y la palabra clave `pub` en el compilador, y
cómo la mayoría de los desarrolladores organizan su código. Vamos a ir
tratando ejemplos de cada una de estas reglas a lo largo de este capítulo, pero
esta es una buena referencia para tener a mano cuando necesites recordar cómo
funcionan los módulos.

- **Empezamos desde la raíz del crate**: Cuando se compila un crate, el
  compilador primero busca el código en el archivo raíz del crate (usualmente
  *src/lib.rs* para un crate de librería o *src/main.rs* para un crate
  binario) para compilar.
- **Declarando módulos**: En el archivo raíz del crate, puedes declarar nuevos
  módulos; digamos, que declaras un módulo “garden” con `mod garden;`. El
  compilador buscará el código del módulo en estos lugares:
  - En línea, dentro de llaves que reemplazan el punto y coma que sigue a `mod
    garden`
  - En el archivo *src/garden.rs*
  - En el archivo *src/garden/mod.rs*
- **Declarando submódulos**: En cualquier archivo que no sea la raíz del crate,
  puedes declarar submódulos. Por ejemplo, podrías declarar `mod vegetables;` en
  *src/garden.rs*. El compilador buscará el código del submódulo dentro del
  directorio que se llama igual que el módulo padre en estos lugares:
  - En línea, directamente después de `mod vegetables`, dentro de llaves que
    reemplazan el punto y coma que sigue a `mod garden`
  - En el archivo *src/garden/vegetables.rs*
  - En el archivo *src/garden/vegetables/mod.rs*
- **Paths a código en módulos**: Una vez que un módulo es parte de tu crate, puedes
  referirte al código de ese módulo desde cualquier otro lugar del mismo crate,
  siempre y cuando las reglas de privacidad lo permitan, usando el path al
  código. Por ejemplo, un tipo `Asparagus` en el módulo de vegetales del garden
  se encontraría en `crate::garden::vegetables::Asparagus`.
- **Privado vs público**: El código dentro de un módulo es privado por defecto
  desde los módulos padres. Para hacer un módulo público, decláralo con `pub
  mod` en vez de `mod`. Para hacer públicos los elementos dentro de un módulo
  público, usa `pub` antes de sus declaraciones.
- **La palabra clave `use`**: Dentro de un alcance, la palabra clave `use` crea
  atajos a elementos para reducir la repetición de paths largos. En cualquier
  alcance que pueda referirse a `crate::garden::vegetables::Asparagus`, puedes
  crear un atajo con `use crate::garden::vegetables::Asparagus;` y a partir de
  entonces solo necesitarás escribir `Asparagus` para hacer uso de ese tipo en
  el alcance.

Aquí crearemos un crate binario llamado `backyard` que ilustra estas reglas. El
directorio del crate, también llamado `backyard`, contiene estos archivos y
directorios:
Here we create a binary crate named `backyard` that illustrates these rules. The
crate’s directory, also named `backyard`, contains these files and directories:

```text
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

El crate raíz es *src/main.rs*, y contiene:

<span class="filename">Filename: src/main.rs</span>

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/main.rs}}
```

La línea `mod garden;` le dice al compilador que incluya el código que encuentra
en *src/garden.rs*, que es:

<span class="filename">Filename: src/garden.rs</span>

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden.rs}}
```

Aquí, `pub mod vegetables;` significa que el código en *src/garden/vegetables.rs*
también se incluye. Ese código es:

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden/vegetables.rs}}
```

¡Ahora entremos en los detalles de estas reglas y demostrémoslas en acción!

### Agrupando código relacionado en módulos

Los *módulos* nos permiten organizar el código dentro de un crate para facilitar
su lectura y reutilización. También nos permiten controlar la privacidad de los
elementos, ya que el código dentro de un módulo es privado por defecto. Los
elementos privados son detalles de la implementación interna que no están
disponibles para su uso externo. Podemos elegir hacer públicos los módulos y los
elementos que contienen para exponerlos y permitir que el código externo los
use y dependa de ellos.

Como un ejemplo, vamos a escribir una librería que provee la funcionalidad de un
restaurante. Vamos a definir las firmas de las funciones, pero dejaremos sus
cuerpos vacíos para concentrarnos en la organización del código, en vez de la
implementación de un restaurante.

En la industria de restaurantes, algunas partes de un restaurante se llaman
*front of house* y otras *back of house*. El *front of house* es donde están
los clientes; esto incluye donde los anfitriones se sientan a los clientes,
los camareros toman los pedidos y el pago, y los bartenders preparan las
bebidas. El *back of house* es donde los chefs y los cocineros trabajan en la
cocina, los lavaplatos limpian, y los gerentes hacen el trabajo administrativo.

Para estructurar nuestro crate de esta manera, podemos organizar sus funciones
dentro de módulos anidados. Crea una nueva librería llamada `restaurant` 
ejecutando `cargo new restaurant --lib`; luego ingresa el código en la 
Lista 7-1 para definir algunos módulos y firmas de funciones. Aquí está la 
sección *front of house*:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-01/src/lib.rs}}
```

<span class="caption">Listing 7-1: Un módulo `front_of_house` que contiene otros
módulos que luego contienen funciones</span>

Definimos un módulo con la palabra clave `mod` seguida del nombre del módulo
(en este caso, `front_of_house`). El cuerpo del módulo va dentro de llaves.
Dentro de los módulos, podemos colocar otros módulos, como en este caso con los
módulos `hosting` y `serving`. Los módulos también pueden contener definiciones
de otros elementos, como structs, enums, constantes, traits, y como en la Lista
7-1—funciones.

Mediante el uso de módulos, podemos agrupar definiciones relacionadas y nombrar
por qué están relacionadas. Los programadores que usen este código pueden
navegar el código basándose en los grupos en vez de tener que leer todas las
definiciones, haciendo más fácil encontrar las definiciones relevantes para
ellos. Los programadores que agreguen nueva funcionalidad a este código sabrán
dónde colocar el código para mantener el programa organizado.

Anteriormente, mencionamos que *src/main.rs* y *src/lib.rs* se llaman raíces de
crate. La razón de su nombre es que el contenido de cualquiera de estos dos
archivos forma un módulo llamado `crate` en la raíz de la estructura de módulos
del crate, conocida como el *árbol de módulos*.

El Listado 7-2 muestra el árbol de módulos para la estructura en el listado 7-1

```text
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

<span class="caption">Listing 7-2: El árbol de módulos para el código del Listing
7-1</span>

Este árbol muestra como algunos de los módulos se anidan dentro de otros; por
ejemplo, `hosting` se anida dentro de `front_of_house`. El árbol también muestra
que algunos módulos son *hermanos* entre sí, lo que significa que están
definidos en el mismo módulo; `hosting` y `serving` son hermanos definidos
dentro de `front_of_house`. Si el módulo A está contenido dentro del módulo B,
decimos que el módulo A es el *hijo* del módulo B y que el módulo B es el
*padre* del módulo A. Nota que el árbol de módulos completo está enraizado bajo
el módulo implícito llamado `crate`.

El árbol de módulos puede recordarte al árbol de directorios del sistema de
archivos en tu computadora; ¡esta es una comparación muy apropiada! Al igual que
los directorios en un sistema de archivos, usas módulos para organizar tu
código. Y al igual que los archivos en un directorio, necesitamos una forma de
encontrar nuestros módulos.
