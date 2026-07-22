# Requirements — multimediaApp

## 1. Visión (1 frase)
App multimedia de escritorio (offline, Tauri) que transcribe audio/video con Whisper corriendo en el webview y convierte entre formatos (mp3/wav/mp4/mpeg) con ffmpeg, siguiendo las líneas de diseño de PhotoCut; a futuro, posible fusión con PhotoCut.

## 2. Funcionalidades (IN)
| ID | Funcionalidad | MoSCoW | Estado |
|----|---------------|--------|--------|
| F-01 | Transcripción local con Whisper (transformers.js, corre en el webview) — salida texto + SRT | Must | Core hecho (web), a portar |
| F-02 | Conversión de formato con ffmpeg (mp3/wav/mp4/mpeg) sobre archivo del usuario | Must | Core hecho (web), a portar |
| F-03 | UI React siguiendo el diseño de PhotoCutOnline | Must | No iniciado |
| F-04 | Empaquetado desktop offline (Tauri, instalador) | Must | No iniciado |
| F-05 | Descarga/transcripción desde link YouTube/IG (yt-dlp) | Could | Core hecho (web); scope v1 a confirmar (caveat legal) |

## 3. Fuera de alcance (OUT)
- (a definir al cerrar el scope de planificación)

## 4. Reglas de negocio
| ID | Regla | Origen/fecha |
|----|-------|--------------|
| RN-01 | Procesamiento local/offline: el audio/video del usuario no sale de su máquina. | Sesión 2026-07-22 |
| RN-02 | La descarga desde plataformas de terceros (YouTube/IG) es sólo para uso personal local; NO se distribuye como servicio público (riesgo legal stream-ripping, ver charla de sesión). | Sesión 2026-07-22 |

## 5. Enlaces (espejo del registry)
- Tracker: ninguno
- Board: —
- Figma: — (diseño derivado de PhotoCutOnline + photocutapp.com)

## 6. Preguntas abiertas al PO
| # | Pregunta | Estado |
|---|----------|--------|
| 1 | ¿El v1 de escritorio incluye la descarga desde links de YouTube/IG (F-05), o el desktop se limita a transcribir+convertir archivos propios y el link queda sólo para uso local/web? | Abierta |
