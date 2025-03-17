<!-- TODO: find the right home for this (maybe nowhere!) -->

Como vimos en el capítulo anterior, los hilos proporcionan un enfoque para la concurrencia
y nos permiten resolver algunos de estos problemas. Sin embargo, también tienen ciertos
inconvenientes. En muchos sistemas operativos, los hilos requieren una cantidad
considerable de memoria por cada uno, y vienen con una sobrecarga para su inicio y
apagado. Los hilos también son solo una opción cuando tu sistema operativo y hardware
los soportan. Aunque los sistemas operativos de escritorio y móviles más comunes han
tenido soporte para hilos durante muchos años, muchos sistemas operativos embebidos,
como los que se usan en algunos microcontroladores, no lo tienen.

El modelo async ofrece un enfoque diferente, con ventajas y desventajas que, en muchos casos,
complementan las del uso de hilos. En el modelo async, las operaciones concurrentes no requieren sus
propios hilos. En lugar de eso, pueden ejecutarse en tareas (tasks). Una tarea es algo
similar a un hilo, pero en lugar de ser gestionada por el sistema operativo, es gestionada
por el código a nivel de bibliotecas.

<!--
  TODO: connective tissue as it were. Also, an open question on whether we want
  to use “task” as the primary term here. Futures do not actually *require* a
  task primitive to run (and there are runtimes which do *not* use tasks!) so it
  might make more sense to find another common, appropriate term for it instead.
-->
