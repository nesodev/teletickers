interface Props {
  data: any;
  updateData: (data: any) => void;
  onSubmit: () => void;
  onBack: () => void;
  loading: boolean;
}

export default function TermsStep({ data, updateData, onSubmit, onBack, loading }: Props) {
  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (data.acepta_terminos) {
      onSubmit();
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      <h2 className="text-2xl font-bold mb-4">Términos y Condiciones</h2>

      <div className="bg-gray-50 p-6 rounded-lg h-64 overflow-y-scroll border">
        <h3 className="font-semibold mb-3">Términos del Servicio</h3>
        <div className="space-y-3 text-sm text-gray-700">
          <p>
            Al publicar un evento en Ticky, aceptas cumplir con las siguientes condiciones:
          </p>
          <ul className="list-disc pl-5 space-y-2">
            <li>La información del evento debe ser verídica y precisa</li>
            <li>Eres responsable de la realización del evento en la fecha indicada</li>
            <li>Ticky cobrará una comisión del 10% sobre cada venta</li>
            <li>Los reembolsos deben procesarse dentro de 48 horas de cancelación</li>
            <li>No se permite contenido ofensivo, discriminatorio o ilegal</li>
            <li>Debes proporcionar soporte a los compradores de entradas</li>
            <li>Ticky se reserva el derecho de cancelar eventos que violen estos términos</li>
          </ul>
          <p className="mt-4">
            Al continuar, confirmas que has leído y aceptas estos términos.
          </p>
        </div>
      </div>

      <label className="flex items-start">
        <input
          type="checkbox"
          checked={data.acepta_terminos}
          onChange={(e) => updateData({ acepta_terminos: e.target.checked })}
          className="w-5 h-5 mr-3 mt-1"
          required
        />
        <span className="text-sm">
          Acepto los términos y condiciones de Ticky. Confirmo que la información
          proporcionada es correcta y me comprometo a realizar el evento según lo especificado.
        </span>
      </label>

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
          disabled={!data.acepta_terminos || loading}
          className="flex-1 bg-green-600 text-white py-3 rounded-lg hover:bg-green-700 disabled:bg-gray-400"
        >
          {loading ? 'Creando evento...' : 'Crear Evento'}
        </button>
      </div>
    </form>
  );
}