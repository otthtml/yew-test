mod login;

use yew;
use login::login::LoginModel;

fn main() {
    yew::start_app::<LoginModel>();
}