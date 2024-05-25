## Apéndice G - Cómo se hace Rust y “Rust Nightly”

Este apéndice trata sobre cómo se hace Rust y cómo eso te afecta como
desarrollador de Rust.

### Estabilidad sin estancamiento

Como lenguaje, Rust se preocupa _mucho_ por la estabilidad de tu código.
Queremos que Rust sea una base sólida sobre la que puedas construir, y si las
cosas cambian constantemente, eso sería imposible. Al mismo tiempo, si no
podemos experimentar con nuevas características, es posible que no descubramos
fallos importantes hasta después de su lanzamiento, cuando ya no podamos
cambiar las cosas.

Nuestra solución a este problema es lo que llamamos “estabilidad sin
estancamiento”, y nuestro principio rector es el siguiente: nunca debes temer
actualizar a una nueva versión de Rust estable. Cada actualización debe ser
indolora, pero también debe traerte nuevas características, menos errores y
tiempos de compilación más rápidos.

### ¡Choo, Choo! Canales de lanzamiento y montando los trenes

El desarrollo de Rust funciona con un _horario de trenes_. Es decir, todo el
desarrollo se hace en la rama `master` del repositorio de Rust. Las versiones
siguen un modelo de tren de lanzamiento de software, que ha sido utilizado por
Cisco IOS y otros proyectos de software. Hay tres _canales de lanzamiento_ para
Rust:

- Nightly
- Beta
- Stable

La mayoría de los desarrolladores de Rust utilizan principalmente el canal
estable, pero aquellos que quieran probar nuevas características experimentales
pueden utilizar nightly o beta.

Aquí hay un ejemplo de cómo funciona el proceso de desarrollo y lanzamiento:
supongamos que el equipo de Rust está trabajando en el lanzamiento de Rust 1.5.
Ese lanzamiento ocurrió en diciembre de 2015, pero nos proporcionará números de
versión realistas. Se añade una nueva característica a Rust: un nuevo commit
aterriza en la rama `master`. Cada noche, se produce una nueva versión nightly
de Rust. Cada día es un día de lanzamiento, y estos lanzamientos son creados
por nuestra infraestructura de lanzamiento automáticamente. Así que a medida
que pasa el tiempo, nuestros lanzamientos se ven así, una vez por noche:

```text
nightly: * - - * - - *
```

Cada seis semanas, es hora de preparar un nuevo lanzamiento! La rama `beta` del
repositorio de Rust se ramifica de la rama `master` utilizada por nightly.
Ahora, hay dos lanzamientos:

```text
nightly: * - - * - - *
                     |
beta:                *
```

La mayoría de los usuarios de Rust no utilizan las versiones beta activamente,
pero prueban contra beta en su sistema CI para ayudar a Rust a descubrir
posibles regresiones. Mientras tanto, hay un lanzamiento nightly cada
noche:

```text
nightly: * - - * - - * - - * - - *
                     |
beta:                *
```

Digamos que se encuentra una regresión. ¡Qué bueno que tuvimos algo de tiempo
para probar la versión beta antes de que la regresión se colara en una versión
estable! La solución se aplica a `master`, de modo que nightly se arregla, y
luego la solución se vuelve a aplicar a la rama `beta`, y se produce una nueva
versión de beta:

```text
nightly: * - - * - - * - - * - - * - - *
                     |
beta:                * - - - - - - - - *
```

Seis semanas después de que se creó la primera beta, ¡es hora de un lanzamiento
estable! La rama `stable` se produce a partir de la rama `beta`:

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |
beta:                * - - - - - - - - *
                                       |
stable:                                *
```

¡Hurra! ¡Rust 1.5 está listo! Sin embargo, nos hemos olvidado de una cosa:
porque han pasado las seis semanas, también necesitamos una nueva beta de la
_siguiente_ versión de Rust, 1.6. Así que después de que `stable` se ramifica de
`beta`, la siguiente versión de `beta` se ramifica de `nightly` de nuevo:

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |                         |
beta:                * - - - - - - - - *       *
                                       |
stable:                                *
```

Esto se llama el “modelo de tren” porque cada seis semanas, un lanzamiento
“sale de la estación”, pero aún tiene que hacer un viaje a través del canal
beta antes de llegar como un lanzamiento estable.

Rust se lanza cada seis semanas, como un reloj. Si conoces la fecha de un
lanzamiento de Rust, puedes conocer la fecha del siguiente: es seis semanas
después. Un aspecto agradable de tener lanzamientos programados cada seis
semanas es que el próximo tren está llegando pronto. Si una característica
llega a perder un lanzamiento en particular, no hay necesidad de preocuparse:
¡otro está sucediendo en un corto tiempo! Esto ayuda a reducir la presión para
colar posiblemente características poco pulidas cerca de la fecha límite de
lanzamiento.

