use leptos::*;
use leptos_router::*;

#[component(transparent)]
pub fn RoutesForDemoNestedRoute() -> impl IntoView {
    view! {
        <Route path="demo_nested_route" view=DemoNestedRoute>

            <Route path="home" view=|| view! { <h3>"Nested Route Home"</h3> }/>
            <Route path="contacts" view=ContactList>
                // if no id specified, fall back
                <Route path=":id" view=ContactInfo>
                    <Route path="" view=|| view! { <div class="tab">"(Contact Info)"</div> }/>
                    <Route
                        path="conversations"
                        view=|| view! { <div class="tab">"(Conversations)"</div> }
                    />
                </Route>
                // if no id specified, fall back
                <Route
                    path=""
                    view=|| {
                        view! {
                            <div class="select-user">"Select a user to view contact info."</div>
                        }
                    }
                />

            </Route>

            <Route path="" view=|| view! { <p>"Select to see more"</p> }/>
        </Route>
    }
}

#[component]
pub fn DemoNestedRoute() -> impl IntoView {
    view! {
        <h3>Demo nested route</h3>

        <ul>
            <li>
                <A href="home">"Demo route home"</A>
            </li>
            <li>
                <A href="contacts">"Contacts"</A>
            </li>
        </ul>

        // <Outlet/> will show the nested child route
        // we can position this outlet wherever we want
        // within the layout
        <Outlet/>
    }
}

#[component]
fn ContactList() -> impl IntoView {
    view! {
        <div class="contact-list">
            // here's our contact list component itself
            <h3>"Contacts"</h3>
            <div class="contact-list-contacts">
                <ul>
                    <li>
                        <A href="alice">"Alice"</A>
                    </li>
                    <li>

                        <A href="bob">"Bob"</A>
                    </li>
                    <li>
                        <A href="steve">"Steve"</A>
                    </li>
                </ul>

            </div>

            // <Outlet/> will show the nested child route
            // we can position this outlet wherever we want
            // within the layout
            <Outlet/>
        </div>
    }
}

#[component]
fn ContactInfo() -> impl IntoView {
    // we can access the :id param reactively with `use_params_map`.
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    // imagine we're loading data from an API here
    let name = move || match id().as_str() {
        "alice" => "Alice",
        "bob" => "Bob",
        "steve" => "Steve",
        _ => "User not found.",
    };

    view! {
        <h4>{name}</h4>
        <div class="contact-info">
            <div class="tabs">
                <A href="" exact=true>
                    "Contact Info"
                </A>
                <A href="conversations">"Conversations"</A>
            </div>

            // <Outlet/> here is the tabs that are nested
            // underneath the /contacts/:id route
            <Outlet/>
        </div>
    }
}

#[component]
pub fn DemoNestRoute() -> impl IntoView {}
