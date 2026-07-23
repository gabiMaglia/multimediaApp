# Architecture — multimediaApp

## Stack
| Capa | Tech | Versión | ADR |
|------|------|---------|-----|
| Shell desktop | Tauri | v2 | — |
| Backend | Rust (workspace del boilerplate) | — | — |
| Frontend | React + Vite | — | — |
| ASR (transcripción) | transformers.js (Whisper), WebGPU/WASM | v3 | ADR-002 |
| Conversión media | ffmpeg (binario, sidecar) | 7.1 | ADR-001 |
| Descarga media | yt-dlp (binario, sidecar) | — | ADR-001 |

## DB
N/A — sin base de datos. Opera sobre archivos que el usuario provee.

## Contrato Rust ↔ React (comandos Tauri `invoke`)
| Comando | Payload | Retorno | Sidecar |
|---------|---------|---------|---------|
| `convert_media` | `{ inputPath, targetFormat }` | `{ outputPath }` | ffmpeg |
| `download_media` | `{ url }` | `{ mediaPath }` | yt-dlp |
| (transcripción) | — corre 100% en el webview (transformers.js), sin comando Rust | — | — |

## Índice ADRs
| # | Título | Estado | Fecha |
|---|--------|--------|-------|
| 1 | Empaquetado del core: ffmpeg + yt-dlp como sidecars Tauri v2 (externalBin + plugin-shell) | Aceptado | 2026-07-22 |
| 2 | Transcripción offline: Whisper (transformers.js) en el webview con modelo ONNX bundled | Aceptado | 2026-07-22 |
| 3 | Design system derivado de PhotoCutOnline (extracción de tokens/componentes) | Propuesto | 2026-07-22 |

---
## ADR-001: Empaquetado del core (ffmpeg + yt-dlp) como sidecars Tauri v2
- **Estado:** Aceptado
- **Contexto:** El core probado corre en Python (`server.py`: ffmpeg vía imageio-ffmpeg + yt-dlp). Tauri es Rust + webview. Hay que llevar la conversión y la descarga a la app de escritorio offline sin arrastrar Python.
- **Decisión:** Empaquetar `ffmpeg` y `yt-dlp` como **binarios sidecar** (`bundle.externalBin` en tauri.conf.json) e invocarlos desde Rust con `tauri-plugin-shell` (`Command` sidecar). React llama a comandos `invoke` (`convert_media`, `download_media`); Rust ejecuta el sidecar y devuelve la ruta de salida.
- **Descartado:**
  - *Python embebido + server.py*: arrastra un runtime de ~50 MB, fragilidad de empaquetado, arranque de servidor local. Innecesario.
  - *Reescribir conversión/descarga en Rust puro*: reinventar ffmpeg/yt-dlp. Absurdo.
- **Consecuencias:** El instalador pesa más (ffmpeg ~40-80 MB + yt-dlp ~15 MB por plataforma). yt-dlp se desactualiza rápido (tokens de YouTube) → prever mecanismo de update del binario. Ejecución de sidecars = superficie sensible → revisión QA Strong en esos tickets (P-11).

## ADR-002: Transcripción offline con Whisper en el webview
- **Estado:** Aceptado
- **Contexto:** Whisper ya corre en el navegador vía transformers.js. Pero por defecto descarga el modelo de Hugging Face en runtime — incompatible con "offline".
- **Decisión:** Mantener la transcripción 100% en el webview (sin comando Rust) y **bundlear el modelo ONNX** (`whisper-base`, ~80 MB) como recurso de la app; apuntar transformers.js al modelo local (`env.allowLocalModels`, `localModelPath`). WebGPU con fallback a WASM.
- **Descartado:** Descargar el modelo en el primer arranque (rompe el offline puro; se puede ofrecer como opción para instalador liviano más adelante).
- **Consecuencias:** +80 MB al instalador. Transcripción sin red, privada. El audio del usuario nunca sale de la máquina (RN-01).

## ADR-003: Design system derivado de PhotoCutOnline
- **Estado:** Propuesto
- **Contexto:** El PO quiere seguir las líneas de diseño de PhotoCut, sin Figma. La fuente de verdad es el código de `gabiMaglia/PhotoCutOnline` + el sitio vivo.
- **Decisión (propuesta):** Primer ticket de UI = extraer tokens (colores, tipografía, spacing) y componentes base de PhotoCutOnline y montarlos como design system del frontend (no copiar pantallas: extraer el sistema). Ratificar al arrancar el sprint de UI.
- **Consecuencias:** Consistencia visual con PhotoCut, habilita la fusión futura. Requiere leer el repo de PhotoCutOnline en el ticket, no antes (P-E).

## Origen del core (referencia, FUERA del repo)
- Transcriptor web funcional en `C:\Users\Gabriel Maglia\transcriptor`: `index.html` + `worker.js` (Whisper en el navegador vía transformers.js) y `server.py` (ffmpeg vía imageio-ffmpeg 7.1 + yt-dlp). Probado end-to-end 2026-07-22. Base a portar, NO el diseño final.
