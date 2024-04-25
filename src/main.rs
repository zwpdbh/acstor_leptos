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
        <h1>Leptos demos</h1>
        <Router>
            <nav>
                <ul>

                    <li>
                        <a href="/demos/basic_component">basic components</a>
                    </li>
                    <li>
                        <a href="/demos/components_and_pros">components and props</a>
                    </li>
                    <li>
                        <a href="/demos/demo_basic_iteration">basic iterator</a>
                    </li>
                    <li>
                        <a href="/demos/demo_form_and_input">form and input</a>
                    </li>
                    <li>
                        <a href="/demos/demo_error_handling">error handling</a>
                    </li>
                    <li>
                        <a href="/demos/demo_reactivity">reactivity</a>
                    </li>
                    <li>
                        <a href="/demos/demo_parent_children_communication">
                            parent child communication
                        </a>
                    </li>
                    <li>
                        <a href="/demos/demo_async">demo async</a>
                    </li>
                    <li>
                        <a href="/demos/control_flow">demo control flow</a>
                    </li>
                </ul>

            </nav>
            <main>
                // all our routes will appear inside <main>
                <Routes>
                    <Route path="/demos/basic_component" view=BasicComponent/>
                    <Route path="/demos/components_and_pros" view=ComponentsAndProps/>
                    <Route path="/demos/demo_basic_iteration" view=DemoBasicIteration/>
                    <Route path="/demos/demo_form_and_input" view=DemoFormAndInput/>
                    <Route path="/demos/demo_error_handling" view=DemoErrorHandling/>
                    <Route path="/demos/demo_reactivity" view=DemoReactivity/>
                    <Route
                        path="/demos/demo_parent_children_communication"
                        view=DemoParentChildrenCommunication
                    />
                    <Route path="/demos/demo_async" view=DemoAsync/>
                    <Route path="/demos/control_flow" view=DemoControlFlow/>
                    <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
                </Routes>

            </main>
        </Router>
    }
}
