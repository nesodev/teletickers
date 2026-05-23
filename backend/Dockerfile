FROM rust:latest AS builder

WORKDIR /app

# Dependencias del sistema
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Copiar manifiestos primero para cachear dependencias
COPY Cargo.toml ./

# Crear src dummy para compilar dependencias
RUN mkdir -p src && echo 'fn main() {}' > src/main.rs

# Compilar dependencias (cacheado si Cargo.toml no cambia)
RUN cargo build --release 2>/dev/null || true

# Copiar código fuente real
COPY src ./src
COPY assets ./assets
COPY migrations ./migrations

# Compilar el binario final
RUN touch src/main.rs && cargo build --release

# ─── Imagen final ───────────────────────────────────────────────
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    libssl3 \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/sistema_backend /app/ticky
COPY --from=builder /app/assets ./assets
COPY --from=builder /app/migrations ./migrations

EXPOSE 8080

CMD ["/app/ticky"]
