pub mod bot;
pub mod config;
pub mod commands;

pub mod prelude {
    pub use crate::arma::bot::Manager;
    pub use crate::arma::bot::ArmaServerDescriptor;
}

