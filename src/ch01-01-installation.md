<a id="instalacion"></a>

## Instalación

El primer paso es instalar Rust. Descargaremos Rust a través de `rustup`, una
herramienta de línea de comandos para administrar las versiones de Rust y las
herramientas asociadas. Necesitarás una conexión a Internet para la descarga.

> Nota: Si prefieres no usar `rustup` por alguna razón, consulta la página
> [Otros métodos de instalación de Rust][otherinstall] para obtener más opciones.

Los siguientes pasos instalan la última versión estable del compilador de Rust.
Las garantías de estabilidad de Rust aseguran que todos los ejemplos del libro
que se compilan seguirán compilando con versiones más nuevas de Rust. La salida
puede diferir ligeramente entre versiones porque Rust a menudo mejora los
mensajes de error y las advertencias. En otras palabras, cualquier versión más
nueva, estable de Rust que instales usando estos pasos debería funcionar como se
espera con el contenido de este libro.

> ### Notación de línea de comandos
>
> En este capítulo y en todo el libro, mostraremos algunos comandos utilizados
> en la terminal. Las líneas que debes ingresar en una terminal comienzan con `$`.
> No necesitas escribir el carácter `$`; es el indicador de línea de comandos
> mostrado para indicar el comienzo de cada comando. Las líneas que no comienzan
> con `$` generalmente muestran la salida del comando anterior. Además, los
> ejemplos específicos de PowerShell usarán `>` en lugar de `$`.

### Instalación de `rustup` en Linux o macOS

Si estás utilizando Linux o macOS, abre una terminal y escribe lo siguiente

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

El comando descarga un script y comienza la instalación de la herramienta
`rustup`, que instala la última versión estable de Rust. Es posible que se te
solicite tu contraseña. Si la instalación es exitosa, aparecerá la siguiente
línea:

```text
Rust is installed now. Great!
```

También necesitarás un *enlazador*, que es un programa que Rust utiliza para
unir sus salidas compiladas en un solo archivo. Es probable que ya lo tengas.
Si obtienes errores de enlace, debes instalar un compilador C, que generalmente
incluye un enlazador. Un compilador C también es útil porque algunos paquetes
comunes de Rust dependen de código C y necesitarán un compilador C.

En macOS, puedes obtener un compilador C ejecutando:

```console
$ xcode-select --install
```

Los usuarios de Linux deben instalar generalmente GCC o Clang, según la
documentación de su distribución. Por ejemplo, si usas Ubuntu, puede instalar el
paquete `build-essential`.

### Instalación de `rustup` en Windows

En Windows, ve a [https://www.rust-lang.org/tools/install][install] y sigue las
instrucciones para instalar Rust. En algún momento de la instalación, recibirás
un mensaje para instalar Visual Studio. Este provee un linker y las bibliotecas
nativas necesarias para compilar programas. 

Para obtener las herramientas de compilación, deberás instalar 
[Visual Studio][visualstudio]. Cuando se te pregunte qué paquetes de trabajo 
instalar, incluye:

* “Desarrollo de escritorio con C ++”
* El SDK de Windows 10 o 11
* El componente de paquete de idioma inglés, junto con cualquier otro paquete de
  idioma de tu elección

El resto de este libro usa comandos que funcionan tanto en *cmd.exe* como en
PowerShell. Si hay diferencias específicas, explicaremos cuál usar.

Si tu necesitas más ayuda con este 
paso, mira [MSVC prerequisites][msvc] o escríbenos en nuestro [discord](https://discord.rustlang-es.org)

<a id="solucion-de-problemas"></a>

### Solución de problemas

Para verificar si has instalado Rust correctamente, abra una shell y escribe esta
línea:

```console
$ rustc --version
```

Deberías ver el número de versión, el hash de confirmación y la fecha de
confirmación de la última versión estable que se ha publicado, en el siguiente
formato:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

Si ves esta información, ¡has instalado Rust correctamente! Si no ves esta
información, verifica que Rust esté en la variable de sistema `%PATH%` de la
siguiente manera.

En Windows CMD, usa:

```console
> echo %PATH%
```

En PowerShell, usa:

```powershell
> echo $env:Path
```

En Linux y macOS, usa:

```console
$ echo $PATH
```

Si todo está correcto y Rust aún no funciona, hay varios lugares donde puedes
obtener ayuda. Obten información sobre cómo comunicarte con otros Rustaceans
(un apodo tonto que nos llamamos a nosotros mismos) en [la página de la
comunidad][community].

### Actualización y desinstalación

Una vez que Rust se instala a través de `rustup`, actualizar a una versión
recién lanzada es fácil. Desde tu shell, ejecuta el siguiente script de
actualización:

```console
$ rustup update
```

Para desinstalar Rust y `rustup`, ejecuta el siguiente script de desinstalación
desde tu shell:

```console
$ rustup self uninstall
```

### Documentación local

La instalación de Rust también incluye una copia local de la documentación para
que puedas leerla sin conexión. Ejecuta `rustup doc` para abrir la documentación
local en tu navegador.

En cualquier momento en que se proporcione un tipo o una función de la biblioteca
estándar y no estés seguro de lo que hace o cómo usarlo, usa la documentación de
la interfaz de programación de aplicaciones (API) para averiguarlo.

[otherinstall]: https://forge.rust-lang.org/infra/other-installation-methods.html
[install]: https://www.rust-lang.org/tools/install
[msvc]: https://rust-lang.github.io/rustup/installation/windows-msvc.html
[community]: https://www.rust-lang.org/community
[visualstudio]: https://visualstudio.microsoft.com/downloads/
