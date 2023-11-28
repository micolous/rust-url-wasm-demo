use url::Url;
use web_sys::{wasm_bindgen::JsCast, window, HtmlInputElement};
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let feedback = use_state(String::new);
    let onsubmit = {
        let feedback = feedback.clone();
        move |event: SubmitEvent| {
            let input = window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("url")
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value();

            match Url::parse(&input) {
                Ok(url) => feedback.set(format!(
                    "Parsed OK! url.to_string() = {:?}\n{:#?}",
                    url.to_string(),
                    url
                )),
                Err(err) => feedback.set(format!("Error: {err:?}")),
            }
            event.prevent_default();
        }
    };

    html! {
        <>
            <h1>{"Rust URL WASM demo"}</h1>
            <form {onsubmit}>
                <label for="url">{"URL:"}</label>
                <input type="text" id="url" autocomplete="off" />
                <input type="submit" value="Parse URL" />
            </form>
            if !feedback.is_empty() {

                <h2>{"Parse result"}</h2>
                <div>
                  <pre>{feedback.as_str()}</pre>
                </div>
            }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
