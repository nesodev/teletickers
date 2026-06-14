// src/components/ui/AdvancedSearchBar.tsx
import { useState, useEffect, useRef } from 'react';
import { useQuery } from '@apollo/client/react';
import { SEARCH_EVENTOS } from '../../lib/graphql/queries';

const CATEGORIAS = [
  'Todos',
  'Concierto',
  'Conferencia',
  'Teatro',
  'Deportes',
  'Festival',
  'Otro',
];

const CIUDADES = [
  'Todas',
  'Lima',
  'Arequipa',
  'Cusco',
  'Trujillo',
  'Chiclayo',
  'Piura',
  'Ica',
];

const PRECIOS = [
  { label: 'Todos', value: 'all' },
  { label: 'Gratis', value: '0' },
  { label: 'Hasta S/ 15', value: '15' },
  { label: 'Hasta S/ 50', value: '50' },
  { label: 'Hasta S/ 100', value: '100' },
  { label: 'Más de S/ 100', value: '100+' },
];

interface SearchEventosData {
  searchEventos: Array<{
    id: string;
    titulo: string;
  }>;
}


export default function AdvancedSearchBar() {
  const [searchText, setSearchText] = useState('');
  const [showSuggestions, setShowSuggestions] = useState(false);
  const [ciudad, setCiudad] = useState('Todas');
  const [precio, setPrecio] = useState('all');
  const [categoria, setCategoria] = useState('Todos');
  const [suggestions, setSuggestions] = useState<string[]>([]);
  const searchRef = useRef<HTMLDivElement>(null);

  const { data } = useQuery<SearchEventosData>(SEARCH_EVENTOS, {
    variables: { query: searchText },
    skip: searchText.length < 2,
  });

  useEffect(() => {
    if (data?.searchEventos) {
      const titles = data.searchEventos
        .map((e: any) => e.titulo)
        .slice(0, 4);
      setSuggestions(titles);
    }
  }, [data]);

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (searchRef.current && !searchRef.current.contains(event.target as Node)) {
        setShowSuggestions(false);
      }
    };
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    const params = new URLSearchParams();
    if (searchText) params.append('q', searchText);
    if (ciudad !== 'Todas') params.append('ciudad', ciudad);
    if (precio !== 'all') params.append('precio', precio);
    if (categoria !== 'Todos') params.append('categoria', categoria);
    window.location.href = `/buscar?${params.toString()}`;
  };

  return (
    <form onSubmit={handleSearch} className="w-full max-w-6xl mx-auto mb-12">
      <div className="bg-white dark:bg-gray-800 rounded-2xl shadow-lg dark:shadow-gray-900/50 p-4 flex flex-wrap gap-4 items-center transition-colors duration-300">
        {/* Búsqueda de texto con sugerencias */}
        <div className="flex-1 min-w-[250px] relative" ref={searchRef}>
          <div className="relative">
            <svg
              className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400 dark:text-gray-500"
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
            <input
              type="text"
              value={searchText}
              onChange={(e) => {
                setSearchText(e.target.value);
                setShowSuggestions(true);
              }}
              onFocus={() => setShowSuggestions(true)}
              placeholder="Buscar eventos..."
              className="w-full pl-12 pr-4 py-3 border-2 border-gray-200 dark:border-gray-600 rounded-lg focus:border-green-500 dark:focus:border-green-400 focus:outline-none bg-white dark:bg-gray-700 text-gray-900 dark:text-white placeholder-gray-500 dark:placeholder-gray-400 transition-colors"
            />
          </div>

          {/* Sugerencias */}
          {showSuggestions && suggestions.length > 0 && (
            <div className="absolute z-10 w-full mt-2 bg-white dark:bg-gray-800 rounded-lg shadow-xl dark:shadow-gray-900/50 border border-gray-200 dark:border-gray-700">
              {suggestions.map((suggestion, index) => (
                <button
                  key={index}
                  type="button"
                  onClick={() => {
                    setSearchText(suggestion);
                    setShowSuggestions(false);
                  }}
                  className="w-full px-4 py-3 text-left hover:bg-green-50 dark:hover:bg-green-900/30 first:rounded-t-lg last:rounded-b-lg transition-colors text-gray-900 dark:text-white hover:text-green-700 dark:hover:text-green-400 font-medium"
                >
                  {suggestion}
                </button>
              ))}

            </div>
          )}
        </div>

        {/* Ciudad */}
        <div className="relative">
          <select
            value={ciudad}
            onChange={(e) => setCiudad(e.target.value)}
            className="appearance-none pl-4 pr-10 py-3 border-2 border-gray-200 dark:border-gray-600 rounded-lg focus:border-green-500 dark:focus:border-green-400 focus:outline-none cursor-pointer bg-white dark:bg-gray-700 text-gray-900 dark:text-white min-w-[150px] transition-colors"
          >
            {CIUDADES.map((c) => (
              <option key={c} value={c}>
                📍 {c}
              </option>
            ))}
          </select>
          <svg
            className="absolute right-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400 dark:text-gray-500 pointer-events-none"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
          </svg>
        </div>

        {/* Precio */}
        <div className="relative">
          <select
            value={precio}
            onChange={(e) => setPrecio(e.target.value)}
            className="appearance-none pl-4 pr-10 py-3 border-2 border-gray-200 dark:border-gray-600 rounded-lg focus:border-green-500 dark:focus:border-green-400 focus:outline-none cursor-pointer bg-white dark:bg-gray-700 text-gray-900 dark:text-white min-w-[150px] transition-colors"
          >
            {PRECIOS.map((p) => (
              <option key={p.value} value={p.value}>
                💰 {p.label}
              </option>
            ))}
          </select>
          <svg
            className="absolute right-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400 dark:text-gray-500 pointer-events-none"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
          </svg>
        </div>

        {/* Categoría */}
        <div className="relative">
          <select
            value={categoria}
            onChange={(e) => setCategoria(e.target.value)}
            className="appearance-none pl-4 pr-10 py-3 border-2 border-gray-200 dark:border-gray-600 rounded-lg focus:border-green-500 dark:focus:border-green-400 focus:outline-none cursor-pointer bg-white dark:bg-gray-700 text-gray-900 dark:text-white min-w-[150px] transition-colors"
          >
            {CATEGORIAS.map((cat) => (
              <option key={cat} value={cat}>
                🎭 {cat}
              </option>
            ))}
          </select>
          <svg
            className="absolute right-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400 dark:text-gray-500 pointer-events-none"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
          </svg>
        </div>

        {/* Botón de búsqueda */}
        <button
          type="submit"
          className="bg-green-600 dark:bg-green-500 text-white px-8 py-3 rounded-lg hover:bg-green-700 dark:hover:bg-green-600 transition-colors font-semibold whitespace-nowrap"
        >
          Buscar
        </button>
      </div>
    </form>
  );
}