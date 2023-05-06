# Escribiendo Tests automatizados

En su ensayo de 1972 "El programador humilde", Edsger W. Dijkstra dijo que
"Los tests de programas pueden ser una forma muy efectiva de mostrar la
presencia de errores, pero es inútil para mostrar su ausencia". Eso no significa
que no debamos intentar probar tanto como podamos!

La corrección en nuestros programas es el grado en que nuestro código hace lo
que pretendemos que haga. Rust está diseñado con un alto grado de preocupación
por la corrección de los programas, pero la corrección es compleja y no es
fácil de probar. El sistema de tipos de Rust soporta una gran parte de esta
carga, pero el sistema de tipos no puede atrapar todo. Como tal, Rust incluye
soporte para escribir tests de software automatizados.

Digamos que escribimos una función `add_two` que suma 2 a cualquier número que
se le pase. La firma de esta función acepta un entero como parámetro y devuelve
un entero como resultado. Cuando implementamos y compilamos esa función, Rust
hace toda la comprobación de tipos y de préstamos que has aprendido hasta ahora
para asegurarse de que, por ejemplo, no estamos pasando un valor `String` o una
referencia no válida a esta función. Pero Rust *no puede* comprobar que esta
función haga precisamente lo que pretendemos, que es devolver el parámetro más
2 en lugar de, por ejemplo, el parámetro más 10 o el parámetro menos 50! Ahí es
donde entran los tests.

Podemos escribir tests que afirmen, por ejemplo, que cuando pasamos `3` a la
función `add_two`, el valor devuelto es `5`. Podemos ejecutar estos tests
siempre que hagamos cambios en nuestro código para asegurarnos de que cualquier
comportamiento correcto existente no haya cambiado.

El Testing es una habilidad compleja: aunque no podemos cubrir todos los
detalles sobre cómo escribir buenos tests en un capítulo, discutiremos los
mecanismos de las instalaciones de testing de Rust. Hablaremos sobre las
anotaciones y macros disponibles para ti cuando escribas tus tests, el
comportamiento predeterminado y las opciones proporcionadas para ejecutar tus
tests, y cómo organizar los tests en tests unitarios y tests de integración.
