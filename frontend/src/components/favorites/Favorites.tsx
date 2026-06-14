import { useState, useEffect } from 'react';
import { useQuery } from '@apollo/client/react';
import { GET_EVENTOS_PUBLICADOS } from '../../lib/graphql/queries';
import EventCard from '../events/EventCard';
import type { Evento } from '../../types';

interface EventosData {
  eventosPublicados: Evento[];
}

export default function Favorites() {
  const [favoriteIds, setFavoriteIds] = useState<string[]>([]);
  const { data, loading, error } = useQuery<EventosData>(GET_EVENTOS_PUBLICADOS);

  useEffect(() => {
    const favorites = JSON.parse(localStorage.getItem('favorites') || '[]');
    setFavoriteIds(favorites);
  }, []);

  const removeFavorite = (eventoId: string) => {
    const favorites = JSON.parse(localStorage.getItem('favorites') || '[]');
    const newFavorites = favorites.filter((id: string) => id !== eventoId);
    localStorage.setItem('favorites', JSON.stringify(newFavorites));
    setFavoriteIds(newFavorites);
  };

  if (loading) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="text-center py-12">
          <div className="inline-block h-12 w-12 animate-spin rounded-full border-4 border-solid border-green-600 dark:border-green-400 border-r-transparent"></div>
          <p className="mt-4 text-gray-700 dark:text-gray-300">Cargando tus favoritos...</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="text-red-500 dark:text-red-400 text-center py-8 bg-red-50 dark:bg-red-900/20 rounded-lg p-4">
          <p className="font-semibold">Error al cargar favoritos</p>
          <p className="text-sm mt-2">{error.message}</p>
        </div>
      </div>
    );
  }

  // Filtrar solo los eventos que están en favoritos
  const allEventos = data?.eventosPublicados || [];
  const favoritos = allEventos.filter(evento => favoriteIds.includes(evento.id));

  return (
    <div className="container mx-auto px-4 py-8">
      {/* Header */}
      <div className="mb-8">
        <div className="flex items-center gap-3 mb-3">
          <svg
            className="w-10 h-10 text-red-500 dark:text-red-400"
            fill="currentColor"
            viewBox="0 0 24 24"
          >
            <path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z" />
          </svg>
          <h1 className="text-4xl font-bold text-gray-900 dark:text-white">
            Mis Favoritos
          </h1>
        </div>
        <p className="text-gray-600 dark:text-gray-300">
          Eventos que guardaste para más tarde
        </p>
      </div>

      {/* Estadísticas */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-8">
        <div className="bg-gradient-to-br from-red-500 to-pink-500 rounded-xl p-6 text-white">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-red-100 text-sm font-medium">Total Favoritos</p>
              <p className="text-3xl font-bold mt-1">{favoritos.length}</p>
            </div>
            <svg
              className="w-12 h-12 text-red-200"
              fill="currentColor"
              viewBox="0 0 24 24"
            >
              <path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z" />
            </svg>
          </div>
        </div>

        <div className="bg-gradient-to-br from-blue-500 to-indigo-500 rounded-xl p-6 text-white">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-green-100 text-sm font-medium">Categorías</p>
              <p className="text-3xl font-bold mt-1">
                {new Set(favoritos.map(e => e.categoria)).size}
              </p>
            </div>
            <svg
              className="w-12 h-12 text-green-200"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z"
              />
            </svg>
          </div>
        </div>

        <div className="bg-gradient-to-br from-purple-500 to-pink-500 rounded-xl p-6 text-white">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-purple-100 text-sm font-medium">Próximos</p>
              <p className="text-3xl font-bold mt-1">
                {favoritos.filter(e => new Date(e.fecha) > new Date()).length}
              </p>
            </div>
            <svg
              className="w-12 h-12 text-purple-200"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
              />
            </svg>
          </div>
        </div>
      </div>

      {/* Lista de favoritos */}
      {favoritos.length === 0 ? (
        <div className="text-center py-16 bg-gray-50 dark:bg-gray-800 rounded-2xl">
          <svg
            className="w-24 h-24 mx-auto text-gray-400 dark:text-gray-600 mb-4"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"
            />
          </svg>
          <h3 className="text-2xl font-bold text-gray-900 dark:text-white mb-2">
            No tienes favoritos aún
          </h3>
          <p className="text-gray-600 dark:text-gray-400 mb-6">
            Explora eventos y guarda tus favoritos para verlos después
          </p>
          <a
            href="/"
            className="inline-flex items-center gap-2 bg-green-600 dark:bg-green-500 text-white px-6 py-3 rounded-lg hover:bg-green-700 dark:hover:bg-green-600 transition-colors font-semibold"
          >
            <svg
              className="w-5 h-5"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
              />
            </svg>
            Explorar Eventos
          </a>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {favoritos.map((evento) => (
            <div key={evento.id} className="relative group">
              <EventCard evento={evento} />
              <button
                className="absolute top-4 right-4 bg-white dark:bg-gray-800 p-2 rounded-full shadow-lg opacity-0 group-hover:opacity-100 transition-opacity duration-200 hover:scale-110 transform z-10"
                onClick={() => removeFavorite(evento.id)}
                title="Quitar de favoritos"
              >
                <svg
                  className="w-6 h-6 text-red-500"
                  fill="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z" />
                </svg>
              </button>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}