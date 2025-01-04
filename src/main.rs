#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    dioxus::LaunchBuilder::new()
    .with_cfg(server_only! {
        ServeConfig::builder()
            // turn on incremental site generation with the .incremental() method
            .incremental(IncrementalRendererConfig::new())
            .build()
            .unwrap()
    })
    .launch(|| {
        rsx! {
            Router::<Route> {}
        }
    })
}

#[server(endpoint = "static_routes")]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    Ok(Route::static_routes()
        .into_iter()
        .map(|route| route.to_string())
        .collect::<Vec<_>>())
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    Home,
}

fn Home() -> Element {
    rsx! {
        "Home test"
    }
}
