use stylist::css;
use yew::prelude::*;
use yew_client::services::routing::{switch, Route};
use yew_router::prelude::*;

const DEFAULT_FONT_FAMILY: &str = "Arial, sans-serif";

#[function_component]
fn App() -> Html {
    let class = css!(
        "
            --DEFAULT_FONT_FAMILY: ${DEFAULT_FONT_FAMILY};

            font-family: var(--DEFAULT_FONT_FAMILY);
        ",
        DEFAULT_FONT_FAMILY = DEFAULT_FONT_FAMILY
    );

    html! {
        <div {class}>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
