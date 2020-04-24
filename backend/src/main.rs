use kombucha_tracker_server::{routes, App};
use sqlx::postgres::PgPool;
use std::{env, sync::Arc};

const PORT: u16 = 8080;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let app_dir = env::var("KOMBUCHA_APP_DIR")?;

    let pool = PgPool::builder()
        .max_size(5) // maximum number of connections in the pool
        .build(&env::var("DATABASE_URL")?)
        .await?;

    let app = Arc::new(App::new(pool));
    let routes = routes(app, app_dir);

    log::info!("Listening on port {}", PORT);
    warp::serve(routes).run(([127, 0, 0, 1], PORT)).await;

    Ok(())
}
