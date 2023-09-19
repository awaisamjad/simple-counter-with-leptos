use leptos::logging::log;
use leptos::*;
use wasm_bindgen::JsCast;
//trunk serve --open
fn main() {
    mount_to_body(|| view! { <App /> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (number, set_number) = create_signal(0);

    let double_count = move || count.get() * 2;
    let values = vec![0, 1, 2];
    log!("{:?}", values);
    view! {
        <div class = "background">
        <div class = "content-div">
            <h1 class = "title">"Simple Counter"</h1>

        <button id = "plus_one_button" class = "button"
            on:click=move |_| {
                set_number.update(|n| *n += 1);
                log!("Number: {}", number.get());
            }
        >
            "+1"
        </button>
        <p id = "output"> {move || number.get()}</p>
        <button id = "minus_one_button" class = "button"
            on:click=move |_| {
                set_number.update(|n| *n -= 1);
                log!("Number: {}", number.get());
            }
        >
            "-1"
        </button>
        <br/>
        <button id = "reset" class = "button"
            on:click=move |_| {
                set_number.update(|n| *n = 0);
                log!("Number: {}", number.get());
            }
        >
            "Reset"
        </button>


        <br/>

        <ProgressBar progress=number/>
    </div>
    </div>

    }
}

/// Shows progress toward a goal.
#[component]
fn ProgressBar(
    // Marks this as an optional prop. It will default to the default
    // value of its type, i.e., 0.
    #[prop(default = 100)]
    /// The maximum value of the progress bar.
    max: u16,
    // Will run `.into()` on the value passed into the prop.
    #[prop(into)]
    // `Signal<T>` is a wrapper for several reactive types.
    // It can be helpful in component APIs like this, where we
    // might want to take any kind of reactive value
    /// How much progress should be displayed.
    progress: MaybeSignal<i32>,
) -> impl IntoView {
    view! {
        <progress
            max={max}
            value=progress
        />
        <br/>
    }
}

#[component]
fn DisplayName(name: String) -> impl IntoView {
    view! {
        <h1>
            "Hello, "
            {name}
        </h1>
    }
}
