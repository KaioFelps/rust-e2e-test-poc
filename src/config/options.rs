#![allow(unused)]

use std::env::var;
use std::sync::OnceLock;

use base64::engine::general_purpose;
use base64::Engine;

static APP_OPTIONS: OnceLock<Options> = OnceLock::new();

pub struct Options {
    pub main_database_schema: Option<&'static str>,
    pub main_database_url: &'static str,
    pub app_url: &'static str,
    pub app_key: &'static [u8],
    pub environment: RustEnv,
    pub default_per_page: u8,
    pub sessions_flash_key: &'static str,
    pub sessions_errors_key: &'static str,
    pub sessions_prev_req_key: &'static str,
    pub sessios_curr_req_key: &'static str,
    pub sessions_dir: &'static str,
    pub sessions_exp_key: &'static str,
    pub sessions_cookie_name: &'static str,
    pub garbage_collector_lottery: [u8; 2],
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
            default_per_page: 24,
            app_key: var("APP_KEY")
                .map(|key| {
                    general_purpose::STANDARD
                        .decode(&key)
                        .expect("APP_KEY should be a 64bytes base64-encoded string.")
                        .leak()
                })
                .expect("Environment variables should contan `APP_KEY`."),

            // sessions management
            sessions_dir: "storage/sessions",
            sessions_cookie_name: "e2e_todo_poc_session",
            sessions_exp_key: "__expires_at__",
            garbage_collector_lottery: [2, 100],
            sessions_flash_key: "__flash",
            sessions_errors_key: "_errors",
            sessions_prev_req_key: "_prev_req_url",
            sessios_curr_req_key: "_curr_req_url",
        }
    }

    pub fn initialize() {
        APP_OPTIONS.get_or_init(Self::get_app_options);
    }

    pub fn get() -> &'static Options {
        APP_OPTIONS.get_or_init(Self::get_app_options)
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
