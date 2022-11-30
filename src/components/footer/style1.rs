use yew::html;
use yew::Component;

pub struct FooterOne;

impl Component for FooterOne {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        FooterOne
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <footer class="footer">
                {"copyright &copy; 2019 xxxx有限公司技术支持"}
            </footer>
        }
    }
}
