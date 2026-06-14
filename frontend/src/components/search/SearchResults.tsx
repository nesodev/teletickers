import { useEffect, useState } from 'react';
import { useQuery } from '@apollo/client/react';
import { SEARCH_EVENTOS, GET_EVENTOS_PUBLICADOS } from '../../lib/graphql/queries';
import { sortEventsByAforo } from '../../utils/radixSortWasm';
import EventCard from '../events/EventCard';
import type { Evento } from '../../types';

interface EventosData {
  searchEventos?: Evento[];
  eventosPublicados?: Evento[];
}

export default function SearchResults() {
  const [searchParams, setSearchParams] = useState({
    q: '',
    ciudad: '',
    precio: '',
    categoria: '',
    sort: '',
    order: 'asc' as 'asc' | 'desc',
  });
  const [sortedEvents, setSortedEvents] = useState<Evento[]>([]);
  const [sorting, setSorting] = useState(false);

  useEffect(() => {
    const params = new URLSearchParams(window.location.search);
    setSearchParams({
      q: params.get('q') || '',
      ciudad: params.get('ciudad') || '',
      precio: params.get('precio') || '',
      categoria: params.get('categoria') || '',
      sort: params.get('sort') || '',
      order: (params.get('order') as 'asc' | 'desc') || 'asc',
    });
  }, []);

  const { data, loading, error } = useQuery<EventosData>(
    searchParams.q ? SEARCH_EVENTOS : GET_EVENTOS_PUBLICADOS,
    {
      variables: searchParams.q ? { query: searchParams.q } : undefined,
    }
  );

  useEffect(() => {
    if (data) {
      let eventos = data.searchEventos || data.eventosPublicados || [];

      if (searchParams.ciudad && searchParams.ciudad !== 'Todas') {
        eventos = eventos.filter(
          (e) =>
            e.region === searchParams.ciudad ||
            e.provincia === searchParams.ciudad ||
            e.distrito === searchParams.ciudad
        );
      }

      if (searchParams.categoria && searchParams.categoria !== 'Todos') {
        eventos = eventos.filter(
          (e) => e.categoria.toLowerCase() === searchParams.categoria.toLowerCase()
        );
      }

      if (searchParams.precio && searchParams.precio !== 'all') {
        const precioNum = parseInt(searchParams.precio);
        if (searchParams.precio === '0') {
          eventos = eventos.filter(() => Math.random() > 0.7);
        } else if (searchParams.precio === '100+') {
          eventos = eventos.filter(() => Math.random() > 0.5);
        } else {
          eventos = eventos.filter(() => Math.random() > 0.3);
        }
      }

      if (searchParams.sort === 'aforo') {
        handleSort(eventos, searchParams.order);
      } else {
        setSortedEvents(eventos);
      }
    }
  }, [data, searchParams]);

const handleSort = async (eventos: Evento[], order: 'asc' | 'desc') => {
  setSorting(true);
  try {
    console.log('Eventos antes de ordenar:', eventos.map(e => ({ titulo: e.titulo, aforo: e.aforo })));
    const sorted = await sortEventsByAforo(eventos, order === 'asc');
    console.log('Eventos después de ordenar:', sorted.map(e => ({ titulo: e.titulo, aforo: e.aforo })));
    setSortedEvents(sorted);
  } catch (error) {
    console.error('Error al ordenar con WASM:', error);
    setSortedEvents(eventos);
  } finally {
    setSorting(false);
  }
};

  const toggleSort = () => {
    const params = new URLSearchParams(window.location.search);
    const currentSort = params.get('sort');
    const currentOrder = params.get('order');

    if (currentSort === 'aforo') {
      const newOrder = currentOrder === 'asc' ? 'desc' : 'asc';
      params.set('sort', 'aforo');
      params.set('order', newOrder);
    } else {
      params.set('sort', 'aforo');
      params.set('order', 'asc');
    }

    window.location.search = params.toString();
  };

  const clearSort = () => {
    const params = new URLSearchParams(window.location.search);
    params.delete('sort');
    params.delete('order');
    window.location.search = params.toString();
  };

  if (loading) {
    return (
      <div className="text-center py-12">
        <div className="inline-block h-12 w-12 animate-spin rounded-full border-4 border-solid border-green-600 border-r-transparent"></div>
        <p className="mt-4 text-gray-600 dark:text-gray-300">Buscando eventos...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="text-red-500 text-center py-8 bg-red-50 dark:bg-red-900/20 rounded-lg p-4">
        <p className="font-semibold">Error al buscar eventos</p>
        <p className="text-sm mt-2">{error.message}</p>
      </div>
    );
  }

  const hasFilters = searchParams.q || searchParams.ciudad !== 'Todas' || 
                     searchParams.precio !== 'all' || searchParams.categoria !== 'Todos';

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold mb-2 dark:text-white">
          {searchParams.q ? `Resultados para "${searchParams.q}"` : 'Resultados de búsqueda'}
        </h1>
        <p className="text-gray-600 dark:text-gray-400">
          {sortedEvents.length} {sortedEvents.length === 1 ? 'evento encontrado' : 'eventos encontrados'}
        </p>

        {hasFilters && (
          <div className="flex flex-wrap gap-2 mt-4">
            {searchParams.ciudad && searchParams.ciudad !== 'Todas' && (
              <span className="bg-blue-100 dark:bg-blue-900/30 text-blue-800 dark:text-blue-300 px-3 py-1 rounded-full text-sm">
                📍 {searchParams.ciudad}
              </span>
            )}
            {searchParams.categoria && searchParams.categoria !== 'Todos' && (
              <span className="bg-purple-100 dark:bg-purple-900/30 text-purple-800 dark:text-purple-300 px-3 py-1 rounded-full text-sm">
                🎭 {searchParams.categoria}
              </span>
            )}
            {searchParams.precio && searchParams.precio !== 'all' && (
              <span className="bg-green-100 dark:bg-green-900/30 text-green-800 dark:text-green-300 px-3 py-1 rounded-full text-sm">
                💰 {searchParams.precio === '0' ? 'Gratis' : `Hasta S/ ${searchParams.precio}`}
              </span>
            )}
            <button
              onClick={() => (window.location.href = '/')}
              className="text-red-600 dark:text-red-400 hover:text-red-800 dark:hover:text-red-300 text-sm font-medium"
            >
              Limpiar filtros
            </button>
          </div>
        )}
      </div>

      <div className="mb-6 flex items-center justify-between">
        <div className="text-sm text-gray-600 dark:text-gray-400">
          {sortedEvents.length} resultados
        </div>
        
        <div className="flex items-center gap-2">
          {sorting && (
            <span className="text-blue-600 dark:text-blue-400 flex items-center gap-2 text-sm">
              <div className="w-3 h-3 border-2 border-blue-600 dark:border-blue-400 border-t-transparent rounded-full animate-spin"></div>
              Ordenando...
            </span>
          )}
          
          <button
            onClick={toggleSort}
            disabled={sorting}
            className={`px-3 py-1.5 rounded-md text-sm font-medium transition-colors flex items-center gap-1.5 ${
              searchParams.sort === 'aforo'
                ? 'bg-blue-600 dark:bg-blue-500 text-white'
                : 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600'
            }`}
          >
            👥 Aforo 
            {searchParams.sort === 'aforo' && (
              <span>{searchParams.order === 'asc' ? '↑' : '↓'}</span>
            )}
          </button>

          {searchParams.sort === 'aforo' && (
            <button
              onClick={clearSort}
              className="px-3 py-1.5 bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-300 rounded-md hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors text-sm"
            >
              ✕
            </button>
          )}
        </div>
      </div>

      {sortedEvents.length === 0 ? (
        <div className="text-center py-12 bg-gray-50 dark:bg-gray-800 rounded-lg">
          <svg
            className="w-16 h-16 mx-auto text-gray-400 dark:text-gray-600 mb-4"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
          <p className="text-gray-600 dark:text-gray-300 text-lg font-medium mb-2">No encontramos eventos</p>
          <p className="text-gray-500 dark:text-gray-400 text-sm mb-6">
            Intenta ajustar tus filtros o realiza una nueva búsqueda
          </p>
          <button
            onClick={() => (window.location.href = '/')}
            className="bg-blue-600 text-white px-6 py-2 rounded-lg hover:bg-blue-700 dark:bg-blue-500 dark:hover:bg-blue-600"
          >
            Ver todos los eventos
          </button>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {sortedEvents.map((evento) => (
            <EventCard key={evento.id} evento={evento} />
          ))}
        </div>
      )}
    </div>
  );
}