use yew::html;
use yew::Component;
use yew::Properties;

use crate::components::article::detail::ArticleDetail;
use crate::components::main_body::MainBody;

pub struct Post;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub id: String,
}

impl Component for Post {
    type Message = ();

    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let id = &ctx.props().id;
        log::info!("{:?}", id);

        Post
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <MainBody>
                <ArticleDetail />
            </MainBody>
            // <h1>{"111111111111111111111"}</h1>
        }
    }
}
