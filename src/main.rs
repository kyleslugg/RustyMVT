
extern crate dotenv_codegen;
extern crate dotenv;
extern crate rusty_mvt;

use anyhow::{anyhow, Context, Error};
use axum::{
    routing::get,
    Router,
};

use rusty_mvt::{geocoding::get_latlong, layers::get_layer, db::{get_db_connector, load_table_registry, TableRegistry}, AppState};
use dotenv::dotenv;




use sqlx::{Pool, Postgres};


#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let table_registry: TableRegistry;
    let db_pool: Pool<Postgres>;

    if let Ok(pool) = get_db_connector().await {
        db_pool = pool;
        if let Ok(registry) = load_table_registry(&db_pool, "default".to_string()).await {
            table_registry = registry;
        } else {
            return Err(anyhow!("Failed to load table registry"));
        };
    } else {
        return Err(anyhow!("Failed to connect with provided database string"));
    };

    let state = AppState {
        db_pool,
        table_registry,
    };

    let app = Router::new()
        .route("/geocode/:queryString", get(get_latlong))
        .route("/layers/:schemaid/:tableid/:z/:x/:y_ext", get(get_layer))
        //.route("/api/:schema/:table/:z/:x/:y", get(serve_tile))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .context("Error occurred while starting server")
}
