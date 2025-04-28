use actix_web::{web::Data, HttpServer};
use config::{
    datastore::get_datastore, inertia::get_inertia, options::Options, server::get_server,
    vite::get_vite,
};

mod common;
mod config;
mod domain;
mod infra;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().expect("Could not load environment variables.");
    let options = Options::get();

    let vite = get_vite().await?;
    let inertia = Data::new(get_inertia(vite).await?);
    let datastore = Data::new(
        get_datastore(
            options.get_main_database_url(),
            options.get_main_database_schema(),
        )
        .await?,
    );

    if let Err(err) = sqlx::migrate!().run(datastore.get_db()).await {
        log::error!("{err}");
    }

    Ok(HttpServer::new(move || {
        get_server()
            .app_data(datastore.clone())
            .app_data(inertia.clone())
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await?)
}
