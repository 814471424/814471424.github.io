// use gloo_net::http::Request;
use yew::prelude::*;
use yew_hooks::prelude::*;

pub mod request; // http请求库

pub async fn fetch(url: String) -> Result<String, String> {
    // log::info!("dddddddddddd");
    // let resp = Request::get("www.baidu.com").send().await.unwrap();
    // let text = resp.text().await.unwrap();

    let resp = reqwest::get("http://127.0.0.1:8080/")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    log::info!("{:#?}", resp);

    Ok(resp)
    // Ok(String::from("Jet Li"))
}

#[function_component(Demo)]
pub fn demo() -> Html {
    let state = use_async_with_options(
        async move { fetch("/api/user/123".to_string()).await },
        UseAsyncOptions::enable_auto(),
    );
    // log::info!("{:?}", state.data);

    html! {
        <h1 style="width:100%; word-break: break-all;">
            {
                if let Some(data) = &state.data {
                    html! { data }
                } else {
                    html! {}
                }
            }
            {
                if let Some(error) = &state.error {
                    html! { error }
                } else {
                    html! {}
                }
            }
            {"1"}
        </h1>
    }
}
