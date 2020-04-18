use data_types::db::{Entry as DbEntry, Kombucha as DbKombucha};
use data_types::{Entry, Kombucha};
use sqlx::postgres::PgPool;
use sqlx::prelude::*;
use std::{env, sync::Arc};
use tokio::sync::RwLock;
use warp::{Filter, Rejection};

#[derive(Clone, Debug)]
struct App {
    db: PgPool,
}

impl App {
    fn new(db: PgPool) -> Self {
        Self { db }
    }

    async fn get_kombuchas(&self) -> Result<Vec<DbKombucha>, anyhow::Error> {
        let row = sqlx::query_as::<_, DbKombucha>(
            "SELECT id, name, added FROM kombucha",
        )
        .fetch_all(&self.db)
        .await?;

        Ok(row)
    }

    async fn get_kombucha(
        &self,
        id: i32,
    ) -> Result<Option<DbKombucha>, anyhow::Error> {
        let row = sqlx::query_as::<_, DbKombucha>(
            "SELECT id, name, added FROM kombucha WHERE id  = $1",
        )
        .bind(id)
        .fetch_optional(&self.db)
        .await?;

        Ok(row)
    }

    async fn get_entries_for_kombucha(
        &self,
        kombucha_id: i32,
    ) -> Result<Vec<DbEntry>, anyhow::Error> {
        let row = sqlx::query_as::<_, DbEntry>(
            "SELECT id, kombucha_id, content, added FROM kombucha_entry WHERE kombucha_id = $1",
        )
        .bind(kombucha_id)
        .fetch_all(&self.db)
        .await?;

        Ok(row)
    }

    async fn insert_new_kombucha(
        &self,
        kombucha: &Kombucha,
    ) -> Result<(), anyhow::Error> {
        let (id,) = sqlx::query_as::<_, (i32,)>(
            "INSERT INTO kombucha (name, added) VALUES ($1, $2) RETURNING id",
        )
        .bind(&kombucha.name)
        .bind(&kombucha.added)
        .fetch_one(&self.db)
        .await?;

        let mut transaction = self.db.begin().await?;

        for entry in &kombucha.entries {
            let query = sqlx::query("INSERT INTO kombucha_entry (kombucha_id, added, content) VALUES ($1, $2, $3)")
                .bind(id)
                .bind(&entry.added)
                .bind(&entry.content);

            transaction.execute(query).await?;
        }

        transaction.commit().await?;

        Ok(())
    }
}

async fn get_kombuchas(
    app: Arc<RwLock<App>>,
) -> Result<impl warp::Reply, Rejection> {
    let app = app.read().await;

    let kombuchas = app
        .get_kombuchas()
        .await
        .map_err(|_| warp::reject::not_found())?;

    let mut ret_kombuchas = Vec::with_capacity(kombuchas.len());

    for DbKombucha { id, name, added } in kombuchas {
        let entries = app
            .get_entries_for_kombucha(id)
            .await
            .map_err(|_| warp::reject::not_found())?;

        let entries = entries
            .into_iter()
            .map(|DbEntry { content, added, .. }| Entry { content, added })
            .collect();

        ret_kombuchas.push(Kombucha {
            name,
            added,
            entries,
            ..Kombucha::default_new()
        });
    }

    Ok(warp::reply::json(&ret_kombuchas))
}

async fn get_kombucha(
    id: i32,
    app: Arc<RwLock<App>>,
) -> Result<impl warp::Reply, Rejection> {
    let app = app.read().await;

    let DbKombucha { id, name, added } = app
        .get_kombucha(id)
        .await
        .map_err(|_| warp::reject::not_found())?
        .ok_or_else(warp::reject::not_found)?;

    let entries = app
        .get_entries_for_kombucha(id)
        .await
        .map_err(|_| warp::reject::not_found())?;

    let entries = entries
        .into_iter()
        .map(|DbEntry { content, added, .. }| Entry { content, added })
        .collect();

    let kombucha = Kombucha {
        name,
        added,
        entries,
        ..Kombucha::default_new()
    };

    Ok(warp::reply::json(&kombucha))
}

async fn insert_kombucha(
    kombucha: Kombucha,
    app: Arc<RwLock<App>>,
) -> Result<impl warp::Reply, Rejection> {
    let app = app.read().await;

    app.insert_new_kombucha(&kombucha)
        .await
        .map_err(|_| warp::reject::not_found())?;

    Ok(warp::reply::reply())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    env_logger::init();

    let pool = PgPool::builder()
        .max_size(5) // maximum number of connections in the pool
        .build(&env::var("DATABASE_URL")?)
        .await?;

    let app = Arc::new(RwLock::new(App::new(pool)));
    let app = warp::any().map(move || app.clone());

    let all_kombuchas = warp::path!("kombucha" / "all")
        .and(app.clone())
        .and_then(get_kombuchas);

    let get_kombucha = warp::path!("kombucha" / i32)
        .and(app.clone())
        .and_then(get_kombucha);

    let routes = warp::get().and(all_kombuchas.or(get_kombucha));

    let post_kombucha = warp::path!("kombucha")
        .and(warp::body::json())
        .and(app.clone())
        .and_then(insert_kombucha);

    let routes = warp::post().and(post_kombucha).or(routes);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST"]);

    warp::serve(routes.with(cors))
        .run(([127, 0, 0, 1], 8080))
        .await;

    Ok(())
}
