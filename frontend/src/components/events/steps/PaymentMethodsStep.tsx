interface Props {
  data: any;
  updateData: (data: any) => void;
  onNext: () => void;
  onBack: () => void;
}

export default function PaymentMethodsStep({ data, updateData, onNext, onBack }: Props) {
  const toggleMethod = (method: string) => {
    const current = data.metodos_pago || [];
    if (current.includes(method)) {
      updateData({ metodos_pago: current.filter((m: string) => m !== method) });
    } else {
      updateData({ metodos_pago: [...current, method] });
    }
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (data.metodos_pago.length > 0) {
      onNext();
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      <h2 className="text-2xl font-bold mb-4">Métodos de Pago</h2>
      <p className="text-gray-600 mb-6">
        Selecciona los métodos de pago que aceptarás para este evento
      </p>

      <div className="space-y-4">
        <label className="flex items-center p-4 border-2 rounded-lg cursor-pointer hover:bg-gray-50">
          <input
            type="checkbox"
            checked={data.metodos_pago?.includes('yape')}
            onChange={() => toggleMethod('yape')}
            className="w-5 h-5 mr-3"
          />
          <div>
            <p className="font-semibold">Yape</p>
            <p className="text-sm text-gray-600">Pagos instantáneos con Yape</p>
          </div>
        </label>

        <label className="flex items-center p-4 border-2 rounded-lg cursor-pointer hover:bg-gray-50">
          <input
            type="checkbox"
            checked={data.metodos_pago?.includes('plin')}
            onChange={() => toggleMethod('plin')}
            className="w-5 h-5 mr-3"
          />
          <div>
            <p className="font-semibold">Plin</p>
            <p className="text-sm text-gray-600">Pagos instantáneos con Plin</p>
          </div>
        </label>

        <label className="flex items-center p-4 border-2 rounded-lg cursor-pointer hover:bg-gray-50">
          <input
            type="checkbox"
            checked={data.metodos_pago?.includes('tarjeta')}
            onChange={() => toggleMethod('tarjeta')}
            className="w-5 h-5 mr-3"
          />
          <div>
            <p className="font-semibold">Tarjeta de crédito/débito</p>
            <p className="text-sm text-gray-600">Visa, Mastercard, American Express</p>
          </div>
        </label>
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
          disabled={!data.metodos_pago || data.metodos_pago.length === 0}
          className="flex-1 bg-green-600 text-white py-3 rounded-lg hover:bg-green-700 disabled:bg-gray-400"
        >
          Continuar
        </button>
      </div>
    </form>
  );
}