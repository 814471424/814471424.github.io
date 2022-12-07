use wasm_bindgen::{prelude::*, JsCast};
use yew::{html, Component};

pub struct BackToTop {
    pub image: String,
    // 图片是否虚化 true:不虚化 false:虚化
    pub opacity: bool,
}

pub enum BackToTopMessage {
    // 图片是否虚化事件
    Opacity(bool),
    // 回到顶部事件
    ToTop,
}

impl Component for BackToTop {
    type Message = BackToTopMessage;

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        // 监听滚动事件
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("no document here");
        let scroll = Closure::<dyn Fn()>::new(move || {
            let to_top_ele = document.get_element_by_id("toTopImage");

            if let Some(ele) = to_top_ele {
                let element = document
                    .document_element()
                    .expect("should have a element on document");
                // log::info!("目前位置: {}", element.scroll_top());
                if element.scroll_top() > 300 {
                    // 显示
                    ele.set_class_name("toTopImage");
                } else {
                    // 隐藏
                    ele.set_class_name("toTopImage inactive");
                }
            }
        });
        window.set_onscroll(scroll.as_ref().dyn_ref());
        scroll.forget();

        let image_vec = vec!["/reimu.png", "/marisa.png", "/flandre.png"];
        let index = (chrono::Local::now().timestamp() % 3) as usize;
        let image = image_vec[index];

        BackToTop {
            image: image.to_string(),
            opacity: false,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <img
                id="toTopImage"
                class="toTopImage inactive"
                style={if self.opacity {"opacity: 1.0;"} else {"opacity: 0.5;"}}
                onmouseover={ctx.link().callback(|_| BackToTopMessage::Opacity(true))}
                onmouseout={ctx.link().callback(|_| BackToTopMessage::Opacity(false))}
                src={self.image.to_string()}
                onclick={ctx.link().callback(|_| BackToTopMessage::ToTop )}
            />
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            BackToTopMessage::Opacity(data) => {
                self.opacity = data;
                true
            }
            BackToTopMessage::ToTop => {
                let window = web_sys::window().expect("no global `window` exists");
                let document = window.document().expect("should have a document on window");
                let element = document
                    .document_element()
                    .expect("should have a element on document");
                element.set_scroll_top(0);
                // document().set_scroll_top(0);

                self.opacity = false;
                true
            }
        }
    }
}
