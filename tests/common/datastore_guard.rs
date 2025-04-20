use actix_web::web::Data;
use e2e_test_poc::config::{
    datastore::{get_datastore, DataStore},
    options::Options,
};

/// Structure responsible for ensuring that each test which instantiates it has its database schema
/// dropped after the test finishes running (whatever it passes or not).
///
/// This behaves as a sort of teardrop which cleans up the database schema.
pub struct DataStoreGuard<'a> {
    pub datastore: Data<DataStore>,
    schema: &'a str,
}

impl<'a> DataStoreGuard<'a> {
    pub async fn new(schema: &'static str) -> Self {
        let options = Options::get();

        let datastore = get_datastore(options.get_main_database_url(), Some(schema))
            .await
            .expect("Failed to initialize datastore in test fixture.");

        if let Err(err) = sqlx::query(&format!("CREATE SCHEMA IF NOT EXISTS {}", schema))
            .execute(datastore.get_db())
            .await
        {
            log::error!("{err}");
        };

        if let Err(err) = sqlx::migrate!().run(datastore.get_db()).await {
            panic!("Failed to run migration on test setup: {err}");
        }

        Self {
            datastore: Data::new(datastore),
            schema,
        }
    }
}

impl Drop for DataStoreGuard<'_> {
    fn drop(&mut self) {
        let schema = self.schema.to_owned();
        let datastore = self.datastore.clone();
        let database_url = Options::get().get_main_database_url().to_owned();

        let _ = std::thread::spawn(|| {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Couldn't initialize tokio runtime to drop database test schema.")
                .block_on(async move {
                    datastore.get_db().close().await;

                    let database_url = database_url.leak();
                    let schema = schema.leak();
                    let datastore = get_datastore(database_url, Some(schema)).await.unwrap();

                    if let Err(err) =
                        sqlx::query(&format!("DROP SCHEMA IF EXISTS {} CASCADE", schema))
                            .execute(datastore.get_db())
                            .await
                    {
                        log::error!("{err}");
                    };
                })
        })
        .join();
    }
}
