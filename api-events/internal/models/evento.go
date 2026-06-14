package models

import (
	"database/sql"
	"time"
)

type Evento struct {
	ID                   string         `json:"id"`
	Titulo               string         `json:"titulo"`
	Descripcion          sql.NullString `json:"descripcion,omitempty"`
	Fecha                string         `json:"fecha"`
	Hora                 string         `json:"hora"`
	Region               string         `json:"region"`
	Provincia            string         `json:"provincia"`
	Distrito             string         `json:"distrito"`
	OrganizadorID        string         `json:"organizador_id"`
	Categoria            string         `json:"categoria"`
	Aforo                int            `json:"aforo"`
	Etiquetas            sql.NullString `json:"etiquetas,omitempty"`
	RestriccionEdad      string         `json:"restriccion_edad"`
	Miniatura            sql.NullString `json:"miniatura,omitempty"`
	MetodosPagoAceptados []byte         `json:"metodos_pago_aceptados"`
	Estado               string         `json:"estado"`
	FechaCreacion        time.Time      `json:"fecha_creacion"`
	FechaActualizacion   time.Time      `json:"fecha_actualizacion"`
}

// EventoResponse es la estructura que se envía al cliente
type EventoResponse struct {
	ID                   string `json:"id"`
	Titulo               string `json:"titulo"`
	Descripcion          string `json:"descripcion,omitempty"`
	Fecha                string `json:"fecha"`
	Hora                 string `json:"hora"`
	Region               string `json:"region"`
	Provincia            string `json:"provincia"`
	Distrito             string `json:"distrito"`
	Categoria            string `json:"categoria"`
	Aforo                int    `json:"aforo"`
	Etiquetas            string `json:"etiquetas,omitempty"`
	RestriccionEdad      string `json:"restriccion_edad"`
	Miniatura            string `json:"miniatura,omitempty"`
	MetodosPagoAceptados string `json:"metodos_pago_aceptados"`
	Estado               string `json:"estado"`
	FechaCreacion        string `json:"fecha_creacion"`
	FechaActualizacion   string `json:"fecha_actualizacion"`
}

// ToResponse convierte un Evento a EventoResponse
func (e *Evento) ToResponse() EventoResponse {
	return EventoResponse{
		ID:                   e.ID,
		Titulo:               e.Titulo,
		Descripcion:          e.Descripcion.String,
		Fecha:                e.Fecha,
		Hora:                 e.Hora,
		Region:               e.Region,
		Provincia:            e.Provincia,
		Distrito:             e.Distrito,
		Categoria:            e.Categoria,
		Aforo:                e.Aforo,
		Etiquetas:            e.Etiquetas.String,
		RestriccionEdad:      e.RestriccionEdad,
		Miniatura:            e.Miniatura.String,
		MetodosPagoAceptados: string(e.MetodosPagoAceptados),
		Estado:               e.Estado,
		FechaCreacion:        e.FechaCreacion.Format(time.RFC3339),
		FechaActualizacion:   e.FechaActualizacion.Format(time.RFC3339),
	}
}
