use leptos::*;

use components::demo_iteration::*;
use components::*;

mod components;
fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <ul>
            <li>
                <BasicComponent/>
            </li>
            <li>
                <ComponentsAndProps/>
            </li>
            <li>
                <DemoStaticView/>
            </li>
        </ul>
    }
}
