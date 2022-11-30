use yew::{html, Component, Html};

pub struct Home;
use crate::components::article::card::ArticleCard;
use crate::components::main_body::MainBody;

impl Component for Home {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Home
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        let names = vec!["Bob", "Ray", "1", "d"];

        html! {
            <>
                <MainBody>
                    {
                        names.into_iter().map(|_| {
                            html!{<ArticleCard />}
                        }).collect::<Html>()
                    }
                </MainBody>
            </>
        }
    }
}
