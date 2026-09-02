use anyhow::{Context, Result};
use std::fs;
use toml_edit::DocumentMut;

pub fn bump_detaxine_ui_version(project: &str, version: &str) -> Result<()> {
    let manifest_path = format!("{}/Cargo.toml", project);
    let contents = fs::read_to_string(&manifest_path).context("could not read Cargo.toml")?;
    let mut doc = contents
        .parse::<DocumentMut>()
        .context("could not parse Cargo.toml")?;

    let dep = doc["dependencies"]["detaxine-ui"]
        .as_table_like_mut()
        .ok_or_else(|| {
            anyhow::anyhow!("detaxine-ui dependency not found or not a table in Cargo.toml")
        })?;
    dep.insert("version", toml_edit::value(version));

    fs::write(&manifest_path, doc.to_string()).context("could not write Cargo.toml")?;
    Ok(())
}

pub fn write_manifest(name: &str) -> Result<()> {
    let contents = format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2024"
[dependencies]
detaxine-ui = "0.8.60"
leptos = {{ version = "0.8.20", features = ["csr"] }}
leptos_meta = "0.8.6"
"#
    );
    fs::write(format!("{}/Cargo.toml", name), contents)?;
    Ok(())
}

pub fn write_manifest_ssr(name: &str) -> Result<()> {
    // Rust identifiers can't contain dashes; Cargo auto-converts them for
    // the crate's `use` path (e.g. package "my-app" -> `use my_app::...`).
    // main.rs needs this normalized form, so we compute it once here and
    // reuse it in write_main_ssr / write_lib_ssr.
    let crate_ident = name.replace('-', "_");

    let contents = format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
detaxine-ui = {{ version = "0.8.60", default-features = false }}
leptos = "0.8.20"
leptos_router = "0.8.14"
leptos_meta = "0.8.6"
axum = {{ version = "0.8.9", optional = true }}
console_error_panic_hook = {{ version = "0.1", optional = true }}
leptos_axum = {{ version = "0.8.10", optional = true }}
tokio = {{ version = "1", features = ["rt-multi-thread"], optional = true }}
wasm-bindgen = {{ version = "0.2.126", optional = true }}

[features]
hydrate = [
    "leptos/hydrate",
    "dep:console_error_panic_hook",
    "dep:wasm-bindgen",
    "detaxine-ui/hydrate",
]
ssr = [
    "dep:axum",
    "dep:tokio",
    "dep:leptos_axum",
    "leptos/ssr",
    "leptos_meta/ssr",
    "leptos_router/ssr",
    "detaxine-ui/ssr",
]

[profile.wasm-release]
inherits = "release"
opt-level = 'z'
lto = true
codegen-units = 1
panic = "abort"

[package.metadata.leptos]
output-name = "{crate_ident}"
site-root = "target/site"
site-pkg-dir = "pkg"
tailwind-input-file = "styles/input.css"
assets-dir = "public"
site-addr = "127.0.0.1:3000"
reload-port = 3001
browserquery = "defaults"
env = "DEV"
bin-features = ["ssr"]
bin-default-features = false
lib-features = ["hydrate"]
lib-default-features = false
lib-profile-release = "wasm-release"
"#
    );
    fs::write(format!("{}/Cargo.toml", name), contents)?;
    Ok(())
}

pub fn write_main(name: &str) -> Result<()> {
    let contents = r#"use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use detaxine_ui::{
    components::{
        actions::button::{BasicButton, ButtonGroup},
        forms::toggle_switch::ToggleSwitch,
    },
    icondata::{AiCheckCircleOutlined, BsXCircle},
};
use detaxine_ui::stacks::z_stack::provide_z_stack;

#[component]
fn App() -> impl IntoView {
    provide_z_stack();
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/style/output.css"/>
        <h1>"Hello from detaxine-ui!"</h1>
        <ButtonGroup style_ext="font-bold bg-primary text-white hover:bg-secondary">
            <BasicButton
                button_text="First"
                icon=Some(AiCheckCircleOutlined)
                icon_before=true
            />
            <BasicButton
                button_text="Second"
                icon=Some(BsXCircle)
                icon_before=false
            />
            <BasicButton
                button_text="Third"
                disabled=true
            />
        </ButtonGroup>
        <ToggleSwitch
           initial_active_state=true
           label_active="Enabled"
           label_inactive="Disabled"
           name="status"
        />
    }
}
fn main() {
    mount_to_body(App)
}
"#;
    fs::write(format!("{}/src/main.rs", name), contents)?;
    Ok(())
}

pub fn write_main_ssr(name: &str) -> Result<()> {
    let crate_ident = name.replace('-', "_");

    let contents = format!(
        r#"#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {{
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{{generate_route_list, LeptosRoutes}};
    use {crate_ident}::app::*;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {{
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        }})
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    log!("listening on http://{{}}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}}

#[cfg(not(feature = "ssr"))]
pub fn main() {{
    // no client-side main function; see lib.rs for the hydrate entry point
}}
"#
    );
    fs::write(format!("{}/src/main.rs", name), contents)?;
    Ok(())
}

pub fn write_lib_ssr(name: &str) -> Result<()> {
    let contents = r#"pub mod app;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
"#;
    fs::write(format!("{}/src/lib.rs", name), contents)?;
    Ok(())
}

pub fn write_app_ssr(name: &str) -> Result<()> {
    let crate_ident = name.replace('-', "_");

    let contents = format!(
        r#"use leptos::prelude::*;
use leptos_meta::{{MetaTags, Stylesheet, Title, provide_meta_context}};
use leptos_router::{{
    StaticSegment,
    components::{{Route, Router, Routes}},
}};
use detaxine_ui::{{
    components::{{
        actions::button::{{BasicButton, ButtonGroup}},
        forms::toggle_switch::ToggleSwitch,
    }},
    icondata::{{AiCheckCircleOutlined, BsXCircle}},
}};
use detaxine_ui::stacks::z_stack::provide_z_stack;

pub fn shell(options: LeptosOptions) -> impl IntoView {{
    view! {{
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }}
}}

#[component]
pub fn App() -> impl IntoView {{
    provide_meta_context();
    provide_z_stack();

    view! {{
        <Stylesheet id="leptos" href="/pkg/{crate_ident}.css"/>
        <Title text="detaxine-ui"/>
        <div id="modal-root"></div>
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }}
}}

#[component]
fn HomePage() -> impl IntoView {{
    view! {{
        <h1>"Hello from detaxine-ui!"</h1>
        <ButtonGroup style_ext="font-bold bg-primary text-white hover:bg-secondary">
            <BasicButton
                button_text="First"
                icon=Some(AiCheckCircleOutlined)
                icon_before=true
            />
            <BasicButton
                button_text="Second"
                icon=Some(BsXCircle)
                icon_before=false
            />
            <BasicButton
                button_text="Third"
                disabled=true
            />
        </ButtonGroup>
        <ToggleSwitch
           initial_active_state=true
           label_active="Enabled"
           label_inactive="Disabled"
           name="status"
        />
    }}
}}
"#
    );
    fs::write(format!("{}/src/app.rs", name), contents)?;
    Ok(())
}

pub fn write_cargo_config_ssr(name: &str) -> Result<()> {
    fs::create_dir_all(format!("{}/.cargo", name))?;
    let contents = r#"[env]
LEPTOS_TAILWIND_VERSION = "v4.3.2"
"#;
    fs::write(format!("{}/.cargo/config.toml", name), contents)?;
    Ok(())
}
