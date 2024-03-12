use leptos::*;
use leptos_meta::*;

#[component]
pub fn GreenBtn(children: Children) -> impl IntoView {
    view! {
        <button class="text-center text-black p-4 rounded 
        bg-gradient-to-tl from-green-600 to-green-400 
        hover:bg-gradient-to-tl hover:from-emerald-500 hover:to-emerald-400 
        active:bg-gradient-to-tl active:from-teal-600 active:to-teal-500">{children()}</button>
    }
}
#[component]
pub fn PurpleBtn(children: Children) -> impl IntoView {
    view! {
        <button class="text-center text-black p-4 rounded 
        bg-gradient-to-tl from-fuchsia-500 to-purple-400 
        hover:bg-gradient-to-tl hover:from-violet-500 hover:to-violet-400 
        active:bg-gradient-to-tl active:from-purple-600 active:to-purple-300">{children()}</button>
    }
}
#[component]
pub fn RedBtn(children: Children) -> impl IntoView {
    view! {
        <button class="text-center text-black p-4 rounded 
        bg-gradient-to-tl from-rose-400 to-red-500 
        hover:bg-gradient-to-tl hover:from-pink-500 hover:to-pink-500 
        active:bg-gradient-to-tl active:from-rose-500 active:to-pink-600">{children()}</button>
    }
}
#[component]
pub fn OrangeBtn(children: Children) -> impl IntoView {
    view! {
        <button class="text-center text-black p-4 rounded 
        bg-gradient-to-tl from-amber-600 to-amber-500 
        hover:bg-gradient-to-tl hover:from-orange-700 hover:to-orange-400 
        active:bg-gradient-to-tl active:from-yellow-800 active:to-yellow-600">{children()}</button>
    }
}
#[component]
pub fn GrayBtn(children: Children) -> impl IntoView {
    view! {
        <button class="text-center text-black p-4 rounded 
        bg-stone-500">{children()}</button>
    }
}
