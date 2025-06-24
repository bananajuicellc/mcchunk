use yew::prelude::*;
use wasm_bindgen::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="container">
            <form>
                <div class="form-group">
                    <label for="input1">{"Input 1"}</label>
                    <input type="number" id="input1" />
                </div>
                <div class="form-group">
                    <label for="input2">{"Input 2"}</label>
                    <input type="number" id="input2" />
                </div>
                <div class="form-group">
                    <label for="input3">{"Input 3"}</label>
                    <input type="number" id="input3" />
                </div>
            </form>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    yew::Renderer::<App>::new().render();
}
