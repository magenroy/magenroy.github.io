use crate::Route;
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        nav {
            id: "navbar",
                Link {
                    to: Route::Home {},
                    "Home"
                }

                a {
                    href: "https://arxiv.org/a/0009-0000-9597-071X",
                    "Research"
                }

                Link {
                    to: Route::Seminars {},
                    "Seminars"
                }

                Link {
                    to: Route::ResourceLists {},
                    "Resources"
                }

                /* Link {
                    to: Route::Blog { id: 1 },
                    "Blog"
                } */
        }

        Outlet::<Route> {}
    }
}
