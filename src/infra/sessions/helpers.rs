use actix_session::SessionExt;
use actix_web::HttpRequest;
use serde_json::Map;

use crate::config::options::Options;

#[inline]
pub fn flash_silently<T: ToString>(req: &HttpRequest, key: &str, msg: T) {
    if let Err(err) = flash(req, key, msg) {
        log::warn!("{err}");
    }
}

#[inline]
pub fn flash<T: ToString>(req: &HttpRequest, key: &str, msg: T) -> anyhow::Result<()> {
    let options = Options::get();

    let mut flash_messages = req
        .get_session()
        .remove(options.sessions_flash_key)
        .map(|map| serde_json::from_str::<Map<_, _>>(&map).unwrap_or_default())
        .unwrap_or_default();

    flash_messages.insert(key.to_string(), msg.to_string().into());

    req.get_session()
        .insert(options.sessions_flash_key, flash_messages)?;

    Ok(())
}
