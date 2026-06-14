export function showToast(message: string, type: 'success' | 'error' | 'info' = 'success') {
  const existingToast = document.getElementById('custom-toast');
  if (existingToast) {
    existingToast.remove();
  }

  const colors = {
    success: 'bg-green-500',
    error: 'bg-red-500',
    info: 'bg-blue-500'
  };

  const icons = {
    success: `
      <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
      </svg>
    `,
    error: `
      <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
      </svg>
    `,
    info: `
      <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
      </svg>
    `
  };

  const toast = document.createElement('div');
  toast.id = 'custom-toast';
  toast.className = `fixed bottom-4 right-4 z-50 flex items-center gap-3 ${colors[type]} text-white px-6 py-4 rounded-xl shadow-2xl transform translate-x-0 opacity-0 transition-all duration-300 ease-out`;
  
  toast.innerHTML = `
    <div class="flex-shrink-0">
      ${icons[type]}
    </div>
    <p class="font-medium text-sm">${message}</p>
    <button onclick="this.parentElement.remove()" class="ml-2 hover:bg-white/20 rounded-lg p-1 transition-colors">
      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
      </svg>
    </button>
  `;

  document.body.appendChild(toast);

  setTimeout(() => {
    toast.classList.remove('opacity-0');
    toast.classList.add('opacity-100');
  }, 10);

  setTimeout(() => {
    toast.classList.add('opacity-0', 'translate-x-full');
    setTimeout(() => toast.remove(), 300);
  }, 3000);
}