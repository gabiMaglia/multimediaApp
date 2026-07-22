# Architecture — multimediaApp

## Stack
| Capa | Tech | Versión | ADR |
|------|------|---------|-----|
| Shell desktop | Tauri | v2 | — |
| Backend | Rust (workspace del boilerplate) | — | — |
| Frontend | React + Vite | — | — |
| ASR (transcripción) | transformers.js (Whisper), WebGPU/WASM | v3 | — |
| Conversión media | ffmpeg (binario) | 7.1 | ADR-001 (pendiente) |
| Descarga media | yt-dlp | — | ADR-001 (pendiente) |

## DB
N/A — sin base de datos. Opera sobre archivos que el usuario provee.

## Índice ADRs
| # | Título | Estado | Fecha |
|---|--------|--------|-------|
| 1 | Estrategia de empaquetado del core (ffmpeg/yt-dlp) en Tauri: sidecars vs Python embebido vs reescritura Rust | Pendiente | 2026-07-22 |

---
(ADRs a completar por nerv-arquitecto)

## Origen del core (referencia, FUERA del repo)
- Transcriptor web funcional en `C:\Users\Gabriel Maglia\transcriptor`: `index.html` + `worker.js` (Whisper en el navegador vía transformers.js) y `server.py` (ffmpeg vía imageio-ffmpeg 7.1 + yt-dlp). Probado end-to-end 2026-07-22. Es la base a portar, NO el diseño final.
