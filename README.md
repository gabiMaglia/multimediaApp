# Input Router

Aplicación de escritorio para Windows que rutea inputs de distintos dispositivos (joystick, teclado, mouse) a aplicaciones específicas sin necesidad de foco de ventana.

- **Joystick** → Steam (vía gamepad virtual ViGEm)
- **Mouse + Teclado** → Ableton / Chrome (vía PostMessage)

Built with **Rust** + **Tauri v2** + **React** + **Vite**.

---

## Estructura del proyecto

```
input-router/
├── apps/
│   └── desktop/
│       ├── src-tauri/       # Backend Rust (Tauri v2)
│       └── ui/              # Frontend React + Vite
├── crates/
│   ├── input-core/          # Captura de input y tipos base
│   ├── injector/            # Inyección (ViGEm, PostMessage)
│   └── device-debugger/     # Herramientas de debug
└── docs/
```

---

## Requisitos previos

| Herramienta | Versión mínima | Instalación |
|---|---|---|
| **Rust** | 1.70+ | [rustup.rs](https://rustup.rs) |
| **Node.js** | 18+ | [nodejs.org](https://nodejs.org) |
| **npm** | 9+ | Viene con Node.js |
| **Visual Studio Build Tools** | 2022 | [visualstudio.microsoft.com](https://visualstudio.microsoft.com/visual-cpp-build-tools/) — seleccionar "Desktop development with C++" |

### Verificar instalación

```bash
rustc --version
cargo --version
node --version
npm --version
```

---

## Instalación

### 1. Clonar el repositorio

```bash
git clone https://github.com/tu-usuario/input-router.git
cd input-router
```

### 2. Instalar dependencias de Node

```bash
cd apps/desktop
npm install

cd ui
npm install

cd ../../..
```

### 3. Compilar crates de Rust (verificar que todo está bien)

```bash
cargo check
```

---

## Ejecutar en modo desarrollo

Desde la raíz del proyecto:

```bash
cd apps/desktop
npx tauri dev
```

Esto hace dos cosas simultáneamente:
1. Levanta el frontend React en `http://localhost:5173` (Vite dev server)
2. Compila el backend Rust y abre la ventana de Tauri

> **Nota:** La primera compilación de Rust puede tardar varios minutos. Las siguientes son incrementales y mucho más rápidas.

---

## Build de producción

```bash
cd apps/desktop
npx tauri build
```

El instalador se genera en `apps/desktop/src-tauri/target/release/bundle/`.

---

## Comandos útiles

| Comando | Ubicación | Descripción |
|---|---|---|
| `cargo check` | Raíz | Verifica compilación de todos los crates |
| `cargo build` | Raíz | Compila todos los crates |
| `cargo test` | Raíz | Corre todos los tests |
| `npm run dev` | `apps/desktop/ui` | Solo frontend (sin Tauri) |
| `npx tauri dev` | `apps/desktop` | App completa en modo dev |
| `npx tauri build` | `apps/desktop` | Build de producción |

---

## Stack técnico

- **Rust** — Backend, captura de input, inyección
- **Tauri v2** — Bridge nativo desktop ↔ frontend
- **React 19** — UI
- **Vite 7** — Bundler frontend
- **gilrs** — Captura de gamepad/joystick (PS4, Xbox, genéricos)
- **rdev** — Captura de teclado y mouse vía hooks
- **windows-rs** — Win32 API bindings

---

## Troubleshooting

### `cargo check` falla con errores de linker
Asegurate de tener instalado Visual Studio Build Tools con el workload "Desktop development with C++".

### `npx tauri dev` no abre la ventana
Verificá que el frontend esté corriendo en el puerto 5173. Podés probar primero:
```bash
cd apps/desktop/ui
npm run dev
```

### El frontend no se comunica con Rust
La función `invoke` solo funciona dentro de la ventana de Tauri, no en el browser directo.

---

## Licencia

MIT