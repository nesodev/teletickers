# Ticky - Sistema de Venta de Entradas

Proyecto del curso **Taller de Desarrollo Móvil** — Grupo 8  
Universidad Nacional Mayor de San Marcos

## Integrantes

| Nombre | Rol |
|--------|-----|
| Ever Frank Avendaño Meza | Líder técnico / Backend |
| Rodrigo Joaquín Calderón Zúñiga | Backend / Base de datos |
| Wilson Fabrizzio Espinoza Aponte | Frontend / Integración |
| Daniel Eduardo Tintaya Avila | Backend / Infraestructura |
| Abraham Josue Carbajal | Backend / API Events |

## Arquitectura

```
teletickers/
├── backend/        # API GraphQL (Rust + Actix-Web)  → :8080
├── api-events/     # API REST de eventos (Go + Gin)  → :8081
├── frontend/       # Web app (Astro + React)          → :4321
└── docker-compose.yml
```

## Levantar el proyecto

### Requisitos
- Docker Desktop instalado y corriendo

### Pasos

```bash
# 1. Clonar el repositorio
git clone https://github.com/nesodev/teletickers
cd teletickers

# 2. Levantar todos los servicios
docker compose up --build

# 3. Acceder a la aplicación
# Frontend:   http://localhost:4321
# Backend:    http://localhost:8080/graphql
# API Events: http://localhost:8081/api/eventos
# MinIO:      http://localhost:9901
```

## Servicios

| Servicio | URL | Descripción |
|----------|-----|-------------|
| Frontend | http://localhost:4321 | Interfaz web |
| Backend GraphQL | http://localhost:8080/graphql | API principal |
| API Events | http://localhost:8081/api/eventos | Listado de eventos |
| MinIO Console | http://localhost:9901 | Almacenamiento S3 |
| PostgreSQL | localhost:5432 | Base de datos |

## Tecnologías

- **Backend**: Rust, Actix-Web, async-graphql, SeaORM, PostgreSQL
- **API Events**: Go, Gin, PostgreSQL
- **Frontend**: Astro, React, Apollo Client, TailwindCSS
- **Infraestructura**: Docker, MinIO, PostgreSQL
