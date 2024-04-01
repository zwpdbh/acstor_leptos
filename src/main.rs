use components::Counter;
use leptos::*;
mod components;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! { <Counter/> }
}
