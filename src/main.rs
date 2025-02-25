use async_sqlx_session::SqliteSessionStore;
use sqlx::sqlite::SqlitePool;
use std::{env, time::Duration};
use tide::{sessions::SessionMiddleware, Redirect};

mod templates;
pub mod tideys;

mod routes;
mod utils;

#[derive(Clone)]
pub struct State {
    db: SqlitePool,
}

pub type Request = tide::Request<State>;

async fn db_connection() -> tide::Result<SqlitePool> {
    let database_url = env::var("DATABASE_URL")?;
    Ok(SqlitePool::new(&database_url).await?)
}

async fn build_session_middleware(
    db: SqlitePool,
) -> tide::Result<SessionMiddleware<SqliteSessionStore>> {
    let session_store = SqliteSessionStore::from_client(db);
    session_store.migrate().await?;
    session_store.spawn_cleanup_task(Duration::from_secs(60 * 15));
    let session_secret = env::var("TIDE_SECRET").unwrap();
    Ok(SessionMiddleware::new(
        session_store,
        session_secret.as_bytes(),
    ))
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::with_level(tide::log::LevelFilter::Info);
    let db = db_connection().await?;
    let mut app = tide::with_state(State { db: db.clone() });

    app.with(build_session_middleware(db).await?);

    app.at("/").get(Redirect::new("/home"));

    app.at("/home").get(routes::home);
    app.at("/welcome").get(routes::welcome);

    let mut tideys = app.at("/tideys");

    tideys
        .post(routes::tideys::create)
        .get(routes::tideys::index);

    tideys.at("/new").get(routes::tideys::new);

    tideys
        .at("/:tidey_id")
        .get(routes::tideys::show)
        .delete(routes::tideys::delete)
        .put(routes::tideys::update)
        .post(routes::tideys::update);

    app.listen("127.0.0.1:8000").await?;
    Ok(())
}
