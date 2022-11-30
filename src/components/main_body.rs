use yew::{html, Children, Component, Html, Properties};

use super::article::card::ArticleCard;
use super::article::sidebar::ArticleSidebar;
use super::header_image::HeaderImage;

pub struct MainBody;

#[derive(Properties, PartialEq)]
pub struct MainProps {
    pub children: Children,
}

impl Component for MainBody {
    type Message = ();

    type Properties = MainProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        MainBody
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        // let names = vec!["Sam", "Bob", "Ray", "1", "2", "3", "4"];
        let names = vec!["Bob", "Ray", "1", "d"];

        html! {
            <>
                <HeaderImage />
                <div id="kratos-blog-post">
                    <div class="container">
                        <div class="row">
                            <div class="col-12 col-sm-12 col-md-12 col-lg-8">
                            // {
                            //     names.into_iter().map(|_| {
                            //         html!{<ArticleCard />}
                            //     }).collect::<Html>()
                            // }
                                { for ctx.props().children.iter() }
                            </div>
                            <div class="d-none d-sm-block d-sm-none d-md-block d-md-none d-lg-block col-lg-4">
                                <ArticleSidebar />
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
