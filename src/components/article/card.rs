use yew::{html, Component, Html};
use yew_router::prelude::Link;

use crate::route::Route;

pub struct ArticleCard;

impl Component for ArticleCard {
    type Message = ();

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        ArticleCard
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        html! {
            <article class="kratos-hentry">
                <div class="kratos-post-inner-new">
                    <header class="kratos-entry-header-new">
                        <a class="label">
                            {"模板"}
                            <i class="label-arrow"></i>
                        </a>
                        <h2 class="kratos-entry-title-new">
                            // <a href="https://demo.typecho.me/typecho/Kratos/blog/10.html">
                            //     {"Typecho 演示说明"}
                            // </a>
                            <Link<Route> to={Route::Post{id: String::from("1")}}>{ "Typecho 演示说明" }</Link<Route>>
                        </h2>
                    </header>
                    <div class="kratos-entry-content-new">
                        <p>
                            {"注：本站为typecho演示说明，仅供参考使用。相关模板下载请移戚：https://typecho.me 或 https://pro.typecho.me/谢谢。"}
                        </p>
                    </div>
                </div>
                <div class="kratos-post-meta-new">
                    <span class="pull-left d-none d-sm-block d-sm-none d-md-block d-md-none d-lg-block">
                        <a href="#">
                            <i class="bi bi-calendar3"></i>
                            {" 2028/04/04"}
                        </a>
                        <a href="#">
                            <i class="bi bi-chat-dots"></i>
                            {" 3 Comments"}
                        </a>
                    </span>
                    <span class="pull-left">
                        <a href="#">
                            <i class="bi bi-eye"></i>
                            {" 2549 Views"}
                        </a>
                    </span>
                    <span class="pull-right">
                        <a href="#">
                            {"阅读全文 "}
                            <i class="bi bi-arrow-right-circle-fill"></i>
                        </a>
                    </span>
                </div>
            </article>
        }
    }
}
