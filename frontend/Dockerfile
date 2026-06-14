FROM node:22-alpine AS builder

WORKDIR /app

# Copiar dependencias
COPY package.json package-lock.json ./

# Instalar dependencias (incluyendo @astrojs/node)
RUN npm install

# Copiar el código fuente
COPY . .

# Build de producción
RUN npm run build

# ─── Imagen final ───────────────────────────────────────────────
FROM node:22-alpine

WORKDIR /app

COPY --from=builder /app/dist ./dist
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/package.json ./

EXPOSE 4321

ENV HOST=0.0.0.0
ENV PORT=4321

CMD ["node", "./dist/server/entry.mjs"]
