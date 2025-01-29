use dioxus::prelude::*;

use components::Navbar;
use views::{Blog, Home};

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/graphics/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

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
        document::Meta { name: "description", content: "Website of Roy Magen" }
        document::Meta { name: "author", content: "Roy Magen" }
        document::Meta { itemprop: "description", content: "Website of Roy Magen" }
        document::Meta { itemprop: "name", content: "Roy Magen" }

        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }


        Router::<Route> {}
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
