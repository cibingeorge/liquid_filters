#[macro_use]
mod macros;

mod filters;
mod currency_config;
mod template;
mod error;
mod utils;

use std::sync::Arc;

pub use error::Error;
pub use error::Result;

#[derive(Clone, Debug, Default)]
pub struct RenderContext {
    pub currency_type: Arc<String>,
    pub tokio_rt: Option<Arc<tokio::runtime::Runtime>>,
}

impl RenderContext{
    pub fn new() -> Self {
        Self {..Default::default()}
    }

    pub fn set_currency_type(&mut self, currency_type: String) {
        self.currency_type = Arc::new(currency_type);
    }

    pub fn set_tokio_runtime(&mut self, tokio_rt: Arc<tokio::runtime::Runtime>) {
        self.tokio_rt = Some(tokio_rt);
    }
}
