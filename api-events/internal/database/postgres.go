package database

import (
	"database/sql"
	"fmt"
	"log"

	_ "github.com/lib/pq"
)

func NewPostgresDB(databaseURL string) (*sql.DB, error) {
	// Abre la conexión
	db, err := sql.Open("postgres", databaseURL)
	if err != nil {
		return nil, fmt.Errorf("error al abrir la conexión: %w", err)
	}

	if err := db.Ping(); err != nil {
		return nil, fmt.Errorf("error al conectar con la base de datos: %w", err)
	}

	// Configuración el pool
	db.SetMaxOpenConns(25)
	db.SetMaxIdleConns(5)

	log.Println("✅ Conexión exitosa a PostgreSQL")
	return db, nil
}
