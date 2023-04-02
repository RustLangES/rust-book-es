## Appendix B: Operators and Symbols
## Apéndice B: Operadores y símbolos

Este apéndice contiene una lista de los operadores y símbolos que aparecen en
Rust, incluyendo los operadores y otros símbolos que aparecen por sí mismos o en
el contexto de rutas, genéricos, límites de trait, macros, atributos, comentarios,
tuplas y corchetes.

### Operadores
La tabla B-1 contiene los operadores en Rust, un ejemplo de cómo aparecería el
operador en contexto, una breve explicación y si ese operador es
sobrecargable. Si un operador es sobrecargable, se lista el rasgo relevante para
sobrecargar ese operador.

<span class="caption">Tabla B-1: Operadores</span>

| Operador | Ejemplo | Explicación | Sobrecargable? |
|----------|---------|-------------|---------------|
| `!` | `ident!(...)`, `ident!{...}`, `ident![...]` | Expansor de Macros | |
| `!` | `!expr` | Operador bit a bit o complemento lógico | `Not` |
| `!=` | `expr != expr` | Comparador de No Igualdad | `PartialEq` |
| `%` | `expr % expr` | Modulo | `Rem` |
| `%=` | `var %= expr` | Modulo y asignación | `RemAssign` |
| `&` | `&expr`, `&mut expr` | Préstamo | |
| `&` | `&type`, `&mut type`, `&'a type`, `&'a mut type` | Préstamo del puntero del tipo | |
| `&` | `expr & expr` | Operador bit a bit AND | `BitAnd` |
| `&=` | `var &= expr` | Operador bit a bit AND y asignación | `BitAndAssign` |
| `&&` | `expr && expr` | Operador lógico AND | |
| `*` | `expr * expr` | Multiplicación | `Mul` |
| `*=` | `var *= expr` | Multiplicación y asignación | `MulAssign` |
| `*` | `*expr` | Direferencia | `Deref` |
| `*` | `*const type`, `*mut type` | Puntero | |
<!-- TODO: | `+` | `trait + trait`, `'a + trait` | Compound type constraint | | -->
| `+` | `expr + expr` | Aritmético adición | `Add` |
| `+=` | `var += expr` | Adición y asignación | `AddAssign` |
| `,` | `expr, expr` | Separador de argumentos y elementos | |
| `-` | `- expr` | Aritmético de Negación | `Neg` |
| `-` | `expr - expr` | Aritmético de sustracción  | `Sub` |
| `-=` | `var -= expr` | Aritmético de sustracción y asignación | `SubAssign` |
| `->` | `fn(...) -> type`, <code>&vert;...&vert; -> type</code> | Tipo de retorno en funciones y clausuras | |
| `.` | `expr.ident` | Acceso a miembro | |
| `..` | `..`, `expr..`, `..expr`, `expr..expr` | Rango exclusivo a la derecha | `PartialOrd` |
| `..=` | `..=expr`, `expr..=expr` | Rango inclusivo a la derecha | `PartialOrd` |
| `..` | `..expr` | Sintaxis de actualización de estructuras | |
| `..` | `variant(x, ..)`, `struct_type { x, .. }` | Patrón “y el resto” | |
| `...` | `expr...expr` | (Obsoleto, use `..=` en su lugar) En un patrón: Patrón de rango inclusivo | |
| `/` | `expr / expr` | División aritmética | `Div` |
| `/=` | `var /= expr` | División aritmética y asignación | `DivAssign` |
| `:` | `pat: type`, `ident: type` | Restricciones | |
| `:` | `ident: expr` | Inicializador de campo de estructura | |
| `:` | `'a: loop {...}` | Etiqueta de bucle | |
| `;` | `expr;` | Terminador de declaración y elemento | |
| `;` | `[...; len]` | Parte de la sintaxis de matriz de tamaño fijo | |
| `<<` | `expr << expr` | Desplazamiento a la izquierda | `Shl` |
| `<<=` | `var <<= expr` | Desplazamiento a la izquierda y asignación | `ShlAssign` |
| `<` | `expr < expr` | Comparador de menor que | `PartialOrd` |
| `<=` | `expr <= expr` | Comparador de menor o igual que | `PartialOrd` |
| `=` | `var = expr`, `ident = type` | Asignación/equivalencia | |
| `==` | `expr == expr` | Comparador de igualdad | `PartialEq` |
| `=>` | `pat => expr` | Parte de la sintaxis de match | |
| `>` | `expr > expr` | Comparador de mayor que | `PartialOrd` |
| `>=` | `expr >= expr` | Comparador de mayor o igual que | `PartialOrd` |
| `>>` | `expr >> expr` | Desplazamiento a la derecha | `Shr` |
| `>>=` | `var >>= expr` | Desplazamiento a la derecha y asignación | `ShrAssign` |
| `@` | `ident @ pat` | Patrón de enlace | |
| `^` | `expr ^ expr` | Operador bit a bit XOR | `BitXor` |
| `^=` | `var ^= expr` | Operador bit a bit XOR y asignación | `BitXorAssign` |
| <code>&vert;</code> | <code>pat &vert; pat</code> | Patrón alternativo | |
| <code>&vert;</code> | <code>expr &vert; expr</code> | Operador bit a bit OR | `BitOr` |
| <code>&vert;=</code> | <code>var &vert;= expr</code> | Operador bit a bit OR y asignación | `BitOrAssign` |
| <code>&vert;&vert;</code> | <code>expr &vert;&vert; expr</code> | Operador lógico OR | |
| `?` | `expr?` | Operador de propagación de errores | |

