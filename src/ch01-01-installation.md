## Instalación

El primer paso es instalar Rust. Descargaremos Rust a través de `rustup`, una
herramienta de línea de comandos para administrar las versiones de Rust y las
herramientas asociadas. Necesitarás una conexión a Internet para la descarga.
Necesitarás una conexión a Internet para la descarga.

> Nota: Si prefiere no usar `rustup` por alguna razón, consulte la página
> [Otros métodos de instalación de Rust][otherinstall] para obtener más opciones.

Los siguientes pasos instalan la última versión estable del compilador de Rust.
Las garantías de estabilidad de Rust aseguran que todos los ejemplos del libro
que se compilan seguirán compilando con versiones más nuevas de Rust. La salida
puede diferir ligeramente entre versiones porque Rust a menudo mejora los
mensajes de error y las advertencias. En otras palabras, cualquier versión más
nueva, estable de Rust que instale usando estos pasos debería funcionar como se
espera con el contenido de este libro.

> ### Notación de línea de comandos
>
> En este capítulo y en todo el libro, mostraremos algunos comandos utilizados
> en la terminal. Las líneas que debe ingresar en una terminal comienzan con `$`.
> No necesita escribir el carácter `$`; es el indicador de línea de comandos
> mostrado para indicar el comienzo de cada comando. Las líneas que no comienzan
> con `$` generalmente muestran la salida del comando anterior. Además, los
> ejemplos específicos de PowerShell usarán `>` en lugar de `$`.

### Instalación de `rustup` en Linux o macOS

Si está utilizando Linux o macOS, abra una terminal y escriba el siguiente

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

El comando descarga un script y comienza la instalación de la herramienta
`rustup`, que instala la última versión estable de Rust. Es posible que se le
solicite su contraseña. Si la instalación es exitosa, aparecerá la siguiente
línea:

```text
Rust is installed now. Great!
```

También necesitará un *enlazador*, que es un programa que Rust utiliza para
unir sus salidas compiladas en un solo archivo. Es probable que ya lo tenga. Si
obtiene errores de enlace, debe instalar un compilador C, que generalmente
incluye un enlazador. Un compilador C también es útil porque algunos paquetes
comunes de Rust dependen de código C y necesitarán un compilador C.

En macOS, puede obtener un compilador C ejecutando:

```console
$ xcode-select --install
```

Los usuarios de Linux deben instalar generalmente GCC o Clang, según la
documentación de su distribución. Por ejemplo, si usa Ubuntu, puede instalar el
paquete `build-essential`.

### Instalación de `rustup` en Windows

En Windows, vaya a [https://www.rust-lang.org/tools/install][install] y siga
las instrucciones para instalar Rust. En algún momento de la instalación,
recibirá un mensaje que explica que también necesitará las herramientas de
compilación de MSVC para Visual Studio 2013 o posterior.

Para obtener las herramientas de compilación, deberá instalar [Visual Studio
2022][visualstudio]. Cuando se le pregunte qué paquetes de trabajo instalar,
incluya:

* “Desarrollo de escritorio con C ++”
* El SDK de Windows 10 o 11
* El componente de paquete de idioma inglés, junto con cualquier otro paquete de
  idioma de su elección

El resto de este libro usa comandos que funcionan tanto en *cmd.exe* como en
PowerShell. Si hay diferencias específicas, explicaremos cuál usar.

### Solución de problemas

Para verificar si ha instalado Rust correctamente, abra una shell y escriba esta
línea:

```console
$ rustc --version
```

Debería ver el número de versión, el hash de confirmación y la fecha de
confirmación de la última versión estable que se ha publicado, en el siguiente
formato:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

Si ve esta información, ¡ha instalado Rust correctamente! Si no ve esta
información, verifique que Rust esté en su variable de sistema `%PATH%` de la
siguiente manera.

En Windows CMD, use:

```console
> echo %PATH%
```

En PowerShell, use:

```powershell
> echo $env:Path
```

En Linux y macOS, use:

```console
$ echo $PATH
```

Si todo está correcto y Rust aún no funciona, hay varios lugares donde puede
obtener ayuda. Obtenga información sobre cómo comunicarse con otros Rustaceans
(un apodo tonto que nos llamamos a nosotros mismos) en [la página de la
comunidad][community].

### Actualización y desinstalación

Una vez que Rust se instala a través de `rustup`, actualizar a una versión
recién lanzada es fácil. Desde su shell, ejecute el siguiente script de
actualización:

```console
$ rustup update
```

Para desinstalar Rust y `rustup`, ejecute el siguiente script de desinstalación
desde su shell:

```console
$ rustup self uninstall
```

### Documentación local

La instalación de Rust también incluye una copia local de la documentación para
que pueda leerla sin conexión. Ejecute `rustup doc` para abrir la documentación
local en su navegador.

Cualquier momento en que se proporcione un tipo o una función de la biblioteca
estándar y no esté seguro de lo que hace o cómo usarlo, use la documentación de
la interfaz de programación de aplicaciones (API) para averiguarlo.

[otherinstall]: https://forge.rust-lang.org/infra/other-installation-methods.html
[install]: https://www.rust-lang.org/tools/install
[visualstudio]: https://visualstudio.microsoft.com/downloads/
[community]: https://www.rust-lang.org/community
