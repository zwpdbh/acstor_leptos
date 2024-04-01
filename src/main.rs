use leptos::*;

use components::*;

mod components;
fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <ul>
            <BasicComponent/>
        </ul>

        <ul>
            <DynamicAttributes/>
        </ul>
    }
}
