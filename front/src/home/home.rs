use gloo_utils::{document};
use yew::prelude::{Component, Context, Html};

const HTML_PATH: &str = include_str!("home.html");

pub struct HomeComponent;
impl Component for HomeComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        return Self;
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let div = document().create_element("div").unwrap();
        div.set_inner_html(HTML_PATH);

        return Html::VRef(div.into());
    }
}