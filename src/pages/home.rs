use leptos::*;
use leptos_meta::*;

#[component]
pub fn Home() -> impl IntoView {
    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    view! {
        <Title text="Seapower"/>
        <main>
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
                    <a href="/init_game">InitGame</a>
                </div>
            </div>
        </main>
    }
}