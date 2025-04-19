use sqlx::{postgres::PgPoolOptions, PgPool};

pub struct DataStore {
    db: PgPool,
}

impl DataStore {
    pub fn get_db(&self) -> &PgPool {
        &self.db
    }
}

pub async fn get_datastore(connection_str: &str) -> anyhow::Result<DataStore> {
    let pg_pool = PgPoolOptions::new()
        .max_connections(50)
        .connect(connection_str)
        .await?;

    Ok(DataStore { db: pg_pool })
}
