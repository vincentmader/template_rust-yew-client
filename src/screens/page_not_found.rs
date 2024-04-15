use stylist::css;
use yew::prelude::*;

#[function_component]
pub fn PageNotFoundScreen() -> Html {
    let class = css!(
        "
            h1 {
                width: 100%;
                text-align: center;
                color: red;
            }
        "
    );

    html! {
        <div {class}>
            <h1>{ "404" }</h1>
        </div>
    }
}
