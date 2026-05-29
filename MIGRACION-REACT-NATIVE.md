# Guía Maestra de Migración: Frontend Web a React Native

**Proyecto:** TeleTickers
**Objetivo:** Migración total y paridad funcional 1:1 de Astro/React (Web) a React Native/Expo (Mobile).
**Fecha Límite:** 29 de Mayo de 2026.
**Audiencia:** Equipo de Desarrollo (Ever, Fabrizzio, Rodrigo, Daniel, Josue) y Asistentes de IA (Claude, Copilot, Cursor).

---

## 1. Alcance y Fuentes de Verdad

Este documento establece las especificaciones técnicas rigurosas para la migración del cliente web `frontend/` hacia un nuevo cliente móvil `mobile/` dentro del monorepo. **Se ha auditado el 100% de los archivos web y no existe ningún módulo sin asignar.** 

**ALERTA CRÍTICA PARA ASISTENTES DE IA:**
1.  **Cero Web Primitives:** Prohibido usar DOM (`document`, `window`), tags HTML (`div`, `span`, `svg`), o CSS puro (`Tailwind`). Use equivalentes de React Native (`View`, `Text`, `StyleSheet`, `react-native-svg`).
2.  **No Inventar Dependencias:** Limítese a usar las dependencias listadas en la Sección 3.
3.  **No Modificar Backend:** Los esquemas GraphQL (Rust) no se tocan. Todo payload enviado desde la app móvil debe coincidir tipográficamente con las mutaciones existentes.

---

## 2. Arquitectura de Directorios (`mobile/`)

```text
mobile/
├── App.tsx                    (Entry point: ApolloProvider, SafeAreaProvider, Toast)
├── src/
│   ├── components/            
│   │   ├── auth/              (Fabrizzio)
│   │   ├── categories/        (Daniel)
│   │   ├── events/            (Rodrigo)
│   │   ├── search/            (Josue)
│   │   ├── tickets/           (Daniel)
│   │   └── ui/                (Ever - Headers nativos y layout)
│   ├── hooks/                 
│   ├── lib/
│   │   ├── apollo-client.ts   
│   │   └── graphql/           (Copias exactas de queries.ts y mutations.ts)
│   ├── navigation/            (React Navigation: Root, Auth, Tab, Drawer)
│   ├── screens/               (Mapeo de src/pages/*.astro)
│   ├── styles/                (theme.ts)
│   ├── types/                 (Clonado de frontend/src/types/index.ts)
│   └── utils/                 (toast.ts, radixSortWasm.ts)
```

---

## 3. Dependencias Estrictas Aprobadas

```bash
# Core
npx create-expo-app@latest mobile --template blank-typescript

# Navegación y UI Base
npx expo install @react-navigation/native @react-navigation/stack @react-navigation/bottom-tabs @react-navigation/drawer
npx expo install react-native-screens react-native-safe-area-context
npx expo install react-native-reanimated react-native-gesture-handler expo-image react-native-svg

# Datos y Storage
npm install @apollo/client graphql
npx expo install expo-secure-store @react-native-async-storage/async-storage

# Utilidades Funcionales (Críticas)
npx expo install react-native-qrcode-svg @gorhom/bottom-sheet react-native-chart-kit react-native-toast-message expo-image-picker
```

---

## 4. Matriz Exhaustiva de Asignación de Tareas

A continuación se detalla la responsabilidad exacta sobre cada archivo del frontend original. **Si un archivo está en tu bloque, eres el único responsable de que funcione en móvil.**

### 4.1. Ever F. Avendaño - Infraestructura, UI Global y Navegación
*El bloque fundamental. Debe completarse primero.*

*   **Archivos Web de Origen:**
    *   `src/components/ui/BottomNav.tsx` -> `src/navigation/TabNavigator.tsx`
    *   `src/components/ui/Sidebar.tsx` -> `src/navigation/DrawerNavigator.tsx` (Si aplica)
    *   `src/components/ui/MobileHeader.tsx`, `Header.tsx` -> Custom headers integrados en el `screenOptions` de React Navigation.
    *   `src/utils/toast.ts` -> Utilidad nativa usando `react-native-toast-message`.
    *   `src/layouts/Layout.astro` -> Lógica de áreas seguras en `App.tsx`.
*   **Restricciones e Instrucciones IA:**
    *   **Toast:** Eliminar toda manipulación del DOM (`document.createElement`). Envolver la app con `<Toast />` y exportar un wrapper que invoque `Toast.show()`.
    *   **Apollo:** El middleware de auth debe extraer el JWT de `AsyncStorage` de forma asíncrona.
    *   **Layout:** Omitir el componente web `Footer.tsx`, no aplica en experiencia nativa de app.
    *   **SVGs Globales:** Convertir iconos raw a componentes de `react-native-svg` o `@expo/vector-icons`.

