# Backlog — multimediaApp

> Estados: Backlog → To Do → En progreso → En revisión QA → Done | Bloqueado
> Solo nerv-qa escribe "Done". Mapear SIEMPRE el ID externo si existe.
> Niv (P-11, nivel de revisión QA): A=Advisory (default) · S=Strong · X=Adversarial.

## Sprint 0 — Alta + planificación (en curso)
| ID | Ext | Tarea | Asignado | Estado | Niv | Criterios de aceptación | Rama |
|----|-----|-------|----------|--------|-----|--------------------------|------|
| T-000 | — | Alta de proyecto: clonar boilerplate, crear engram, actualizar registry, commit local inicial. | nerv-orquestador | En progreso | A | Engram creado (7 archivos), fila en registry, commit local. Remoto GitHub pendiente (sin gh/token). | dev |

## Backlog de producto
> Se detalla al cerrar el scope v1 (§6 requirements) + ADR-001. NO se escriben tickets de producto antes de decidir la estrategia de port (el ADR define cómo se parten).

## Veredictos QA
| Tarea | Veredicto | Defectos (si rechazo) | Fecha |
|-------|-----------|------------------------|-------|

## Deuda técnica
| ID | Descripción | Origen | Prioridad |
|----|-------------|--------|-----------|
| DT-01 | El boilerplate trae crates de input-router (ViGEm / PostMessage / device-debugger) irrelevantes para multimedia; decidir limpieza al arrancar. | Alta 2026-07-22 | Media |

## Histórico (sprints cerrados: 3 líneas c/u, máx. 5; el resto a ~/.nerv/archive/)
