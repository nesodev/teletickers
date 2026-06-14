use std::env;

pub struct ComprobanteConfig {
    pub ruc_empresa: String,
    pub razon_social: String,
    pub direccion: String,
    pub serie_boleta: String,
    pub serie_factura: String,
}

impl ComprobanteConfig {
    pub fn from_env() -> Self {
        Self {
            ruc_empresa: env::var("RUC_EMPRESA").expect("RUC_EMPRESA must be set"),
            razon_social: env::var("RAZON_SOCIAL").expect("RAZON_SOCIAL must be set"),
            direccion: env::var("DIRECCION_EMPRESA").expect("DIRECCION_EMPRESA must be set"),
            serie_boleta: env::var("SERIE_BOLETA").unwrap_or_else(|_| "B001".to_string()),
            serie_factura: env::var("SERIE_FACTURA").unwrap_or_else(|_| "F001".to_string()),
        }
    }
}
