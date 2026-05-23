const CACHE_NAME = 'ticky-v1';
const STATIC_ASSETS = [
  '/',
  '/login',
  '/register',
  '/manifest.json',
];

// Instalar: cachear assets estáticos
self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME).then((cache) => {
      return cache.addAll(STATIC_ASSETS);
    })
  );
  self.skipWaiting();
});

// Activar: limpiar caches viejos
self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches.keys().then((keys) =>
      Promise.all(
        keys.filter((key) => key !== CACHE_NAME).map((key) => caches.delete(key))
      )
    )
  );
  self.clients.claim();
});

// Fetch: network-first para API, cache-first para assets
self.addEventListener('fetch', (event) => {
  const url = new URL(event.request.url);

  // Requests a la API GraphQL o api-events: siempre network
  if (url.port === '8080' || url.port === '8081') {
    return;
  }

  // Para navegación: network-first con fallback a cache
  if (event.request.mode === 'navigate') {
    event.respondWith(
      fetch(event.request).catch(() =>
        caches.match('/').then((cached) => cached || fetch(event.request))
      )
    );
    return;
  }

  // Para assets estáticos: cache-first
  event.respondWith(
    caches.match(event.request).then((cached) => {
      return cached || fetch(event.request).then((response) => {
        if (response.ok) {
          const clone = response.clone();
          caches.open(CACHE_NAME).then((cache) => cache.put(event.request, clone));
        }
        return response;
      });
    })
  );
});
