use yew::{html, Component, Html};

pub struct ArticleDetail;

impl Component for ArticleDetail {
    type Message = ();

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        ArticleDetail
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        html! {
            <article>
                <div class="kratos-post-inner">
                    <header class="kratos-entry-header">
                        <h1 class="kratos-entry-title text-center">{"Typecho 演示说明"}</h1>
                        <div class="kratos-post-meta text-center">
                            <span>
                                <i class="bi bi-calendar3"></i>{" 2018-04-04  "}
                                <i class="bi bi-chat-dots"></i>{" 3 条评论  "}
                                <i class="bi bi-eye"></i>{" 2555 次阅读  "}
                            </span>
                        </div>
                    </header>
                    <div class="kratos-post-content">
                        <p>{"注：本站为typecho演示说明，仅供参考使用。"}</p>
                        <p>{"相关模板下载请移戚：https://typecho.me 或 https://pro.typecho.me/"}</p>
                        <p>{"谢谢。"}</p>
                    </div>
                    <footer class="kratos-entry-footer">
                        <div class="footer-tag text-center">
                            <div class="pull-left">
                                <a href="#">{"typecho"}</a>
                            </div>
                        </div>
                    </footer>
                </div>
                <nav class="post-navigation">
                    <div class="nav-previous nav-data"><a href="#">{"无上一篇文章"}</a></div>
                    <div class="nav-next nav-data"><a href="#">{"下一篇"}</a></div>
                </nav>
                <div class="comments-area">
                    <ol class="comment-list"></ol>
                    <div class="comments-nav"></div>
                    <div class="comment-respond"></div>
                </div>
            </article>
        }
    }
}
