## Personalizando compilaciones con perfiles de lanzamiento

En Rust, los *release profiles* son perfiles predefinidos y personalizables con
diferentes configuraciones que permiten a un programador tener más control sobre
varias opciones para compilar código. Cada perfil se configura de forma
independiente de los demás.

Cargo tiene dos perfiles principales: el perfil `dev` que Cargo usa cuando
ejecutas `cargo build` y el perfil `release` que Cargo usa cuando ejecutas
`cargo build --release`. El perfil `dev` está definido con buenos valores
predeterminados para el desarrollo, y el perfil `release` tiene buenos valores
predeterminados para las compilaciones de lanzamiento.

Estos nombres de perfil pueden ser familiares en la salida de tus compilaciones:

<!-- manual-regeneration
anywhere, run:
cargo build
cargo build --release
and ensure output below is accurate
-->

```console
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
$ cargo build --release
    Finished `release` profile [optimized] target(s) in 0.32s
```

El perfil `dev` y `release` son estos perfiles diferentes utilizados por el
compilador.

Cargo tiene valores predeterminados para cada uno de los perfiles que se
aplican cuando no has agregado explícitamente ninguna sección `[profile.*]` en
el archivo *Cargo.toml* del proyecto. Al agregar secciones `[profile.*]` para
cualquier perfil que desees personalizar, anularás cualquier subconjunto de los
valores predeterminados. Por ejemplo, aquí están los valores predeterminados
para la configuración `opt-level` para los perfiles `dev` y `release`:

<span class="filename">Filename: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

El ajuste `opt-level` controla la cantidad de optimizaciones que Rust aplicará
a tu código, con un rango de 0 a 3. Aplicar más optimizaciones extiende el
tiempo de compilación, por lo que si estás en desarrollo y compilando tu código
con frecuencia, querrás menos optimizaciones para compilar más rápido, incluso
si el código resultante se ejecuta más lento. El `opt-level` predeterminado para
`dev` es, por lo tanto, `0`. Cuando estés listo para lanzar tu código, es mejor
dedicar más tiempo a compilar. Solo compilarás en modo de lanzamiento una vez,
pero ejecutarás el programa compilado muchas veces, por lo que el modo de
lanzamiento intercambia un tiempo de compilación más largo por un código que se
ejecuta más rápido. Es por eso que el `opt-level` predeterminado para el perfil
`release` es `3`.

Puedes anular un ajuste predeterminado agregando un valor diferente para él en
*Cargo.toml*. Por ejemplo, si queremos usar el nivel de optimización 1 en el
perfil de desarrollo, podemos agregar estas dos líneas al archivo *Cargo.toml*
del proyecto:

<span class="filename">Filename: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 1
```

Este código anula la configuración predeterminada de `0`. Ahora, cuando
ejecutemos `cargo build`, Cargo usará los valores predeterminados para el perfil
`dev` más nuestra personalización de `opt-level`. Debido a que establecimos
`opt-level` en `1`, Cargo aplicará más optimizaciones que el valor 
predeterminado, pero no tantas como en una compilación de lanzamiento.

Para la lista completa de opciones de configuración y valores predeterminados
para cada perfil, consulta la 
[documentación de Cargo](https://doc.rust-lang.org/cargo/reference/profiles.html).
