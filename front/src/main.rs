mod home;
mod login;
mod navbar;
mod page_not_found;

use yew::prelude::*;
use yew_router::prelude::*;

use home::home::HomeComponent;
use login::login::LoginComponent;
use navbar::navbar::NavbarComponent;
use page_not_found::page_not_found::PageNotFoundComponent;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/login")]
    Login,

    #[at("/")]
    Home,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub struct App {}
impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        return Self{};
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <NavbarComponent />
                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
            </BrowserRouter>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Login => {
            html! { <LoginComponent /> }
        }
        Route::Home => {
            html! { <HomeComponent /> }
        }
        Route::NotFound => {
            html! { <PageNotFoundComponent /> }
        }
    }
}

fn main() {
    yew::start_app::<App>();
}