use dioxus::prelude::*;

#[component]
pub fn ResourceLists() -> Element {
    // let path = RESOURCELIST_DIR.resolve();
    // let Content: Element = match std::fs::read_dir(path) {
    //     Ok(x) => rsx! {
    //         "a"
    //     },
    //     Err(_) => rsx! {
    //         "b"
    //     },
    // }; 

    rsx! {
        h2 {"Resources"}
        p { "These are unorganized lists of resources on various topics that I decided to collect at some point in the beginning of my PhD" }
        // {Content}
        ul {
            li {
                // a { href: "https://math.columbia.edu/~magenroy/resources.html", "Unsorted resources" }
                a { href: "Columbia/resources.html", "Unsorted resources" }
            }
            li {
                // a { href: "https://math.columbia.edu/~magenroy/DAG-resources.html", "DAG/SAG resources" }
                a { href: "Columbia/DAG-resources.html", "DAG/SAG resources" }
            }
            li {
                // a { href: "https://math.columbia.edu/~magenroy/Motives-resources.html", "Motives resources" }
                a { href: "Columbia/Motives-resources.html", "Motives resources" }
            }
        }
    }
}
