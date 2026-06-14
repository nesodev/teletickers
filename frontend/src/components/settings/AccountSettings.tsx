import { useState, useEffect } from 'react';

export default function AccountSettings() {
  const [user, setUser] = useState({
    nombre: '',
    email: '',
    telefono: '',
    dni: ''
  });
  const [isEditing, setIsEditing] = useState(false);
  const [showDniModal, setShowDniModal] = useState(false);
  const [dniLastChanged, setDniLastChanged] = useState<Date | null>(null);
  const [canChangeDni, setCanChangeDni] = useState(true);
  const [formData, setFormData] = useState({
    nombre: '',
    telefono: '',
    dni: ''
  });

  useEffect(() => {
    const userData = localStorage.getItem('user');
    if (userData) {
      const parsedUser = JSON.parse(userData);
      setUser(parsedUser);
      setFormData({
        nombre: parsedUser.nombre || '',
        telefono: parsedUser.telefono || '',
        dni: parsedUser.dni || ''
      });
    }

    // Verificar última vez que cambió el DNI
    const lastDniChange = localStorage.getItem('lastDniChange');
    if (lastDniChange) {
      const lastChangeDate = new Date(lastDniChange);
      setDniLastChanged(lastChangeDate);
      
      const daysSinceChange = Math.floor((Date.now() - lastChangeDate.getTime()) / (1000 * 60 * 60 * 24));
      setCanChangeDni(daysSinceChange >= 30);
    }
  }, []);

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setFormData(prev => ({ ...prev, [name]: value }));
  };

  const handleDniChange = () => {
    if (!canChangeDni) {
      alert('Solo puedes cambiar tu DNI una vez al mes');
      return;
    }
    setShowDniModal(true);
  };

  const confirmDniChange = () => {
    // Guardar cambio de DNI
    const updatedUser = { ...user, ...formData };
    localStorage.setItem('user', JSON.stringify(updatedUser));
    localStorage.setItem('lastDniChange', new Date().toISOString());
    
    setUser(updatedUser);
    setCanChangeDni(false);
    setDniLastChanged(new Date());
    setShowDniModal(false);
    setIsEditing(false);
    
    alert('DNI actualizado correctamente. Podrás cambiarlo nuevamente en 30 días.');
  };

  const handleSave = () => {
    // Guardar cambios (excepto DNI si no se puede cambiar)
    const dataToSave = { ...formData };
    
    if (!canChangeDni) {
      dataToSave.dni = user.dni; // Mantener DNI original
    }
    
    const updatedUser = { ...user, ...dataToSave };
    localStorage.setItem('user', JSON.stringify(updatedUser));
    setUser(updatedUser);
    setIsEditing(false);
    
    alert('Información actualizada correctamente');
  };

  const getDaysUntilDniChange = () => {
    if (!dniLastChanged || canChangeDni) return 0;
    const daysSinceChange = Math.floor((Date.now() - dniLastChanged.getTime()) / (1000 * 60 * 60 * 24));
    return Math.max(0, 30 - daysSinceChange);
  };

  return (
    <div className="container mx-auto px-4 py-8 max-w-4xl">
      {/* Header */}
      <div className="mb-8">
        <h1 className="text-4xl font-bold text-gray-900 dark:text-white mb-2">
          Configuración de Cuenta
        </h1>
        <p className="text-gray-600 dark:text-gray-300">
          Administra tu información personal y preferencias
        </p>
      </div>

      {/* Información Personal */}
      <div className="bg-white dark:bg-gray-800 rounded-2xl shadow-lg border-2 border-gray-200 dark:border-gray-700 p-6 mb-6">
        <div className="flex items-center justify-between mb-6">
          <h2 className="text-2xl font-bold text-gray-900 dark:text-white">Información Personal</h2>
          {!isEditing ? (
            <button
              onClick={() => setIsEditing(true)}
              className="flex items-center gap-2 bg-blue-600 dark:bg-blue-500 text-white px-4 py-2 rounded-lg hover:bg-blue-700 dark:hover:bg-blue-600 transition-colors"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
              </svg>
              Editar
            </button>
          ) : (
            <div className="flex gap-2">
              <button
                onClick={() => {
                  setIsEditing(false);
                  setFormData({
                    nombre: user.nombre,
                    telefono: user.telefono,
                    dni: user.dni
                  });
                }}
                className="px-4 py-2 border-2 border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
              >
                Cancelar
              </button>
              <button
                onClick={handleSave}
                className="flex items-center gap-2 bg-green-600 dark:bg-green-500 text-white px-4 py-2 rounded-lg hover:bg-green-700 dark:hover:bg-green-600 transition-colors"
              >
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                </svg>
                Guardar
              </button>
            </div>
          )}
        </div>

        <div className="space-y-6">
          {/* Nombre */}
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Nombre Completo
            </label>
            {isEditing ? (
              <input
                type="text"
                name="nombre"
                value={formData.nombre}
                onChange={handleInputChange}
                className="w-full px-4 py-3 border-2 border-gray-300 dark:border-gray-600 rounded-lg focus:border-blue-500 dark:focus:border-blue-400 focus:outline-none bg-white dark:bg-gray-700 text-gray-900 dark:text-white transition-colors"
              />
            ) : (
              <p className="text-lg text-gray-900 dark:text-white font-medium">{user.nombre || 'No especificado'}</p>
            )}
          </div>

          {/* Email (no editable) */}
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Correo Electrónico
            </label>
            <div className="flex items-center gap-2">
              <p className="text-lg text-gray-900 dark:text-white font-medium">{user.email}</p>
              <span className="px-2 py-1 bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-400 rounded text-xs">
                No editable
              </span>
            </div>
          </div>

          {/* Teléfono */}
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Teléfono
            </label>
            {isEditing ? (
              <input
                type="tel"
                name="telefono"
                value={formData.telefono}
                onChange={handleInputChange}
                placeholder="+51 999 999 999"
                className="w-full px-4 py-3 border-2 border-gray-300 dark:border-gray-600 rounded-lg focus:border-blue-500 dark:focus:border-blue-400 focus:outline-none bg-white dark:bg-gray-700 text-gray-900 dark:text-white transition-colors"
              />
            ) : (
              <p className="text-lg text-gray-900 dark:text-white font-medium">{user.telefono || 'No especificado'}</p>
            )}
          </div>

          {/* DNI */}
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              DNI
            </label>
            <div className="flex items-center gap-4">
              {isEditing ? (
                <div className="flex-1">
                  <input
                    type="text"
                    name="dni"
                    value={formData.dni}
                    onChange={handleInputChange}
                    maxLength={8}
                    placeholder="12345678"
                    disabled={!canChangeDni}
                    className={`w-full px-4 py-3 border-2 rounded-lg focus:outline-none transition-colors ${
                      canChangeDni
                        ? 'border-gray-300 dark:border-gray-600 focus:border-blue-500 dark:focus:border-blue-400 bg-white dark:bg-gray-700'
                        : 'border-gray-200 dark:border-gray-700 bg-gray-100 dark:bg-gray-800 cursor-not-allowed'
                    } text-gray-900 dark:text-white`}
                  />
                  {!canChangeDni && (
                    <p className="text-sm text-orange-600 dark:text-orange-400 mt-2 flex items-center gap-1">
                      <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                        <path fillRule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clipRule="evenodd" />
                      </svg>
                      Podrás cambiar tu DNI en {getDaysUntilDniChange()} días
                    </p>
                  )}
                </div>
              ) : (
                <p className="text-lg text-gray-900 dark:text-white font-medium flex-1">{user.dni || 'No especificado'}</p>
              )}
              {isEditing && formData.dni !== user.dni && canChangeDni && (
                <button
                  onClick={handleDniChange}
                  className="px-4 py-2 bg-orange-600 dark:bg-orange-500 text-white rounded-lg hover:bg-orange-700 dark:hover:bg-orange-600 transition-colors whitespace-nowrap"
                >
                  Confirmar DNI
                </button>
              )}
            </div>
            <p className="text-sm text-gray-500 dark:text-gray-400 mt-2">
              ⚠️ El DNI solo puede cambiarse una vez al mes
            </p>
          </div>
        </div>
      </div>

      {/* Modal de confirmación de DNI */}
      {showDniModal && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
          <div className="bg-white dark:bg-gray-800 rounded-2xl p-6 max-w-md w-full shadow-2xl">
            <div className="flex items-center gap-3 mb-4">
              <div className="w-12 h-12 bg-orange-100 dark:bg-orange-900/30 rounded-full flex items-center justify-center">
                <svg className="w-6 h-6 text-orange-600 dark:text-orange-400" fill="currentColor" viewBox="0 0 20 20">
                  <path fillRule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clipRule="evenodd" />
                </svg>
              </div>
              <h3 className="text-xl font-bold text-gray-900 dark:text-white">Confirmar cambio de DNI</h3>
            </div>
            <p className="text-gray-600 dark:text-gray-300 mb-2">
              Estás a punto de cambiar tu DNI a: <strong className="text-gray-900 dark:text-white">{formData.dni}</strong>
            </p>
            <p className="text-orange-600 dark:text-orange-400 font-medium mb-6">
              ⚠️ Esta acción solo se puede realizar una vez cada 30 días. ¿Estás seguro?
            </p>
            <div className="flex gap-3">
              <button
                onClick={() => setShowDniModal(false)}
                className="flex-1 px-4 py-3 border-2 border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors font-medium"
              >
                Cancelar
              </button>
              <button
                onClick={confirmDniChange}
                className="flex-1 px-4 py-3 bg-orange-600 dark:bg-orange-500 text-white rounded-lg hover:bg-orange-700 dark:hover:bg-orange-600 transition-colors font-medium"
              >
                Confirmar
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}