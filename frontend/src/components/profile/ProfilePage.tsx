import { useEffect, useState } from 'react';
import { useQuery } from '@apollo/client/react';
import { gql } from '@apollo/client';
import {
  LineChart,
  Line,
  PieChart,
  Pie,
  Cell,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from 'recharts';

const GET_DASHBOARD_METRICS = gql`
  query DashboardMetrics {
    dashboardMetrics {
      totalEventos
      totalVentas
      ingresosTotales
      eventosActivos
    }
  }
`;

const COLORS = ['#3B82F6', '#10B981', '#F59E0B', '#EF4444', '#8B5CF6', '#EC4899'];

interface DashboardMetrics {
  totalEventos: number;
  totalVentas: number;
  ingresosTotales: number;
  eventosActivos: number;
}

interface DashboardData {
  dashboardMetrics: DashboardMetrics;
}

export default function ProfilePage() {
  const [user, setUser] = useState<any>(null);

  useEffect(() => {
    const token = localStorage.getItem('token');
    const userData = localStorage.getItem('user');
    
    if (!token) {
      window.location.href = '/login';
    } else if (userData) {
      setUser(JSON.parse(userData));
    }
  }, []);

  const { data, loading } = useQuery<DashboardData>(GET_DASHBOARD_METRICS);

  // Datos simulados para las gráficas
  const ventasMensuales = [
    { mes: 'Ene', ventas: 45, ingresos: 2400 },
    { mes: 'Feb', ventas: 52, ingresos: 3200 },
    { mes: 'Mar', ventas: 61, ingresos: 3800 },
    { mes: 'Abr', ventas: 78, ingresos: 4500 },
    { mes: 'May', ventas: 85, ingresos: 5200 },
    { mes: 'Jun', ventas: 92, ingresos: 6100 },
  ];

  const ventasPorCategoria = [
    { name: 'Conciertos', value: 35 },
    { name: 'Teatro', value: 20 },
    { name: 'Deportes', value: 18 },
    { name: 'Conferencias', value: 15 },
    { name: 'Festivales', value: 12 },
  ];

  const eventosPopulares = [
    { nombre: 'Concierto Rock Nacional', ventas: 245, ingresos: 12250 },
    { nombre: 'Festival de Verano 2024', ventas: 189, ingresos: 9450 },
    { nombre: 'Teatro: La Casa de Bernarda', ventas: 156, ingresos: 4680 },
    { nombre: 'Conferencia Tech Summit', ventas: 134, ingresos: 6700 },
    { nombre: 'Partido Clásico Peruano', ventas: 98, ingresos: 4900 },
  ];

  if (!user) {
    return null;
  }

  return (
    <div className="container mx-auto px-4 py-8">
      {/* Header con info del usuario */}
      <div className="bg-gradient-to-r from-green-500 to-green-700 rounded-2xl p-8 mb-8 text-white">
        <div className="flex items-center gap-6">
          <div className="w-24 h-24 bg-white rounded-full flex items-center justify-center text-green-600 text-4xl font-bold">
            {user.nombre.charAt(0).toUpperCase()}
          </div>
          <div>
            <h1 className="text-3xl font-bold mb-2">{user.nombre}</h1>
            <p className="text-green-100 mb-1">{user.email}</p>
            <p className="text-green-100">DNI: {user.dni}</p>
          </div>
        </div>
      </div>

      {/* Stats Cards */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
          <div className="flex items-center justify-between mb-4">
            <div className="bg-blue-100 p-3 rounded-lg">
              <svg className="w-6 h-6 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
              </svg>
            </div>
          </div>
          <p className="text-gray-600 dark:text-gray-400 text-sm mb-1">Total Eventos</p>
          <p className="text-3xl font-bold dark:text-white">{data?.dashboardMetrics.totalEventos || 0}</p>
        </div>

        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
          <div className="flex items-center justify-between mb-4">
            <div className="bg-green-100 p-3 rounded-lg">
              <svg className="w-6 h-6 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 5v2m0 4v2m0 4v2M5 5a2 2 0 00-2 2v3a2 2 0 110 4v3a2 2 0 002 2h14a2 2 0 002-2v-3a2 2 0 110-4V7a2 2 0 00-2-2H5z" />
              </svg>
            </div>
          </div>
          <p className="text-gray-600 dark:text-gray-400 text-sm mb-1">Total Ventas</p>
          <p className="text-3xl font-bold dark:text-white">{data?.dashboardMetrics.totalVentas || 0}</p>
        </div>

        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
          <div className="flex items-center justify-between mb-4">
            <div className="bg-yellow-100 p-3 rounded-lg">
              <svg className="w-6 h-6 text-yellow-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
          </div>
          <p className="text-gray-600 dark:text-gray-400 text-sm mb-1">Ingresos Totales</p>
          <p className="text-3xl font-bold dark:text-white">S/ {data?.dashboardMetrics.ingresosTotales || 0}</p>
        </div>

        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
          <div className="flex items-center justify-between mb-4">
            <div className="bg-purple-100 p-3 rounded-lg">
              <svg className="w-6 h-6 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6" />
              </svg>
            </div>
          </div>
          <p className="text-gray-600 dark:text-gray-400 text-sm mb-1">Eventos Activos</p>
          <p className="text-3xl font-bold dark:text-white">{data?.dashboardMetrics.eventosActivos || 0}</p>
        </div>
      </div>

      {/* Gráficas */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8 mb-8">
        {/* Ventas Mensuales */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
          <h2 className="text-xl font-bold mb-4 dark:text-white">Balance Mensual</h2>
          <ResponsiveContainer width="100%" height={300}>
            <LineChart data={ventasMensuales}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="mes" />
              <YAxis />
              <Tooltip />
              <Legend />
              <Line type="monotone" dataKey="ventas" stroke="#3B82F6" strokeWidth={2} />
            </LineChart>
          </ResponsiveContainer>
        </div>

        {/* Ganancias Mensuales */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
          <h2 className="text-xl font-bold mb-4 dark:text-white">Ganancia Mensual</h2>
          <ResponsiveContainer width="100%" height={300}>
            <LineChart data={ventasMensuales}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="mes" />
              <YAxis />
              <Tooltip />
              <Legend />
              <Line type="monotone" dataKey="ingresos" stroke="#10B981" strokeWidth={2} />
            </LineChart>
          </ResponsiveContainer>
        </div>
      </div>

      {/* Ventas por Categoría y Eventos Populares */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        {/* Ventas por Categoría */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
          <h2 className="text-xl font-bold mb-4 dark:text-white">Ventas por Categoría</h2>
          <ResponsiveContainer width="100%" height={300}>
            <PieChart>
              <Pie
                data={ventasPorCategoria}
                cx="50%"
                cy="50%"
                labelLine={false}
                label={({ name, percent = 0 }) => `${name} ${(percent * 100).toFixed(0)}%`}
                outerRadius={100}
                fill="#8884d8"
                dataKey="value"
              >
                {ventasPorCategoria.map((entry, index) => (
                  <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
                ))}
              </Pie>
              <Tooltip />
            </PieChart>
          </ResponsiveContainer>
        </div>

        {/* Eventos Populares */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
          <h2 className="text-xl font-bold mb-4 dark:text-white">Eventos Populares</h2>
          <div className="space-y-4">
            {eventosPopulares.map((evento, index) => (
              <div key={index} className="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
                <div className="flex items-center gap-4">
                  <div className="bg-green-500 text-white w-8 h-8 rounded-full flex items-center justify-center font-bold">
                    {index + 1}
                  </div>
                  <div>
                    <p className="font-semibold dark:text-white">{evento.nombre}</p>
                    <p className="text-sm text-gray-600 dark:text-gray-300">{evento.ventas} tickets vendidos</p>
                  </div>
                </div>
                <div className="text-right">
                  <p className="font-bold text-green-600">S/ {evento.ingresos.toLocaleString()}</p>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}