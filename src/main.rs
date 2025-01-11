#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    dioxus::launch(Home);
}

fn Home() -> Element {
    let mut weight = use_signal(|| 0.0);
    let mut reps = use_signal(|| 1);
    let mut onerm = use_signal(|| 0.);
    let set_reps = move |event: Event<FormData>| {
        reps.set(event.parsed::<f64>().unwrap_or_else(|_| {0.}).round().abs() as i32);
        if reps.cloned() > 9 {
            onerm.set(weight.cloned() * (0. + reps.cloned() as f64 / 30.));
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
        // document::Stylesheet { href: CSS }
        br {  }
        div {
            class: "container",
            nav {
                class: "nav-center",
                "1RM Calculator"
            }
            br{}
            div {
                class: "row",
                div {
                    class: "col-4",
                    "Repetitions"
                }
                div {
                    class: "col-8",
                    input {
                        type: "number",
                        oninput: set_reps,
                        value: "{reps}"
                    }
                }
            }
            div {
                class: "row",
                div {
                    class: "col-4",
                    "Weight"
                }
                div {
                    class: "col-8",
                    input {
                        type: "number",
                        oninput: set_weight,
                        value: "{weight}"
                    }
                }
            }
            br {}
            div {
                class: "row",
                div {
                    class: "col",
                    "Your 1RM: {onerm:.2}"
                }
            }
            br {}
            for i in 0..30 {
                div {
                    class: "row bd-light",
                    padding: "1rem 0rem",
                    div {
                        class: "col",
                        margin: "0rem",
                        div {
                            class: "is-center",
                            "Weight: {calculate(i):.2}"
                        }
                    }
                    div {
                        class: "col",
                        margin: "0rem",
                        div {
                            class: "is-center",
                            "Repetitions: {i+1}"
                        }
                    }
                }
            }
            br {}
        }
    }
}
