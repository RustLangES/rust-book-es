# Administrando proyectos en crecimiento con paquetes, crates y módulos

A medida que escribes programas grandes, organizar tu código se volverá cada vez más
importante. Al agrupar funcionalidades relacionadas y separar el código con
características distintas, tendrás más claro dónde encontrar el código que implementa
una característica concreta y dónde ir para cambiar el funcionamiento de una
característica.

Los programas que hemos escrito hasta ahora han estado en un módulo en un
archivo. A medida que un proyecto crece, debes organizar el código dividiéndolo
en múltiples módulos y luego en múltiples archivos. Un paquete puede contener
múltiples _crates_ binarios y opcionalmente un _crate_ de biblioteca. A medida
que un paquete crece, puedes extraer partes en _crates_ separados que se
convierten en dependencias externas. Este capítulo cubre todas estas
técnicas. Para proyectos muy grandes que comprenden un conjunto de paquetes
interrelacionados que evolucionan juntos, Cargo proporciona _workspaces_, que
cubriremos en la sección [“Cargo Workspaces”][workspaces]<!-- ignore --> en el
Capítulo 14.

También discutiremos la encapsulación de detalles de implementación, que le
permite reutilizar el código a un nivel superior: una vez que ha implementado
una operación, otro código puede llamar a su código a través de su interfaz
pública sin tener que saber cómo funciona la implementación. La forma en que
escribes el código define qué partes son públicas para que otro código las use
y qué partes son detalles de implementación privados que te reservas el
derecho de cambiar. Esta es otra forma de limitar la cantidad de detalles que
tienes que mantener en tu cabeza.

Un concepto relacionado es el ámbito: el contexto anidado en el que se
escribe el código tiene un conjunto de nombres que se definen como
"en el ámbito". Al leer, escribir y compilar código, los programadores y
compiladores necesitan saber si un nombre concreto en un punto determinado
se refiere a una variable, función, estructura, enumeración, módulo, constante
u otro elemento, y qué significa ese elemento. Se pueden crear ámbitos y cambiar
los nombres que están dentro o fuera de ellos.
No puede haber dos elementos con el mismo nombre en el mismo ámbito;
existen herramientas para resolver conflictos de nombres.

Rust tiene una serie de características que te permiten administrar la
organización de tu código, incluidos los detalles que se exponen, los detalles
que son privados y los nombres que están en cada ámbito en tus programas. Estas
características, a veces denominadas colectivamente _sistema de módulos_,
incluyen:

- **Paquetes**: Una característica de Cargo que te permite construir, probar y
  compartir _crates_
- **Crates**: Un árbol de módulos que produce una biblioteca o ejecutable
- **Módulos y use**: Te permiten controlar la organización, el ámbito y
  la privacidad de las rutas
- **Rutas**: Una forma de nombrar un elemento, como una estructura, función o
  módulo

En este capítulo, cubriremos todas estas características, discutiremos cómo
interactúan y explicaremos cómo usarlas para administrar el ámbito. Al final,
deberías tener una comprensión sólida del sistema de módulos y poder trabajar
con ámbitos como un profesional!

[workspaces]: ch14-03-cargo-workspaces.html
