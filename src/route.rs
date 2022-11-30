use yew::{html, Html};
use yew_router::Routable;

use crate::components::not_found::NotFound;
use crate::view::home::Home;
use crate::view::posts::detail::Post;
use crate::view::posts::list::PostList;

#[derive(Clone, Debug, PartialEq, Eq, Routable)]
pub enum Route {
    #[at("/posts/:id")]
    Post { id: String },
    #[at("/posts")]
    Posts,
    #[at("/authors/:id")]
    Author { id: u64 },
    #[at("/authors")]
    Authors,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("no document here");

    match routes.clone() {
        Route::Post { id } => {
            html! { <Post id={id} /> }
        }
        Route::Posts => {
            // document.set_title("文章列表");
            html! { <PostList id={String::from("111")} /> }
        }
        Route::Author { id } => {
            // html! { <Author seed={id} /> }
            html! { <h1>{ "Author" }</h1> }
        }
        Route::Authors => {
            // html! { <AuthorList /> }
            html! { <h1>{ "Authors" }</h1> }
        }
        Route::Home => {
            html! { <Home /> }
        }
        Route::NotFound => {
            html! {
                <NotFound />
            }
        }
    }
}
