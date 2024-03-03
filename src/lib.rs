use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use thiserror::Error;
// Modules
mod pages;
mod types;
mod components;

use pages::init_game::InitGame;
use pages::play_game::PlayGame;
use types::util::GameState;
#[derive(Debug, Error)]
pub enum SPError {
    #[error("I'm so lazy")]
    CatchAll,
}

// Top-Level pages

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let rw_glob_state = create_rw_signal(GameState::default());
    let rw_glob_selected = create_rw_signal(false);
    provide_context(rw_glob_state);
    provide_context(rw_glob_selected);
    view! {
        <Html lang="en" dir="ltr" attr:data-theme="light"/>

        // sets the document title
        <Title text="Welcome to Leptos CSR"/>

        // injects metadata in the <head> of the page
        <Meta charset="utf-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <Router>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/init_game" view=InitGame/>
                    <Route path="/play_game" view=PlayGame/>
                </Routes>
            </Router>

        </ErrorBoundary>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    /* TODO:
       - Probably should implement some kind of help/instructions/rules
       - Sidebar/
    */
    view! {
        <Title text="Seapower"/>
        <main>
            <div class="base-div">
                <div class="mx-auto p-8 space-y-8">
                    <h1 class="h1 text-center">Welcome to Seapower</h1>
                    <p>"This is the home page, which isn't much"</p>
                </div>
                <div class="mx-auto p-4 space-y-4">
                    <a href="/init_game">
                        <button class="btn-green">"Start a Game"</button>
                    </a>
                </div>
            </div>
        </main>
    }
}
