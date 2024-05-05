use leptos::*;
use leptos_router::*;

use components::demo_async::*;
use components::demo_control_flow::*;
use components::demo_error_handling::*;
use components::demo_form_and_input::*;
use components::demo_iteration::*;
use components::demo_nested_route::*;
use components::demo_parent_children_communication::*;
use components::demo_reactivity::*;
use components::*;

mod components;
fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
pub fn DemoIndex() -> impl IntoView {
    view! {
        <h2>Demo index</h2>

        <ul>
            <li>
                <A href="basic_component">basic components</A>
            </li>
            <li>
                <A href="components_and_pros">components and props</A>
            </li>
            <li>
                <A href="demo_basic_iteration">basic iterator</A>
            </li>
            <li>
                <A href="demo_form_and_input">form and input</A>
            </li>
            <li>
                <A href="demo_error_handling">error handling</A>
            </li>
            <li>
                <A href="demo_reactivity">reactivity</A>
            </li>
            <li>
                <A href="demo_parent_children_communication">parent child communication</A>
            </li>
            <li>
                <A href="demo_async">demo async</A>
            </li>
            <li>
                <A href="control_flow">demo control flow</A>
            </li>
            <li>
                <A href="demo_nested_route">demo nested route</A>
            </li>
        </ul>

        // <Outlet/> will show the nested child route
        // we can position this outlet wherever we want
        // within the layout
        <Outlet/>
    }
}

#[component]
pub fn DemoDetail() -> impl IntoView {
    // we can access the :id param reactively with `use_params_map`.
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    let component = move || match id().as_str() {
        "basic_component" => view! { <BasicComponent/> },
        "components_and_pros" => view! { <ComponentsAndProps/> },
        "demo_basic_iteration" => view! { <DemoBasicIteration/> },
        "demo_form_and_input" => view! { <DemoFormAndInput/> },
        "demo_error_handling" => view! { <DemoErrorHandling/> },
        "demo_reactivity" => view! { <DemoReactivity/> },
        "demo_parent_children_communication" => view! { <DemoParentChildrenCommunication/> },
        "demo_async" => view! { <DemoAsync/> },
        "control_flow" => view! { <DemoControlFlow/> },
        "demo_nested_route" => view! { <DemoNestedRoute/> },
        _ => view! { <MessageComponent/> },
    };

    component.into_view()
}

#[component]
fn App() -> impl IntoView {
    view! {
        <h1>Leptos demos nav</h1>
        <Router>
            <nav>
                <ul>
                    <li>
                        <a href="/">Home</a>
                    </li>
                    <li>
                        <a href="/demos">Demo Index</a>
                    </li>
                </ul>
            </nav>
            <main>
                // all our routes will appear inside <main>
                <Routes>

                    <Route path="/" view=|| view! { <h2>"Home"</h2> }/>
                    <Route path="/demos" view=DemoIndex>

                        <Route path=":id" view=DemoDetail/>
                        <Route
                            path=""
                            view=|| {
                                view! {
                                    <div class="select-demo">
                                        "Select a demo to see the details."
                                    </div>
                                }
                            }
                        />

                    </Route>

                    <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
                </Routes>

            </main>
        </Router>
    }
}
