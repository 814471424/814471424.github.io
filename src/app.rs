use yew::{html, Component, Context, Html};
use yew_router::BrowserRouter;
use yew_router::Switch;

use crate::components::back_to_top::BackToTop;
// use crate::components::footer::style1::FooterOne;
use crate::components::footer::style2::FooterTwo;
use crate::components::header::Header;
use crate::route::{switch, Route};

pub struct App;
impl Component for App {
    type Message = ();
    type Properties = ();

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <div class="wrapper">
                    // 头部
                    <Header />
                    // 主要内容区域
                    <main>
                        <Switch<Route> render={switch} />
                    </main>
                </div>
                // 底部
                <FooterTwo />
                // 其他组件(放回顶部)
                <BackToTop />
            </BrowserRouter>
        }
    }

    fn create(_ctx: &Context<Self>) -> Self {
        App
    }
}
