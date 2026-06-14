export interface User {
  id: string;
  nombre: string;
  email: string;
  dni: string;
  numeroCel?: string;
}

export interface AuthPayload {
  user: User;
  token: string;
}

export interface Evento {
  id: string;
  titulo: string;
  descripcion?: string;
  fecha: string;
  hora: string;
  region: string;
  provincia: string;
  distrito: string;
  categoria: string;
  aforo: number;
  estado: string;
  miniatura?: string;
}

export interface TipoEntrada {
  id: string;
  nombre: string;
  precio: number;
}

export interface Entrada {
  id: string;
  qrCode: string;
  estado: string;
}

export interface Compra {
  id: string;
  montoTotal: number;
  estadoPago: string;
}