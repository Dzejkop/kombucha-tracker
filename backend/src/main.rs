use data_types::db::{Entry as DbEntry, Kombucha as DbKombucha};
use data_types::{Entry, EntryId, Kombucha, KombuchaId};
use sqlx::postgres::PgPool;
use sqlx::prelude::*;
use std::{env, sync::Arc};
use warp::{Filter, Rejection};

const PORT: u16 = 8080;

type AppType = Arc<App>;

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
            "SELECT id, name, added FROM kombucha ORDER BY id",
        )
        .fetch_all(&self.db)
        .await?;

        Ok(row)
    }

    async fn get_kombucha(
        &self,
        id: KombuchaId,
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
        kombucha_id: KombuchaId,
    ) -> Result<Vec<DbEntry>, anyhow::Error> {
        let row = sqlx::query_as::<_, DbEntry>(
            "SELECT id, kombucha_id, content, added FROM kombucha_entry WHERE kombucha_id = $1",
        )
        .bind(kombucha_id)
        .fetch_all(&self.db)
        .await?;

        Ok(row)
    }

    async fn create_new_kombucha(&self) -> Result<KombuchaId, anyhow::Error> {
        let (id,) = sqlx::query_as::<_, (KombuchaId,)>(
            "INSERT INTO kombucha (name, added) VALUES ('', NOW()) RETURNING id",
        )
        .fetch_one(&self.db)
        .await?;

        Ok(id)
    }

    async fn update_kombucha(
        &self,
        kombucha: &Kombucha,
    ) -> Result<(), anyhow::Error> {
        let mut transaction = self.db.begin().await?;

        let query = sqlx::query(
            "UPDATE kombucha SET (name, added) = ($1, $2) WHERE id = $3",
        )
        .bind(&kombucha.name)
        .bind(&kombucha.added)
        .bind(kombucha.id);

        transaction.execute(query).await?;

        for entry in &kombucha.entries {
            let query = sqlx::query("UPDATE kombucha_entry SET (added, content) = ($1, $2) WHERE id = $3")
                .bind(&entry.added)
                .bind(&entry.content)
                .bind(entry.id);

            transaction.execute(query).await?;
        }

        transaction.commit().await?;

        Ok(())
    }

    // async fn update_kombucha(
    //     &self,
    //     kombucha: &Kombucha,
    // ) -> Result<(), anyhow::Error> {
    //     Ok(())
    // }
}

async fn get_kombuchas(app: AppType) -> Result<impl warp::Reply, Rejection> {
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
            .map(
                |DbEntry {
                     id, content, added, ..
                 }| Entry { id, content, added },
            )
            .collect();

        ret_kombuchas.push(Kombucha {
            id,
            name,
            added,
            entries,
        });
    }

    Ok(warp::reply::json(&ret_kombuchas))
}

async fn get_kombucha(
    id: KombuchaId,
    app: AppType,
) -> Result<impl warp::Reply, Rejection> {
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
        .map(
            |DbEntry {
                 id, content, added, ..
             }| Entry { id, content, added },
        )
        .collect();

    let kombucha = Kombucha {
        id,
        name,
        added,
        entries,
    };

    Ok(warp::reply::json(&kombucha))
}

async fn update_kombucha(
    kombucha: Kombucha,
    app: AppType,
) -> Result<impl warp::Reply, Rejection> {
    app.update_kombucha(&kombucha)
        .await
        .map_err(|_| warp::reject::not_found())?;

    Ok(warp::reply::reply())
}

async fn create_kombucha(app: AppType) -> Result<impl warp::Reply, Rejection> {
    let kombucha_id: KombuchaId = app
        .create_new_kombucha()
        .await
        .map_err(|_| warp::reject::not_found())?;

    Ok(warp::reply::json(&kombucha_id))
}

async fn create_kombucha_entry(
    id: KombuchaId,
    app: AppType,
) -> Result<impl warp::Reply, Rejection> {
    log::info!("Creating new entry for kombucha {}", id);
    let (id,) = sqlx::query_as::<_, (EntryId,)>("INSERT INTO kombucha_entry (kombucha_id, content, added) VALUES ($1, '', NOW()) RETURNING id")
        .bind(id)
        .fetch_one(&app.db)
        .await
        .map_err(|err| {log::error!("{}", err); warp::reject::not_found()})?;

    log::info!("Created entry with id {}", id);
    Ok(warp::reply::json(&id))
}

async fn get_kombucha_entries(
    id: KombuchaId,
    app: AppType,
) -> Result<impl warp::Reply, Rejection> {
    log::info!("Getting entries for kombucha {}", id);

    let db_entries = sqlx::query_as::<_, DbEntry>("SELECT id, kombucha_id, content, added FROM kombucha_entry WHERE kombucha_id = $1")
        .bind(id)
        .fetch_all(&app.db)
        .await
        .map_err(|err| { log::error!("{}", err); warp::reject::not_found() })?;

    let entries: Vec<Entry> = db_entries
        .into_iter()
        .map(
            |DbEntry {
                 id, content, added, ..
             }| Entry { id, content, added },
        )
        .collect();

    Ok(warp::reply::json(&entries))
}

