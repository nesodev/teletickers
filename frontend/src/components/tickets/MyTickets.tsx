// src/components/tickets/MyTickets.tsx
import { useEffect, useState } from 'react';
import { useQuery } from '@apollo/client/react';
import { GET_MIS_COMPRAS } from '../../lib/graphql/queries';

interface Compra {
  id: string;
  eventoId: string;
  montoTotal: number;
  metodoPago: string;
  estadoPago: string;
}

interface MisComprasData {
  misCompras: Compra[];
}

export default function MyTickets() {
  const [isLoggedIn, setIsLoggedIn] = useState(false);

  useEffect(() => {
    const token = localStorage.getItem('token');
    if (!token) {
      window.location.href = '/login';
    } else {
      setIsLoggedIn(true);
    }
  }, []);

  const { data, loading, error } = useQuery<MisComprasData>(GET_MIS_COMPRAS, {
    skip: !isLoggedIn,
  });

  if (!isLoggedIn) {
    return null;
  }

  if (loading) {
    return (
      <div className="text-center py-12">
        <div className="inline-block h-12 w-12 animate-spin rounded-full border-4 border-solid border-green-600 border-r-transparent"></div>
        <p className="mt-4 text-gray-500">Cargando tus tickets...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="text-red-500 text-center py-8 bg-red-50 rounded-lg p-4">
        <p className="font-semibold">Error al cargar tickets</p>
        <p className="text-sm mt-2">{error.message}</p>
      </div>
    );
  }

  const compras = data?.misCompras || [];
  const ticketsActivos = compras.filter((c) => c.estadoPago === 'pagado');
  const ticketsPendientes = compras.filter((c) => c.estadoPago === 'pendiente');
  const ticketsCancelados = compras.filter((c) => c.estadoPago === 'cancelado');

  return (
    <div className="container mx-auto px-4 py-8">
      {/* Header */}
      <div className="mb-8">
        <h1 className="text-4xl font-bold mb-2 dark:text-white">Mis Tickets</h1>
        <p className="text-gray-500">Todos tus boletos en un solo lugar</p>
      </div>

      {/* Stats */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
        <div className="bg-green-50 dark:bg-green-800 rounded-lg p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-600 dark:text-gray-300 mb-1">Tickets Activos</p>
              <p className="text-3xl font-bold text-green-600 dark:text-green-400">{ticketsActivos.length}</p>
            </div>
            <div className="bg-green-500 text-white p-3 rounded-full">
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
              </svg>
            </div>
          </div>
        </div>

        <div className="bg-yellow-50 dark:bg-yellow-800 rounded-lg p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-600 dark:text-gray-300 mb-1">Pendientes</p>
              <p className="text-3xl font-bold text-yellow-600 dark:text-yellow-400">{ticketsPendientes.length}</p>
            </div>
            <div className="bg-yellow-500 text-white p-3 rounded-full">
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
          </div>
        </div>

        <div className="bg-gray-50 dark:bg-gray-800 rounded-lg p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-600 dark:text-gray-300 mb-1">Total Compras</p>
              <p className="text-3xl font-bold text-gray-700 dark:text-gray-400">{compras.length}</p>
            </div>
            <div className="bg-gray-500 text-white p-3 rounded-full">
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 5v2m0 4v2m0 4v2M5 5a2 2 0 00-2 2v3a2 2 0 110 4v3a2 2 0 002 2h14a2 2 0 002-2v-3a2 2 0 110-4V7a2 2 0 00-2-2H5z" />
              </svg>
            </div>
          </div>
        </div>
      </div>

      {/* Tabs */}
      <div className="mb-6">
        <div className="border-b border-gray-200 dark:border-gray-600">
          <nav className="-mb-px flex space-x-8">
            <button className="border-b-2 border-green-500 py-4 px-1 text-green-600 font-medium">
              Activos ({ticketsActivos.length})
            </button>
            <button className="border-b-2 border-transparent py-4 px-1 text-gray-500 hover:text-gray-700 hover:border-gray-300">
              Pendientes ({ticketsPendientes.length})
            </button>
            <button className="border-b-2 border-transparent py-4 px-1 text-gray-500 hover:text-gray-700 hover:border-gray-300">
              Historial ({ticketsCancelados.length})
            </button>
          </nav>
        </div>
      </div>

      {/* Lista de tickets */}
      {ticketsActivos.length === 0 ? (
        <div className="text-center py-16 bg-gray-50 dark:bg-gray-800 rounded-lg">
          <svg
            className="w-20 h-20 mx-auto text-gray-400 dark:text-gray-500 mb-4"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M15 5v2m0 4v2m0 4v2M5 5a2 2 0 00-2 2v3a2 2 0 110 4v3a2 2 0 002 2h14a2 2 0 002-2v-3a2 2 0 110-4V7a2 2 0 00-2-2H5z"
            />
          </svg>
          <h3 className="text-xl font-semibold text-gray-700 dark:text-gray-400 mb-2">
            No tienes tickets aún
          </h3>
          <p className="text-gray-500 dark:text-gray-400 mb-6">
            Explora eventos y compra tus primeras entradas
          </p>
          <a
            href="/"
            className="inline-block bg-green-600 text-white px-6 py-3 rounded-lg hover:bg-green-700 transition-colors font-semibold"
          >
            Explorar Eventos
          </a>
        </div>
      ) : (
        <div className="space-y-4">
          {ticketsActivos.map((compra) => (
            <div
              key={compra.id}
              className="bg-white dark:bg-gray-900 border-2 border-gray-200 dark:border-gray-700 rounded-lg p-6 hover:border-green-500 transition-colors"
            >
              <div className="flex justify-between items-start">
                <div className="flex-1">
                  <div className="flex items-center gap-3 mb-2">
                    <span className="bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-400 px-3 py-1 rounded-full text-sm font-semibold">
                      ✓ Confirmado
                    </span>
                  </div>
                  <h3 className="text-xl font-bold text-gray-900 dark:text-gray-100 mb-2">Evento #{compra.eventoId.slice(0, 8)}</h3>
                  <div className="flex items-center gap-4 text-sm text-gray-500 dark:text-gray-400">
                    <span>💳 {compra.metodoPago}</span>
                    <span>💰 S/ {compra.montoTotal.toFixed(2)}</span>
                  </div>
                </div>
                <div className="flex gap-2">
                  <button className="bg-green-600 text-white px-4 py-2 rounded-lg hover:bg-green-700 transition-colors">
                    Ver Ticket
                  </button>
                  <button className="bg-gray-100 text-gray-700 dark:bg-gray-800 dark:text-gray-300 px-4 py-2 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors">
                    Descargar
                  </button>
                </div>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}