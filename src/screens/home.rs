use stylist::css;
use yew::prelude::*;

#[function_component]
pub fn HomeScreen() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let class = css!(
        "
            p {
                color: green;
            }
        "
    );

    html! {
        <div {class}>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}
