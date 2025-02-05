use dioxus::{logger, prelude::*};

use crate::components::Header;

#[derive(serde::Deserialize)]
struct SeminarApi {
    header: String,
    main: String,
}

// const SEMINAR_STYLE: Asset = asset!("/assets/styling/seminar.css");

const SEMINAR_DIR: Asset = asset!("/assets/static/seminars/");

#[component]
pub fn Seminars() -> Element {
    let path = SEMINAR_DIR.resolve();
    let Content: Element = match std::fs::read_dir(path) {
        Ok(x) => rsx! {
            "a"
        },
        Err(_) => rsx! {
            "b"
        },
    }; 

    rsx! {
        main {
            Header {  }
            section { id: "seminars", class: "content",
                h2 {"Seminars"}
                p { "Seminars I have organized:" }
                // {Content}
                ul {
                    li {
                    a { href: "https://math.columbia.edu/~magenroy/DAG-seminar.html", "Derived algebraic geometry seminar at Columbia University" }
                    }
                    li {
                    a { href: "https://math.columbia.edu/~magenroy/motivicseminar.html", "Motivic homotopy theory seminar at Columbia University" }
                    }
                    li {
                    a { href: "https://math.columbia.edu/~magenroy/MilnorWittMotivesSeminar.html", "Milnor-Witt motives seminar at Columbia University" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Seminar(name: String) -> Element {
    // let path = format!("assets/static/seminars/{}.toml", name);

    let mut path = SEMINAR_DIR.resolve();
    path.push(&name);

    logger::tracing::debug!("{:?}", path);

    let Content: Element = match std::fs::read_to_string(path) {
        Ok(c) => {
            match toml::from_str(&c) {
                Ok(SeminarApi { header, main }) => rsx! {
                    header { div { class: "content",
                        dangerous_inner_html: "{header}",
                    } }
                    main { class: "content", class: "seminar",
                        dangerous_inner_html: "{main}",
                    }
                },
                Err(_) => rsx! {
                    main { class: "error",
                        h1 { "Error loading seminar webpage" }
                        pre { color: "red", "log:\nattemped to load {name:?}" }
                    }
                }
            }
        },
        Err(_) => rsx! {
            main { class: "error",
                h1 { "Seminar not found" }
                p { "The page you requested doesn't exist." }
                pre { color: "red", "log:\nattemped to load {name:?}" }
            }
        }
    };

    rsx! {
        // document::Link { rel: "stylesheet", href: SEMINAR_STYLE }
        main {
            {Content}
        }
    }
}
