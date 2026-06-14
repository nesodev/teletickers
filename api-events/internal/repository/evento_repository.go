package repository

import (
	"context"
	"database/sql"
	"fmt"

	"github.com/Teletickers/t-min-api-events/internal/models"
)

type EventoRepository struct {
	db *sql.DB
}

// NewEventoRepository crea un nuevo repositorio de eventos
func NewEventoRepository(db *sql.DB) *EventoRepository {
	return &EventoRepository{db: db}
}

// GetAll obtiene todos los eventos
func (r *EventoRepository) GetAll(ctx context.Context) ([]models.Evento, error) {
	query := `
        SELECT
            id, titulo, descripcion, fecha, hora, region, provincia,
            distrito, categoria, aforo, etiquetas,
            restriccion_edad, miniatura, metodos_pago_aceptados,
            estado, fecha_creacion, fecha_actualizacion
        FROM public.evento
        ORDER BY fecha DESC, hora DESC
    `

	rows, err := r.db.QueryContext(ctx, query)
	if err != nil {
		return nil, fmt.Errorf("error al ejecutar la consulta: %w", err)
	}
	defer rows.Close()

	var eventos []models.Evento

	for rows.Next() {
		var evento models.Evento
		err := rows.Scan(
			&evento.ID,
			&evento.Titulo,
			&evento.Descripcion,
			&evento.Fecha,
			&evento.Hora,
			&evento.Region,
			&evento.Provincia,
			&evento.Distrito,
			&evento.Categoria,
			&evento.Aforo,
			&evento.Etiquetas,
			&evento.RestriccionEdad,
			&evento.Miniatura,
			&evento.MetodosPagoAceptados,
			&evento.Estado,
			&evento.FechaCreacion,
			&evento.FechaActualizacion,
		)
		if err != nil {
			return nil, fmt.Errorf("error al escanear fila: %w", err)
		}
		eventos = append(eventos, evento)
	}

	if err = rows.Err(); err != nil {
		return nil, fmt.Errorf("error al iterar filas: %w", err)
	}

	return eventos, nil
}
