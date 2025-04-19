use inertia_rust::{
    template_resolvers::ViteHBSTemplateResolver, Inertia, InertiaConfig, InertiaVersion,
};
use vite_rust::Vite;

pub async fn get_inertia(vite: Vite) -> anyhow::Result<Inertia> {
    let assets_version =
        InertiaVersion::Literal(vite.get_hash().unwrap_or("DEVELOPMENT").to_owned());

    let template_resolver = ViteHBSTemplateResolver::builder()
        .set_vite(vite)
        .set_template_path("www/root.hbs")
        .build()?;

    let inertia_config = InertiaConfig::builder()
        .set_version(assets_version)
        .set_template_resolver(Box::new(template_resolver))
        .set_url("http://localhost:3000")
        .build();

    Ok(Inertia::new(inertia_config)?)
}
