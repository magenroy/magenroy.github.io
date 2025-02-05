use dioxus::prelude::*;

use crate::components::Header;



#[derive(serde::Deserialize)]
struct ResourceListApi {
    header: String,
    content: String,
}

const RESOURCELIST_DIR: Asset = asset!("/assets/static/resources/");

#[component]
pub fn ResourceLists() -> Element {
    let path = RESOURCELIST_DIR.resolve();
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
            section { id: "resources", class: "content",
                h2 {"Resources"}
                p { "These are unorganized lists of resources on various topics that I decided to collect at some point in the beginning of my PhD" }
                // {Content}
                ul {
                    li {
                        a { href: "https://math.columbia.edu/~magenroy/resources.html", "Unsorted resources" }
                    }
                    li {
                        a { href: "https://math.columbia.edu/~magenroy/DAG-resources.html", "DAG/SAG resources" }
                    }
                    li {
                        a { href: "https://math.columbia.edu/~magenroy/Motives-resources.html", "Motives resources" }
                    }
                }
            }
        }
    }
}

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
