---
title: "[DATA] Integración con Hugging Face Hub para Data Lake"
labels:
  - data
  - infrastructure
---

## Descripción
Configurar el almacenamiento persistente de los datasets procesados en Hugging Face Hub.

## Detalles de Configuración
- Dataset Target: `iberi22/veeduria-secop-ii`
- Formato: Parquet
- Herramienta: `hf-hub` crate
- Credenciales: `HF_TOKEN` (GitHub Secrets)

## Tareas
- [ ] Implementar subida de archivos Parquet
- [ ] Manejo de secretos (HF_TOKEN)
- [ ] Lógica de versionamiento de datos
