use std::env::var;
use std::sync::OnceLock;

static APP_OPTIONS: OnceLock<Options> = OnceLock::new();

pub struct Options {
    main_database_schema: Option<&'static str>,
    main_database_url: &'static str,
    app_url: &'static str,
    environment: RustEnv,
}

#[derive(Default, PartialEq, Eq, Debug)]
pub enum RustEnv {
    #[default]
    Development,
    Production,
    Test,
}

impl Options {
    fn get_app_options() -> Options {
        Options {
            app_url: Box::leak(
                var("APP_URL")
                    .expect("Environment variables should contain `APP_URL`.")
                    .into_boxed_str(),
            ),
            main_database_url: Box::leak(
                var("MAIN_DATABASE_URL")
                    .expect("Environment variables should contain `MAIN_DATABASE_URL`.")
                    .into_boxed_str(),
            ),
            main_database_schema: var("MAIN_DB_SCHEMA")
                .ok()
                .map(|schema| Box::leak(schema.into_boxed_str()) as &'static str),
            environment: var("RUST_ENV").map(Into::into).unwrap_or_default(),
        }
    }

    pub fn initialize() {
        APP_OPTIONS.get_or_init(Self::get_app_options);
    }

    pub fn get() -> &'static Options {
        APP_OPTIONS.get_or_init(Self::get_app_options)
    }

    pub fn get_main_database_url(&self) -> &'static str {
        self.main_database_url
    }

    pub fn get_main_database_schema(&self) -> Option<&'static str> {
        self.main_database_schema
    }

    pub fn get_app_url(&self) -> &'static str {
        self.app_url
    }

    pub fn get_environment(&self) -> &RustEnv {
        &self.environment
    }
}

impl<T: ToString> From<T> for RustEnv {
    fn from(value: T) -> Self {
        match value.to_string().as_str() {
            "DEVELOPMENT" | "development" => RustEnv::Development,
            "PRODUCTION" | "production" => RustEnv::Production,
            "TEST" | "test" => RustEnv::Test,
            _ => panic!("Invalid RustEnv value"),
        }
    }
}
