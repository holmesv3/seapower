use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::init_game::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <link rel="stylesheet" href="/customstyle.css" />
        <link rel="default" href="/shipmanifest.json" />
        <script src="https://unpkg.com/htmx.org@1.9.10" integrity="sha384-D1Kt99CQMDuVetoL1lrYwg5t+9QdHe7NLX/SoJYkXDFfX37iInKRy5xLSi8nO7UC" crossorigin="anonymous"></script>
        <Router>
            <Routes>
                <Route path="/" view=  move || view! { <Home/> }/>
                <Route path="/init_game" view=  move || view! { <InitGame/> }/>
                <Route path="/init_game/load_default" view=  move || view! {<LoadDefault/>}/>
            </Routes>
        </Router>
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
                    <p>
                        "This is the home page, which isn't much."
                    </p>
                </div>
                <div class="mx-auto p-4 space-y-4">
                    <a href="/init_game">
                    <button class="btn-green">
                        "Start a Game"
                    </button>
                    </a>
                </div>
            </div>
        </main>
    }
}
