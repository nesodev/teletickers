import { useState } from 'react';

const FAQ_CATEGORIES = [
  {
    title: 'Compra de Tickets',
    icon: '🎫',
    questions: [
      {
        q: '¿Cómo compro tickets?',
        a: 'Para comprar tickets, navega al evento que te interesa, selecciona la cantidad de entradas y procede al pago. Recibirás tus tickets digitales por correo electrónico.'
      },
      {
        q: '¿Puedo cancelar mi compra?',
        a: 'Las cancelaciones están sujetas a la política de cada evento. Revisa los términos específicos antes de comprar.'
      },
      {
        q: '¿Qué métodos de pago aceptan?',
        a: 'Aceptamos tarjetas de crédito, débito, Yape y Plin.'
      }
    ]
  },
  {
    title: 'Mi Cuenta',
    icon: '👤',
    questions: [
      {
        q: '¿Cómo cambio mi información personal?',
        a: 'Ve a Configuración de Cuenta desde el menú de tu perfil. Ahí podrás actualizar tu nombre, teléfono y DNI (este último solo una vez al mes).'
      },
      {
        q: '¿Por qué solo puedo cambiar mi DNI una vez al mes?',
        a: 'Por seguridad, limitamos los cambios de DNI a una vez cada 30 días para proteger tu cuenta contra uso fraudulento.'
      },
      {
        q: '¿Cómo recupero mi contraseña?',
        a: 'En la página de inicio de sesión, haz clic en "¿Olvidaste tu contraseña?" y sigue las instrucciones enviadas a tu correo.'
      }
    ]
  },
  {
    title: 'Crear Eventos',
    icon: '📅',
    questions: [
      {
        q: '¿Cómo creo un evento?',
a: 'Haz clic en "Crear Evento" en el header, completa la información del evento, configura los tickets y métodos de pago, y publica tu evento.'
},
{
q: '¿Cuánto cuesta publicar un evento?',
a: 'La publicación de eventos es gratuita. Solo cobramos una pequeña comisión por cada ticket vendido.'
},
{
q: '¿Puedo editar mi evento después de publicarlo?',
a: 'Sí, puedes editar la información de tu evento en cualquier momento desde "Mis Eventos".'
}
]
},
{
title: 'Problemas Técnicos',
icon: '🔧',
questions: [
{
q: 'No recibí mis tickets por correo',
a: 'Verifica tu carpeta de spam. Si no los encuentras, contacta a soporte con tu número de orden.'
},
{
q: 'El sitio web no carga correctamente',
a: 'Intenta limpiar la caché de tu navegador o usar un navegador diferente. Si el problema persiste, contáctanos.'
},
{
q: '¿Cómo descargo mis tickets?',
a: 'Puedes descargar tus tickets desde la sección "Mis Tickets" en tu perfil, o directamente desde el correo de confirmación.'
}
]
}
];
export default function Help() {
const [openQuestion, setOpenQuestion] = useState<string | null>(null);
const [contactForm, setContactForm] = useState({
nombre: '',
email: '',
asunto: '',
mensaje: ''
});
const toggleQuestion = (question: string) => {
setOpenQuestion(openQuestion === question ? null : question);
};
const handleContactSubmit = (e: React.FormEvent) => {
e.preventDefault();
// Aquí iría la lógica para enviar el mensaje
alert('Mensaje enviado. Nos pondremos en contacto contigo pronto.');
setContactForm({ nombre: '', email: '', asunto: '', mensaje: '' });
};
return (
<div className="container mx-auto px-4 py-8 max-w-6xl">
{/* Header */}
<div className="text-center mb-12">
<h1 className="text-5xl font-bold text-gray-900 dark:text-white mb-4">
¿Cómo podemos ayudarte?
</h1>
<p className="text-xl text-gray-600 dark:text-gray-300">
Encuentra respuestas rápidas o contáctanos directamente
</p>
</div>
{/* Búsqueda rápida */}
  <div className="max-w-2xl mx-auto mb-16">
    <div className="relative">
      <input
        type="text"
        placeholder="Buscar en ayuda..."
        className="w-full px-6 py-4 pr-12 rounded-2xl border-2 border-gray-200 dark:border-gray-600 focus:border-blue-500 dark:focus:border-blue-400 focus:outline-none text-lg shadow-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white placeholder-gray-500 dark:placeholder-gray-400 transition-colors"
      />
      <svg
        className="absolute right-4 top-1/2 -translate-y-1/2 w-6 h-6 text-gray-400"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth={2}
          d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
        />
      </svg>
    </div>
  </div>

  {/* Categorías de ayuda */}
  <div className="mb-16">
    <h2 className="text-3xl font-bold text-gray-900 dark:text-white mb-8">
      Preguntas Frecuentes
    </h2>
    <div className="space-y-6">
      {FAQ_CATEGORIES.map((category) => (
        <div
          key={category.title}
          className="bg-white dark:bg-gray-800 rounded-2xl shadow-lg border-2 border-gray-200 dark:border-gray-700 p-6"
        >
          <div className="flex items-center gap-3 mb-4">
            <span className="text-4xl">{category.icon}</span>
            <h3 className="text-2xl font-bold text-gray-900 dark:text-white">
              {category.title}
            </h3>
          </div>
          <div className="space-y-2">
            {category.questions.map((item, index) => (
              <div key={index} className="border-b border-gray-200 dark:border-gray-700 last:border-0">
                <button
                  onClick={() => toggleQuestion(`${category.title}-${index}`)}
                  className="w-full flex items-center justify-between py-4 text-left hover:bg-gray-50 dark:hover:bg-gray-700 rounded-lg px-4 transition-colors"
                >
                  <span className="font-medium text-gray-900 dark:text-white">
                    {item.q}
                  </span>
                  <svg
                    className={`w-5 h-5 text-gray-500 dark:text-gray-400 transition-transform ${
                      openQuestion === `${category.title}-${index}` ? 'rotate-180' : ''
                    }`}
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M19 9l-7 7-7-7"
                    />
                  </svg>
                </button>
                {openQuestion === `${category.title}-${index}` && (
                  <div className="px-4 pb-4 text-gray-600 dark:text-gray-300 animate-fade-in">
                    {item.a}
                  </div>
                )}
              </div>
            ))}
          </div>
        </div>
      ))}
    </div>
  </div>

  {/* Tarjetas de contacto rápido */}
  <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-16">
    <div className="bg-gradient-to-br from-blue-500 to-blue-600 rounded-2xl p-6 text-white">
      <div className="w-12 h-12 bg-white/20 rounded-full flex items-center justify-center mb-4">
        <svg className="w-6 h-6" fill="currentColor" viewBox="0 0 20 20">
          <path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z" />
          <path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z" />
        </svg>
      </div>
      <h3 className="text-xl font-bold mb-2">Email</h3>
      <p className="text-blue-100 mb-3">Respuesta en 24 horas</p>
      <a href="mailto:soporte@ticky.com" className="text-white underline font-medium">
        soporte@ticky.com
      </a>
    </div>

    <div className="bg-gradient-to-br from-green-500 to-green-600 rounded-2xl p-6 text-white">
      <div className="w-12 h-12 bg-white/20 rounded-full flex items-center justify-center mb-4">
        <svg className="w-6 h-6" fill="currentColor" viewBox="0 0 20 20">
          <path d="M2 3a1 1 0 011-1h2.153a1 1 0 01.986.836l.74 4.435a1 1 0 01-.54 1.06l-1.548.773a11.037 11.037 0 006.105 6.105l.774-1.548a1 1 0 011.059-.54l4.435.74a1 1 0 01.836.986V17a1 1 0 01-1 1h-2C7.82 18 2 12.18 2 5V3z" />
        </svg>
      </div>
      <h3 className="text-xl font-bold mb-2">WhatsApp</h3>
      <p className="text-green-100 mb-3">Respuesta inmediata</p>
      <a href="https://wa.me/51999999999" target="_blank" rel="noopener noreferrer" className="text-white underline font-medium">
        +51 999 999 999
      </a>
    </div>

    <div className="bg-gradient-to-br from-purple-500 to-purple-600 rounded-2xl p-6 text-white">
      <div className="w-12 h-12 bg-white/20 rounded-full flex items-center justify-center mb-4">
        <svg className="w-6 h-6" fill="currentColor" viewBox="0 0 20 20">
          <path fillRule="evenodd" d="M18 10c0 3.866-3.582 7-8 7a8.841 8.841 0 01-4.083-.98L2 17l1.338-3.123C2.493 12.767 2 11.434 2 10c0-3.866 3.582-7 8-7s8 3.134 8 7zM7 9H5v2h2V9zm8 0h-2v2h2V9zM9 9h2v2H9V9z" clipRule="evenodd" />
        </svg>
      </div>
      <h3 className="text-xl font-bold mb-2">Chat en Vivo</h3>
      <p className="text-purple-100 mb-3">Lun - Vie: 9am - 6pm</p>
      <button className="text-white underline font-medium">
        Iniciar chat
      </button>
    </div>
  </div>

  {/* Formulario de contacto */}
  <div className="bg-white dark:bg-gray-800 rounded-2xl shadow-lg border-2 border-gray-200 dark:border-gray-700 p-8">
    <h2 className="text-3xl font-bold text-gray-900 dark:text-white mb-2">
      ¿Aún necesitas ayuda?
    </h2>
    <p className="text-gray-600 dark:text-gray-300 mb-6">
      Envíanos un mensaje y te responderemos lo antes posible
    </p>
    <form onSubmit={handleContactSubmit} className="space-y-6">
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            Nombre Completo
          </label>
          <input
            type="text"
            required
            value={contactForm.nombre}
            onChange={(e) => setContactForm({ ...contactForm, nombre: e.target.value })}
            className="w-full px-4 py-3 border-2 border-gray-300 dark:border-gray-600 rounded-lg focus:border-blue-500 dark:focus:border-blue-400 focus:outline-none bg-white dark:bg-gray-700 text-gray-900 dark:text-white transition-colors"
            placeholder="Tu nombre"
          />
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            Correo Electrónico
          </label>
          <input
            type="email"
            required
            value={contactForm.email}
            onChange={(e) => setContactForm({ ...contactForm, email: e.target.value })}
            className="w-full px-4 py-3 border-2 border-gray-300 dark:border-gray-600 rounded-lg focus:border-blue-500 dark:focus:border-blue-400 focus:outline-none bg-white dark:bg-gray-700 text-gray-900 dark:text-white transition-colors"
            placeholder="tu@email.com"
          />
        </div>
      </div>
      <div>
        <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
          Asunto
        </label>
        <input
          type="text"
          required
          value={contactForm.asunto}
          onChange={(e) => setContactForm({ ...contactForm, asunto: e.target.value })}
          className="w-full px-4 py-3 border-2 border-gray-300 dark:border-gray-600 rounded-lg focus:border-blue-500 dark:focus:border-blue-400 focus:outline-none bg-white dark:bg-gray-700 text-gray-900 dark:text-white transition-colors"
          placeholder="¿En qué podemos ayudarte?"
        />
      </div>
      <div>
        <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
          Mensaje
        </label>
        <textarea
          required
          rows={6}
          value={contactForm.mensaje}
          onChange={(e) => setContactForm({ ...contactForm, mensaje: e.target.value })}
          className="w-full px-4 py-3 border-2 border-gray-300 dark:border-gray-600 rounded-lg focus:border-blue-500 dark:focus:border-blue-400 focus:outline-none bg-white dark:bg-gray-700 text-gray-900 dark:text-white transition-colors resize-none"
          placeholder="Describe tu problema o consulta..."
        />
      </div>
      <button
        type="submit"
        className="w-full bg-blue-600 dark:bg-blue-500 text-white py-4 rounded-xl hover:bg-blue-700 dark:hover:bg-blue-600 transition-colors font-bold text-lg shadow-lg hover:shadow-xl"
      >
        Enviar Mensaje
      </button>
    </form>
  </div>
</div>
);
}