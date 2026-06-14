package main

import (
	"log"

	"github.com/Teletickers/t-min-api-events/internal/config"
	"github.com/Teletickers/t-min-api-events/internal/database"
	"github.com/Teletickers/t-min-api-events/internal/handlers"
	"github.com/Teletickers/t-min-api-events/internal/repository"
	"github.com/gin-gonic/gin"
)

func main() {
	// Carga la configuración
	cfg := config.LoadConfig()

	// Conecta a la base de datos
	db, err := database.NewPostgresDB(cfg.DatabaseURL)
	if err != nil {
		log.Fatal("Error al conectar a la base de datos:", err)
	}
	defer db.Close()

	eventoRepo := repository.NewEventoRepository(db)
	eventoHandler := handlers.NewEventoHandler(eventoRepo)

	// registramos rutas
	router := setupRouter(eventoHandler)

	log.Printf("\nServidor iniciado en http://localhost:%s", cfg.Port)
	if err := router.Run(":" + cfg.Port); err != nil {
		log.Fatal("Error al iniciar el servidor:", err)
	}
}

func setupRouter(eventoHandler *handlers.EventoHandler) *gin.Engine {
	router := gin.Default()

	// Conf CORS básico
	router.Use(func(c *gin.Context) {
		c.Writer.Header().Set("Access-Control-Allow-Origin", "*")
		c.Writer.Header().Set("Access-Control-Allow-Methods", "GET")
		c.Writer.Header().Set("Access-Control-Allow-Headers", "Content-Type, Authorization")

		if c.Request.Method == "OPTIONS" {
			c.AbortWithStatus(204)
			return
		}

		c.Next()
	})

	api := router.Group("/api")
	{
		api.GET("/eventos", eventoHandler.GetEventos)
		api.GET("/eventos/sendpulse", eventoHandler.GetEventosForSendPulse)

		// Health check
		api.GET("/health", func(c *gin.Context) {
			c.JSON(200, gin.H{
				"status":  "ok",
				"message": "API funcional",
			})
		})
	}

	return router
}
