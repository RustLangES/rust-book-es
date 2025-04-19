# Proyecto final: Construyendo un servidor web multithread

Ha sido un largo viaje, pero hemos llegado al final del libro. En este
capítulo, construiremos un proyecto más para demostrar algunos de los
conceptos que cubrimos en los capítulos finales, así como recapitular algunas
lecciones anteriores.

Para nuestro proyecto final, haremos un servidor web que diga "hola" y se vea
como la Figura 21-1 en un navegador web.

![hello from rust](img/trpl21-01.png)

<span class="caption">Figure 21-1: Nuestro proyecto final compartido</span>

Aquí está nuestro plan para construir el web server:

1. Aprender un poco sobre TCP y HTTP.
2. Escuchar conexiones TCP en un socket.
3. Analizar un pequeño número de peticiones HTTP.
4. Crear una respuesta HTTP adecuada.
5. Mejorar el rendimiento de nuestro servidor con un *thread pool*.

Antes de comenzar, debemos mencionar un detalle: 

Antes que nada, el método que usaremos no será la mejor manera de construir un 
servidor web con Rust. Los miembros de la comunidad han publicado una serie de 
*crates* listos para producción disponibles en [crates.io](https://crates.io/) 
que proporcionan servidores web y *thread pools* más completos que los que 
construiremos. Sin embargo, nuestra intención en este capítulo es ayudarte a 
aprender, no tomar el camino fácil. Debido a que Rust es un lenguaje de 
programación de sistemas, podemos elegir el nivel de abstracción con el que 
queremos trabajar y podemos ir a un nivel más bajo de lo que es posible o 
práctico en otros lenguajes. 

En segundo lugar, no utilizaremos `async` ni `await` aquí. ¡Construir un 
*thread pool* ya es un desafío suficientemente grande por sí solo, sin agregar 
la tarea de crear un runtime asincrónico! Sin embargo, mencionaremos cómo 
`async` y `await` podrían ser aplicables a algunos de los mismos problemas que 
veremos en este capítulo. En última instancia, como señalamos en el Capítulo 17, 
muchos *runtimes* asincrónicos utilizan *thread pools* para gestionar su 
trabajo.

Por lo tanto, escribiremos el servidor HTTP básico y el *thread pool* 
manualmente para que puedas aprender las ideas y técnicas generales detrás de 
los *crates* que podrías usar en el futuro.
