use yew::{html, Component};

pub struct FooterTwo;

impl Component for FooterTwo {
    type Message = ();

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        FooterTwo
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <footer class="footer-two">
                <p>
                    <a href="https://demo.typecho.me/typecho/">{"树莓派"}</a>
                    <br />
                    {"备案号："}<a href="http://js.beian.miit.gov.cn">{"苏ICP备18063590号-1"}</a>
                </p>
            </footer>
        }
    }
}
