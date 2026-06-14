import { useEffect, useState } from 'react';
import { useQuery } from '@apollo/client/react';
import { gql } from '@apollo/client';
import EventCard from './EventCard';
import type { Evento } from '../../types';

const GET_MIS_EVENTOS = gql`
  query MisEventos {
    misEventos {
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

interface MisEventosData {
  misEventos: Evento[];
}

export default function MyEvents() {
  const [isLoggedIn, setIsLoggedIn] = useState(false);

  useEffect(() => {
    const token = localStorage.getItem('token');
    if (!token) {
      window.location.href = '/login';
    } else {
      setIsLoggedIn(true);
    }
  }, []);

  const { data, loading, error, refetch } = useQuery<MisEventosData>(GET_MIS_EVENTOS, {
    skip: !isLoggedIn,
  });

  if (!isLoggedIn) return null;

  if (loading) {
    return (
      <div className="text-center py-12">
        <div className="inline-block h-12 w-12 animate-spin rounded-full border-4 border-solid border-green-600 border-r-transparent"></div>
        <p className="mt-4 text-gray-600 dark:text-gray-300">Cargando tus eventos...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="text-red-500 text-center py-8 bg-red-50 dark:bg-red-900/20 rounded-lg p-4">
        <p className="font-semibold">Error al cargar eventos</p>
        <p className="text-sm mt-2">{error.message}</p>
      </div>
    );
  }

  const eventos = data?.misEventos || [];
  const eventosBorrador = eventos.filter((e) => e.estado === 'borrador');
  const eventosPublicados = eventos.filter((e) => e.estado === 'publicado');
  const eventosCancelados = eventos.filter((e) => e.estado === 'cancelado');
  const eventosFinalizados = eventos.filter((e) => e.estado === 'finalizado');

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="flex justify-between items-center mb-8">
        <div>
          <h1 className="text-4xl font-bold mb-2 dark:text-white">Mis Eventos</h1>
          <p className="text-gray-500 dark:text-gray-400">Gestiona todos tus eventos creados</p>
        </div>

        <a
          href="/crear-evento"
          className="bg-green-600 dark:bg-green-500 text-white px-6 py-3 rounded-lg hover:bg-green-700 dark:hover:bg-green-600 transition-colors font-semibold flex items-center gap-2"
        >
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
          </svg>
          Crear Nuevo Evento
        </a>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
        <div className="bg-green-50 dark:bg-green-900/30 rounded-lg p-6 text-center border border-green-200 dark:border-green-800">
          <h2 className="text-2xl font-bold text-green-700 dark:text-green-400">{eventosPublicados.length}</h2>
          <p className="text-gray-600 dark:text-gray-400">Publicados</p>
        </div>
        <div className="bg-yellow-50 dark:bg-yellow-900/30 rounded-lg p-6 text-center border border-yellow-200 dark:border-yellow-800">
          <h2 className="text-2xl font-bold text-yellow-700 dark:text-yellow-400">{eventosBorrador.length}</h2>
          <p className="text-gray-600 dark:text-gray-400">Borradores</p>
        </div>
        <div className="bg-red-50 dark:bg-red-900/30 rounded-lg p-6 text-center border border-red-200 dark:border-red-800">
          <h2 className="text-2xl font-bold text-red-700 dark:text-red-400">{eventosCancelados.length}</h2>
          <p className="text-gray-600 dark:text-gray-400">Cancelados</p>
        </div>
        <div className="bg-blue-50 dark:bg-blue-900/30 rounded-lg p-6 text-center border border-blue-200 dark:border-blue-800">
          <h2 className="text-2xl font-bold text-blue-700 dark:text-blue-400">{eventosFinalizados.length}</h2>
          <p className="text-gray-600 dark:text-gray-400">Finalizados</p>
        </div>
      </div>

      {eventos.length === 0 ? (
        <div className="text-center py-16 bg-gray-50 dark:bg-gray-800 rounded-2xl">
          <svg className="w-24 h-24 mx-auto text-gray-400 dark:text-gray-600 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4" />
          </svg>
          <h3 className="text-2xl font-bold text-gray-900 dark:text-white mb-2">No tienes eventos creados</h3>
          <p className="text-gray-600 dark:text-gray-400 mb-6">Empieza creando tu primer evento</p>
          <a
            href="/crear-evento"
            className="inline-flex items-center gap-2 bg-green-600 dark:bg-green-500 text-white px-6 py-3 rounded-lg hover:bg-green-700 dark:hover:bg-green-600 transition-colors font-semibold"
          >
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
            </svg>
            Crear Evento
          </a>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {eventos.map((evento) => (
            <EventCard key={evento.id} evento={evento} onUpdate={refetch} />
          ))}
        </div>
      )}
    </div>
  );
}