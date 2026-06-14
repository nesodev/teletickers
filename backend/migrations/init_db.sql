-- Tabla USUARIO
CREATE TABLE usuario (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nombre VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    numero_cel VARCHAR(20),
    contraseña VARCHAR(255) NOT NULL,
    estado VARCHAR(20) DEFAULT 'activo' CHECK (estado IN ('activo', 'baneado')),
    dni VARCHAR(8) UNIQUE NOT NULL,
    fecha_registro TIMESTAMP DEFAULT NOW()
);

-- Tabla EVENTO
CREATE TABLE evento (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    titulo VARCHAR(255) NOT NULL,
    descripcion TEXT,
    fecha DATE NOT NULL,
    hora TIME NOT NULL,
    region VARCHAR(100) NOT NULL,
    provincia VARCHAR(100) NOT NULL,
    distrito VARCHAR(100) NOT NULL,
    organizador_id UUID NOT NULL REFERENCES usuario(id) ON DELETE CASCADE,
    categoria VARCHAR(50) NOT NULL,
    aforo INT NOT NULL,
    etiquetas VARCHAR(255),
    restriccion_edad VARCHAR(20) DEFAULT 'todo_publico' CHECK (restriccion_edad IN ('todo_publico', 'con_adulto', 'solo_mayores')),
    miniatura VARCHAR(500),
    metodos_pago_aceptados JSONB DEFAULT '["yape", "plin", "tarjeta"]',
    estado VARCHAR(20) DEFAULT 'borrador' CHECK (estado IN ('borrador', 'publicado', 'cancelado', 'finalizado')),
    fecha_creacion TIMESTAMP DEFAULT NOW(),
    fecha_actualizacion TIMESTAMP DEFAULT NOW()
);

-- Tabla TIPO_ENTRADA
CREATE TABLE tipo_entrada (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    evento_id UUID NOT NULL REFERENCES evento(id) ON DELETE CASCADE,
    nombre VARCHAR(100) NOT NULL,
    precio DECIMAL(10, 2) NOT NULL,
    max_por_compra INT NOT NULL DEFAULT 10,
    cantidad_disponible INT NOT NULL,
    descripcion TEXT
);

-- Tabla COMPROBANTE
CREATE TABLE comprobante (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tipo VARCHAR(20) NOT NULL CHECK (tipo IN ('boleta', 'factura')),
    fecha_emision TIMESTAMP DEFAULT NOW(),
    ruc_empresa VARCHAR(11),
    serie VARCHAR(10) NOT NULL,
    correlativo INT,
    monto_total DECIMAL(10, 2) NOT NULL,
    igv DECIMAL(10, 2),
    archivo_pdf VARCHAR(500)
);

-- Tabla COMPRA
CREATE TABLE compra (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    usuario_id UUID NOT NULL REFERENCES usuario(id) ON DELETE CASCADE,
    evento_id UUID NOT NULL REFERENCES evento(id) ON DELETE CASCADE,
    comprobante_id UUID REFERENCES comprobante(id) ON DELETE SET NULL,
    fecha_compra TIMESTAMP DEFAULT NOW(),
    monto_total DECIMAL(10, 2) NOT NULL,
    metodo_pago VARCHAR(20) NOT NULL CHECK (metodo_pago IN ('yape', 'plin', 'tarjeta')),
    estado_pago VARCHAR(20) DEFAULT 'pendiente' CHECK (estado_pago IN ('pendiente', 'pagado', 'cancelado', 'reembolsado')),
    codigo_operacion VARCHAR(100)
);

-- Tabla ENTRADA (boletos individuales)
CREATE TABLE entrada (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tipo_entrada_id UUID NOT NULL REFERENCES tipo_entrada(id) ON DELETE CASCADE,
    compra_id UUID NOT NULL REFERENCES compra(id) ON DELETE CASCADE,
    qr_code VARCHAR(255) UNIQUE NOT NULL,
    estado VARCHAR(20) DEFAULT 'valido' CHECK (estado IN ('valido', 'usado', 'cancelado')),
    fecha_uso TIMESTAMP
);

-- Tabla NOTIFICACION
CREATE TABLE notificacion (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    usuario_id UUID NOT NULL REFERENCES usuario(id) ON DELETE CASCADE,
    tipo VARCHAR(20) NOT NULL CHECK (tipo IN ('compra', 'evento', 'recordatorio', 'sistema')),
    titulo VARCHAR(255) NOT NULL,
    mensaje TEXT NOT NULL,
    leido BOOLEAN DEFAULT FALSE,
    fecha_creacion TIMESTAMP DEFAULT NOW(),
    evento_id UUID REFERENCES evento(id) ON DELETE SET NULL
);

-- Índices para mejorar rendimiento
CREATE INDEX idx_evento_organizador ON evento(organizador_id);
CREATE INDEX idx_evento_fecha ON evento(fecha);
CREATE INDEX idx_evento_estado ON evento(estado);
CREATE INDEX idx_tipo_entrada_evento ON tipo_entrada(evento_id);
CREATE INDEX idx_entrada_compra ON entrada(compra_id);
CREATE INDEX idx_entrada_qr ON entrada(qr_code);
CREATE INDEX idx_compra_usuario ON compra(usuario_id);
CREATE INDEX idx_compra_evento ON compra(evento_id);
CREATE INDEX idx_notificacion_usuario ON notificacion(usuario_id);
CREATE INDEX idx_notificacion_leido ON notificacion(leido);

-- Trigger para actualizar fecha_actualizacion en evento
CREATE OR REPLACE FUNCTION actualizar_fecha_actualizacion()
RETURNS TRIGGER AS $$
BEGIN
    NEW.fecha_actualizacion = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_actualizar_fecha_evento
BEFORE UPDATE ON evento
FOR EACH ROW
EXECUTE FUNCTION actualizar_fecha_actualizacion();