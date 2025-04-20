use sqlx::{postgres::PgPoolOptions, PgPool};

pub struct DataStore {
    db: PgPool,
}

impl DataStore {
    pub fn get_db(&self) -> &PgPool {
        &self.db
    }
}

pub async fn get_datastore(
    connection_url: &'static str,
    schema: Option<&'static str>,
) -> anyhow::Result<DataStore> {
    let pg_pool = PgPoolOptions::new()
        .max_connections(50)
        .after_connect(move |conn, _meta| {
            Box::pin(async move {
                if let Some(schema) = schema {
                    sqlx::query(&format!("SET search_path = {}", schema))
                        .execute(conn)
                        .await?;
                }
                Ok(())
            })
        })
        .connect(connection_url)
        .await?;

    Ok(DataStore { db: pg_pool })
}
