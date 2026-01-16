---
title: "[BACKEND] Implementación del Cliente SODA API para SECOP II"
labels:
  - backend
  - enhancement
---

## Descripción
Desarrollar el cliente de ingestión de datos utilizando la API SODA (Socrata Open Data API) para obtener contratos de SECOP II.

## Detalles de Configuración
- App Token: Requerido (datos.gov.co)
- Dataset SECOP II Contratos: `jbjy-vk9h`
- Dataset SECOP II Procesos: `p6dx-8zbt`
- SIGEP Funcionarios: `2jzx-383z`

## Tareas
- [ ] Implementar cliente en `backend/src/obs/ingest.rs`
- [ ] Conexión al dataset `jbjy-92t2`
- [ ] Soporte para paginación y filtros por fecha
- [ ] Manejo de App Token para evitar rate-limiting
