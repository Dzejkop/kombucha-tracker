use std::sync::Arc;

mod app;
pub mod handlers;

pub use self::app::App;

type AppType = Arc<App>;
