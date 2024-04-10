
use gloo_timers::future::TimeoutFuture;
use leptos::*;

#[component]
pub fn DemoAsync() -> impl IntoView {
    view! {
        <h1>Demo Async</h1>
        <ul>
            <li>
                <Demo01/>
            </li>
            <li>
                <Demo02V1/>
            </li>
            <li>
                <Demo02V2/>
            </li>
            <li>
                <Demo03/>
            </li>
        </ul>
    }
}

// Here we define an async function
// This could be anything: a network request, database read, etc.
// Here, we just multiply a number by 10
async fn load_data(value: i32) -> i32 {
    // fake a one-second delay
    TimeoutFuture::new(1_000).await;
    value * 10
}
   

#[component]
pub fn Demo01() -> impl IntoView {
    // this count is our synchronous, local state
    let (count, set_count) = create_signal(0);

    // create_resource takes two arguments after its scope
    let async_data = create_resource(
        // the first is the "source signal"
        count,
        // the second is the loader
        // it takes the source signal's value as its argument
        // and does some async work
        |value| async move { load_data(value).await }, 
    );
    // whenever the source signal changes, the loader reloads

    // you can also create resources that only load once
    // just return the unit type () from the source signal
    // that doesn't depend on anything: we just load it once
    let stable = create_resource(|| (), |_| async move { load_data(1).await });

    // we can access the resource values with .read()
    // this will reactively return None before the Future has resolved
    // and update to Some(T) when it has resolved
    let async_result = move || {
        async_data
            .get()
            .map(|value| format!("Server returned {value:?}"))
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Loading...".to_string())
    };

    // the resource's loading() method gives us a
    // signal to indicate whether it's currently loading
    let loading = async_data.loading();
    let is_loading = move || if loading() { "Loading..." } else { "Idle."};

    view! {
        <h3>Load data with resource</h3>

        <button on:click=move |_| {
            set_count.update(|n| *n += 1);
        }>

            "Click me"
        </button>
        <p>
            <code>"stable"</code>
            ": "
            {move || stable.get()}
        </p>
        <p>
            <code>"count"</code>
            ": "
            {count}
        </p>
        <p>
            <code>"async_value"</code>
            ": "
            {async_result}
            <br/>
            {is_loading}
        </p>
    }
}

#[component]
fn ShowA(a: i32) -> impl IntoView {
    view! { <p>some A: {a}</p> }
}

#[component]
fn ShowB(b: i32) -> impl IntoView {
    view! { <p>some B: {b}</p> }
}


// Here we define an async function
// This could be anything: a network request, database read, etc.
// Here, we just multiply a number by 10
async fn load_a(value: i32) -> i32 {
    // fake a one-second delay
    TimeoutFuture::new(5_000).await;
    value * 10
}

async fn load_b(value: i32) -> i32 {
    // fake a one-second delay
    TimeoutFuture::new(1_000).await;
    value * 10
}
   

#[component]
pub fn Demo02V1() -> impl IntoView {
    let (count, _set_count) = create_signal(0);
    let (count2, _set_count2) = create_signal(0);
    let a = create_resource(count, |count| async move { load_a(count).await });
    let b = create_resource(count2, |count| async move { load_b(count).await });

    view! {
        <h3>"Demo: Wait two resources v1"</h3>
        {move || match (a.get(), b.get()) {
            (Some(a), Some(b)) => {
                view! {
                    <ShowA a/>
                    <ShowB b/>
                }
                    .into_view()
            }
            _ => view! { <p>"Loading..."</p> }.into_view(),
        }}
    }
}


#[component]
pub fn Demo02V2() -> impl IntoView {
    let (count, _set_count) = create_signal(0);
    let (count2, _set_count2) = create_signal(0);
    let a = create_resource(count, |count| async move { load_a(count).await });
    let b = create_resource(count2, |count| async move { load_b(count).await });

    view! {
        <h3>"Demo: Wait two resources v2"</h3>
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            <h4>"My Data"</h4>
            <h5>"A"</h5>
            {move || { a.get().map(|a| view! { <ShowA a/> }) }}

            <h5>"B"</h5>
            {move || { b.get().map(|b| view! { <ShowB b/> }) }}

        </Suspense>
    }
}

async fn fetch_monkeys(monkey: i32) -> i32 {
    // maybe this didn't need to be async
     TimeoutFuture::new(5_000).await;
    monkey * 2
}

#[component]
pub fn Demo03() -> impl IntoView {
    view! {
        <h3>"Demo: wait for some future to resolve before rendering"</h3>
        <Await
            // `future` provides the `Future` to be resolved
            future=|| fetch_monkeys(3)
            // the data is bound to whatever variable name you provide
            let:data
        >
            // you receive the data by reference and can use it in your view here
            <p>{*data} " little monkeys, jumping on the bed."</p>
        </Await>
    }
}