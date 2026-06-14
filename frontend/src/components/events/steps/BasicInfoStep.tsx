import { useState } from 'react';

interface Props {
  data: any;
  updateData: (data: any) => void;
  onNext: () => void;
}

export default function BasicInfoStep({ data, updateData, onNext }: Props) {
  const [imagePreview, setImagePreview] = useState<string | null>(data.miniatura || null);

  const handleImageChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      if (file.size > 5 * 1024 * 1024) {
        alert('La imagen no debe superar los 5MB');
        return;
      }

      // Validar tipo
      if (!file.type.startsWith('image/')) {
        alert('Solo se permiten archivos de imagen');
        return;
      }

      const reader = new FileReader();
      reader.onloadend = () => {
        const result = reader.result as string;
        setImagePreview(result);
        updateData({ miniatura: result });
      };
      reader.readAsDataURL(file);
    }
  };

  const removeImage = () => {
    setImagePreview(null);
    updateData({ miniatura: null });
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onNext();
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      <h2 className="text-2xl font-bold mb-4">Información del Evento</h2>

      <div>
        <label className="block text-sm font-medium mb-2">Título del evento *</label>
        <input
          type="text"
          value={data.titulo}
          onChange={(e) => updateData({ titulo: e.target.value })}
          className="w-full px-4 py-2 border rounded-lg"
          required
        />
      </div>

      <div>
        <label className="block text-sm font-medium mb-2">Descripción *</label>
        <textarea
          value={data.descripcion}
          onChange={(e) => updateData({ descripcion: e.target.value })}
          className="w-full px-4 py-2 border rounded-lg h-32"
          required
        />
      </div>

      {/* Miniatura */}
      <div>
        <label className="block text-sm font-medium mb-2">Imagen del evento</label>
        {imagePreview ? (
          <div className="relative">
            <img
              src={imagePreview}
              alt="Preview"
              className="w-full h-64 object-cover rounded-lg"
            />
            <button
              type="button"
              onClick={removeImage}
              className="absolute top-2 right-2 bg-red-500 text-white p-2 rounded-full hover:bg-red-600 transition-colors"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>
        ) : (
          <div className="border-2 border-dashed border-gray-300 rounded-lg p-8 text-center hover:border-green-500 transition-colors">
            <input
              type="file"
              accept="image/*"
              onChange={handleImageChange}
              className="hidden"
              id="image-upload"
            />
            <label
              htmlFor="image-upload"
              className="cursor-pointer flex flex-col items-center"
            >
              <svg
                className="w-12 h-12 text-gray-400 mb-4"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                />
              </svg>
              <span className="text-gray-600 font-medium">Haz clic para subir una imagen</span>
              <span className="text-gray-400 text-sm mt-1">PNG, JPG hasta 5MB</span>
            </label>
          </div>
        )}
      </div>

      <div>
        <label className="block text-sm font-medium mb-2">Categoría *</label>
        <select
          value={data.categoria}
          onChange={(e) => updateData({ categoria: e.target.value })}
          className="w-full px-4 py-2 border rounded-lg"
          required
        >
          <option value="">Selecciona una categoría</option>
          <option value="concierto">Concierto</option>
          <option value="conferencia">Conferencia</option>
          <option value="teatro">Teatro</option>
          <option value="deportes">Deportes</option>
          <option value="festival">Festival</option>
          <option value="otro">Otro</option>
        </select>
      </div>

      <div>
        <label className="block text-sm font-medium mb-2">
          Etiquetas (separadas por coma)
        </label>
        <input
          type="text"
          value={data.etiquetas.join(', ')}
          onChange={(e) =>
            updateData({ etiquetas: e.target.value.split(',').map((t) => t.trim()) })
          }
          className="w-full px-4 py-2 border rounded-lg"
          placeholder="música, rock, concierto"
        />
      </div>

      <div className="grid grid-cols-2 gap-4">
        <div>
          <label className="block text-sm font-medium mb-2">Fecha del evento *</label>
          <input
            type="date"
            value={data.fecha}
            onChange={(e) => updateData({ fecha: e.target.value })}
            className="w-full px-4 py-2 border rounded-lg"
            required
          />
        </div>
        <div>
          <label className="block text-sm font-medium mb-2">Hora *</label>
          <input
            type="time"
            value={data.hora}
            onChange={(e) => updateData({ hora: e.target.value + ':00' })}
            className="w-full px-4 py-2 border rounded-lg"
            required
          />
        </div>
      </div>

      <div>
        <label className="block text-sm font-medium mb-2">Restricción de edad *</label>
        <div className="space-y-2">
          <label className="flex items-center">
            <input
              type="radio"
              name="restriccion"
              value="todo_publico"
              checked={data.restriccion_edad === 'todo_publico'}
              onChange={(e) => updateData({ restriccion_edad: e.target.value })}
              className="mr-2"
            />
            Apto para todo el público
          </label>
          <label className="flex items-center">
            <input
              type="radio"
              name="restriccion"
              value="con_adulto"
              checked={data.restriccion_edad === 'con_adulto'}
              onChange={(e) => updateData({ restriccion_edad: e.target.value })}
              className="mr-2"
            />
            Menores con adulto responsable
          </label>
          <label className="flex items-center">
            <input
              type="radio"
              name="restriccion"
              value="solo_mayores"
              checked={data.restriccion_edad === 'solo_mayores'}
              onChange={(e) => updateData({ restriccion_edad: e.target.value })}
              className="mr-2"
            />
            Solo mayores de 18 años
          </label>
        </div>
      </div>

      <button
        type="submit"
        className="w-full bg-green-600 text-white py-3 rounded-lg hover:bg-green-700"
      >
        Continuar
      </button>
    </form>
  );
}