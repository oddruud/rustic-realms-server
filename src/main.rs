#![allow(dead_code)]

mod api;
mod services;
mod player;
mod combat;
mod properties;
mod food;
mod conversation;

use api::PlayerAdminAPI;
use api::ExperimentalAPI;

    #[async_std::main]
async fn main() -> tide::Result<()> {
    //use sqlx::Acquire; 
    //use sqlx::postgres::Postgres;
    //use tide_sqlx::SQLxMiddleware;
    //use tide_sqlx::SQLxRequestExt;

    tide::log::start();

    let mut app = tide::new();

    PlayerAdminAPI::initialize_endpoints(&mut app);
    ExperimentalAPI::initialize_endpoints(&mut app);

    app.listen("127.0.0.1:8081").await?;
    Ok(())
}