use vite_rust::{Vite, ViteConfig};

pub async fn get_vite() -> anyhow::Result<Vite> {
    let vite_config = ViteConfig::default()
        .set_manifest_path("public/bundle/manifest.json")
        .set_entrypoints(vec!["www/app.tsx"])
        .set_prefix("bundle");

    Ok(Vite::new(vite_config).await?)
}
