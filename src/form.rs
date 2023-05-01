use yew::prelude::*;

use web_sys::HtmlInputElement;

#[function_component]
pub fn Form() -> Html {
    let name_ref = NodeRef::default();
    let name_ref_outer = name_ref.clone();

    let price_ref = NodeRef::default();
    let price_ref_outer = price_ref.clone();

    let onclick = Callback::from(move |_| gloo_console::log!("Button clicked"));

    html! {
      <div class="container">
        <h2>{"Add a product"}</h2>
        <div>
          <label for="name" class="">
            {"Name"}
            <input ref={name_ref_outer.clone()}
              id="name"
              class=""
              type="text"
            />
          </label> <br /> <br />
          <label for="price" class="">
            {"Price"}
            <input ref={price_ref_outer.clone()}
              id="price"
              class=""
              type="number"
            />
          </label> <br /> <br />
          <button {onclick} class="">{"Add product"}</button>
        </div>
      </div>

    }
}