async fn get_kombucha_entry(
    id: KombuchaId,
    entry_id: EntryId,
    app: AppType,
) -> Result<impl warp::Reply, Rejection> {
    log::info!("Getting entry {} for kombucha {}", entry_id, id);

    let DbEntry { id, content, added, .. } = sqlx::query_as::<_, DbEntry>("SELECT id, kombucha_id, content, added FROM kombucha_entry WHERE id = $1 AND kombucha_id = $2")
        .bind(entry_id)
        .bind(id)
        .fetch_optional(&app.db)
        .await
        .map_err(|err| { log::error!("{}", err); warp::reject::not_found() })?
        .ok_or_else(warp::reject::not_found)?;

    let entry = Entry { id, content, added };

    Ok(warp::reply::json(&entry))
}

async fn delete_kombucha_entry(
    id: KombuchaId,
    entry_id: EntryId,
    app: AppType,
) -> Result<impl warp::Reply, Rejection> {
    log::info!("Deleting entry {} for kombucha {}", entry_id, id);

    let query = sqlx::query(
        "DELETE FROM kombucha_entry WHERE kombucha_id = $1 and id = $2",
    )
    .bind(id)
    .bind(entry_id);

    app.db
        .acquire()
        .await
        .expect("Failed to connect")
        .execute(query)
        .await
        .expect("Invalid query");

    Ok(warp::reply::reply())
}

async fn delete_kombucha(
    id: KombuchaId,
    app: AppType,
) -> Result<impl warp::Reply, Rejection> {
    log::info!("Deleting kombucha {}", id);

    let mut transaction = app.db.begin().await.unwrap();

    let delete_entries_query =
        sqlx::query("DELETE FROM kombucha_entry WHERE kombucha_id = $1")
            .bind(id);

    let delete_kombucha_query =
        sqlx::query("DELETE FROM kombucha WHERE id = $1").bind(id);

    transaction.execute(delete_entries_query).await.unwrap();
    transaction.execute(delete_kombucha_query).await.unwrap();

    transaction.commit().await.unwrap();

    Ok(warp::reply::reply())
}

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
    let app = warp::any().map(move || app.clone());

    let all_kombuchas = warp::path!("kombucha" / "all")
        .and(app.clone())
        .and_then(get_kombuchas);

    let get_kombucha = warp::path!("kombucha" / KombuchaId)
        .and(app.clone())
        .and_then(get_kombucha);

    let get_kombucha_entry =
        warp::path!("kombucha" / KombuchaId / "entry" / EntryId)
            .and(app.clone())
            .and_then(get_kombucha_entry);

    let get_kombucha_entries = warp::path!("kombucha" / KombuchaId / "entry")
        .and(app.clone())
        .and_then(get_kombucha_entries);

    let update_kombucha = warp::path!("kombucha")
        .and(warp::body::json())
        .and(app.clone())
        .and_then(update_kombucha);

    let create_kombucha = warp::path!("kombucha")
        .and(app.clone())
        .and_then(create_kombucha);

    let create_kombucha_entry = warp::path!("kombucha" / KombuchaId / "entry")
        .and(app.clone())
        .and_then(create_kombucha_entry);

    let delete_kombucha_entry =
        warp::path!("kombucha" / KombuchaId / "entry" / EntryId)
            .and(app.clone())
            .and_then(delete_kombucha_entry);

    let delete_kombucha = warp::path!("kombucha" / KombuchaId)
        .and(app.clone())
        .and_then(delete_kombucha);

    // GET
    let routes = warp::get().and(
        all_kombuchas
            .or(get_kombucha)
            .or(get_kombucha_entry)
            .or(get_kombucha_entries),
    );

    // POST
    let routes = warp::post()
        .and(create_kombucha_entry.or(create_kombucha))
        .or(routes);

    // PUT
    let routes = warp::put().and(update_kombucha).or(routes);

    // DELETE
    let routes = warp::delete()
        .and(delete_kombucha.or(delete_kombucha_entry))
        .or(routes);

    // api/1/... routes
    let api_routes = warp::path("api").and(warp::path("1")).and(routes);

    // Static App route
    let app = warp::fs::dir(app_dir);

    let routes = api_routes.or(app);

    let routes = routes.boxed();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT"]);

    log::info!("Listening on port {}", PORT);
    warp::serve(routes.with(cors))
        .run(([127, 0, 0, 1], PORT))
        .await;

    Ok(())
}
