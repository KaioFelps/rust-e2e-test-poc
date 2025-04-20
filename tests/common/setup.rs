use dotenvy::dotenv;
use e2e_test_poc::config::options::Options;
use env_logger::Target;
use rstest::fixture;

#[fixture]
pub async fn __setup() {
    dotenv().expect("Failed to inject `.env` into environment variables.");

    std::env::set_var("RUST_ENV", "test");

    let _ = env_logger::builder()
        .parse_env("RUST_LOG")
        .target(Target::Stdout)
        .try_init();

    Options::initialize();
}
