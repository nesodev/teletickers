use crate::domain::value_objects::Money;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TipoEntrada {
    pub id: Uuid,
    pub evento_id: Uuid,
    pub nombre: String,
    pub precio: Money,
    pub max_por_compra: i32,
    pub cantidad_disponible: i32,
    pub descripcion: Option<String>,
}

impl TipoEntrada {
    pub fn new(evento_id: Uuid, nombre: String, precio: Money, max_por_compra: i32, cantidad_disponible: i32, descripcion: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            evento_id,
            nombre,
            precio,
            max_por_compra,
            cantidad_disponible,
            descripcion,
        }
    }

    pub fn hay_disponibilidad(&self, cantidad: i32) -> bool {
        self.cantidad_disponible >= cantidad
    }

    pub fn validar_cantidad(&self, cantidad: i32) -> Result<(), String> {
        if cantidad <= 0 {
            return Err("La cantidad debe ser mayor a 0".to_string());
        }

        if cantidad > self.max_por_compra {
            return Err(format!("La cantidad máxima por compra es {}", self.max_por_compra));
        }

        if !self.hay_disponibilidad(cantidad) {
            return Err("No hay suficientes entradas disponibles".to_string());
        }

        Ok(())
    }

    pub fn reservar(&mut self, cantidad: i32) -> Result<(), String> {
        self.validar_cantidad(cantidad)?;
        self.cantidad_disponible -= cantidad;
        Ok(())
    }

    pub fn liberar(&mut self, cantidad: i32) {
        self.cantidad_disponible += cantidad;
    }
}
