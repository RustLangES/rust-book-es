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
| `+` | `trait + trait`, `'a + trait` | Restricción de tipo compuesta | |
| `+` | `expr + expr` | Aritmético adición | `Add` |
| `+=` | `var += expr` | Adición y asignación | `AddAssign` |
| `,` | `expr, expr` | Separador de argumentos y elementos | |
| `-` | `- expr` | Aritmético de Negación | `Neg` |
| `-` | `expr - expr` | Aritmético de sustracción  | `Sub` |
| `-=` | `var -= expr` | Aritmético de sustracción y asignación | `SubAssign` |
| `->` | `fn(...) -> type`, <code>&vert;...&vert; -> type</code> | Tipo de retorno en funciones y clausuras | |
| `.` | `expr.ident` | Acceso a atributo | |
| `.` | `expr.ident(expr, ...)` | Llamada a método | |
| `.` | `expr.0`, `expr.1`, etc. | Indexación de tuplas | |
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
| `;` | `[...; len]` | Parte de la sintaxis de arreglos de tamaño fijo | |
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
| `ident::ident` | Ruta del Namespace |
| `::path` | Ruta relativa al prelude externo, donde están enraizados todos los demás crates (es decir, una ruta explícitamente absoluta que incluye el nombre del crate) |
| `self::path` | Ruta relativa al módulo actual (es decir, una ruta explícitamente relativa). |
| `super::path` | Ruta relativa al módulo padre del módulo actual |
| `type::ident`, `<type as trait>::ident` | Constantes, funciones y tipos asociados |
| `<type>::...` | Elemento asociado a un tipo que no puede nombrarse directamente (por ejemplo, `<&T>::...`, `<[T]>::...`, etc.) |
| `trait::method(...)` | Desambiguar una llamada a un método nombrando el trait que lo define |
| `type::method(...)` | Desambiguar una llamada a un método nombrando el tipo para el cual está definido |
| `<type as trait>::method(...)` | Desambiguar una llamada a un método nombrando tanto el trait como el tipo |

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

Tabla B-8 muestra el contexto en los que paréntesis son usados.

<span class="caption">Table B-8: Paréntesis</span>

| Símbolos | Explicación |
|--------|-------------|
| `()` | Tupla vacía (también conocida como unidad), tanto literal como tipo |
| `(expr)` | Expresión entre paréntesis |
| `(expr,)` | Expresión de tupla de un solo elemento |
| `(type,)` | Tipo de tupla de un solo elemento |
| `(expr, ...)` | Expresión de tupla |
| `(type, ...)` | Tipo de tupla |
| `expr(expr, ...)` | Expresión de llamada de función; también se usa para inicializar `struct`s de tupla y variantes de `enum` de tupla |
| `expr.0`, `expr.1`, etc. | Índice de tupla |

Tabla B-9 muestra los contextos en los que se usan las llaves.

<span class="caption">Tabla B-9: Llaves</span>

| Contexto | Explicación |
|---------|-------------|
| `{...}` | Expresión de bloque |
| `Type {...}` | Literal de `struct` |

Tabla B-10 muestra los contextos en los que se usan los corchetes.

<span class="caption">Tabla B-10: Corchetes</span>

| Contexto | Explicación |
|---------|-------------|
| `[...]` | Expresión de arreglo |
| `[type; expr]` | Arreglo de tipo y tamaño |
| `expr[expr]` | Índice de colección. Sobrecargable (`Index`, `IndexMut`) |
| `expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]` | Índice de colección fingiendo ser recortes de colección, usando `Range`, `RangeFrom`, `RangeTo`, o `RangeFull` como el “índice” |

