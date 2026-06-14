import { useState, useEffect } from 'react';

const BANNERS = [
  {
    id: 1,
    image: 'https://images.unsplash.com/photo-1540039155733-5bb30b53aa14?w=1200&h=400&fit=crop',
    title: 'Conciertos en Vivo',
    subtitle: 'Vive la mejor música en directo',
    backgroundColor: 'from-purple-600 to-pink-600',
  },
  {
    id: 2,
    image: 'https://images.unsplash.com/photo-1492684223066-81342ee5ff30?w=1200&h=400&fit=crop',
    title: 'Eventos Deportivos',
    subtitle: 'No te pierdas ningún partido',
    backgroundColor: 'from-green-600 to-teal-600',
  },
  {
    id: 3,
    image: 'https://images.unsplash.com/photo-1505236858219-8359eb29e329?w=1200&h=400&fit=crop',
    title: 'Teatro y Cultura',
    subtitle: 'Descubre las mejores obras',
    backgroundColor: 'from-red-600 to-orange-600',
  },
  {
    id: 4,
    image: 'https://images.unsplash.com/photo-1514525253161-7a46d19cd819?w=1200&h=400&fit=crop',
    title: 'Festivales y Más',
    subtitle: 'Experiencias inolvidables',
    backgroundColor: 'from-blue-600 to-cyan-600',
  },
  {
    id: 5,
    image: 'https://images.unsplash.com/photo-1501281668745-f7f57925c3b4?w=1200&h=400&fit=crop',
    title: 'Conferencias y Talleres',
    subtitle: 'Aprende con los mejores',
    backgroundColor: 'from-indigo-600 to-purple-600',
  },
];

export default function BannerCarousel() {
  const [currentIndex, setCurrentIndex] = useState(0);

  useEffect(() => {
    const interval = setInterval(() => {
      setCurrentIndex((prev) => (prev + 1) % BANNERS.length);
    }, 5000); // Cambia cada 5 segundos

    return () => clearInterval(interval);
  }, []);

  const goToSlide = (index: number) => {
    setCurrentIndex(index);
  };

  const goToPrevious = () => {
    setCurrentIndex((prev) => (prev - 1 + BANNERS.length) % BANNERS.length);
  };

  const goToNext = () => {
    setCurrentIndex((prev) => (prev + 1) % BANNERS.length);
  };

  return (
    <div className="relative w-full max-w-6xl mx-auto mb-8 rounded-2xl overflow-hidden shadow-2xl">
      {/* Banners */}
      <div className="relative h-[400px]">
        {BANNERS.map((banner, index) => (
          <div
            key={banner.id}
            className={`absolute inset-0 transition-opacity duration-700 ${
              index === currentIndex ? 'opacity-100' : 'opacity-0'
            }`}
          >
            {/* Imagen de fondo */}
            <img
              src={banner.image}
              alt={banner.title}
              className="w-full h-full object-cover"
            />
            
            {/* Overlay degradado */}
            <div className={`absolute inset-0 bg-gradient-to-r ${banner.backgroundColor} opacity-70`}></div>
            
            {/* Contenido */}
            <div className="absolute inset-0 flex flex-col justify-center items-center text-white text-center px-4">
              <h2 className="text-5xl font-bold mb-4 drop-shadow-lg animate-fade-in">
                {banner.title}
              </h2>
              <p className="text-2xl mb-8 drop-shadow-md animate-fade-in-delay">
                {banner.subtitle}
              </p>
              <button className="bg-white text-gray-900 px-8 py-3 rounded-full font-semibold hover:bg-gray-100 transition-all transform hover:scale-105 shadow-lg">
                Explorar Eventos
              </button>
            </div>
          </div>
        ))}
      </div>

      {/* Botones de navegación */}
      <button
        onClick={goToPrevious}
        className="absolute left-4 top-1/2 -translate-y-1/2 bg-white/30 hover:bg-white/50 backdrop-blur-sm text-white p-3 rounded-full transition-all"
      >
        <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
        </svg>
      </button>
      <button
        onClick={goToNext}
        className="absolute right-4 top-1/2 -translate-y-1/2 bg-white/30 hover:bg-white/50 backdrop-blur-sm text-white p-3 rounded-full transition-all"
      >
        <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
        </svg>
      </button>

      {/* Indicadores */}
      <div className="absolute bottom-6 left-1/2 -translate-x-1/2 flex gap-2">
        {BANNERS.map((_, index) => (
          <button
            key={index}
            onClick={() => goToSlide(index)}
            className={`w-3 h-3 rounded-full transition-all ${
              index === currentIndex
                ? 'bg-white w-8'
                : 'bg-white/50 hover:bg-white/75'
            }`}
          />
        ))}
      </div>
    </div>
  );
}