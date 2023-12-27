use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::home::Home;
use crate::pages::init_game::InitGame;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <link rel="stylesheet" href="/customstyle.css" />
        <Router>
            <Routes>
                <Route path="/" view=  move || view! { <Home/> }/>
                <Route path="/init_game" view=  move || view! { <InitGame/> }/>
            </Routes>
        </Router>
    }
}


