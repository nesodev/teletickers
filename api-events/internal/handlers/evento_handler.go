package handlers

import (
	"fmt"
	"net/http"

	"github.com/Teletickers/t-min-api-events/internal/models"
	"github.com/Teletickers/t-min-api-events/internal/repository"
	"github.com/gin-gonic/gin"
)

type EventoHandler struct {
	repo *repository.EventoRepository
}

// NewEventoHandler crea un nuevo handler de eventos
func NewEventoHandler(repo *repository.EventoRepository) *EventoHandler {
	return &EventoHandler{repo: repo}
}

// GetEventos maneja GET /api/eventos
func (h *EventoHandler) GetEventos(c *gin.Context) {
	// Obtiene todos los eventos
	eventos, err := h.repo.GetAll(c.Request.Context())
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"error":   "Error al obtener eventos",
			"message": err.Error(),
		})
		return
	}

	// Convierte a formato de respuesta
	response := make([]models.EventoResponse, 0, len(eventos))
	for _, evento := range eventos {
		response = append(response, evento.ToResponse())
	}

	c.JSON(http.StatusOK, gin.H{
		"success": true,
		"count":   len(response),
		"data":    response,
	})
}

// Retorna eventos en formato de mensaje único compatible con flujos de WhatsApp de SendPulse
func (h *EventoHandler) GetEventosForSendPulse(c *gin.Context) {
	// Obtiene todos los eventos
	eventos, err := h.repo.GetAll(c.Request.Context())
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"success": false,
			"message": "Lo siento, no pude cargar los eventos en este momento. Por favor intenta más tarde.",
		})
		return
	}

	// Si no hay eventos
	if len(eventos) == 0 {
		c.JSON(http.StatusOK, gin.H{
			"success": true,
			"count":   0,
			"message": "😔 Lo sentimos, actualmente no hay eventos disponibles.\n\n¡Te notificaremos cuando haya nuevos eventos!",
		})
		return
	}

	// Construye el mensaje formateado
	var mensaje string
	mensaje = "🎉 *EVENTOS DISPONIBLES* 🎉\n\n"
	mensaje += fmt.Sprintf("Tenemos %d eventos para ti:\n", len(eventos))
	mensaje += "━━━━━━━━━━━━━━━━━━━━━━\n\n"

	// Limita a los primeros 10 eventos para no hacer el mensaje muy largo
	maxEventos := len(eventos)
	if maxEventos > 10 {
		maxEventos = 10
	}

	for i, evento := range eventos[:maxEventos] {
		// Título del evento
		mensaje += fmt.Sprintf("*%d. %s*\n", i+1, evento.Titulo)

		// Categoría
		mensaje += fmt.Sprintf("🎭 %s\n", evento.Categoria)

		// Fecha y hora
		mensaje += fmt.Sprintf("📅 %s  ⏰ %s\n", evento.Fecha, evento.Hora)

		// Ubicación
		mensaje += fmt.Sprintf("📍 %s, %s\n", evento.Distrito, evento.Provincia)

		// Aforo
		mensaje += fmt.Sprintf("👥 Aforo: %d personas\n", evento.Aforo)

		// Descripción corta si existe
		if evento.Descripcion.Valid && evento.Descripcion.String != "" {
			desc := evento.Descripcion.String
			// Limita la descripción a 100 caracteres
			if len(desc) > 100 {
				desc = desc[:97] + "..."
			}
			mensaje += fmt.Sprintf("📝 %s\n", desc)
		}

		// ID para referencia
		mensaje += fmt.Sprintf("🆔 ID: %s\n", evento.ID)

		mensaje += "\n━━━━━━━━━━━━━━━━━━━━━━\n\n"
	}

	// Si hay más eventos
	if len(eventos) > 10 {
		mensaje += fmt.Sprintf("✨ Y %d eventos más disponibles...\n\n", len(eventos)-10)
	}

	mensaje += "💬 *Para más información sobre un evento, envía su número (ej: 1, 2, 3...)*"

	// Prepara arrays con datos individuales para acceso directo
	eventosData := make([]gin.H, 0, len(eventos))
	for _, evento := range eventos {
		eventData := gin.H{
			"id":        evento.ID,
			"titulo":    evento.Titulo,
			"fecha":     evento.Fecha,
			"hora":      evento.Hora,
			"categoria": evento.Categoria,
			"distrito":  evento.Distrito,
			"provincia": evento.Provincia,
			"region":    evento.Region,
			"aforo":     evento.Aforo,
		}

		if evento.Descripcion.Valid {
			eventData["descripcion"] = evento.Descripcion.String
		}

		if evento.Miniatura.Valid && evento.Miniatura.String != "" {
			eventData["imagen"] = evento.Miniatura.String
		}

		eventosData = append(eventosData, eventData)
	}

	c.JSON(http.StatusOK, gin.H{
		"success": true,
		"count":   len(eventos),
		"message": mensaje,
	})
}
