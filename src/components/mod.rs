use leptos::*;

#[component]
pub fn Counter() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button on:click=move |_| {
            set_count(3);
        }>

            // on stable, this is move || count.get();
            "Click me: " {move || count()}
        </button>
    }
}
