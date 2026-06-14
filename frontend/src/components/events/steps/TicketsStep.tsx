import { useState } from 'react';

interface Props {
  data: any;
  updateData: (data: any) => void;
  onNext: () => void;
  onBack: () => void;
}

export default function TicketsStep({ data, updateData, onNext, onBack }: Props) {
  const [tickets, setTickets] = useState(data.entradas || []);
  const [currentTicket, setCurrentTicket] = useState({
    nombre: '',
    precio: 0,
    cantidad_disponible: 0,
    max_por_compra: 1,
    descripcion: '',
  });

  const addTicket = () => {
    if (currentTicket.nombre && currentTicket.precio > 0) {
      setTickets([...tickets, currentTicket]);
      setCurrentTicket({
        nombre: '',
        precio: 0,
        cantidad_disponible: 0,
        max_por_compra: 1,
        descripcion: '',
      });
    }
  };

  const removeTicket = (index: number) => {
    setTickets(tickets.filter((_: any, i: number) => i !== index));
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    updateData({ entradas: tickets });
    onNext();
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      <h2 className="text-2xl font-bold mb-4">Configurar Entradas</h2>

      <div>
        <label className="block text-sm font-medium mb-2">Aforo total del evento *</label>
        <input
          type="number"
          value={data.aforo}
          onChange={(e) => updateData({ aforo: parseInt(e.target.value) })}
          className="w-full px-4 py-2 border rounded-lg"
          min="1"
          required
        />
      </div>

      <div>
        <label className="block text-sm font-medium mb-2">
          Fecha límite de venta *
        </label>
        <input
          type="datetime-local"
          value={data.fecha_limite_venta}
          onChange={(e) => updateData({ fecha_limite_venta: e.target.value })}
          className="w-full px-4 py-2 border rounded-lg"
          required
        />
      </div>

      <div className="border-t pt-6">
        <h3 className="text-lg font-semibold mb-4">Agregar tipo de entrada</h3>

        <div className="space-y-4 bg-gray-50 p-4 rounded-lg mb-4">
          <div>
            <label className="block text-sm font-medium mb-2">Nombre de la entrada</label>
            <input
              type="text"
              value={currentTicket.nombre}
              onChange={(e) =>
                setCurrentTicket({ ...currentTicket, nombre: e.target.value })
              }
              className="w-full px-4 py-2 border rounded-lg"
              placeholder="General, VIP, Platea, etc."
            />
          </div>

          <div className="grid grid-cols-3 gap-4">
            <div>
              <label className="block text-sm font-medium mb-2">Precio (S/)</label>
              <input
                type="number"
                value={currentTicket.precio}
                onChange={(e) =>
                  setCurrentTicket({ ...currentTicket, precio: parseFloat(e.target.value) })
                }
                className="w-full px-4 py-2 border rounded-lg"
                min="0"
                step="0.01"
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-2">Cantidad</label>
              <input
                type="number"
                value={currentTicket.cantidad_disponible}
                onChange={(e) =>
                  setCurrentTicket({
                    ...currentTicket,
                    cantidad_disponible: parseInt(e.target.value),
                  })
                }
                className="w-full px-4 py-2 border rounded-lg"
                min="1"
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-2">Máx por persona</label>
              <input
                type="number"
                value={currentTicket.max_por_compra}
                onChange={(e) =>
                  setCurrentTicket({
                    ...currentTicket,
                    max_por_compra: parseInt(e.target.value),
                  })
                }
                className="w-full px-4 py-2 border rounded-lg"
                min="1"
              />
            </div>
          </div>

          <button
            type="button"
            onClick={addTicket}
            className="w-full bg-green-600 text-white py-2 rounded-lg hover:bg-green-700"
          >
            + Agregar entrada
          </button>
        </div>

        {tickets.length > 0 && (
          <div className="space-y-3">
            <h4 className="font-medium">Entradas agregadas:</h4>
            {tickets.map((ticket: any, index: number) => (
              <div
                key={index}
                className="flex justify-between items-center bg-green-50 p-4 rounded-lg"
              >
                <div>
                  <p className="font-semibold">{ticket.nombre}</p>
                  <p className="text-sm text-gray-600">
                    S/ {ticket.precio} • {ticket.cantidad_disponible} disponibles • Máx{' '}
                    {ticket.max_por_compra} por persona
                  </p>
                </div>
                <button
                  type="button"
                  onClick={() => removeTicket(index)}
                  className="text-red-600 hover:text-red-800"
                >
                  Eliminar
                </button>
              </div>
            ))}
          </div>
        )}
      </div>

      <div className="flex gap-4">
        <button
          type="button"
          onClick={onBack}
          className="flex-1 bg-gray-200 text-gray-700 py-3 rounded-lg hover:bg-gray-300"
        >
          Atrás
        </button>
        <button
          type="submit"
          disabled={tickets.length === 0}
          className="flex-1 bg-green-600 text-white py-3 rounded-lg hover:bg-green-700 disabled:bg-gray-400"
        >
          Continuar
        </button>
      </div>
    </form>
  );
}