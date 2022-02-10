use yew::prelude::{Component, Context, Html, html};
use yew_router::prelude::*;

pub(crate) struct NavbarComponent;
impl Component for NavbarComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        return Self;
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-menu is-active">
                    <div class="navbar-start">
                        <Link<crate::Route> classes={crate::classes!("navbar-item")} to={crate::Route::Home}>
                            { "Home" }
                        </Link<crate::Route>>
                        <Link<crate::Route> classes={crate::classes!("navbar-item")} to={crate::Route::Login}>
                            { "Login" }
                        </Link<crate::Route>>
                    </div>
                </div>
            </nav>
        }
    }
}