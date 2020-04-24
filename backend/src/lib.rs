use std::sync::Arc;

mod app;
mod handlers;
mod routes;

pub use self::app::App;
pub use self::routes::routes;

type AppType = Arc<App>;
