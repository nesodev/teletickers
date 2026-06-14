'use client';

import { useQuery } from '@apollo/client/react';
import { GET_EVENTOS_PUBLICADOS } from '../../lib/graphql/queries';
import EventCard from './EventCard';
import type { Evento } from '../../types';

interface EventosData {
  eventosPublicados: {
    eventos: Evento[];
    hasMore: boolean;
    total: number;
  };
}

export default function EventList() {
  const { data, loading, error } = useQuery<EventosData>(GET_EVENTOS_PUBLICADOS, {
    variables: { skip: 0, limit: 12 },
  });

  if (loading) {
    return (
      <div className="text-center py-8">
        <div className="inline-block h-8 w-8 animate-spin rounded-full border-4 border-solid border-green-600 dark:border-green-400 border-r-transparent"></div>
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

  const eventos = data?.eventosPublicados?.eventos || [];

  if (eventos.length === 0) {
    return (
      <div className="text-center py-12 bg-gray-50 dark:bg-gray-800 rounded-lg">
        <p className="text-gray-600 dark:text-gray-300 text-lg">No hay eventos disponibles</p>
        <p className="text-gray-500 dark:text-gray-400 text-sm mt-2">Vuelve pronto para ver nuevos eventos</p>
      </div>
    );
  }

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {eventos.map((evento) => (
        <EventCard key={evento.id} evento={evento} />
      ))}
    </div>
  );
}
