use actix_web::{cookie::Cookie, dev::ServiceResponse, http::header};
use e2e_test_poc::config::options::Options;

pub fn extract_session_cookie(response: &ServiceResponse) -> Cookie<'_> {
    response
        .response()
        .cookies()
        .find(|cookie| cookie.name() == Options::get().sessions_cookie_name)
        .expect("Response should contain a session cookie.")
}

pub fn extract_location_from_headers(response: &ServiceResponse) -> &str {
    response
        .headers()
        .get(header::LOCATION)
        .unwrap()
        .to_str()
        .unwrap()
}
