pub mod compra_repo;
pub mod comprobante_repo;
pub mod entrada_repo;
pub mod evento_repo;
pub mod notificacion_repo;
pub mod tipo_entrada_repo;
pub mod user_repo;

pub use compra_repo::CompraRepository;
pub use comprobante_repo::ComprobanteRepository;
pub use entrada_repo::EntradaRepository;
pub use evento_repo::EventoRepository;
pub use notificacion_repo::NotificacionRepository;
pub use tipo_entrada_repo::TipoEntradaRepository;
pub use user_repo::UserRepository;
