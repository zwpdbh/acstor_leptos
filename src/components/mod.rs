use leptos::*;

#[component]
pub fn BasicComponent() -> impl IntoView {
    view! {
        <h2>Basic Component</h2>
        <Counter/>
    }
}

#[component]
pub fn Counter() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button on:click=move |_| {
            set_count.update(|n| *n += 1);
        }>

            // on stable, this is move || count.get();
            "Click me: " {move || count()}
        </button>
    }
}

#[component]
pub fn DynamicAttributes() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (x, set_x) = create_signal(0);

    let double_count = move || count() * 2;
    view! {
        <h2>Dynamic Classes, Styles and Attributes</h2>

        <h3>Dynamic Class</h3>
        <ol>
            <li>
                <button
                    on:click=move |_| {
                        set_count.update(|n| *n += 1);
                    }

                    class:red=move || count() % 2 == 1
                    class=("button-20", move || count() % 2 == 1)
                >

                    "Click me: "
                    {move || count}
                </button>
            </li>
        </ol>

        <h3>Dynamic Sytle</h3>
        <ol>

            <li>
                <button
                    on:click=move |_| {
                        set_x.update(|n| *n += 50);
                    }

                    // set the `style` attribute
                    style="position: absolute"
                    // and toggle individual CSS properties with `style:`
                    style:left=move || format!("{}px", x() + 100)
                    style:background-color=move || format!("rgb({}, {}, 100)", x(), 100)
                    style:max-width="400px"
                    // Set a CSS variable for stylesheet use
                    style=("--columns", x)
                >
                    "Click to Move"
                </button>
            </li>

        </ol>

        <h3>Dynamic Attributes</h3>
        <ol>
            <li>
                <progress
                    max="50"
                    // signals are functions, so `value=count` and `value=move || count.get()`
                    // are interchangeable.
                    value=count
                ></progress>
            </li>

        </ol>

        <h3>Derived Signals</h3>
        <ol>
            <li>
                <progress
                    max="50"
                    // we use it once here
                    value=double_count
                ></progress>

            </li>
            <li>
                <p>
                    // and again here
                    "Double Count: " {double_count}
                </p>
            </li>
        </ol>
    }
}
