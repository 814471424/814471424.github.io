use yew::{html, Component, Context, Html};

pub struct HeaderImage;

impl Component for HeaderImage {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        HeaderImage
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="kratos-start">
                <div class="kratos-overlay" />
                <div class="kratos-cover" style="background-image: url(https://demo.typecho.me/typecho/usr/themes/Kratos/images/head.png);"> </div>
            </div>
        }
    }
}