### Simbolos no operadores

La siguiente lista contiene todos los símbolos que no funcionan como
operadores; es decir, no se comportan como una llamada de función o método.

Tabla B-2 muestra los símbolos que aparecen por sí mismos y son válidos en una
variedad de ubicaciones.

<span class="caption">Tabla B-2: Sintaxis únicas</span>

| Símbolos | Explicación |
|--------|-------------|
| `'ident` | Lifetime nombrado o etiqueta de bucle |
| `...u8`, `...i32`, `...f64`, `...usize`, etc. | Literal numérico de un tipo especifico |
| `"..."` | Literal de tipo String |
| `r"..."`, `r#"..."#`, `r##"..."##`, etc. | Literal de tipo String sin procesar |
| `b'...'` | Literal de tipo byte; construye un array de bytes en lugar de una cadena |
| `br"..."`, `br#"..."#`, `br##"..."##`, etc. | Literal de tipo String sin procesar, combinación de literal de tipo String y literal de tipo byte |
| `'...'` | Literal de tipo caracter |
| `b'...'` | Literal de tipo byte ASCII |
| <code>&vert;...&vert; expr</code> | Clausura |
| `!` | Tipo de dato vacío siempre vacío para funciones divergentes |
| `_` | “Ignored” patrón de enlace; también se usa para hacer que los literales enteros sean legibles |

Tabla B-3 muestra los símbolos que aparecen en el contexto de un camino a través
del módulo de la jerarquía para un elemento.

<span class="caption">Tabla B-3: Sintaxis relacionado a Rutas</span>

| Símbolos | Explicación |
|--------|-------------|
| `ident::ident` | Ruta de espacio de trabajo |
| `:path` | Ruta relativa a la raíz del espacio de trabajo (es decir, una ruta absoluta explícita) |
| `self::path` | Ruta relativa al módulo actual (es decir, una ruta relativa explícita).
| `super::path` | Ruta relativa al padre del módulo actual |
| `type::ident`, `<type as trait>::ident` | Constantes asociadas, funciones y tipos |
| `<type>::...` | Elemento asociado para un tipo que no se puede nombrar directamente (por ejemplo, `<&T>::...`, `<[T]>::...`, etc.) |
| `trait::method(...)` | Desambiguación de una llamada de método nombrando la interfaz que lo define |
| `type::method(...)` | Desambiguación de una llamada de método nombrando el tipo para el que está definido |
| `<type as trait>::method(...)` | Desambiguación de una llamada de método nombrando la interfaz y el tipo |

Tabla B-4 muestra los símbolos que aparecen en el contexto de usar parámetros de
tipo genérico.

<span class="caption">Tabla B-4: Genericos</span>

