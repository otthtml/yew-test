mod home;
mod login;
mod page_not_found;

use yew::html::Scope;
use yew::prelude::*;
use yew_router::prelude::*;

use home::home::HomeComponent;
use login::login::LoginComponent;
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



pub enum Msg {
    ToggleNavbar,
}
pub struct App {
    navbar_active: bool,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(ctx.link()) }

                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
            </BrowserRouter>
        }
    }
}

impl App {
    fn view_nav(&self, _link: &Scope<Self>) -> Html {
        let Self { navbar_active, .. } = *self;

        let active_class = if !navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-start">
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                            { "Home" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Login}>
                            { "Login" }
                        </Link<Route>>
                    </div>
                </div>
            </nav>
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