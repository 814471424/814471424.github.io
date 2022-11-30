use gloo::history::History;
use wasm_bindgen::UnwrapThrowExt;
use yew::{html, Callback, Component, Html};
use yew_router::{prelude::*, AnyRoute};

use crate::route::Route;

pub struct Header {
    pub show: bool,
    pub menus: Vec<HeaderMenu>,
}

#[derive(Debug, Clone)]
pub struct HeaderMenu {
    pub title: String,
    pub path: String,
    pub hide: Option<bool>,
    pub children: Vec<HeaderMenu>,
}

pub enum HeaderMsg {
    Show,
    Jump(String),
}

impl Component for Header {
    type Message = HeaderMsg;

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        let menus = vec![
            HeaderMenu {
                title: String::from("首页"),
                path: String::from("/"),
                hide: None,
                children: vec![],
            },
            HeaderMenu {
                title: String::from("漫画"),
                path: String::from("/comic"),
                hide: None,
                children: vec![],
            },
            HeaderMenu {
                title: String::from("分类"),
                path: String::from(""),
                hide: None,
                children: vec![
                    HeaderMenu {
                        title: String::from("个人项目"),
                        path: String::from("/posts/1"),
                        hide: None,
                        children: vec![],
                    },
                    HeaderMenu {
                        title: String::from("编程笔记"),
                        path: String::from("/posts/2"),
                        hide: None,
                        children: vec![],
                    },
                    HeaderMenu {
                        title: String::from("轻松日常"),
                        path: String::from("/posts/3"),
                        hide: None,
                        children: vec![],
                    },
                    HeaderMenu {
                        title: String::from("包罗万象"),
                        path: String::from("/posts/4"),
                        hide: None,
                        children: vec![],
                    },
                    HeaderMenu {
                        title: String::from("玩物有志"),
                        path: String::from("/posts/5"),
                        hide: None,
                        children: vec![],
                    },
                ],
            },
            HeaderMenu {
                title: String::from("服务"),
                path: String::from("/posts/6"),
                hide: None,
                children: vec![
                    HeaderMenu {
                        title: String::from("RSSHub"),
                        path: String::from("/posts/7"),
                        hide: None,
                        children: vec![],
                    },
                    HeaderMenu {
                        title: String::from("Proxy CORS"),
                        path: String::from("/posts/8"),
                        hide: None,
                        children: vec![],
                    },
                    HeaderMenu {
                        title: String::from("ICE IDE"),
                        path: String::from("/posts/9"),
                        hide: None,
                        children: vec![],
                    },
                    HeaderMenu {
                        title: String::from("MP MD"),
                        path: String::from("/posts/10"),
                        hide: None,
                        children: vec![],
                    },
                    HeaderMenu {
                        title: String::from("FreshRSS"),
                        path: String::from("/posts/11"),
                        hide: None,
                        children: vec![],
                    },
                ],
            },
            HeaderMenu {
                title: String::from("项目"),
                path: String::from("/posts/12"),
                hide: None,
                children: vec![
                    HeaderMenu {
                        title: String::from("随机座位生成器"),
                        path: String::from("/posts/13"),
                        hide: None,
                        children: vec![],
                    },
                    HeaderMenu {
                        title: String::from("随机密码生成器"),
                        path: String::from("/posts/14"),
                        hide: None,
                        children: vec![],
                    },
                    HeaderMenu {
                        title: String::from("WINDS主题"),
                        path: String::from("/posts/15"),
                        hide: None,
                        children: vec![],
                    },
                    HeaderMenu {
                        title: String::from("EASE主题"),
                        path: String::from("/posts/16"),
                        hide: None,
                        children: vec![],
                    },
                    HeaderMenu {
                        title: String::from("HGNUOJ"),
                        path: String::from("/posts/17"),
                        hide: None,
                        children: vec![],
                    },
                    HeaderMenu {
                        title: String::from("HGNUMAN"),
                        path: String::from("/posts/18"),
                        hide: None,
                        children: vec![],
                    },
                    HeaderMenu {
                        title: String::from("DEMOS"),
                        path: String::from("/posts/19"),
                        hide: None,
                        children: vec![],
                    },
                ],
            },
        ];

        Header { show: false, menus }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        html! {
            <header class="header-ex">
                <nav class="navbar-ex">
                    // <a href="/" class="nav-logo">{"WebDev."}</a>
                    <Link<Route> to={ Route::Home } classes="nav-logo">{ "主页" }</Link<Route>>
                    <ul class={if self.show {"nav-menu-ex active closeFloat"} else {"nav-menu-ex closeFloat"} }>
                        { for self.menus.iter().map(|i|
                            {
                                let path = i.path.clone();
                                if i.children.len() > 0 {
                                    html!{
                                        <li class="nav-item-ex">
                                        <a>{&i.title}</a>
                                        // <a href="#" class="nav-link" onclick={ctx.link().callback(move|_| {HeaderMsg::Jump(String::from(&path))})}>{&i.title}</a>
                                            <ul>
                                            {for i.children.iter().map(|i1| {
                                                let path1 = i1.path.clone();
                                                html!{
                                                    <li>
                                                        <a href="#" class="nav-link" onclick={ctx.link().callback(move|_| {HeaderMsg::Jump(String::from(&path1))})}>{&i1.title}
                                                        </a>
                                                    </li>
                                                }
                                            } )}
                                            </ul>
                                        </li>
                                    }
                                } else {
                                    html!{
                                        <li class="nav-item-ex">
                                            <a href="#" class="nav-link" onclick={ctx.link().callback(move|_| {HeaderMsg::Jump(String::from(&path))})}>{&i.title}</a>
                                        </li>
                                    }
                                }
                            }
                        )}
                    </ul>
                    <div></div>
                    <a
                        class={if self.show {"hamburger active"} else {"hamburger"} }
                        onclick={ctx.link().callback(|_| HeaderMsg::Show)}
                    >
                        <span class="bar"></span>
                        <span class="bar"></span>
                        <span class="bar"></span>
                    </a>
                </nav>
            </header>
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HeaderMsg::Show => {
                self.show = !self.show;
                true
            }
            HeaderMsg::Jump(url) => {
                self.show = false;

                gloo::history::BrowserHistory::new().push(url);
                // let history = ctx.link().history().expect_throw("failed to read history");
                // history.push(AnyRoute::new(url));

                true
            }
        }
    }
}
