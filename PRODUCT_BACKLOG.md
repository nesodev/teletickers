# Product Backlog: TeleTickers App Móvil

Este documento define el Product Backlog estructurado en Épicas y User Stories para la migración de **TeleTickers** (Plataforma web de venta de entradas) hacia una aplicación móvil nativa (React Native/Expo), manteniendo integración directa con el Backend en Rust y la base de datos PostgreSQL.

---

## ÉPICA 1: Infraestructura Base y UI Global
*Fundación técnica de la app, navegación y componentes globales sin DOM.*

*   **US 1.1 - Navegación Principal:** Como usuario, quiero navegar entre las secciones principales (Home, Mis Tickets, Búsqueda, Perfil) usando una barra de pestañas inferior (Bottom Tabs) nativa.
*   **US 1.2 - Gestión de Estado Global (Apollo):** Como sistema, necesito conectarme al backend de Rust vía GraphQL extrayendo el JWT de `AsyncStorage` automáticamente en cada petición para no perder la sesión.
*   **US 1.3 - Notificaciones Toast:** Como usuario, quiero ver mensajes de éxito o error flotantes (toasts) que no dependan del DOM de la web para saber el estado de mis acciones.

## ÉPICA 2: Autenticación y Preferencias
*Módulo de acceso seguro validado con DNI peruano.*

*   **US 2.1 - Registro Nativo:** Como nuevo usuario, quiero registrarme ingresando mi DNI (con teclado numérico estricto limitado a 8 dígitos) y correo para crear mi cuenta.
*   **US 2.2 - Login Seguro:** Como asistente u organizador, quiero iniciar sesión en la app usando mis credenciales y ser redirigido al Home automáticamente sin recargar la pantalla.
*   **US 2.3 - Configuración de Cuenta:** Como usuario, quiero acceder a mis configuraciones para actualizar mi perfil desde el celular de manera cómoda.

## ÉPICA 3: Catálogo de Eventos (Core Business)
*Descubrimiento y creación de eventos.*

*   **US 3.1 - Feed Principal (Home):** Como usuario, quiero ver una lista de eventos disponibles y un carrusel de eventos destacados que pueda deslizar horizontalmente.
*   **US 3.2 - Refrescar Feed:** Como usuario, quiero arrastrar la pantalla principal hacia abajo (Pull-to-refresh) para obtener los eventos más recientes.
*   **US 3.3 - Creación de Eventos (Organizador):** Como organizador, quiero crear un evento mediante un formulario de 5 pasos asegurándome de que el teclado táctil no tape los campos de texto (`KeyboardAvoidingView`).
*   **US 3.4 - Acciones de Evento:** Como organizador, quiero tocar un botón de opciones en la tarjeta de mi evento para desplegar un menú inferior (Bottom Sheet) que me permita "Publicar" o "Cancelar" el evento.

## ÉPICA 4: Entradas Digitales, Pagos y Favoritos
*Gestión de compra, almacenamiento de tickets y código QR.*

*   **US 4.1 - Detalle y Compra:** Como asistente, quiero ver los detalles de un evento y acceder rápidamente al botón "Comprar Entradas" anclado en la parte inferior de la pantalla.
*   **US 4.2 - Billetera Digital (Mis Tickets):** Como asistente, quiero ver todas las entradas que he comprado almacenadas en la aplicación.
*   **US 4.3 - Código QR Seguro:** Como asistente, quiero que mi entrada muestre un código QR (generado nativamente con fondo blanco) que garantice la lectura en puerta de los escáneres físicos.
*   **US 4.4 - Favoritos (Swipe-to-delete):** Como usuario, quiero guardar eventos favoritos y poder eliminarlos de mi lista deslizando la tarjeta hacia la izquierda.

## ÉPICA 5: Búsqueda y Analíticas
*Encontrar eventos rápidamente y estadísticas para organizadores.*

*   **US 5.1 - Búsqueda en Tiempo Real:** Como usuario, quiero escribir en un buscador y que los resultados aparezcan sin congelar la app (usando debounce), cerrando el teclado al hacer scroll.
*   **US 5.2 - Filtros Avanzados:** Como usuario, quiero tocar un botón de filtros para que emerja un panel inferior (Bottom Sheet) con opciones de categorías y fechas.
*   **US 5.3 - Dashboard del Organizador:** Como organizador, quiero ver gráficas nativas de las ventas y asistencia de mis eventos (reemplazando las librerías web por gráficos adaptados a móvil).

---
**Nota Técnica:** El criterio de aceptación global para todas las User Stories es que pasen la compilación estricta de TypeScript (`npx tsc --noEmit`) y no presenten cierres inesperados (crashes) en entornos físicos o emuladores iOS/Android.
