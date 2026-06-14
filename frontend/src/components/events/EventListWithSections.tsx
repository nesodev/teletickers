import { useQuery } from '@apollo/client/react';
import { useState, useEffect, useRef } from 'react';
import { GET_EVENTOS_PUBLICADOS } from '../../lib/graphql/queries';
import EventCard from './EventCard';
import type { Evento } from '../../types';

interface EventosData {
  eventosPublicados: Evento[];
}

export default function EventListWithSections() {
  const { data, loading, error } = useQuery<EventosData>(GET_EVENTOS_PUBLICADOS);
  const [displayedEvents, setDisplayedEvents] = useState<Evento[]>([]);
  const [page, setPage] = useState(1);
  const [hasMore, setHasMore] = useState(true);
  const [showCategories, setShowCategories] = useState(false);
  const observerTarget = useRef<HTMLDivElement>(null);
  const EVENTS_PER_PAGE = 6;
  const INITIAL_EVENTS = 9; // Mostrar 9 eventos iniciales antes de las categorías

  const eventos = data?.eventosPublicados || [];

  // Simulamos precios para la demo
  const eventosConPrecio = eventos.map(e => ({
    ...e,
    precioMinimo: Math.random() > 0.3 ? Math.floor(Math.random() * 100) : 0
  }));

  const eventosGratis = eventosConPrecio.filter(e => e.precioMinimo === 0);
  const eventosMenos15 = eventosConPrecio.filter(e => e.precioMinimo > 0 && e.precioMinimo <= 15);
  const eventosRestantes = eventosConPrecio.filter(e => e.precioMinimo > 15);

  // Cargar más eventos
  const loadMoreEvents = () => {
    if (!hasMore || loading) return;

    const startIndex = displayedEvents.length;
    const endIndex = startIndex + EVENTS_PER_PAGE;
    const newEvents = eventosRestantes.slice(startIndex, endIndex);

    if (newEvents.length === 0) {
      setHasMore(false);
      return;
    }

    setDisplayedEvents(prev => [...prev, ...newEvents]);

    if (endIndex >= eventosRestantes.length) {
      setHasMore(false);
    }
  };

  // Intersection Observer para scroll infinito
  useEffect(() => {
    const observer = new IntersectionObserver(
      entries => {
        if (entries[0].isIntersecting && hasMore && !loading && showCategories) {
          loadMoreEvents();
        }
      },
      { threshold: 0.1 }
    );

    const currentTarget = observerTarget.current;
    if (currentTarget) {
      observer.observe(currentTarget);
    }

    return () => {
      if (currentTarget) {
        observer.unobserve(currentTarget);
      }
    };
  }, [hasMore, loading, showCategories, displayedEvents.length]);

  // Cargar eventos iniciales
  useEffect(() => {
    if (eventosRestantes.length > 0 && displayedEvents.length === 0) {
      const initialEvents = eventosRestantes.slice(0, INITIAL_EVENTS);
      setDisplayedEvents(initialEvents);
      
      // Mostrar categorías después de cargar los eventos iniciales
      setTimeout(() => {
        setShowCategories(true);
      }, 300);
    }
  }, [eventosRestantes.length]);

  if (loading && displayedEvents.length === 0) {
    return (
      <div className="text-center py-8">
        <div className="inline-block h-8 w-8 animate-spin rounded-full border-4 border-solid border-blue-600 dark:border-blue-400 border-r-transparent"></div>
        <p className="mt-4 text-gray-700 dark:text-gray-300">Cargando eventos...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="text-red-500 dark:text-red-400 text-center py-8 bg-red-50 dark:bg-red-900/20 rounded-lg p-4">
        <p className="font-semibold">Error al cargar eventos</p>
        <p className="text-sm mt-2">{error.message}</p>
      </div>
    );
  }

  if (eventos.length === 0) {
    return (
      <div className="text-center py-12 bg-gray-50 dark:bg-gray-800 rounded-lg">
        <p className="text-gray-600 dark:text-gray-300 text-lg">No hay eventos disponibles</p>
        <p className="text-gray-500 dark:text-gray-400 text-sm mt-2">Vuelve pronto para ver nuevos eventos</p>
      </div>
    );
  }

  return (
    <div className="space-y-16">
      {/* Eventos principales - Solo los primeros */}
      <section>
        <div className="flex items-center justify-between mb-6">
          <h2 className="text-3xl font-bold text-gray-900 dark:text-white">Eventos destacados</h2>
          <span className="text-sm text-gray-500 dark:text-gray-400">
            {displayedEvents.length} eventos
          </span>
        </div>
        
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {displayedEvents.slice(0, INITIAL_EVENTS).map((evento, index) => (
            <div
              key={evento.id}
              className="animate-fade-in"
              style={{ animationDelay: `${index * 0.1}s` }}
            >
              <EventCard evento={evento} />
            </div>
          ))}
        </div>
      </section>

      {/* Secciones de categorías - Se muestran después de los eventos iniciales */}
      {showCategories && (
        <>
          {/* Eventos Gratis */}
          {eventosGratis.length > 0 && (
            <section className="bg-green-50 dark:bg-green-900/20 -mx-4 px-4 py-12 rounded-2xl transition-colors duration-300 animate-fade-in">
              <div className="container mx-auto">
                <div className="flex items-center gap-3 mb-6">
                  <div className="bg-green-500 dark:bg-green-600 text-white px-4 py-2 rounded-full font-bold">
                    GRATIS
                  </div>
                  <h2 className="text-3xl font-bold text-gray-900 dark:text-white">Eventos gratuitos</h2>
                </div>
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                  {eventosGratis.slice(0, 6).map((evento) => (
                    <EventCard key={evento.id} evento={evento} />
                  ))}
                </div>
                {eventosGratis.length > 6 && (
                  <div className="text-center mt-6">
                    <a
                      href="/buscar?precio=0"
                      className="inline-flex items-center gap-2 bg-green-600 dark:bg-green-500 text-white px-6 py-3 rounded-lg hover:bg-green-700 dark:hover:bg-green-600 transition-colors font-semibold"
                    >
                      Ver todos los eventos gratis
                      <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17 8l4 4m0 0l-4 4m4-4H3" />
                      </svg>
                    </a>
                  </div>
                )}
              </div>
            </section>
          )}

          {/* Eventos a menos de 15 soles */}
          {eventosMenos15.length > 0 && (
            <section className="bg-blue-50 dark:bg-blue-900/20 -mx-4 px-4 py-12 rounded-2xl transition-colors duration-300 animate-fade-in">
              <div className="container mx-auto">
                <div className="flex items-center gap-3 mb-6">
                  <div className="bg-blue-500 dark:bg-blue-600 text-white px-4 py-2 rounded-full font-bold">
                    ECONÓMICOS
                  </div>
                  <h2 className="text-3xl font-bold text-gray-900 dark:text-white">Eventos desde S/ 15</h2>
                </div>
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                  {eventosMenos15.slice(0, 6).map((evento) => (
                    <EventCard key={evento.id} evento={evento} />
                  ))}
                </div>
                {eventosMenos15.length > 6 && (
                  <div className="text-center mt-6">
                    <a
                      href="/buscar?precio=15"
                      className="inline-flex items-center gap-2 bg-blue-600 dark:bg-blue-500 text-white px-6 py-3 rounded-lg hover:bg-blue-700 dark:hover:bg-blue-600 transition-colors font-semibold"
                    >
                      Ver todos los eventos económicos
                      <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17 8l4 4m0 0l-4 4m4-4H3" />
                      </svg>
                    </a>
                  </div>
                )}
              </div>
            </section>
          )}

          {/* Más eventos con scroll infinito */}
          {displayedEvents.length > INITIAL_EVENTS && (
            <section>
              <div className="flex items-center justify-between mb-6">
                <h2 className="text-3xl font-bold text-gray-900 dark:text-white">Descubre más eventos</h2>
                <span className="text-sm text-gray-500 dark:text-gray-400">
                  {displayedEvents.length} de {eventosRestantes.length} eventos
                </span>
              </div>
              
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {displayedEvents.slice(INITIAL_EVENTS).map((evento, index) => (
                  <div
                    key={evento.id}
                    className="animate-fade-in"
                    style={{ animationDelay: `${(index % EVENTS_PER_PAGE) * 0.1}s` }}
                  >
                    <EventCard evento={evento} />
                  </div>
                ))}
              </div>
            </section>
          )}

          {/* Loader para scroll infinito */}
          {hasMore && displayedEvents.length >= INITIAL_EVENTS && (
            <div ref={observerTarget} className="text-center py-8">
              <div className="inline-block h-8 w-8 animate-spin rounded-full border-4 border-solid border-blue-600 dark:border-blue-400 border-r-transparent"></div>
              <p className="mt-4 text-gray-600 dark:text-gray-400">Cargando más eventos...</p>
            </div>
          )}

          {!hasMore && displayedEvents.length > INITIAL_EVENTS && (
            <div className="text-center py-8">
              <div className="bg-gradient-to-r from-blue-50 to-purple-50 dark:from-blue-900/20 dark:to-purple-900/20 rounded-2xl p-8">
                <p className="text-2xl font-bold text-gray-900 dark:text-white mb-2">
                  ✨ Has visto todos los eventos
                </p>
                <p className="text-gray-600 dark:text-gray-400 mb-4">
                  ¿No encontraste lo que buscabas?
                </p>
                <a
                  href="/buscar"
                  className="inline-flex items-center gap-2 bg-blue-600 dark:bg-blue-500 text-white px-6 py-3 rounded-lg hover:bg-blue-700 dark:hover:bg-blue-600 transition-colors font-semibold"
                >
                  Búsqueda avanzada
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                  </svg>
                </a>
              </div>
            </div>
          )}
        </>
      )}
    </div>
  );
}