| Símbolos | Explicación |
|--------|-------------|
| `path<...>` | Especifica parámetros de tipo genérico en un tipo (por ejemplo, `Vec<u8>`) |
| `path::<...>`, `method::<...>` | Especifica parámetros de tipo genérico, función o método en una expresión; a menudo se refiere como pez espada (por ejemplo, `"42".parse::<i32>()`) |
| `fn ident<...> ...` | Define una función genérica |
| `struct ident<...> ...` | Define una estructura genérica |
| `enum ident<...> ...` | Define una enumeración genérica |
| `impl<...> ...` | Define una implementación genérica |
| `for<...> type` | Límites de vida de rango superior |
| `type<ident=type>` | Un tipo genérico donde uno o más tipos asociados tienen asignaciones específicas (por ejemplo, `Iterator<Item=T>`) |

Tabla B-5 muestra los símbolos que aparecen en el contexto de restringir
parámetros de tipo genérico con límites de tipo.

<span class="caption">Tabla B-5: Restricciones de tipo</span>

| Simbolos | Explicación |
|--------|-------------|
| `T: U` | Parámetro de tipo genérico `T` restringido a tipos que implementan `U` |
| `T: 'a` | Tipo genérico `T` debe sobrevivir al tiempo de vida `'a` (es decir, el tipo no puede contener de forma transitiva referencias con tiempos de vida más cortos que `'a`) |
| `T: 'static` | Tipo genérico `T` no contiene referencias prestadas, excepto las de `'static` |
| `'b: 'a` | Tiempo de vida genérico `'b` debe sobrevivir al tiempo de vida `'a` |
| `T: ?Sized` | Permitir que el parámetro de tipo genérico sea un tipo de tamaño dinámico |
| `'a + trait`, `trait + trait` | Restricción de tipo compuesta |

Tabla B-6 muestra los símbolos que aparecen en el contexto de llamar o definir
macros y especificar atributos en un elemento.

<span class="caption">Tabla B-6: Macros y Atributos</span>

| Símbolos | Explicación |
|--------|-------------|
| `#[meta]` | Atributo externo |
| `#![meta]` | Atributo interno |
| `$ident` | Sustitución de macro |
| `$ident:kind` | Captura de macro |
| `$(...)...` | Repetición de macro |
| `ident!(...)`, `ident!{...}`, `ident![...]` | Invocación de macro |

Tabla B-7 muestra los símbolos que crean comentarios.

<span class="caption">Tabla B-7: Comentarios</span>

| Símbolos | Explicación |
|--------|-------------|
| `//` | Comentario de línea |
| `//!` | Comentario de línea de documentación interna |
| `///` | Comentario de línea de documentación externa |
| `/*...*/` | Comentario de bloque |
| `/*!...*/` | Comentario de bloque de documentación interna |
| `/**...*/` | Comentario de bloque de documentación externa |

Tabla B-8 muestra los símbolos que aparecen en el contexto de usar tuplas.

<span class="caption">Tabla B-8: Tuplas</span>

| Símbolos | Explicación |
|--------|-------------|
| `()` | Tupla vacía (también conocida como unidad), tanto literal como tipo |
| `(expr)` | Expresión entre paréntesis |


Table B-8 shows symbols that appear in the context of using tuples.

<span class="caption">Table B-8: Tuples</span>

| Symbol | Explanation |
|--------|-------------|
| `()` | Empty tuple (aka unit), both literal and type |
| `(expr)` | Parenthesized expression |
| `(expr,)` | Single-element tuple expression |
| `(type,)` | Single-element tuple type |
| `(expr, ...)` | Tuple expression |
| `(type, ...)` | Tuple type |
| `expr(expr, ...)` | Function call expression; also used to initialize tuple `struct`s and tuple `enum` variants |
| `expr.0`, `expr.1`, etc. | Tuple indexing |

Table B-9 shows the contexts in which curly braces are used.

<span class="caption">Table B-9: Curly Brackets</span>

| Context | Explanation |
|---------|-------------|
| `{...}` | Block expression |
| `Type {...}` | `struct` literal |

Table B-10 shows the contexts in which square brackets are used.

<span class="caption">Table B-10: Square Brackets</span>

| Context | Explanation |
|---------|-------------|
| `[...]` | Array literal |
| `[expr; len]` | Array literal containing `len` copies of `expr` |
| `[type; len]` | Array type containing `len` instances of `type` |
| `expr[expr]` | Collection indexing. Overloadable (`Index`, `IndexMut`) |
| `expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]` | Collection indexing pretending to be collection slicing, using `Range`, `RangeFrom`, `RangeTo`, or `RangeFull` as the “index” |
