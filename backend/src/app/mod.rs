use sqlx::PgPool;

mod get;
mod create;
mod update;
mod delete;

#[derive(Clone, Debug)]
pub struct App {
    db: PgPool,
}

impl App {
    fn new(db: PgPool) -> Self {
        Self { db }
    }
}
