# Backlog — multimediaApp

> Estados: Backlog → To Do → En progreso → En revisión QA → Done | Bloqueado
> Solo nerv-qa escribe "Done". Mapear SIEMPRE el ID externo si existe.
> Niv (P-11, nivel de revisión QA): A=Advisory (default) · S=Strong · X=Adversarial.
> (*) Ticket Rust/Tauri: el roster NERV no tiene agente Rust dedicado (es Python/PySide6). Ver Deuda DT-02.

## Sprint 0 — Alta + planificación
| ID | Ext | Tarea | Asignado | Estado | Niv | Criterios de aceptación | Rama |
|----|-----|-------|----------|--------|-----|--------------------------|------|
| T-000 | — | Alta: boilerplate, engram, registry, repo remoto, ADRs, backlog. | nerv-orquestador | Done (alta, sin QA) | A | Engram (7 files), registry, main+dev en origin, ADR-001/002/003, backlog. ✅ | dev |

## Sprint 1 — Scaffold limpio
| ID | Ext | Tarea | Asignado | Estado | Niv | Criterios de aceptación | Rama |
|----|-----|-------|----------|--------|-----|--------------------------|------|
| T-001 (*) | — | Limpiar el boilerplate: quitar crates input-router (input-core/injector/device-debugger) y módulos device/router/ipc/storage de src-tauri; renombrar productName→"multimediaApp" e identifier; limpiar Cargo workspace y deps de input (rdev/gilrs). | nerv-desktop (Rust, DT-02) | En progreso | S | `cargo build` OK sin los crates/módulos de input; la app abre una ventana vacía titulada multimediaApp; no quedan referencias a ViGEm/PostMessage/rdev/gilrs. | feature/T-001-scaffold-cleanup |
| T-002 | — | Estructura del frontend React+Vite: carpetas por feature (screaming/atomic), layout base, routing entre las 3 pantallas (Transcribir / Convertir / Link), tema claro-oscuro. Sin diseño final aún. | nerv-web | To Do | A | Vite corre; navegación entre 3 pantallas placeholder; estructura de carpetas por feature documentada. | feature/T-002-frontend-scaffold |

## Sprint 2 — Design system (PhotoCut)
| ID | Ext | Tarea | Asignado | Estado | Niv | Criterios de aceptación | Rama |
|----|-----|-------|----------|--------|-----|--------------------------|------|
| T-003 | — | Extraer el design system de PhotoCutOnline (tokens: colores, tipografía, spacing; componentes base: botón, input, card, dropzone) y montarlo en el frontend (ADR-003). Extraer el SISTEMA, no copiar pantallas. | nerv-web | To Do | A | Tokens y componentes base aplicados; las 3 pantallas placeholder usan el sistema; look consistente con photocutapp.com. | feature/T-003-design-system |

## Sprint 3 — Core: Conversión (ffmpeg sidecar)
| ID | Ext | Tarea | Asignado | Estado | Niv | Criterios de aceptación | Rama |
|----|-----|-------|----------|--------|-----|--------------------------|------|
| T-004 (*) | — | Integrar ffmpeg como sidecar Tauri (externalBin + tauri-plugin-shell). Comando Rust `convert_media(inputPath, targetFormat)`→outputPath, portando la lógica de `server.py::handle_convert` (mp3/wav/mp4/mpeg). | nerv-desktop* | To Do | S | Comando `invoke('convert_media')` devuelve archivo válido para los 4 formatos desde un archivo local; errores de ffmpeg propagados a JS. | feature/T-004-ffmpeg-sidecar |
| T-005 | — | UI de Conversión: elegir archivo, seleccionar formato destino, convertir, guardar resultado (dialog Tauri). Portar UX del panel del transcriptor web. | nerv-web | To Do | A | Flujo completo end-to-end con el comando de T-004; estados de carga/éxito/error visibles. | feature/T-005-convert-ui |

## Sprint 4 — Core: Transcripción (Whisper offline)
| ID | Ext | Tarea | Asignado | Estado | Niv | Criterios de aceptación | Rama |
|----|-----|-------|----------|--------|-----|--------------------------|------|
| T-006 | — | Integrar transformers.js Whisper en el webview con modelo ONNX (`whisper-base`) bundled como recurso (ADR-002), sin red. Worker de transcripción portado de `worker.js`. WebGPU con fallback WASM. | nerv-web | To Do | A | Transcribe un audio local SIN conexión; el modelo carga desde recurso local; salida texto correcta en español. | feature/T-006-whisper-offline |
| T-007 | — | UI de Transcripción: elegir audio/video, transcribir, mostrar texto, exportar .txt y .srt. Portar UX de `index.html`. | nerv-web | To Do | A | Flujo end-to-end con T-006; export txt y srt válidos; timestamps en el SRT. | feature/T-007-transcribe-ui |

## Sprint 5 — Core: Descarga de links (yt-dlp sidecar)
| ID | Ext | Tarea | Asignado | Estado | Niv | Criterios de aceptación | Rama |
|----|-----|-------|----------|--------|-----|--------------------------|------|
| T-008 (*) | — | Integrar yt-dlp como sidecar Tauri. Comando Rust `download_media(url)`→mediaPath (bestaudio, sin re-encode). Caveat legal RN-02 documentado en la UI. | nerv-desktop* | To Do | S | `invoke('download_media')` baja el audio de un link válido; errores propagados; sin credenciales ni red externa más allá del propio yt-dlp. | feature/T-008-ytdlp-sidecar |
| T-009 | — | UI de Link: pegar URL, descargar, y encadenar el resultado a Transcribir o Convertir. Mostrar aviso de uso personal (RN-02). | nerv-web | To Do | A | Pegar link → descarga → se puede transcribir/convertir el resultado; aviso legal visible. | feature/T-009-link-ui |

## Sprint 6 — Empaquetado offline
| ID | Ext | Tarea | Asignado | Estado | Niv | Criterios de aceptación | Rama |
|----|-----|-------|----------|--------|-----|--------------------------|------|
| T-010 (*) | — | Configurar el bundle Tauri (instalador Windows): incluir sidecars (ffmpeg, yt-dlp) + modelo Whisper como recursos; generar instalador; probar la app INSTALADA y OFFLINE. | nerv-desktop* | To Do | S | Instalador generado; app instalada funciona sin red para transcribir y convertir; sidecars encontrados en runtime. | feature/T-010-packaging |

## Veredictos QA
| Tarea | Veredicto | Defectos (si rechazo) | Fecha |
|-------|-----------|------------------------|-------|

## Deuda técnica
| ID | Descripción | Origen | Prioridad |
|----|-------------|--------|-----------|
| DT-01 | El boilerplate trae crates de input-router (ViGEm/PostMessage/device-debugger) irrelevantes; se limpian en T-001. | Alta 2026-07-22 | Media (resuelve T-001) |
| DT-02 | RESUELTA 2026-07-22: PO decidió re-scopear nerv-desktop a Rust/Tauri para este proyecto. Los tickets (*) los ejecuta nerv-desktop. | Planificación 2026-07-22 | Resuelta |

## Histórico (sprints cerrados: 3 líneas c/u, máx. 5; el resto a ~/.nerv/archive/)
