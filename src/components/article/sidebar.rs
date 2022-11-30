use yew::{html, Component};

pub struct ArticleSidebar;

impl Component for ArticleSidebar {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        ArticleSidebar
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <div class="kratos-widget-area">
                <aside id="kratos_about-5" class="widget amadeus_about">
                    <div class="photo-background">
                        <div class="photo-background" style="background:url(https://demo.typecho.me/typecho/usr/themes/Kratos/images/about.jpg) no-repeat center center; -webkit-background-size: cover; -moz-background-size: cover; -o-background-size: cover; background-size: cover;"></div>
                    </div>
                    <div class="photo-wrapper">
                        <div class="photo-wrapper-tip">
                            <img src="https://demo.typecho.me/typecho/usr/themes/Kratos/images/author.jpg" />
                        </div>
                    </div>
                    <div class="textwidget">
                        <p align="center"></p>
                    </div>
                </aside>
                <aside class="widget">
                    <h4 class="widget-title">{"分类目录"}</h4>
                    <ul>
                        <li class="bi bi-folder2">{" 1"}</li>
                        <li class="bi bi-folder2">{" 2"}</li>
                        <li class="bi bi-folder2">{" 3"}</li>
                    </ul>
                </aside>
            </div>
        }
    }
}
