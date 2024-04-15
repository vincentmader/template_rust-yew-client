use crate::screens::{
    home::HomeScreen, imprint::ImprintScreen, login::LoginScreen,
    page_not_found::PageNotFoundScreen,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/imprint")]
    Imprint,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    let screen = match routes {
        Route::Home => html! { <HomeScreen /> },
        Route::Imprint => html! { <ImprintScreen /> },
        Route::Login => html! { <LoginScreen /> },
        Route::NotFound => html! { <PageNotFoundScreen /> },
    };

    html!(
        <div>
            { screen }
        </div>
    )
}
