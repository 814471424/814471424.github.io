mod app;
mod components; // 基础组件
mod error;
mod route; // 路由
mod services;
mod types;
mod view; // 页面

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().render();
}
