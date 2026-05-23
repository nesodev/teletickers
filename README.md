<div align="center">

# 🎟️ Ticky — Sistema de Venta de Entradas

**Universidad Nacional Mayor de San Marcos**  
Facultad de Ingeniería de Sistemas e Informática  
Escuela Profesional de Ingeniería de Software  
Curso: Taller de Desarrollo Móvil — Grupo 8

[![Backend](https://img.shields.io/badge/Backend-Rust%20%2B%20GraphQL-orange)](./backend)
[![API Events](https://img.shields.io/badge/API%20Events-Go%20%2B%20Gin-blue)](./api-events)
[![Frontend](https://img.shields.io/badge/Frontend-Astro%20%2B%20React-purple)](./frontend)
[![DB](https://img.shields.io/badge/DB-PostgreSQL-336791)](./backend/migrations)
[![Docker](https://img.shields.io/badge/Deploy-Docker%20Compose-2496ED)](./docker-compose.yml)

</div>

---

## 📋 Índice

1. [Descripción del Proyecto](#1-descripción-del-proyecto)
2. [Equipo](#2-equipo)
3. [Arquitectura del Sistema](#3-arquitectura-del-sistema)
4. [Tecnologías Utilizadas](#4-tecnologías-utilizadas)
5. [Estructura del Repositorio](#5-estructura-del-repositorio)
6. [Modelo de Datos](#6-modelo-de-datos)
7. [API GraphQL — Backend](#7-api-graphql--backend)
8. [API REST — Events Service](#8-api-rest--events-service)
9. [Frontend](#9-frontend)
10. [Cómo Levantar el Proyecto](#10-cómo-levantar-el-proyecto)
11. [Cronograma Kanban](#11-cronograma-kanban)
12. [Flujo de Trabajo Git](#12-flujo-de-trabajo-git)

---

## 1. Descripción del Proyecto

**Ticky** es una plataforma web de venta de entradas para eventos en Perú. Permite a organizadores crear y gestionar eventos, y a los asistentes comprar entradas de forma segura con métodos de pago locales (Yape, Plin, tarjeta).

### Funcionalidades principales

| Módulo | Descripción |
|--------|-------------|
| 🔐 **Autenticación** | Registro e inicio de sesión con JWT. Validación de DNI y email |
| 🎭 **Eventos** | Crear, publicar, cancelar y buscar eventos por categoría o texto |
| 🎫 **Entradas** | Tipos de entrada con precio, aforo y límite por compra |
| 💳 **Compras** | Flujo completo de compra con Yape, Plin o tarjeta |
| 📄 **Comprobantes** | Generación de boleta/factura en PDF con IGV |
| 📱 **QR** | Código QR único por entrada para validación en puerta |
| 🔔 **Notificaciones** | Alertas de compra, recordatorios y eventos del sistema |
| 📊 **Analytics** | Dashboard de métricas para organizadores |
| 🤖 **API Events** | Servicio REST para integración con chatbots (SendPulse/WhatsApp) |

---

## 2. Equipo

| Nombre | Rama Git | Rol Principal |
|--------|----------|---------------|
| **Ever Frank Avendaño Meza** | `ever` | Líder técnico · Backend · DevOps |
| **Rodrigo Joaquín Calderón Zúñiga** | `rodrigo` | Backend · Base de datos · Arquitectura |
| **Wilson Fabrizzio Espinoza Aponte** | `fabrizzio` | Frontend · Integración · QA |
| **Daniel Eduardo Tintaya Avila** | `daniel` | Backend · Infraestructura · CI/CD |
| **Abraham Josue Carbajal** | `josue` | Backend · API Events · Casos de uso |

**Docente:** Javier Antonio Prudencio Vidal

---

## 3. Arquitectura del Sistema

```
┌─────────────────────────────────────────────────────────────┐
│                        CLIENTE                              │
│              Browser / App Móvil / Chatbot                  │
└──────────┬──────────────────┬──────────────────┬────────────┘
           │ :4321            │ :8080            │ :8081
           ▼                  ▼                  ▼
┌──────────────────┐ ┌────────────────┐ ┌────────────────────┐
│    FRONTEND      │ │    BACKEND     │ │   API EVENTS       │
│  Astro + React   │ │ Rust + Actix   │ │   Go + Gin         │
│  Apollo Client   │ │ async-graphql  │ │   REST JSON        │
│  TailwindCSS     │ │ JWT Auth       │ │   SendPulse fmt    │
└──────────────────┘ └───────┬────────┘ └────────┬───────────┘
                             │                   │
                    ┌────────▼───────────────────▼────────┐
                    │           PostgreSQL :5432           │
                    │         (Base de datos única)        │
                    └─────────────────────────────────────┘
                             │
                    ┌────────▼────────┐
                    │   MinIO :9900   │
                    │  (Storage S3)   │
                    └─────────────────┘
```

### Patrón arquitectónico del Backend

El backend sigue **arquitectura hexagonal (Ports & Adapters)**:

```
src/
├── domain/          ← Entidades y reglas de negocio puras
│   ├── value_objects/  ← Email, DNI, Money, QRCode
│   ├── evento.rs
│   ├── user.rs
│   ├── compra.rs
│   └── entrada.rs
├── ports/           ← Interfaces (traits) del dominio
├── usecases/        ← Lógica de aplicación
├── infrastructure/  ← Implementaciones concretas (DB, Auth)
├── repositories/    ← Acceso a datos con SeaORM
├── graphql/         ← Capa de presentación GraphQL
│   ├── queries/
│   ├── mutations/
│   └── subscriptions/
└── external/        ← Servicios externos (email, pagos, S3)
```

---

## 4. Tecnologías Utilizadas

### Backend (`/backend`)
| Tecnología | Versión | Uso |
|------------|---------|-----|
| Rust | latest | Lenguaje principal |
| Actix-Web | 4 | Framework HTTP |
| async-graphql | 7.0 | Servidor GraphQL |
| SeaORM | 1.0 | ORM para PostgreSQL |
| jsonwebtoken | 10.2 | Autenticación JWT |
| argon2 / bcrypt | — | Hash de contraseñas |
| printpdf | 0.8 | Generación de PDFs |
| qrcode | 0.14 | Generación de QR |
| lettre | 0.11 | Envío de emails SMTP |

### API Events (`/api-events`)
| Tecnología | Versión | Uso |
|------------|---------|-----|
| Go | 1.24 | Lenguaje principal |
| Gin | 1.11 | Framework HTTP |
| lib/pq | 1.10 | Driver PostgreSQL |
| godotenv | 1.5 | Variables de entorno |

### Frontend (`/frontend`)
| Tecnología | Versión | Uso |
|------------|---------|-----|
| Astro | 5.x | Framework web SSR |
| React | 19 | Componentes UI |
| Apollo Client | 4.x | Cliente GraphQL |
| TailwindCSS | 3.4 | Estilos |
| Recharts | 3.x | Gráficas analytics |

### Infraestructura
| Tecnología | Uso |
|------------|-----|
| PostgreSQL 16 | Base de datos relacional |
| MinIO | Almacenamiento de imágenes (compatible S3) |
| Docker + Compose | Contenedores y orquestación local |

---

## 5. Estructura del Repositorio

```
teletickers/
│
├── backend/                    # API GraphQL principal (Rust)
│   ├── src/
│   │   ├── domain/             # Entidades del negocio
│   │   ├── ports/              # Interfaces/contratos
│   │   ├── usecases/           # Casos de uso
│   │   │   ├── auth/           # login, register
│   │   │   ├── eventos/        # CRUD eventos
│   │   │   ├── compras/        # flujo de compra
│   │   │   ├── entradas/       # tickets y QR
│   │   │   └── notificaciones/
│   │   ├── infrastructure/     # JWT, password hashing
│   │   ├── repositories/       # Acceso a BD
│   │   ├── graphql/            # Schema, queries, mutations
│   │   └── external/           # Email, pagos, storage
│   ├── migrations/
│   │   └── init_db.sql         # Schema completo de la BD
│   ├── queries/                # Ejemplos de queries GraphQL
│   ├── Cargo.toml
│   └── Dockerfile
│
├── api-events/                 # API REST de eventos (Go)
│   ├── cmd/api/main.go
│   ├── internal/
│   │   ├── config/
│   │   ├── database/
│   │   ├── handlers/           # GET /api/eventos
│   │   ├── models/
│   │   └── repository/
│   ├── go.mod
│   └── Dockerfile
│
├── frontend/                   # Web app (Astro + React)
│   ├── src/
│   │   ├── components/
│   │   │   ├── auth/           # Login, Register
│   │   │   ├── events/         # Listado, creación
│   │   │   ├── tickets/        # Mis entradas
│   │   │   ├── profile/        # Perfil de usuario
│   │   │   └── ui/             # Header, Sidebar, Footer
│   │   ├── lib/
│   │   │   ├── apollo-client.ts
│   │   │   └── graphql/        # queries.ts, mutations.ts
│   │   └── pages/              # Rutas Astro
│   ├── package.json
│   └── Dockerfile
│
└── docker-compose.yml          # Orquestación completa
```

---

## 6. Modelo de Datos

```sql
-- Diagrama simplificado de relaciones

usuario (id, nombre, email, dni, contraseña, estado, fecha_registro)
    │
    ├──< evento (id, titulo, fecha, hora, ubicacion, categoria,
    │           aforo, estado, organizador_id)
    │       │
    │       └──< tipo_entrada (id, nombre, precio, cantidad_disponible)
    │
    └──< compra (id, usuario_id, evento_id, monto_total,
                 metodo_pago, estado_pago, comprobante_id)
            │
            ├──> comprobante (id, tipo, serie, monto_total, igv, pdf)
            │
            └──< entrada (id, tipo_entrada_id, qr_code, estado)

notificacion (id, usuario_id, tipo, titulo, mensaje, leido)
```

### Estados del sistema

**Evento:** `borrador` → `publicado` → `cancelado` / `finalizado`

**Compra:** `pendiente` → `pagado` → `cancelado` / `reembolsado`

**Entrada:** `valido` → `usado` / `cancelado`

---

## 7. API GraphQL — Backend

**Endpoint:** `http://localhost:8080/graphql`  
**Playground:** `http://localhost:8080/graphql` (interfaz interactiva en el navegador)

### Autenticación

Todas las operaciones protegidas requieren el header:
```
Authorization: Bearer <token>
```

### Mutations de Autenticación

#### Registro de usuario
```graphql
mutation {
  register(
    nombre: "Juan Pérez"
    email: "juan@example.com"
    password: "password123"
    dni: "12345678"
    numeroCel: "987654321"
  ) {
    token
    user {
      id
      nombre
      email
    }
  }
}
```

#### Login
```graphql
mutation {
  login(
    email: "juan@example.com"
    password: "password123"
  ) {
    token
    user {
      id
      nombre
      email
    }
  }
}
```

### Queries de Eventos

#### Listar eventos publicados
```graphql
query {
  eventosPublicados {
    id
    titulo
    fecha
    hora
    categoria
    aforo
    estado
    ubicacion {
      region
      provincia
      distrito
    }
  }
}
```

#### Buscar eventos
```graphql
query {
  searchEventos(query: "concierto") {
    id
    titulo
    fecha
    categoria
  }
}
```

#### Ver mis eventos (requiere auth)
```graphql
query {
  misEventos {
    id
    titulo
    estado
    fecha
  }
}
```

### Mutations de Eventos

#### Crear evento (requiere auth)
```graphql
mutation {
  createEvento(
    titulo: "Concierto Rock 2026"
    descripcion: "Gran concierto en Lima"
    fecha: "2026-12-31"
    hora: "20:00:00"
    region: "Lima"
    provincia: "Lima"
    distrito: "Miraflores"
    categoria: "musica"
    aforo: 500
    restriccionEdad: "todo_publico"
  ) {
    id
    titulo
    estado
  }
}
```

#### Publicar evento (requiere auth)
```graphql
mutation {
  publishEvento(eventoId: "<uuid>") {
    id
    estado
  }
}
```

### Mutations de Compra

#### Crear compra (requiere auth)
```graphql
mutation {
  createCompra(
    eventoId: "<uuid>"
    tipoEntradaId: "<uuid>"
    cantidad: 2
    metodoPago: "yape"
  ) {
    id
    montoTotal
    estadoPago
  }
}
```

#### Procesar pago
```graphql
mutation {
  processPayment(
    compraId: "<uuid>"
    email: "juan@example.com"
  ) {
    transactionId
    status
    paymentUrl
  }
}
```

### Subscriptions

#### Cola virtual en tiempo real
```graphql
subscription {
  colaVirtual(eventoId: "<uuid>") {
    posicion
    estimadoMinutos
  }
}
```

---

## 8. API REST — Events Service

**Base URL:** `http://localhost:8081/api`

| Método | Endpoint | Descripción | Auth |
|--------|----------|-------------|------|
| `GET` | `/api/health` | Health check | No |
| `GET` | `/api/eventos` | Lista todos los eventos | No |
| `GET` | `/api/eventos/sendpulse` | Eventos en formato WhatsApp/SendPulse | No |

### Ejemplo: GET /api/eventos

```bash
curl http://localhost:8081/api/eventos
```

```json
{
  "success": true,
  "count": 3,
  "data": [
    {
      "id": "uuid",
      "titulo": "Concierto Rock 2026",
      "fecha": "2026-12-31",
      "hora": "20:00:00",
      "categoria": "musica",
      "distrito": "Miraflores",
      "provincia": "Lima",
      "aforo": 500
    }
  ]
}
```

### Ejemplo: GET /api/eventos/sendpulse

Retorna los eventos formateados como mensaje de WhatsApp para integración con **SendPulse**:

```json
{
  "success": true,
  "count": 3,
  "message": "🎉 *EVENTOS DISPONIBLES* 🎉\n\n1. *Concierto Rock 2026*\n🎭 musica\n📅 2026-12-31  ⏰ 20:00\n📍 Miraflores, Lima\n..."
}
```

---

## 9. Frontend

**URL:** `http://localhost:4321`

### Páginas disponibles

| Ruta | Descripción |
|------|-------------|
| `/` | Home — listado de eventos con scroll infinito |
| `/login` | Inicio de sesión |
| `/register` | Registro de usuario |
| `/buscar` | Búsqueda avanzada de eventos |
| `/categorias` | Explorar por categoría |
| `/eventos/[id]` | Detalle de un evento |
| `/crear-evento` | Formulario de creación de evento |
| `/mis-eventos` | Eventos del organizador |
| `/mis-tickets` | Entradas compradas |
| `/perfil` | Perfil del usuario |
| `/configuracion-cuenta` | Ajustes de cuenta |
| `/favoritos` | Eventos guardados |
| `/ayuda` | Centro de ayuda |

### Conexión al backend

El frontend se conecta al backend GraphQL mediante Apollo Client:

```typescript
// src/lib/apollo-client.ts
const httpLink = new HttpLink({
  uri: import.meta.env.PUBLIC_GRAPHQL_URL || 'http://localhost:8080/graphql',
});
```

La variable de entorno `PUBLIC_GRAPHQL_URL` se puede configurar para apuntar a producción.

---

## 10. Cómo Levantar el Proyecto

### Requisitos previos

- [Docker Desktop](https://www.docker.com/products/docker-desktop/) instalado y corriendo
- Git

### Pasos

```bash
# 1. Clonar el repositorio
git clone https://github.com/nesodev/teletickers
cd teletickers

# 2. Levantar todos los servicios (primera vez tarda ~5 min por el build de Rust)
docker compose up --build

# 3. En ejecuciones posteriores (sin rebuild)
docker compose up
```

### Servicios disponibles

| Servicio | URL | Descripción |
|----------|-----|-------------|
| 🌐 **Frontend** | http://localhost:4321 | Aplicación web |
| ⚡ **GraphQL API** | http://localhost:8080/graphql | API principal + Playground |
| 🔌 **Events API** | http://localhost:8081/api/eventos | API REST de eventos |
| 🗄️ **MinIO Console** | http://localhost:9901 | Gestión de archivos |
| 🐘 **PostgreSQL** | localhost:5432 | Base de datos |

### Credenciales por defecto

| Servicio | Usuario | Contraseña |
|----------|---------|------------|
| PostgreSQL | `admin` | `dontship` |
| MinIO | `minio_access_key` | `minio_secret_key` |

### Comandos útiles

```bash
# Ver logs de un servicio específico
docker compose logs backend -f
docker compose logs frontend -f
docker compose logs api-events -f

# Parar todos los servicios
docker compose down

# Parar y eliminar volúmenes (reset completo de BD)
docker compose down -v

# Reconstruir solo un servicio
docker compose up --build backend
```

---

## 11. Cronograma Kanban

El proyecto siguió metodología **Kanban** con 5 fases de flujo y límite WIP de 2 tareas por integrante.

### Resumen de Fases

| Fase | Semanas | Nombre | Objetivo |
|------|---------|--------|----------|
| **F1** | Sem. 3–4 | Investigación y Definición | Base académica y alcance tecnológico |
| **F2** | Sem. 5–7 | Marco Teórico y Arquitectura Base | Objetivos, metodología y arquitectura inicial |
| **F3** | Sem. 8–11 | Desarrollo Core: Backend | Backend funcional y ciclo completo |
| **F4** | Sem. 12–14 | Integración y App Móvil | Integración + arquitectura distribuida |
| **F5** | Sem. 15–17 | Validación, Documentación y Entrega | Pruebas, documentación y presentación |

### Vista Semanal

| Integrante | S3 | S4 | S5 | S6 | S7 | S8 | S9 | S10 | S11 | S12 | S13 | S14 | S15 | S16 |
|------------|----|----|----|----|----|----|----|----|-----|-----|-----|-----|-----|-----|
| **Ever** | | R | | | R | R | R | R | R | R | | R | R | R |
| **Rodrigo** | R | R | R | | R | R | R | R | R | R | | | R | R |
| **Fabrizzio** | R | R | | R | | R | R | R | R | R | R | | R | R |
| **Daniel** | | R | | | R | R | R | R | R | R | | R | R | R |
| **Josue** | | | R | R | | R | R | R | R | R | R | | R | R |

*R = Responsable Principal*

### Tarjetas Kanban por Fase

#### F1 — Investigación y Definición (Sem. 3–4)

| ID | Tarea | Responsable | Entregable |
|----|-------|-------------|------------|
| K-01 | Búsqueda de 20 artículos Scopus/WoS (Q1/Q2) | Rodrigo | 20 artículos clasificados |
| K-02 | Verificación de cuartiles Scimago | Fabrizzio | Lista validada con cuartiles |
| K-03 | Redacción del problema de investigación | Josue | Problema redactado |
| K-04 | Ratificación del título y configuración repositorio Git | **Ever** | Título aprobado + repo activo |
| K-05 | Configuración del tablero Kanban | Daniel | Tablero operativo |

#### F2 — Marco Teórico y Arquitectura Base (Sem. 5–7)

| ID | Tarea | Responsable | Entregable |
|----|-------|-------------|------------|
| K-06 | Redacción de justificación y objetivo general | Rodrigo | Objetivo general aprobado |
| K-07 | Definición de los 4 objetivos específicos | **Josue** | 4 objetivos documentados |
| K-08 | Documentación de metodología (Kanban) | Fabrizzio | Sección metodología |
| K-09 | Diseño de arquitectura monolítica modular | **Daniel** | Diagrama de arquitectura |
| K-10 | Búsqueda y adquisición del dataset de eventos | **Ever** | Dataset almacenado |

#### F3 — Desarrollo Core: Backend (Sem. 8–11)

| ID | Tarea | Responsable | Entregable |
|----|-------|-------------|------------|
| K-11 | Análisis exploratorio de datos (EDA) | **Ever** | Notebook EDA documentado |
| K-12 | Preprocesamiento y selección de features | Fabrizzio | Dataset limpio |
| K-13 | Entrenamiento Modelo 1: Random Forest | **Ever** | Modelo RF + métricas |
| K-14 | Entrenamiento Modelo 2: XGBoost/LightGBM | Fabrizzio | Modelo XGB + métricas |
| K-15 | Entrenamiento Modelo 3: Red Neuronal (MLP) | Josue | Modelo DL + métricas |
| K-16 | Tabla comparativa y selección del modelo óptimo | **Ever** | Modelo óptimo elegido |
| K-17 | Desarrollo endpoints GraphQL backend | **Daniel** | API funcional |
| K-18 | Diseño e implementación BD PostgreSQL | **Rodrigo** | BD operativa |

#### F4 — Integración y App Móvil (Sem. 12–14)

| ID | Tarea | Responsable | Entregable |
|----|-------|-------------|------------|
| K-19 | Despliegue del modelo ML en API | **Josue** | Endpoint de predicción |
| K-20 | Desarrollo MVP React Native (login, lista eventos) | Rodrigo | App con pantallas base |
| K-21 | Pantalla de compra de entradas | Fabrizzio | Flujo de compra funcional |
| K-22 | Pantalla de predicción de demanda | Josue | Pantalla IA visible |
| K-23 | Integración completa app ↔ backend | Fabrizzio | App integrada |
| K-24 | Migración a arquitectura distribuida (monorepo) | **Daniel** | Arquitectura desplegada |

#### F5 — Validación, Documentación y Entrega (Sem. 15–17)

| ID | Tarea | Responsable | Entregable |
|----|-------|-------------|------------|
| K-25 | Pruebas funcionales completas del sistema (QA) | **Ever** | Reporte sin bugs críticos |
| K-26 | Criterios de usabilidad y accesibilidad | Rodrigo | App verificada |
| K-27 | Redacción de resultados | Fabrizzio | Sección resultados |
| K-28 | Redacción de conclusiones | Rodrigo | Conclusiones alineadas |
| K-29 | Compilación referencias IEEE | Fabrizzio | Bibliografía IEEE |
| K-30 | Elaboración de presentación final | **Ever** | Presentación lista |
| K-31 | Revisión general y control de calidad | Daniel | Documento final entregado |

### Métricas de Flujo Kanban

| Métrica | Descripción | Objetivo |
|---------|-------------|----------|
| **Lead Time** | Desde Backlog hasta Hecho | Reducir progresivamente |
| **Cycle Time** | Desde En Progreso hasta Hecho | Identificar cuellos de botella |
| **Throughput** | Tarjetas completadas por semana | Mínimo 2–3 por semana |
| **WIP actual** | Tarjetas en progreso simultáneo | Máximo 2 por integrante |
| **Tasa de bloqueo** | % tarjetas que pasan por Bloqueado | No superar 20% |

---

## 12. Flujo de Trabajo Git

### Ramas del proyecto

| Rama | Propósito |
|------|-----------|
| `main` | Código estable e integrado |
| `ever` | Trabajo de Ever Avendaño |
| `rodrigo` | Trabajo de Rodrigo Calderón |
| `fabrizzio` | Trabajo de Fabrizzio Espinoza |
| `daniel` | Trabajo de Daniel Tintaya |
| `josue` | Trabajo de Josue Carbajal |

### Convención de commits

Los commits siguen el formato `K-XX: descripción` alineado con las tarjetas Kanban:

```
K-04: Ratificación del título y configuración inicial del repositorio
K-09: Diseño de arquitectura monolítica modular - estructura inicial src/
K-18: Entidades de base de datos PostgreSQL con SeaORM
K-17: GraphQL schema - queries y mutations para eventos, tickets y usuarios
K-24: Migración a monorepo - integración backend, api-events y frontend
K-30: Configurar CORS para permitir origen del frontend en producción
```

---

<div align="center">

**Universidad Nacional Mayor de San Marcos**  
Facultad de Ingeniería de Sistemas e Informática  
Escuela Profesional de Ingeniería de Software

*Lima, Perú — 2026*

</div>
