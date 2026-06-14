import { gql } from '@apollo/client';

export const GET_EVENTOS_PUBLICADOS = gql`
  query EventosPublicados {
    eventosPublicados {
      id
      titulo
      descripcion
      fecha
      hora
      region
      provincia
      distrito
      categoria
      aforo
      estado
      miniatura
    }
  }
`;

export const GET_EVENTO = gql`
  query Evento($id: String!) {
    evento(id: $id) {
      id
      titulo
      descripcion
      fecha
      hora
      region
      provincia
      distrito
      categoria
      aforo
      estado
      miniatura
    }
  }
`;

export const SEARCH_EVENTOS = gql`
  query SearchEventos($query: String!) {
    searchEventos(query: $query) {
      id
      titulo
      descripcion
      fecha
      hora
      region
      provincia
      distrito
      categoria
      aforo
      estado
      miniatura
    }
  }
`;

export const GET_MIS_EVENTOS = gql`
  query MisEventos {
    misEventos {
      id
      titulo
      descripcion
      fecha
      hora
      region
      provincia
      distrito
      categoria
      aforo
      estado
      miniatura
    }
  }
`;

export const GET_MIS_COMPRAS = gql`
  query MisCompras {
    misCompras {
      id
      eventoId
      montoTotal
      metodoPago
      estadoPago
    }
  }
`;

export const GET_DASHBOARD_METRICS = gql`
  query DashboardMetrics {
    dashboardMetrics {
      totalEventos
      totalVentas
      ingresosTotales
      eventosActivos
    }
  }
`;

export const GET_MIS_FAVORITOS = gql`
  query MisFavoritos {
    misFavoritos {
      id
      titulo
      descripcion
      fecha
      hora
      region
      provincia
      distrito
      categoria
      aforo
      estado
      miniatura
    }
  }
`;