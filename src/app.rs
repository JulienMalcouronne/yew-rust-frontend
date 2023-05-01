use yew::prelude::*;

use crate::{form, products};
use form::Form;
use products::Products;

#[function_component(App)]
pub fn app() -> Html {
    let name = "Julien";
    html! {

      <div class="container">
        <h1 class="title">{"Products App"}</h1>
        <Form />
        <Products/>
      </div>

    }
}
