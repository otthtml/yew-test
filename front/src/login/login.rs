use gloo_utils::{document};
use yew::prelude::{Component, Context, Html};

const LOGIN_HTML: &str = include_str!("login.html");

pub(crate) enum Msg {
    AddOne,
}

pub(crate) struct LoginModel {
    username: i32,
    password: i32
}

pub fn check_credentials(username: &str, password: &str) -> bool {
    return true
}

impl Component for LoginModel {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            username: 0,
            password: 0
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                return true;
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let div = document().create_element("div").unwrap();
        div.set_inner_html(LOGIN_HTML);

        Html::VRef(div.into())
    }
}
