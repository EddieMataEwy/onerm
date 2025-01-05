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

static CSS: Asset = asset!("/assets/chota.min.css");

fn Home() -> Element {
    let mut weight = use_signal(|| 0.0);
    let mut reps = use_signal(|| 1);
    let mut onerm = use_signal(|| 0.);
    let set_reps = move |event: Event<FormData>| {
        reps.set(event.parsed::<f64>().unwrap_or_else(|_| {1.}).round().abs() as i32);
        if reps.cloned() > 9 {
            onerm.set(weight.cloned() * (1. + reps.cloned() as f64 / 30.));
        } else {
            onerm.set(weight.cloned() / (1.0278 - reps.cloned() as f64 * 0.0278));
        }
    };
    let set_weight = move |event: Event<FormData>| {
        weight.set(event.parsed().unwrap_or_else(|_| {0. as f64}).abs());
        if reps.cloned() > 9 {
            onerm.set(weight.cloned() * (1. + reps.cloned() as f64 / 30.));
        } else {
            onerm.set(weight.cloned() / (1.0278 - reps.cloned() as f64 * 0.0278));
        }
    };
    let calculate = move |repetitions: usize| {
        let repetitions = repetitions + 1;
        if repetitions < 9 {
            onerm.cloned() * (1.0278 - repetitions as f64 * 0.0278)
        } else {
            onerm.cloned() / (1. + repetitions as f64 / 30.)
        }
    };
    rsx! {
        document::Stylesheet { href: CSS }
        br {  }
        div {
            class: "container",
            nav {
                class: "nav-center",
                "1RM Calculator"
            }
            br{}
            div {
                "Repetitions"
            }
            input {
                type: "number",
                oninput: set_reps,
                value: "{reps}"
            }
            div {
                "Weight"
            }
            input {
                type: "number",
                oninput: set_weight,
                value: "{weight}"
            }
            br {}
            div {
                "Your 1RM: {onerm:.2}"
            }
            for i in 0..30 {
                div {
                    "Weight: {calculate(i):.2}, Repetitions: {i+1}"
                }
            }
        }
    }
}
