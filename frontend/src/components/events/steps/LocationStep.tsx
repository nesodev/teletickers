interface Props {
  data: any;
  updateData: (data: any) => void;
  onNext: () => void;
  onBack: () => void;
}

export default function LocationStep({ data, updateData, onNext, onBack }: Props) {
  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onNext();
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      <h2 className="text-2xl font-bold mb-4">Ubicación del Evento</h2>

      <div>
        <label className="block text-sm font-medium mb-2">Región *</label>
        <input
          type="text"
          value={data.region}
          onChange={(e) => updateData({ region: e.target.value })}
          className="w-full px-4 py-2 border rounded-lg"
          placeholder="Lima"
          required
        />
      </div>

      <div>
        <label className="block text-sm font-medium mb-2">Provincia *</label>
        <input
          type="text"
          value={data.provincia}
          onChange={(e) => updateData({ provincia: e.target.value })}
          className="w-full px-4 py-2 border rounded-lg"
          placeholder="Lima"
          required
        />
      </div>

      <div>
        <label className="block text-sm font-medium mb-2">Distrito *</label>
        <input
          type="text"
          value={data.distrito}
          onChange={(e) => updateData({ distrito: e.target.value })}
          className="w-full px-4 py-2 border rounded-lg"
          placeholder="Miraflores"
          required
        />
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
          className="flex-1 bg-green-600 text-white py-3 rounded-lg hover:bg-green-700"
        >
          Continuar
        </button>
      </div>
    </form>
  );
}