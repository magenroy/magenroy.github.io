use dioxus::prelude::*;

use components::Navbar;
use views::{Blog, Home, Seminars, Seminar, ResourceList, ResourceLists};

mod components;
mod views;

// NOTE: remember to change this if changing domain name!
pub const URL: &str = "https://magenroy.github.io";

pub const DESCRIPTION: &str = "Website of Roy Magen";
pub const AUTHOR: &str = "Roy Magen";
pub const NAME: &str = "Roy Magen";

pub const FAVICON: Asset = asset!("/assets/graphics/favicon.ico");
pub const MAIN_STYLE: Asset = asset!("/assets/styling/main.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
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
    // dioxus::LaunchBuilder::new()
    //     .with_cfg(server_only! {
    //         ServeConfig::builder()
    //             // turn on incremental site generation with the .incremental() method
    //             .incremental(IncrementalRendererConfig::new())
    //             .build()
    //             .unwrap()
    //     })
    // .launch(App)
    // now SOURCE: https://dioxuslabs.com/learn/0.7/essentials/fullstack/static_site_generation#setting-up-the-serveconfig
    dioxus::LaunchBuilder::new()
        // Set the server config only if we are building the server target
        .with_cfg(server_only! {
            ServeConfig::builder()
                // Enable incremental rendering
                .incremental(
                    dioxus::server::IncrementalRendererConfig::new()
                        // Store static files in the public directory where other static assets like wasm are stored
                        .static_dir(
                            std::env::current_exe()
                                .unwrap()
                                .parent()
                                .unwrap()
                                .join("public")
                        )
                        // Don't clear the public folder on every build. The public folder has other files including the wasm
                        // binary and static assets required for the app to run
                        .clear_cache(false)
                )
                .enable_out_of_order_streaming()
        })
        .launch(App);
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
// #[server(endpoint = "static_routes")]
// async fn static_routes() -> Result<Vec<String>, ServerFnError> {
//     Ok(Route::static_routes()
//         .into_iter()
//         .map(|route| route.to_string())
//         .collect::<Vec<_>>())
// }
// now SOURCE: https://dioxuslabs.com/learn/0.7/essentials/fullstack/static_site_generation#configuring-static-routes
//
// The server function at the endpoint "static_routes" will be called by the CLI to generate the list of static
// routes. You must explicitly set the endpoint to `"static_routes"` in the server function attribute instead of
// the default randomly generated endpoint.
#[server(endpoint = "static_routes", output = server_fn::codec::Json)]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    // The `Routable` trait has a `static_routes` method that returns all static routes in the enum
    Ok(Route::static_routes()
        .iter()
        .map(ToString::to_string)
        .collect())
}
