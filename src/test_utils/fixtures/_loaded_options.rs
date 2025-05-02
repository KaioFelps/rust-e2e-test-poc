use rstest::fixture;

use crate::config::options::Options;

#[fixture]
pub fn loaded_options<'a>() -> &'a Options {
    dotenvy::from_filename(".env.test")
        .expect("Could not to load .env.test file into environment variables.");

    Options::get()
}
