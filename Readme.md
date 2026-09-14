# File Tree Generator

Herramienta de consola en Rust que genera el árbol de una carpeta como
Markdown, respetando `.gitignore` — funcione o no haya un repositorio de
git de por medio.

## Cómo funciona

Se ejecuta desde dentro de la carpeta que quieres documentar (siempre
escanea la carpeta actual, no pide ninguna ruta). Hace 3 preguntas por
consola y, al terminar, escribe el resultado en `file_tree.md`:

```
==================================================
 📦 FILE TREE GENERATOR
==================================================

[1/3] Selecciona el modo de escaneo:
      0 = Carpetas y archivos (Completo)
      1 = Solo carpetas
> Opción: 0

[2/3] Estilo visual del árbol:
      0 = Con emojis (📦, 📂, 📜)
      1 = Texto plano clásico (Ramas puras)
> Opción: 0

[3/3] Límite de profundidad del árbol:
      [Enter] para escanear sin límite, o escribe un número positivo (ej. 3)
> Nivel de profundidad:

[✓] Generando estructura...
[✓] ¡Éxito! Archivo 'file_tree.md' creado.
Presiona [Enter] para salir...
```

Cualquier respuesta que no sea una opción válida (`-1`, texto, `0` en el
límite de profundidad, etc.) repregunta con "Opción inválida" en vez de
asumir un valor por defecto en silencio.

## `.gitignore`, aunque no haya git

Si existe un `.gitignore` en la carpeta, sus reglas se respetan igual —
no hace falta que la carpeta sea un repositorio de git. Es una forma
simple de decirle a la herramienta qué excluir sin tener que pasar flags:
si no quieres que salga cierta carpeta, la agregas ahí y listo.

`.git` (cuando sí existe) se excluye siempre, automáticamente, sin que
haga falta declararlo en ningún lado — igual que el propio `git` nunca se
sube a sí mismo.

## Ejemplo de salida

```text
📦 mi-proyecto
 ┣ 📂 src
 ┃ ┣ 📜 main.rs
 ┃ ┗ 📜 config.rs
 ┗ 📜 config.toml
```

En modo texto plano (sin emojis), las carpetas se distinguen con un `/`
al final (`src/`), ya que sin ícono son indistinguibles de un archivo a
simple vista.

## Estructura del proyecto

```
📦file_tree_generator
 ┣ 📂project_information
 ┃ ┣ 📜Registro de Bugs.txt
 ┃ ┗ 📜Sistema de Orquestacion Modular (SOM) SE.md
 ┣ 📂src
 ┃ ┣ 📂generator
 ┃ ┃ ┣ 📜core.rs
 ┃ ┃ ┣ 📜mod.rs
 ┃ ┃ ┣ 📜tree.rs
 ┃ ┃ ┗ 📜ui.rs
 ┃ ┗ 📜main.rs
 ┣ 📜Cargo.lock
 ┣ 📜Cargo.toml
 ┗ 📜Readme.md
```

Sigue SOM: `core.rs` es el orquestador (solo delega, sin lógica propia),
`tree.rs` recorre y arma el árbol, `ui.rs` es toda la interacción por
consola.

## Requisitos

- [Rust](https://www.rust-lang.org) vía `rustup`, solo para compilar.
- Depende de la crate [`ignore`](https://docs.rs/ignore) (la misma que usa
  `ripgrep`) para el filtrado real de `.gitignore`.

## Compilar y ejecutar

```powershell
cargo build --release
```

El binario queda en `target\release\file_tree_generator.exe` — se puede
copiar a cualquier carpeta y ejecutar ahí directo.

## Limitación conocida

Si lo ejecutas dentro de una terminal integrada (Zed, VS Code, etc.) y
cierras esa pestaña de terminal **antes** de presionar `Enter` en el
mensaje final ("Presiona [Enter] para salir..."), es posible que el
proceso quede corriendo en segundo plano sin ninguna ventana visible,
bloqueando el `.exe` (Windows no te deja recompilar ni borrarlo hasta
matar el proceso a mano). Ver `project_information/Registro de Bugs.txt`.
Mientras tanto: siempre presiona `Enter` en ese mensaje final antes de
cerrar la terminal.

## Créditos

Inspirado en la extensión de VS Code
[File Tree Generator](https://marketplace.visualstudio.com/items?itemName=Shinotatwu-DS.file-tree-generator)
de Shinotatwu-DS — esta es, en espíritu, una versión de escritorio de esa
misma idea: un ejecutable independiente, sin necesitar VS Code instalado,
con soporte real de `.gitignore` vía la crate `ignore`.
