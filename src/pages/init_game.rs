use leptos::*;
use leptos_meta::*;

#[component]
pub fn InitGame() -> impl IntoView {
        // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    view! {
    <Title text="Seapower - Initialization"/>
    // <Stylesheet id="stylesheet" href="/pkg/style.css"/>
    <script src="https://unpkg.com/htmx.org@1.9.10" integrity="sha384-D1Kt99CQMDuVetoL1lrYwg5t+9QdHe7NLX/SoJYkXDFfX37iInKRy5xLSi8nO7UC" crossorigin="anonymous"></script>
    <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
    <div class="mx-auto p-8 space-y-8">
    <h1 class="h1">Welcome to Seapower</h1>
    <p>
    "This is the home page, which isn't much."
    </p>
    <p>
    "(spoiler alert, it probably won't work right)"
    </p>
    </div>
            <div class="mx-auto p-8 space-y-8">
            <a href="/">Home</a>
            </div>
            </div>
        }
}
