import { useState } from 'react';
import { useMutation } from '@apollo/client/react';
import { LOGIN_MUTATION } from '../../lib/graphql/mutations';
import type { AuthPayload } from '../../types';

interface LoginData {
  login: AuthPayload;
}

export default function LoginForm() {
  const [formData, setFormData] = useState({
    email: '',
    password: '',
  });
  const [error, setError] = useState('');
  const [login, { loading }] = useMutation<LoginData>(LOGIN_MUTATION);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');

    try {
      const { data } = await login({ variables: formData });
      if (data) {
        localStorage.setItem('token', data.login.token);
        localStorage.setItem('user', JSON.stringify(data.login.user));
        window.location.href = '/';
      }
    } catch (err: any) {
      setError(err.message);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4 w-full max-w-md">
      <div>
        <label className="block text-sm font-medium mb-2">Email</label>
        <input
          type="email"
          value={formData.email}
          onChange={(e) => setFormData({ ...formData, email: e.target.value })}
          className="w-full px-4 py-2 border rounded-lg"
          required
        />
      </div>

      <div>
        <label className="block text-sm font-medium mb-2">Contraseña</label>
        <input
          type="password"
          value={formData.password}
          onChange={(e) => setFormData({ ...formData, password: e.target.value })}
          className="w-full px-4 py-2 border rounded-lg"
          required
        />
      </div>

      {error && <p className="text-red-500 text-sm">{error}</p>}

      <button
        type="submit"
        disabled={loading}
        className="w-full bg-green-600 text-white py-2 rounded-lg hover:bg-green-700 disabled:bg-gray-400"
      >
        {loading ? 'Iniciando sesión...' : 'Ingresar'}
      </button>

      <p className="text-center text-sm">
        ¿No tienes cuenta?{' '}
        <a href="/register" className="text-green-600 hover:underline">
          Regístrate
        </a>
      </p>
    </form>
  );
}