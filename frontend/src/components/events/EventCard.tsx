import { useState } from 'react';
import { useMutation } from '@apollo/client/react';
import { gql } from '@apollo/client';
import { showToast } from '../../utils/toast';
import type { Evento } from '../../types';

const PUBLISH_EVENTO = gql`
  mutation PublishEvento($eventoId: String!) {
    publishEvento(eventoId: $eventoId) {
      id
      estado
    }
  }
`;

const CANCEL_EVENTO = gql`
  mutation CancelEvento($eventoId: String!) {
    cancelEvento(eventoId: $eventoId) {
      id
      estado
    }
  }
`;

interface Props {
  evento: Evento;
  onUpdate?: () => void;
}

export default function EventCard({ evento, onUpdate }: Props) {
  const [showMenu, setShowMenu] = useState(false);
  const [publishEvento] = useMutation(PUBLISH_EVENTO);
  const [cancelEvento] = useMutation(CANCEL_EVENTO);

  const handlePublish = async (e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    
    try {
      await publishEvento({ variables: { eventoId: evento.id } });
      showToast('✅ Evento publicado exitosamente', 'success');
      onUpdate?.();
    } catch (error: any) {
      showToast(`❌ Error: ${error.message}`, 'error');
    }
  };

  const handleCancel = async (e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    
    if (!confirm('¿Estás seguro de cancelar este evento?')) return;
    
    try {
      await cancelEvento({ variables: { eventoId: evento.id } });
      showToast('🚫 Evento cancelado', 'info');
      onUpdate?.();
    } catch (error: any) {
      showToast(`❌ Error: ${error.message}`, 'error');
    }
  };

  const handleEdit = (e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    window.location.href = `/eventos/${evento.id}/editar`;
  };

  const getEstadoBadge = () => {
    const badges = {
      borrador: 'bg-yellow-100 dark:bg-yellow-900 text-yellow-800 dark:text-yellow-200',
      publicado: 'bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200',
      cancelado: 'bg-red-100 dark:bg-red-900 text-red-800 dark:text-red-200',
      finalizado: 'bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200',
    };
    return badges[evento.estado as keyof typeof badges] || badges.borrador;
  };

  return (
    <div className="relative group">
      <a
        href={`/eventos/${evento.id}`}
        className="block bg-white dark:bg-gray-800 rounded-lg shadow-md hover:shadow-xl transition-all duration-300 overflow-hidden border border-transparent dark:border-gray-700"
      >
        {evento.miniatura && (
          <div className="relative">
            <img
              src={evento.miniatura}
              alt={evento.titulo}
              className="w-full h-48 object-cover"
            />
            <div className="absolute top-2 left-2">
              <span className={`px-3 py-1 rounded-full text-xs font-semibold ${getEstadoBadge()}`}>
                {evento.estado}
              </span>
            </div>
          </div>
        )}
        <div className="p-4">
          <h3 className="font-bold text-xl mb-2 text-gray-900 dark:text-white">{evento.titulo}</h3>
          <p className="text-gray-600 dark:text-gray-300 text-sm mb-2 line-clamp-2">{evento.descripcion}</p>
          <div className="flex items-center gap-2 text-sm text-gray-500 dark:text-gray-400">
            <span>📅 {new Date(evento.fecha).toLocaleDateString()}</span>
            <span>🕐 {evento.hora}</span>
          </div>
          <div className="mt-2 text-sm text-gray-500 dark:text-gray-400">
            📍 {evento.distrito}, {evento.provincia}
          </div>
          <div className="mt-3 flex justify-between items-center">
            <span className="px-3 py-1 bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 rounded-full text-xs font-medium">
              {evento.categoria}
            </span>
            <span className="text-sm text-gray-600 dark:text-gray-400">Aforo: {evento.aforo}</span>
          </div>
        </div>
      </a>

      {/* Botón de menú */}
      <div className="absolute top-2 right-2">
        <button
          onClick={(e) => {
            e.preventDefault();
            setShowMenu(!showMenu);
          }}
          className="bg-white/90 dark:bg-gray-800/90 backdrop-blur-sm p-2 rounded-full shadow-lg hover:scale-110 transition-transform"
        >
          <svg className="w-5 h-5 text-gray-700 dark:text-gray-300" fill="currentColor" viewBox="0 0 20 20">
            <path d="M10 6a2 2 0 110-4 2 2 0 010 4zM10 12a2 2 0 110-4 2 2 0 010 4zM10 18a2 2 0 110-4 2 2 0 010 4z" />
          </svg>
        </button>

        {showMenu && (
          <>
            <div 
              className="fixed inset-0 z-10" 
              onClick={() => setShowMenu(false)}
            />
            <div className="absolute right-0 mt-2 w-48 bg-white dark:bg-gray-800 rounded-lg shadow-xl border border-gray-200 dark:border-gray-700 z-20">
              {evento.estado === 'borrador' && (
                <button
                  onClick={handlePublish}
                  className="w-full text-left px-4 py-3 hover:bg-green-50 dark:hover:bg-green-900/20 text-green-600 dark:text-green-400 font-medium flex items-center gap-2 rounded-t-lg transition-colors"
                >
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                  </svg>
                  Publicar
                </button>
              )}
              
              <button
                onClick={handleEdit}
                className="w-full text-left px-4 py-3 hover:bg-blue-50 dark:hover:bg-blue-900/20 text-blue-600 dark:text-blue-400 font-medium flex items-center gap-2 transition-colors"
              >
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                </svg>
                Configurar
              </button>

              {(evento.estado === 'publicado' || evento.estado === 'borrador') && (
                <button
                  onClick={handleCancel}
                  className="w-full text-left px-4 py-3 hover:bg-red-50 dark:hover:bg-red-900/20 text-red-600 dark:text-red-400 font-medium flex items-center gap-2 rounded-b-lg transition-colors"
                >
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                  </svg>
                  Cancelar evento
                </button>
              )}
            </div>
          </>
        )}
      </div>
    </div>
  );
}