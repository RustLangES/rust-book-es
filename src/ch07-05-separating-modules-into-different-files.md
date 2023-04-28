## Separando Módulos en Diferentes Archivos

Hasta ahora, todos los ejemplos en este capítulo definieron múltiples módulos
en un solo archivo. Cuando los módulos se vuelven grandes, es posible que
desees mover sus definiciones a un archivo separado para que el código sea más
fácil de navegar.

Por ejemplo, comencemos desde el código en el Listado 7-17 que tenía múltiples
módulos de restaurantes. Extraeremos los módulos en archivos en lugar de tener
todos los módulos definidos en el archivo raíz de la caja. En este caso, el
archivo raíz de la caja es *src/lib.rs*, pero este procedimiento también
funciona con cajas binarias cuyo archivo raíz de la caja es *src/main.rs*.

Primero, extraeremos el módulo `front_of_house` a su propio archivo. Elimina el
código dentro de las llaves para el módulo `front_of_house`, dejando solo la
declaración `mod front_of_house;`, de modo que *src/lib.rs* contenga el código
que se muestra en el Listado 7-21. Ten en cuenta que esto no se compilará hasta
que creemos el archivo *src/front_of_house.rs* en el Listado 7-22.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/lib.rs}}
```

<span class="caption">Listing 7-21: Declarando el módulo `front_of_house` cuyo
cuerpo estará en *src/front_of_house.rs*</span>

Luego, coloca el código que estaba entre las llaves en un nuevo archivo
llamado *src/front_of_house.rs*, como se muestra en el Listado 7-22. El
compilador sabe que debe buscar en este archivo porque encontró la declaración
del módulo en la raíz de la caja con el nombre `front_of_house`.

<span class="filename">Filename: src/front_of_house.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/front_of_house.rs}}
```

<span class="caption">Listing 7-22: Definiciones dentro del módulo `front_of_house`
en *src/front_of_house.rs*</span>

Ten en cuenta que solo necesitas cargar un archivo usando una declaración `mod`
*una vez* en tu árbol de módulos. Una vez que el compilador sabe que el archivo
es parte del proyecto (y sabe dónde en el árbol de módulos reside el código
debido a dónde colocaste la declaración `mod`), otros archivos en tu proyecto
deben hacer referencia al código del archivo cargado usando una ruta a donde se
declaró, como se cubre en la sección [“Rutas para referirse a un elemento en el
árbol de módulos”][paths]<!-- ignore -->. En otras palabras, `mod` no es una
operación de “incluir” que puede haber visto en otros lenguajes de
programación.

Luego, extraeremos el módulo `hosting` a su propio archivo. El proceso es un
poco diferente porque `hosting` es un módulo secundario de `front_of_house`, no
del módulo raíz. Colocaremos el archivo para `hosting` en un nuevo directorio
que se llamará por sus antepasados en el árbol de módulos, en este caso
*src/front_of_house/*.

Para comenzar a mover `hosting`, cambiamos *src/front_of_house.rs* para
contener solo la declaración del módulo `hosting`:

<span class="filename">Filename: src/front_of_house.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house.rs}}
```
Luego creamos un directorio *src/front_of_house* y un archivo *hosting.rs* para
contener las definiciones realizadas en el módulo `hosting`:

<span class="filename">Filename: src/front_of_house/hosting.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house/hosting.rs}}
```

Sí, en cambio, colocamos *hosting.rs* en el directorio *src*, el compilador
esperaría que el código de *hosting.rs* estuviera en un módulo `hosting`
declarado en la raíz de la caja, y no declarado como un hijo del módulo
`front_of_house`. Las reglas del compilador para qué archivos verificar para
el código de qué módulos significa que los directorios y archivos se ajustan
más al árbol de módulos.

> ### Paths de Archivos Alternativas
>
> Hasta ahora hemos cubierto los paths de archivos más idiomáticos que el
> compilador Rust usa, pero Rust también admite un estilo más antiguo de path de
> archivo. Para un módulo llamado `front_of_house` declarado en la raíz de la
> caja, el compilador buscará el código del módulo en:
>
> * *src/front_of_house.rs* (what we covered)
> * *src/front_of_house/mod.rs* (older style, still supported path)
>
> Para un módulo llamado `hosting` que es un submódulo de `front_of_house`, el
> compilador buscará el código del módulo en:
>
> * *src/front_of_house/hosting.rs* (what we covered)
> * *src/front_of_house/hosting/mod.rs* (older style, still supported path)
>
> Si usas ambos estilos para el mismo módulo, obtendrás un error del
> compilador. Usar una mezcla de ambos estilos para diferentes módulos en el
> mismo proyecto está permitido, pero podría ser confuso para las personas que
> navegan por tu proyecto.
>
> La principal desventaja del estilo que usa archivos llamados *mod.rs* es que
> tu proyecto puede terminar con muchos archivos llamados *mod.rs*, lo que puede
> ser confuso cuando los tienes abiertos en tu editor al mismo tiempo.

Hemos movido el código de cada módulo a un archivo separado, y el árbol de
módulos permanece igual. Las llamadas de función en `eat_at_restaurant`
funcionarán sin ninguna modificación, incluso si las definiciones viven en
archivos diferentes. Esta técnica le permite mover módulos a nuevos archivos a
medida que crecen en tamaño.

Ten en cuenta que la declaración `pub use crate::front_of_house::hosting` en
*src/lib.rs* tampoco ha cambiado, ni `use` tiene ningún impacto en qué archivos
se compilan como parte de la caja. La palabra clave `mod` declara módulos, y
Rust busca en un archivo con el mismo nombre que el módulo para el código que
va en ese módulo.

## Resumen

Rust te permite dividir un paquete en múltiples crates y un crate en módulos
para que puedas referirte a elementos definidos en un módulo desde otro módulo.
Puedes hacer esto especificando paths absolutas o relativas. Estas paths se
pueden traer al alcance con una declaración `use` para que puedas usar una ruta
más corta para múltiples usos del elemento en ese alcance. El código del módulo
es privado de forma predeterminada, pero puedes hacer que las definiciones sean
públicas agregando la palabra clave `pub`.

En el siguiente capítulo, veremos algunas estructuras de datos de colección en
la biblioteca estándar que puedes usar en tu código bien organizado.

[paths]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
