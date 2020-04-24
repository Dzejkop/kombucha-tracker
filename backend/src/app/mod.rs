use sqlx::PgPool;

mod create;
mod delete;
mod get;
mod update;

#[derive(Clone, Debug)]
pub struct App {
    db: PgPool,
}

impl App {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}
