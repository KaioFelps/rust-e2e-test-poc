use std::env::var;
use std::sync::OnceLock;

static APP_OPTIONS: OnceLock<Options> = OnceLock::new();

pub struct Options {
    main_database_url: &'static str,
    app_url: &'static str,
}

impl Options {
    pub fn initialize() -> Options {
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
        }
    }

    pub fn get() -> &'static Options {
        APP_OPTIONS.get_or_init(Self::initialize)
    }

    pub fn get_main_database_url(&self) -> &'static str {
        self.main_database_url
    }

    pub fn get_app_url(&self) -> &'static str {
        self.app_url
    }
}
