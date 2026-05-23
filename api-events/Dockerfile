FROM golang:1.24-alpine AS builder

WORKDIR /app

# Instalar dependencias del sistema
RUN apk add --no-cache git

# Copiar módulos y descargar dependencias
COPY go.mod go.sum ./
RUN go mod download

# Copiar el código fuente
COPY . .

# Compilar el binario
RUN CGO_ENABLED=0 GOOS=linux go build -o api-events ./cmd/api/main.go

# ─── Imagen final ───────────────────────────────────────────────
FROM alpine:3.20

WORKDIR /app

RUN apk add --no-cache ca-certificates tzdata

COPY --from=builder /app/api-events .

EXPOSE 8081

CMD ["./api-events"]
