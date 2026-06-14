import { useState, useEffect } from 'react';
import { useQuery } from '@apollo/client/react';
import { gql } from '@apollo/client';
import { sortEventsByPrice, sortEventsByAforo } from '../../utils/radixSortWasm';
import EventCard from '../events/EventCard';

const SEARCH_EVENTOS = gql`
  query SearchEventos($query: String!) {
    searchEventos(query: $query) {
      id
      titulo
      descripcion
      fecha
      hora
      region
      provincia
      distrito
      categoria
      aforo
      estado
      miniatura
    }
  }
`;

interface Props {
  searchQuery: string;
}

export interface Evento {
  id: string;
  titulo: string;
  descripcion: string;
  fecha: string;
  hora: string;
  region: string;
  provincia: string;
  distrito: string;
  categoria: string;
  aforo: number;
  estado: string;
  miniatura: string;
}

export interface SearchEventosData {
  searchEventos: Evento[];
}

export default function SearchWithSort({ searchQuery }: Props) {
  const [sortedEvents, setSortedEvents] = useState<any[]>([]);
  const [sortBy, setSortBy] = useState<'none' | 'price' | 'aforo'>('none');
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc'>('asc');
  const [sorting, setSorting] = useState(false);

const { data, loading } = useQuery<SearchEventosData>(SEARCH_EVENTOS, {
    variables: { query: searchQuery },
    skip: !searchQuery,
  });

  useEffect(() => {
    if (data?.searchEventos) {
      setSortedEvents(data.searchEventos);
    }
  }, [data]);

  const handleSort = async (type: 'price' | 'aforo') => {
    if (!data?.searchEventos) return;

    setSorting(true);
    setSortBy(type);

    try {
      let sorted;
      if (type === 'price') {
        sorted = await sortEventsByPrice(data.searchEventos, sortOrder === 'asc');
      } else {
        sorted = await sortEventsByAforo(data.searchEventos, sortOrder === 'asc');
      }
      setSortedEvents(sorted);
    } catch (error) {
      console.error('Error al ordenar con WASM:', error);
    } finally {
      setSorting(false);
    }
  };

  const toggleSortOrder = () => {
    setSortOrder(prev => prev === 'asc' ? 'desc' : 'asc');
    if (sortBy !== 'none') {
      handleSort(sortBy);
    }
  };

  if (loading) {
    return (
      <div className="text-center py-12">
        <div className="inline-block h-12 w-12 animate-spin rounded-full border-4 border-solid border-blue-600 border-r-transparent"></div>
        <p className="mt-4 text-gray-600 dark:text-gray-300">Buscando eventos...</p>
      </div>
    );
  }

  return (
    <div>
      <div className="flex items-center gap-4 mb-6 bg-white dark:bg-gray-800 p-4 rounded-lg shadow">
        <span className="text-gray-700 dark:text-gray-300 font-medium">Ordenar por:</span>
        
        <button
          onClick={() => handleSort('price')}
          disabled={sorting}
          className={`px-4 py-2 rounded-lg font-medium transition-colors ${
            sortBy === 'price'
              ? 'bg-blue-600 text-white'
              : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600'
          }`}
        >
          💰 Precio {sortBy === 'price' && (sortOrder === 'asc' ? '↑' : '↓')}
        </button>

        <button
          onClick={() => handleSort('aforo')}
          disabled={sorting}
          className={`px-4 py-2 rounded-lg font-medium transition-colors ${
            sortBy === 'aforo'
              ? 'bg-blue-600 text-white'
              : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600'
          }`}
        >
          👥 Aforo {sortBy === 'aforo' && (sortOrder === 'asc' ? '↑' : '↓')}
        </button>

        {sortBy !== 'none' && (
          <button
            onClick={toggleSortOrder}
            className="px-4 py-2 bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors"
          >
            {sortOrder === 'asc' ? '↑ Ascendente' : '↓ Descendente'}
          </button>
        )}

        {sorting && (
          <span className="text-blue-600 dark:text-blue-400 flex items-center gap-2">
            <div className="w-4 h-4 border-2 border-blue-600 border-t-transparent rounded-full animate-spin"></div>
            Ordenando con WASM...
          </span>
        )}

        <div className="ml-auto text-sm text-gray-600 dark:text-gray-400">
          {sortedEvents.length} eventos encontrados
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {sortedEvents.map((evento) => (
          <EventCard key={evento.id} evento={evento} />
        ))}
      </div>

      {sortedEvents.length === 0 && (
        <div className="text-center py-16 bg-gray-50 dark:bg-gray-800 rounded-2xl">
          <svg className="w-24 h-24 mx-auto text-gray-400 dark:text-gray-600 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
          </svg>
          <h3 className="text-2xl font-bold text-gray-900 dark:text-white mb-2">No se encontraron eventos</h3>
          <p className="text-gray-600 dark:text-gray-400">Intenta con otra búsqueda</p>
        </div>
      )}
    </div>
  );
}