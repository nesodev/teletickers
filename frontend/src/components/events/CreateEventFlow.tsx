import { useState } from 'react';
import { useMutation } from '@apollo/client/react';
import { CREATE_EVENTO } from '../../lib/graphql/mutations';
import BasicInfoStep from './steps/BasicInfoStep';
import LocationStep from './steps/LocationStep';
import TicketsStep from './steps/TicketsStep';
import PaymentMethodsStep from './steps/PaymentMethodsStep';
import TermsStep from './steps/TermsStep';

interface EventData {
  titulo: string;
  descripcion: string;
  categoria: string;
  etiquetas: string[];
  fecha: string;
  hora: string;
  restriccion_edad: string;
  region: string;
  provincia: string;
  distrito: string;
  aforo: number;
  entradas: Array<{
    nombre: string;
    precio: number;
    cantidad_disponible: number;
    max_por_compra: number;
    descripcion?: string;
  }>;
  fecha_limite_venta: string;
  metodos_pago: string[];
  acepta_terminos: boolean;
}

interface CreateEventoResponse {
  createEvento: {
    id: string;
    titulo: string;
  };
}



const INITIAL_DATA: EventData = {
  titulo: '',
  descripcion: '',
  categoria: '',
  etiquetas: [],
  fecha: '',
  hora: '',
  restriccion_edad: 'todo_publico',
  region: '',
  provincia: '',
  distrito: '',
  aforo: 0,
  entradas: [],
  fecha_limite_venta: '',
  metodos_pago: ['yape', 'plin', 'tarjeta'],
  acepta_terminos: false,
};



export default function CreateEventFlow() {
  const [step, setStep] = useState(1);
  const [formData, setFormData] = useState<EventData>(INITIAL_DATA);
  const [createEvento, { loading }] = useMutation<CreateEventoResponse>(CREATE_EVENTO);

  const updateFormData = (data: Partial<EventData>) => {
    setFormData({ ...formData, ...data });
  };

  const nextStep = () => setStep(step + 1);
  const prevStep = () => setStep(step - 1);

  const handleSubmit = async () => {
    try {
      const { data } = await createEvento({
        variables: {
          titulo: formData.titulo,
          descripcion: formData.descripcion,
          fecha: formData.fecha,
          hora: formData.hora,
          region: formData.region,
          provincia: formData.provincia,
          distrito: formData.distrito,
          categoria: formData.categoria,
          aforo: formData.aforo,
          etiquetas: formData.etiquetas,
          restriccionEdad: formData.restriccion_edad,
          miniatura: null,
        },
      });

      if (data) {
        window.location.href = `/eventos/${data.createEvento.id}`;
      }
    } catch (error: any) {
      console.error('Error creando evento:', error);
      alert('Error al crear el evento: ' + (error.message || 'Intenta de nuevo'));
    }
  };

  return (
    <div className="max-w-4xl mx-auto">
      <div className="mb-8">
        <div className="flex items-center justify-between">
          {[1, 2, 3, 4, 5].map((num) => (
            <div key={num} className="flex items-center">
              <div
                className={`w-10 h-10 rounded-full flex items-center justify-center ${
                  step >= num ? 'bg-green-600 text-white' : 'bg-gray-200 text-gray-600'
                }`}
              >
                {num}
              </div>
              {num < 5 && (
                <div
                  className={`w-20 h-1 ${step > num ? 'bg-green-600' : 'bg-gray-200'}`}
                />
              )}
            </div>
          ))}
        </div>
        <div className="flex justify-between mt-2 text-sm">
          <span>Información</span>
          <span>Ubicación</span>
          <span>Entradas</span>
          <span>Pago</span>
          <span>Términos</span>
        </div>
      </div>

      <div className="bg-white rounded-lg shadow-lg p-8">
        {step === 1 && (
          <BasicInfoStep
            data={formData}
            updateData={updateFormData}
            onNext={nextStep}
          />
        )}
        {step === 2 && (
          <LocationStep
            data={formData}
            updateData={updateFormData}
            onNext={nextStep}
            onBack={prevStep}
          />
        )}
        {step === 3 && (
          <TicketsStep
            data={formData}
            updateData={updateFormData}
            onNext={nextStep}
            onBack={prevStep}
          />
        )}
        {step === 4 && (
          <PaymentMethodsStep
            data={formData}
            updateData={updateFormData}
            onNext={nextStep}
            onBack={prevStep}
          />
        )}
        {step === 5 && (
          <TermsStep
            data={formData}
            updateData={updateFormData}
            onSubmit={handleSubmit}
            onBack={prevStep}
            loading={loading}
          />
        )}
      </div>
    </div>
  );
}