### 4.2. Fabrizzio Espinoza - Autenticación y Preferencias

*   **Archivos Web de Origen:**
    *   `src/pages/login.astro`, `Register.astro`, `configuracion-cuenta.astro` -> `LoginScreen`, `RegisterScreen`, `AccountSettingsScreen`.
    *   `src/components/auth/LoginForm.tsx`, `RegisterForm.tsx`
    *   `src/components/settings/AccountSettings.tsx`
*   **Restricciones e Instrucciones IA:**
    *   El DNI en `RegisterScreen` debe exigir teclado numérico estricto y limitar la longitud a 8 caracteres exactos.
    *   Redirecciones post-login (`window.location.href = '/'`) se prohíben. Utilizar el estado de sesión global para que React Navigation renderice automáticamente el stack de la aplicación (TabNavigator).

### 4.3. Rodrigo Calderón - Catálogo de Eventos (Core Business)

*   **Archivos Web de Origen:**
    *   `src/pages/index.astro`, `mis-eventos.astro`, `eventos/[id].astro`, `crear-evento.astro`.
    *   `src/components/events/*` (Incluyendo todos los subcomponentes en `steps/`).
    *   `src/components/ui/BannerCarousel.tsx` -> Refactorizar con `FlatList` horizontal paging-enabled.
*   **Restricciones e Instrucciones IA:**
    *   **Home:** Obligatorio usar `FlatList` con `RefreshControl` (pull-to-refresh).
    *   **Action Menus:** El dropdown web de la `EventCard` (publicar, cancelar) debe migrarse a un `ActionSheet` o un `<Modal>` inferior nativo.
    *   **Formularios Complejos:** Usar `KeyboardAvoidingView` en el flujo de 5 pasos (`CreateEventFlow`). Asegurar que las interfaces TypeScript coincidan al milímetro con el esquema GraphQL esperado.

### 4.4. Daniel Tintaya - Entradas Digitales, Favoritos y Clasificación

*   **Archivos Web de Origen:**
    *   `src/pages/mis-tickets.astro`, `favoritos.astro`, `categorias.astro`.
    *   `src/components/tickets/MyTickets.tsx`
    *   `src/components/favorites/Favorites.tsx`
    *   `src/components/categories/CategoriesPage.tsx`
*   **Restricciones e Instrucciones IA:**
    *   **Código QR (¡Crítico!):** Migrar cualquier vista o mock web por el componente `QRCode` de `react-native-qrcode-svg`. **El fondo del QR debe ser estrictamente blanco** (`backgroundColor="white"`) para garantizar escaneo físico. Se debe visualizar en un modal que oscurezca el fondo al 80%.
    *   **Gestos:** `FavoritesScreen` debe implementar Swipe-to-delete utilizando `Swipeable` de `react-native-gesture-handler`.

### 4.5. Josue Carbajal - Búsqueda, Dashboard Analítico y Ayuda

*   **Archivos Web de Origen:**
    *   `src/pages/buscar.astro`, `perfil.astro`, `ayuda.astro`.
    *   `src/components/search/*` (AdvancedSearchBar, SearchResults, SearchWithSort).
    *   `src/components/profile/ProfilePage.tsx`
    *   `src/components/help/Help.tsx`
    *   `src/utils/radixSortWasm.ts` (Usado en el ordenamiento de búsqueda).
*   **Restricciones e Instrucciones IA:**
    *   **Dashboard Analytics (¡Crítico!):** El componente web `ProfilePage.tsx` usa `recharts`. Como no hay DOM en móvil, **debes reescribir estas gráficas desde cero usando `react-native-chart-kit`**. Modifica los arreglos de datos para empatar con la interfaz de ChartKit.
    *   **WebAssembly (WASM):** React Native no soporta `.wasm` o `.zig` de forma nativa sin bridges complejos. En `radixSortWasm.ts`, mantén activa **únicamente la versión de respaldo en JavaScript** (`Array.prototype.sort`). No intentes linkear el archivo `radix_sort.zig`.
    *   **Búsqueda:** El `TextInput` debe poseer un debounce manual de 300-500ms para no sobrecargar el endpoint de GraphQL. Cierra el teclado en el inicio del scroll (`keyboardDismissMode="on-drag"`). Filtros complejos (`SearchWithSort`) van en un Bottom Sheet (`@gorhom/bottom-sheet`).

---

## 5. Reglas de Git y Flujo de Aprobación

1.  El desarrollo se paraliza en ramas: `feat/mobile/[nombre]/[feature]`.
2.  `App.tsx`, `theme.ts` y las carpetas de `navigation` son protegidas. Modificaciones cruzadas requieren aprobación de Ever.
3.  **Criterio de Cierre:** El Pull Request final hacia la rama de despliegue principal sólo se aprueba si pasa `npx tsc --noEmit` y la app no se crashea en un cold boot en Expo Go o emulador físico.
