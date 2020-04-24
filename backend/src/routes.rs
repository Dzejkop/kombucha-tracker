use super::handlers;
use crate::AppType;
use data_types::{EntryId, KombuchaId};
use std::convert::Infallible;
use warp::{Filter, Rejection, Reply};

pub fn routes(
    app: AppType,
    dir: String,
) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    let api_routes = get_routes(app.clone())
        .or(post_routes(app.clone()))
        .or(put_routes(app.clone()))
        .or(delete_routes(app));

    let api_routes = warp::path!("api" / "1" / ..).and(api_routes);

    let static_routes = static_routes(dir);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT"]);

    api_routes
        .or(static_routes)
        .with(cors)
        .with(warp::log("kombucha_tracker_server"))
}

fn get_routes(
    app: AppType,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let base = warp::get().and(with_app(app));

    base.clone()
        .and(warp::path!("kombucha" / KombuchaId))
        .and_then(handlers::get_kombucha)
        .or(base
            .clone()
            .and(warp::path!("kombucha"))
            .and_then(handlers::get_all_kombuchas))
        .or(base
            .clone()
            .and(warp::path!("kombucha" / KombuchaId / "entry" / EntryId))
            .and_then(handlers::get_kombucha_entry))
        .or(base
            .and(warp::path!("kombucha" / KombuchaId / "entry"))
            .and_then(handlers::get_kombucha_entries))
}

fn post_routes(
    app: AppType,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let base = warp::post().and(with_app(app));

    base.clone()
        .and(warp::path!("kombucha"))
        .and_then(handlers::create_kombucha)
        .or(base
            .and(warp::path!("kombucha" / KombuchaId / "entry"))
            .and_then(handlers::create_kombucha_entry))
}

fn put_routes(
    app: AppType,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let base = warp::put().and(with_app(app));

    base.and(warp::body::json())
        .and(warp::path!("kombucha"))
        .and_then(handlers::update_kombucha)
}

fn delete_routes(
    app: AppType,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let base = warp::delete().and(with_app(app));

    base.clone()
        .and(warp::path!("kombucha" / KombuchaId))
        .and_then(handlers::delete_kombucha)
        .or(base
            .and(warp::path!("kombucha" / KombuchaId / "entry" / EntryId))
            .and_then(handlers::delete_kombucha_entry))
}

fn with_app(
    app: AppType,
) -> impl Filter<Extract = (AppType,), Error = Infallible> + Clone {
    warp::any().map(move || app.clone())
}

fn static_routes(
    dir: String,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::fs::dir(dir)
}
