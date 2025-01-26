use leptos::*;
use mount::mount_to_body;

#[component]
fn App() -> impl IntoView {
    view! {
    <div class="container">

            <picture>
                <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg" media="(prefers-color-scheme: dark)" />
                <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo" height="200" width="400"/>
            </picture>

        <h1>"Welcome to Leptos"</h1>
        <h2><i>"On Github Pages"</i></h2>

    </div>
    }
}

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
