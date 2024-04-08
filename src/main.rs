use leptos::*;

use components::demo_control_flow::*;
use components::demo_form_and_input::*;
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
                <DemoBasicIteration/>
            </li>
            <li>
                <DemoComplexDataIteration/>
            </li>
            <li>
                <DemoFormAndInput/>
            </li>
        </ul>
    }
}
