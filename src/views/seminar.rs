use dioxus::prelude::*;

#[derive(serde::Deserialize)]
struct SeminarApi {
    header: String,
    main: String,
}

// const SEMINAR_STYLE: Asset = asset!("/assets/styling/seminar.css");

#[component]
pub fn Seminar(name: String) -> Element {
    let path = format!("assets/static/seminars/{}.toml", name);

    let Content: Element = match std::fs::read_to_string(path) {
        Ok(c) => {
            match toml::from_str(&c) {
                Ok(SeminarApi { header, main }) => {
                    rsx!{
                        header { div { class: "content",
                            dangerous_inner_html: "{header}",
                        } }
                        main { class: "content", class: "seminar",
                            dangerous_inner_html: "{main}",
                        }
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
