use dioxus::prelude::*;

use components::Navbar;
use components::Header;
use views::{Blog, Home, Seminars, Seminar, ResourceList, ResourceLists};

mod components;
mod views;

// NOTE: remember to change this if changing domain name!
const URL: &str = "https://magenroy.github.io";

const DESCRIPTION: &str = "Website of Roy Magen";
const AUTHOR: &str = "Roy Magen";
const NAME: &str = "Roy Magen";

const FAVICON: Asset = asset!("/assets/graphics/favicon.ico");
const MAIN_STYLE: Asset = asset!("/assets/styling/main.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},

        #[route("/seminar")]
        Seminars {},

        #[route("/seminar/:name")]
        Seminar { name: String },

        #[route("/resources")]
        ResourceLists { },

        #[route("/resources/:name")]
        ResourceList { name: String },

        #[route("/blog/:id")]
        Blog { id: i32 },

        #[route("/:..route")]
        PageNotFound { route: Vec<String> },

    // #[layout(!Navbar)]

    // #[end_layout]
}

fn main() {
    // dioxus::launch(App);

    // REF: https://dioxuslabs.com/blog/release-060#static-site-generation-and-isg
    dioxus::LaunchBuilder::new()
        .with_cfg(server_only! {
            ServeConfig::builder()
                // turn on incremental site generation with the .incremental() method
                .incremental(IncrementalRendererConfig::new())
                .build()
                .unwrap()
        })
        .launch(App)
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        document::Meta { name: "description", content: DESCRIPTION }
        document::Meta { name: "author", content: AUTHOR }
        document::Meta { itemprop: "description", content: DESCRIPTION }
        document::Meta { itemprop: "name", content: NAME }

        document::Link { rel: "canonical", href: URL }

        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_STYLE }


        Router::<Route> {}
    }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        main { class: "error",
            h1 { "Page not found" }
            p { "The page you requested doesn't exist." }
            pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
        }
    }
}

// REF: https://dioxuslabs.com/blog/release-060/#static-site-generation-and-isg
#[server(endpoint = "static_routes")]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    Ok(Route::static_routes()
        .into_iter()
        .map(|route| route.to_string())
        .collect::<Vec<_>>())
}
