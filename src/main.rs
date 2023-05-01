mod app;
pub mod form;
pub mod products;
use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
