use yew::Component;
use yew::{html, Html};

pub struct NotFound;

impl Component for NotFound {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        NotFound
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        html! {
            <div class="not_found">
            <img src="/miku404.png" alt="" />
                <h3>{"页面找不到, 请选择"}</h3>
                <ul>
                    <li><a src="/">{"« 首页"}</a></li>
                    <li><a src="/">{"其他 »"}</a></li>
                </ul>
            </div>
        }
    }
}
