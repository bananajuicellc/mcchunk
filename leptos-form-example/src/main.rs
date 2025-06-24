use leptos::*;

#[component]
fn App() -> impl IntoView {
    // Create signals for each input field
    // Leptos uses signals for reactive state management.
    // The initial value is an empty string, but they will store numbers.
    // For simplicity in this example, we'll parse them when needed.
    let (input1, set_input1) = create_signal(String::new());
    let (input2, set_input2) = create_signal(String::new());
    let (input3, set_input3) = create_signal(String::new());

    // Basic form styling will be in style.css
    // For this example, we're not handling form submission, just display.
    view! {
        <div class="container">
            <form on:submit=|ev| ev.prevent_default()> // Prevent default form submission
                <div class="form-group">
                    <label for="input1">"Input 1"</label>
                    <input
                        type="number"
                        id="input1"
                        name="input1"
                        on:input=move |ev| set_input1(event_target_value(&ev))
                        prop:value=input1
                    />
                </div>
                <div class="form-group">
                    <label for="input2">"Input 2"</label>
                    <input
                        type="number"
                        id="input2"
                        name="input2"
                        on:input=move |ev| set_input2(event_target_value(&ev))
                        prop:value=input2
                    />
                </div>
                <div class="form-group">
                    <label for="input3">"Input 3"</label>
                    <input
                        type="number"
                        id="input3"
                        name="input3"
                        on:input=move |ev| set_input3(event_target_value(&ev))
                        prop:value=input3
                    />
                </div>
            </form>
        </div>
    }
}

fn main() {
    // Initialize panic hook for better error messages in the console
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    // Mount the app to the body
    mount_to_body(|| view! { <App /> })
}
