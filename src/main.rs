use yew::prelude::*;
use yew_client::services::routing::{switch, Route};
use yew_router::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
