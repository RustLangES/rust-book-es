## Apéndice A: Palabras clave

La siguiente lista contiene palabras clave que están reservadas para el uso
actual o futuro del lenguaje Rust. Por lo tanto, no se pueden usar como
identificadores (excepto como identificadores brutos como discutiremos en la
sección “[Identificadores brutos][raw-identifiers]<!-- ignore -->”). Los
identificadores son nombres de funciones, variables, parámetros, campos de
estructuras, módulos, cajas, constantes, macros, valores estáticos, atributos,
tipos, rasgos o lifetimes.

[raw-identifiers]: #identificadores-brutos

### Palabras clave actuales en uso
La siguiente lista contiene las palabras clave actuales en uso, con su
funcionalidad descrita.

* `as` - realiza una conversión primitiva, elimina la ambigüedad del *trait*
  específico que contiene
  un elemento, o cambiar el nombre de los elementos en las declaraciones `use` y 
  `extern crate`
* `async` -  retornar un `Future` en lugar de bloquear el hilo actual
* `await` - suspender la ejecución hasta que el resultado de un `Future` esté 
  listo
* `break` - salir de un bucle inmediatamente
* `const` - define elementos constantes o punteros crudos constantes
* `continue` - continuar con la siguiente iteración del bucle
* `crate` - en un camino de módulo, se refiere a la raíz del módulo
* `dyn` - despacho dinámico a un objeto de rasgo
* `else` - alternativa para las construcciones de flujo de control `if` y 
  `if let`
* `enum` - define una enumeración
* `extern` - enlaza una función o variable externa
* `false` - literal booleano falso
* `fn` - define una función o el tipo de puntero de función
* `for` - bucle sobre elementos de un iterador, implementa un rasgo, o 
  especifica una vida más alta
* `if` - ramificación basada en el resultado de una expresión condicional
* `impl` - implementa funcionalidad propia o de rasgo
* `in` - parte de la sintaxis del bucle `for`
* `let` - vincula una variable
* `loop` - bucle sin condición
* `match` - combina un valor con patrones
* `mod` - define un módulo
* `move` - hace que una función clausura tome posesión de todos sus capturas
* `mut` - denota mutabilidad en referencias, punteros crudos o vinculaciones de 
  patrones
* `pub` - denota visibilidad pública en campos de estructuras, bloques `impl` o 
  módulos
* `ref` - vincula por referencia
* `return` - retorna de una función
* `Self` - un alias de tipo para el tipo que estamos definiendo o implementando
* `self` - sujeto de método o módulo actual
* `static` - variable global o duración de vida que dura toda la ejecución del 
  programa
* `struct` - define una estructura
* `super` - módulo padre del módulo actual
* `trait` - define un rasgo
* `true` - literal booleano verdadero
* `type` - define un alias de tipo o tipo asociado
* `union` - define una [unión][union]<!-- ignore -->; solo es una palabra clave 
  cuando se usa en una declaración de unión
* `unsafe` - denota código, funciones, rasgos o implementaciones inseguras
* `use` - importa símbolos en el ámbito
* `where` - denota cláusulas que restringen un tipo
* `while` - bucle condicionalmente basado en el resultado de una expresión

[union]: ../reference/items/unions.html

### Palabras clave reservadas para uso futuro

Las siguientes palabras clave no tienen aún ninguna funcionalidad, pero están
reservadas por Rust para un uso potencial en el futuro.

* `abstract`
* `become`
* `box`
* `do`
* `final`
* `macro`
* `override`
* `priv`
* `try`
* `typeof`
* `unsized`
* `virtual`
* `yield`

### Identificadores brutos

*Identificadores brutos* son la sintaxis que le permite usar palabras clave
donde normalmente no se permitirían. Usted usa un identificador bruto
prefijando una palabra clave con `r#`.

Por ejemplo, `match` es una palabra clave. Si intenta compilar la siguiente
función que usa `match` como su nombre:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore,does_not_compile
fn match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}
```

obtendrá este error:

```text
error: expected identifier, found keyword `match`
 --> src/main.rs:4:4
  |
4 | fn match(needle: &str, haystack: &str) -> bool {
  |    ^^^^^ expected identifier, found keyword
```

El error muestra que no se puede usar la palabra clave `match` como
identificador de función. Para usar `match` como nombre de función, necesita
usar la sintaxis de identificador bruto, como esta:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert!(r#match("foo", "foobar"));
}
```
Este código compilará sin errores. Note el prefijo `r#` en el nombre de la
función en su definición, así como donde se llama la función en `main`.

Los identificadores brutos permiten usar cualquier palabra como identificador,
incluso si esa palabra es una palabra clave. Esto nos da más libertad para
elegir nombres de identificadores, así como nos permite integrarnos con
programas escritos en un lenguaje donde estas palabras no son palabras clave.
Además, los identificadores brutos nos permiten usar bibliotecas escritas en
una edición de Rust diferente a la de su crate. Por ejemplo, `try` no es una
palabra clave en la edición 2015, pero lo es en la edición 2018. Si depende de
una biblioteca que está escrita usando la edición 2015 y tiene una función
`try`, necesitará usar la sintaxis de identificador bruto para llamar a esa
función desde su código de la edición 2018. Vea [Apéndice E][appendix-e]<!--
ignore --> para obtener más información sobre las ediciones.

[appendix-e]: appendix-05-editions.html
