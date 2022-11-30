use yew::html;
use yew::Component;
use yew::Properties;

pub struct PostList;

#[derive(PartialEq, Properties)]
pub struct ListProp {
    pub id: String,
}

impl Component for PostList {
    type Message = ();

    type Properties = ListProp;

    fn create(ctx: &yew::Context<Self>) -> Self {
        PostList
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <h1 style="color:red;">{&ctx.props().id}</h1>
        }
    }
}