Gracias a este proceso, siempre puedes comprobar la siguiente compilación de
Rust y verificar por ti mismo que es fácil de actualizar: si una versión beta
no funciona como se esperaba, puedes informarlo al equipo y solucionarlo antes
de que ocurra el siguiente lanzamiento estable! La rotura en una versión beta
es relativamente rara, pero `rustc` sigue siendo un software, y los errores
existen.

### Tiempo de Mantenimiento

El proyecto Rust admite la versión estable más reciente. Cuando una nueva 
versión estable es lanzada, la versión anterior llega al final de su vida útil 
(EOL). Esto significa que cada versión tiene soporte durante seis semanas.

## Características inestables

Hay alo más con este modelo de lanzamiento: características inestables.
Rust utiliza una técnica llamada “indicadores de características” para
determinar qué características están habilitadas en un lanzamiento dado. Si una
nueva característica está en desarrollo activo, aterriza en `master`, y por lo
tanto, en nightly, pero detrás de un _indicador de característica_. Si, como
usuario, desea probar la característica en progreso, puede hacerlo, pero debe
estar utilizando una versión nightly de Rust y anotar su código fuente con el
indicador apropiado para optar por ello.

Si está utilizando una versión beta o estable de Rust, no puede utilizar
indicadores de características. Esta es la clave que nos permite obtener un uso
práctico con nuevas características antes de declararlas estables para siempre.
Aquellos que deseen optar por el borde sangrante pueden hacerlo, y aquellos que
deseen una experiencia sólida pueden quedarse con estable y saber que su código
no se romperá. Estabilidad sin estancamiento.

Este libro sólo contiene información sobre características estables, ya que las
características en progreso aún están cambiando, y seguramente serán diferentes
entre cuando se escribió este libro y cuando se habiliten en compilaciones
estables. Puede encontrar documentación para características sólo nocturnas en
línea.

## Rustup y el papel de Rust Nightly

Rustup facilita el cambio entre diferentes canales de lanzamiento de Rust, a
nivel global o por proyecto. Por defecto, tendrá instalado Rust estable. Para
instalar nightly, por ejemplo:

```console
$ rustup toolchain install nightly
```

También puede ver todas las _herramientas_ (versiones de Rust y componentes
asociados) que tiene instaladas con `rustup`. Aquí hay un ejemplo en uno de los
autores de Windows:

```powershell
> rustup toolchain list
stable-x86_64-pc-windows-msvc (default)
beta-x86_64-pc-windows-msvc
nightly-x86_64-pc-windows-msvc
```

Como puede ver, la herramienta estable es la predeterminada. La mayoría de los
usuarios de Rust utilizan estable la mayor parte del tiempo. Es posible que
desee utilizar estable la mayor parte del tiempo, pero utilizar nightly en un
proyecto específico, porque le importa una característica de vanguardia. Para
hacerlo, puede utilizar `rustup override` en el directorio del proyecto para
establecer la herramienta nightly como la que `rustup` debe utilizar cuando
esté en ese directorio:

```console
$ cd ~/projects/needs-nightly
$ rustup override set nightly
```

Ahora, cada vez que llame a `rustc` o `cargo` dentro de
_~/projects/needs-nightly_, `rustup` se asegurará de que esté utilizando Rust
nocturno, en lugar de su estable predeterminado. ¡Esto es útil cuando tienes
muchos proyectos de Rust!

## El proceso RFC y los equipos

Entonces, ¿cómo se aprende sobre estas nuevas características? El modelo de
desarrollo de Rust sigue un proceso de _Solicitud de comentarios (RFC)_. Si
desea una mejora en Rust, puede escribir una propuesta, llamada RFC.

Cualquiera puede escribir RFC para mejorar Rust, y las propuestas son revisadas
y discutidas por el equipo de Rust, que está compuesto por muchos subequipos de
temas. Hay una lista completa de los equipos [en el sitio web de
Rust](https://www.rust-lang.org/governance), que incluye equipos para cada área
del proyecto: diseño de lenguaje, implementación de compilador, infraestructura,
documentación y más. El equipo apropiado lee la propuesta y los comentarios,
escribe algunos comentarios propios y, finalmente, hay consenso para aceptar o
rechazar la característica.

Si la característica es aceptada, se abre un problema en el repositorio de
Rust, y alguien puede implementarla. ¡La persona que lo implementa muy bien no
tiene por qué ser la persona que propuso la característica en primer lugar!
Cuando la implementación está lista, aterriza en la rama `master` detrás de una
puerta de características, como discutimos en la sección [“Característicascaracterísticas-inestables
inestables”](#caracteristicas-inestables)<!-- ignore -->.

Después de algún tiempo, una vez que los desarrolladores de Rust que utilizan
las versiones nightly han podido probar la nueva característica, los
miembros del equipo discutirán la característica, cómo ha funcionado en
nightly, y decidirán si debe o no hacerlo en Rust estable. Si la decisión es
seguir adelante, la puerta de la característica se elimina, ¡y la característica
ahora se considera estable! Monta los trenes en una nueva versión estable de
Rust.
