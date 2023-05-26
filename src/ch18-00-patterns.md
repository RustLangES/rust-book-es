# Patterns and Matching

Los *Patterns* son una sintaxis especial en Rust para hacer coincidir la
estructura de los tipos, tanto complejos como simples. El uso de patrones en
conjunción con expresiones `match` y otros constructos le brinda más control
sobre el flujo de control de un programa. Un patrón consta de alguna
combinación de los siguientes:

* Literales
* Arrays, enums, structs, o tuplas desestructuradas
* Variables
* Wildcards
* Placeholders

Algunos ejemplos de patrones incluyen `x`, `(a, 3)` y `Some(Color::Red)`. En
los contextos en los que los patrones son válidos, estos componentes describen
la forma de los datos. Nuestro programa luego compara los valores con los
patrones para determinar si tiene la forma correcta de datos para continuar
ejecutando un código en particular.

Para usar un patrón, lo comparamos con algún valor. Si el patrón coincide con el
valor, usamos las partes de valor en nuestro código. Recuerde las expresiones
`match` en el Capítulo 6 que usaron patrones, como el ejemplo de la máquina
clasificadora de monedas. Si el valor se ajusta a la forma del patrón, podemos
usar las piezas con nombre. Si no lo hace, el código asociado con el patrón no
se ejecutará.

Este capítulo es una referencia sobre todo lo relacionado con los patrones.
Cubriremos los lugares válidos para usar patrones, la diferencia entre patrones
refutables e irrefutables, y los diferentes tipos de sintaxis de patrones que
puede ver. Al final del capítulo, sabrá cómo usar patrones para expresar muchos
conceptos de una manera clara.
