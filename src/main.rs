use leptos::logging::log;
use leptos::*;
//trunk serve --open
fn main() {
    mount_to_body(|| view! { <App /> })
}

#[component]
fn App() -> impl IntoView {
    let (number, set_number) = create_signal(0);
    let (step, set_step) = create_signal(1);

    let values = vec![0, 1, 2];
    log!("{:?}", values);
    view! {
        <div class = "background">
        <div class = "content-div">
            <h1 class = "title">"Simple Counter"</h1>
            <input type = "number" placeholder = "Step"></input>
            <div class = "row">

        <button id = "minus_one_button" class = "button"
            on:click=move |_| {
                set_number.update(|n| *n -= 1);
                log!("Number: {}", step.get())
            }
        >
            "-"
        </button>

        <p class = "output"> {move || number.get()}</p>

        <button id = "plus_one_button" class = "button"
            on:click=move |_| {
                set_number.update(|n| *n += 1);
            }
        >
            "+"
        </button>

    </div>
    <div class = "reset-div">
        <br/>
        <button id = "reset" class = "button"
            on:click=move |_| {
                set_number.update(|n| *n = 0);
                log!("Number: {}", number.get());
            }
        >
            "Reset"
        </button>
    </div>

        <br/>
    </div>
    </div>

    }
}
