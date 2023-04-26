#![allow(dead_code)]

mod api;
mod services;
mod player;
mod combat;
mod properties;
mod food;
mod conversation;

use api::{PlayerAdminAPI, ExperimentalAPI};

    #[async_std::main]
async fn main() -> tide::Result<()> {
    //use sqlx::Acquire; // Or sqlx::prelude::*;
    use sqlx::Acquire;
    use sqlx::postgres::Postgres;

    use tide_sqlx::SQLxMiddleware;
    use tide_sqlx::SQLxRequestExt;

    tide::log::start();

    let url = "postgresql://postgres:md63bps5@127.0.0.1:5432";

    let mut app = tide::new();
    //app.with(SQLxMiddleware::<Postgres>::new(url).await?);
  

    PlayerAdminAPI::register_endpoints(&mut app);
    ExperimentalAPI::register_endpoints(&mut app);

    app.listen("0.0.0.0:8081").await?;
    Ok(())
}