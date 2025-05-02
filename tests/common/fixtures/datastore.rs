use actix_web::web::Data;
use e2e_test_poc::config::{
    datastore::{get_datastore, DataStore},
    options::Options,
};
use rstest::fixture;

use crate::common::get_unique_db_schema;

#[fixture]
pub async fn datastore<'b>(
    #[default(get_unique_db_schema().leak())] schema: &'static str,
) -> DataStoreGuard<'b> {
    DataStoreGuard::new(schema).await
}

/// Structure responsible for ensuring that each test which instantiates it has its database schema
/// dropped after the test finishes running (whatever it passes or not).
///
/// This behaves as a sort of teardrop which cleans up the database schema.
pub struct DataStoreGuard<'a> {
    pub datastore: Option<Data<DataStore>>,
    schema: &'a str,
}

impl<'a> DataStoreGuard<'a> {
    pub async fn new(schema: &'static str) -> Self {
        let options = Options::get();

        let datastore = get_datastore(options.main_database_url, Some(schema))
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
            datastore: Some(Data::new(datastore)),
            schema,
        }
    }
}

impl DataStoreGuard<'_> {
    pub fn take(&mut self) -> Data<DataStore> {
        self.datastore.take().unwrap()
    }

    pub fn clone_datastore(&self) -> Data<DataStore> {
        self.datastore.as_ref().unwrap().clone()
    }
}

impl Drop for DataStoreGuard<'_> {
    fn drop(&mut self) {
        let schema = self.schema.to_owned();
        let database_url = Options::get().main_database_url.to_owned();

        let previous_datasotre = self.datastore.clone();

        let _ = std::thread::spawn(|| {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Couldn't initialize tokio runtime to drop database test schema.")
                .block_on(async move {
                    if let Some(datastore) = previous_datasotre {
                        datastore.get_db().close().await;
                    }

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

impl AsRef<DataStore> for DataStoreGuard<'_> {
    fn as_ref(&self) -> &DataStore {
        self.datastore.as_ref().unwrap()
    }
}
