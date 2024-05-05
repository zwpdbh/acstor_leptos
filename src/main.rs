use leptos::*;
use leptos_router::*;

use components::demo_async::*;
use components::demo_control_flow::*;
use components::demo_error_handling::*;
use components::demo_form_and_input::*;
use components::demo_iteration::*;
use components::demo_parent_children_communication::*;
use components::demo_reactivity::*;
use components::demo_route::*;
use components::*;

mod components;
fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <h1>Leptos demos nav</h1>
        <Router>
            <nav>
                <ul>
                    <li>
                        <a href="/demos">Demo Index</a>
                    </li>
                </ul>
            </nav>
            <main>
                // all our routes will appear inside <main>
                <Routes>
                    <Route path="/demos" view=DemoIndex>
                        <Route path="basic_component" view=BasicComponent/>

                        <Route path="" view=DemoIndex/>
                    </Route>
                    // <Route path="/demos/basic_component" view=BasicComponent/>
                    // <Route path="/demos/components_and_pros" view=ComponentsAndProps/>
                    // <Route path="/demos/demo_basic_iteration" view=DemoBasicIteration/>
                    // <Route path="/demos/demo_form_and_input" view=DemoFormAndInput/>
                    // <Route path="/demos/demo_error_handling" view=DemoErrorHandling/>
                    // <Route path="/demos/demo_reactivity" view=DemoReactivity/>
                    // <Route
                    // path="/demos/demo_parent_children_communication"
                    // view=DemoParentChildrenCommunication
                    // />
                    // <Route path="/demos/demo_async" view=DemoAsync/>
                    // <Route path="/demos/control_flow" view=DemoControlFlow/>
                    // <Route path="/demos/route" view=DemoRoute/>
                    <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
                </Routes>

            </main>
        </Router>
    }
}
