use crate::AppType;
use data_types::{EntryId, Kombucha, KombuchaId};
use warp::Rejection;

// GET

pub async fn get_all_kombuchas(
    app: AppType,
) -> Result<impl warp::Reply, Rejection> {
    app.get_all_kombuchas()
        .await
        .map(|kombuchas| warp::reply::json(&kombuchas))
        .map_err(|_| warp::reject::not_found())
}

pub async fn get_kombucha(
    app: AppType,
    id: KombuchaId,
) -> Result<impl warp::Reply, Rejection> {
    app.get_kombucha(id)
        .await
        .map(|kombucha| warp::reply::json(&kombucha))
        .map_err(|_| warp::reject::not_found())
}

pub async fn get_kombucha_entries(
    app: AppType,
    id: KombuchaId,
) -> Result<impl warp::Reply, Rejection> {
    app.get_kombucha_entries(id)
        .await
        .map(|entries| warp::reply::json(&entries))
        .map_err(|_| warp::reject::not_found())
}

pub async fn get_kombucha_entry(
    app: AppType,
    id: KombuchaId,
    entry_id: EntryId,
) -> Result<impl warp::Reply, Rejection> {
    app.get_kombucha_entry(id, entry_id)
        .await
        .map(|entry| warp::reply::json(&entry))
        .map_err(|_| warp::reject::not_found())
}

// UPDATE

pub async fn update_kombucha(
    app: AppType,
    kombucha: Kombucha,
) -> Result<impl warp::Reply, Rejection> {
    app.update_kombucha(&kombucha)
        .await
        .map(|_| warp::reply::reply())
        .map_err(|_| warp::reject::not_found())
}

// CREATE

pub async fn create_kombucha(
    app: AppType,
) -> Result<impl warp::Reply, Rejection> {
    app.create_new_kombucha()
        .await
        .map(|new_kombucha_id| warp::reply::json(&new_kombucha_id))
        .map_err(|_| warp::reject::not_found())
}

pub async fn create_kombucha_entry(
    app: AppType,
    kombucha_id: KombuchaId,
) -> Result<impl warp::Reply, Rejection> {
    app.create_new_kombucha_entry(kombucha_id)
        .await
        .map(|id| warp::reply::json(&id))
        .map_err(|_| warp::reject::not_found())
}

// DELETE

pub async fn delete_kombucha_entry(
    app: AppType,
    id: KombuchaId,
    entry_id: EntryId,
) -> Result<impl warp::Reply, Rejection> {
    app.delete_kombucha_entry(id, entry_id)
        .await
        .map(|_| warp::reply::reply())
        .map_err(|_| warp::reject::not_found())
}

pub async fn delete_kombucha(
    app: AppType,
    id: KombuchaId,
) -> Result<impl warp::Reply, Rejection> {
    app.delete_kombucha(id)
        .await
        .map(|_| warp::reply::reply())
        .map_err(|_| warp::reject::not_found())
}
