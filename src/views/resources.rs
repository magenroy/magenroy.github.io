use dioxus::prelude::*;



#[derive(serde::Deserialize)]
struct ResourceListApi {
    header: String,
    content: String,
}

const RESOURCELIST_DIR: Asset = asset!("/assets/static/resources/");

#[component]
pub fn ResourceList(name: String) -> Element {
    // let path = format!("assets/static/resources/{}.toml", name);

    let mut path = RESOURCELIST_DIR.resolve();
    path.push(&name);


    let Content: Element = match std::fs::read_to_string(path) {
        Ok(c) => {
            match toml::from_str(&c) {
                Ok(ResourceListApi { header, content }) => {
                    rsx!{
                        header { div { class: "content", "{header}" } }
                        main { class: "content",
                            dangerous_inner_html: "{content}",
                        }
                    }
                },
                Err(_) => rsx! {
                        main { class: "error",
                        h1 { "Resource list error" }
                        p { "Error parsing resource list" }
                        pre { color: "red", "log:\nattemped to load {name:?}" }
                    }
                }
            }
        },
        Err(_) => rsx! {
            main { class: "error",
                h1 { "Resource list not found" }
                p { "The page you requested doesn't exist." }
                pre { color: "red", "log:\nattemped to load {name:?}" }
            }
        }
    };

    rsx! {
        main {
            {Content}
        }
    }
}
