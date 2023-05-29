## Comparando Performance: Bucles vs. Iteradores

Para determinar si usar loops o iterators, necesitas saber cuál implementación
es más rápida: la versión de la función `search` con un `for` loop explícito o
la versión con iterators.

Realizamos un benchmark cargando el contenido completo de *The Adventures of
Sherlock Holmes* de Sir Arthur Conan Doyle en un `String` y buscando la palabra
*the* en el contenido. Aquí están los resultados del benchmark en la versión de
`search` usando el ciclo `for` y la versión usando iterators:

```text
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```

La versión del iterator fue ligeramente más rápida! No explicaremos el código
del benchmark aquí, porque el punto no es probar que las dos versiones son
equivalentes, sino obtener una idea general de cómo estas dos implementaciones
se comparan en términos de performance.

Para un benchmark más completo, deberías verificar usando varios textos de
varios tamaños como el `contents`, diferentes palabras y palabras de diferentes
longitudes como el `query`, y todo tipo de otras variaciones. El punto es este:
los iterators, aunque son una abstracción de alto nivel, se compilan a
aproximadamente el mismo código que si hubieras escrito el código de más bajo
nivel tú mismo. Los iterators son una de las *abstracciones de costo cero* de
Rust, por lo que queremos decir que el uso de la abstracción no impone ningún
costo adicional en tiempo de ejecución. Esto es análogo a cómo Bjarne
Stroustrup, el diseñador e implementador original de C++, define *cero costo* en
“Foundations of C++” (2012):

> En general, las implementaciones de C++ obedecen el principio de cero costo:
> lo que no usas, no pagas. Y además: lo que usas, no podrías codificarlo a
> mano mejor.

Como otro ejemplo, el siguiente código es tomado de un decodificador de audio.
El algoritmo de decodificación usa la operación matemática de predicción lineal
para estimar valores futuros basados en una función lineal de las muestras
anteriores. Este código usa un string de iteradores para hacer algunos cálculos
en tres variables en el scope: un slice `buffer` de datos, un array de 12
`coefficients`, y una cantidad por la cual desplazar datos en `qlp_shift`. Hemos
declarado las variables dentro de este ejemplo, pero no les hemos dado ningún
valor; aunque este código no tiene mucho sentido fuera de su contexto, sigue
siendo un ejemplo conciso y del mundo real de cómo Rust traduce ideas de alto
nivel a código de bajo nivel.

```rust,ignore
let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
                                 .zip(&buffer[i - 12..i])
                                 .map(|(&c, &s)| c * s as i64)
                                 .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}
```

Para calcular el valor de `prediction`, este código itera a través de cada uno
de los 12 valores en `coefficients` y usa el método `zip` para emparejar los
valores de los coeficientes con los 12 valores anteriores en `buffer`. Luego,
para cada par, multiplicamos los valores juntos, sumamos todos los resultados y
desplazamos los bits en la suma `qlp_shift` bits a la derecha.

Calculaciones en aplicaciones como decodificadores de audio a menudo priorizan
el performance. Aquí, estamos creando un iterator, usando dos adaptadores, y
luego consumiendo el valor. ¿Qué código ensamblador compilaría este código Rust?
Bueno, a partir de este escrito, compila al mismo ensamblador que escribirías a
mano. No hay ningún ciclo correspondiente a la iteración sobre los valores en
`coefficients`: Rust sabe que hay 12 iteraciones, por lo que “desenrolla” el
ciclo. *Desenrollar* es una optimización que elimina el overhead del código de
control del ciclo y en su lugar genera código repetitivo para cada iteración del
ciclo.

Todos los coeficientes se almacenan en registros, lo que significa que acceder
a los valores es muy rápido. No hay verificaciones de límites en el acceso al
array en tiempo de ejecución. Todas estas optimizaciones que Rust es capaz de
aplicar hacen que el código resultante sea extremadamente eficiente. Ahora que
sabes esto, ¡puedes usar iterators y closures sin miedo! Hacen que el código
parezca de más alto nivel, pero no imponen una penalización de performance en
tiempo de ejecución por hacerlo.

## Resumen

Los closures e iterators son características de Rust inspiradas en ideas de
lenguajes de programación funcionales. Contribuyen a la capacidad de Rust de
expresar claramente ideas de alto nivel a bajo nivel de performance. Las
implementaciones de closures e iterators son tales que el performance en tiempo
de ejecución no se ve afectado. Esto es parte de la meta de Rust de esforzarse
por proveer abstracciones de costo cero.

Ahora que mejoramos la expresividad de nuestro proyecto I/O, veamos algunas
características más de `cargo` que nos ayudarán a compartir el proyecto con el
mundo.
