use leptos::*;
use leptos_router::*;

use components::demo_async::*;
use components::demo_control_flow::*;
use components::demo_error_handling::*;
use components::demo_form_and_input::*;
use components::demo_iteration::*;
use components::demo_parent_children_communication::*;
use components::demo_reactivity::*;
use components::*;

mod components;
fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <nav></nav>
            <main>
                // all our routes will appear inside <main>
                <Routes>
                    <Route path="/demos/basic_component" view=BasicComponent/>
                    <Route path="/demos/components_and_pros" view=ComponentsAndProps/>
                    <Route path="/demos/demo_basic_iteration" view=DemoBasicIteration/>
                    <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
                </Routes>

            </main>
        </Router>
    }
}

// <ul>
//     <li>
//         <BasicComponent/>
//     </li>
//     <li>
//         <ComponentsAndProps/>
//     </li>
//     <li>
//         <DemoBasicIteration/>
//     </li>
//     <li>
//         <DemoComplexDataIteration/>
//     </li>
//     <li>
//         <DemoFormAndInput/>
//     </li>
//     <li>
//         <DemoControlFlow/>
//     </li>
//     <li>
//         <DemoErrorHandling/>
//     </li>
//     <li>
//         <DemoParentChildrenCommunication/>
//     </li>
//     <li>
//         <DemoReactivity/>
//     </li>
//     <li>
//         <DemoAsync/>
//     </li>
// </ul>
