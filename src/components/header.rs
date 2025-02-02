use dioxus::prelude::*;

use crate::Route;

// const PHOTO: Asset = asset!("/assets/graphics/photo.png");

#[component]
pub fn Header() -> Element {
    rsx! {
        header {
             table { class: "content",
                tr {
                    td { "valign": "top",
                        h1 { "Roy Magen" }
                        p {
                            "PhD student at "
                            br {}
                            a { href: "https://www.math.columbia.edu/", " Columbia University " }
                            br {}
                        }
                        p { "Email: magenroy[at]math[dot]columbia[dot]edu" }
                    }
                    td { "\u{a0} \u{a0} \u{a0} \u{a0} \u{a0}" }
                    // td { img { src: PHOTO, id: "photo" } }
                }
            }
        }
        // Outlet::<Route> {}
    }
}
