use data_types::Kombucha;
use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::get().and(
        warp::path!("kombucha" / "all")
            .map(|| warp::reply::json(&Kombucha::test_default())),
    );

    warp::serve(hello).run(([127, 0, 0, 1], 8080)).await;
}
