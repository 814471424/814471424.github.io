use gloo::storage::{LocalStorage, Storage};
use lazy_static::lazy_static;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::RwLock;

use crate::error::AppError;

const API_ROOT: &str = "www.baidu.com";
const TOKEN_KEY: &str = "yew.token";

lazy_static! {
    pub static ref TOKEN: RwLock<Option<String>> = {
        if let Ok(token) = LocalStorage::get(TOKEN_KEY) {
            RwLock::new(Some(token))
        } else {
            RwLock::new(None)
        }
    };
}

/// 将jwt令牌设置为本地存储。
pub fn set_token(token: Option<String>) {
    if let Some(t) = token.clone() {
        LocalStorage::set(TOKEN_KEY, t).expect("failed to set")
    } else {
        LocalStorage::delete(TOKEN_KEY)
    }

    let mut token_lock = TOKEN.write().unwrap();
    *token_lock = token;
}

/// 从惰性静态获取jwt令牌。
pub fn get_token() -> Option<String> {
    let token_lock = TOKEN.read().unwrap();
    token_lock.clone()
}

// 构建各种http请求：post/get/delete等。
pub async fn request<B, T>(method: reqwest::Method, url: String, body: B) -> Result<T, AppError>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;
    let url = format!("{}{}", API_ROOT, url);
    let mut builder = reqwest::Client::new()
        .request(method, url)
        .header("Content-Type", "application/json");

    if let Some(token) = get_token() {
        builder = builder.bearer_auth(token);
    }
    if allow_body {
        builder = builder.json(&body);
    }
    let response = builder.send().await;

    if let Ok(data) = response {
        if data.status().is_success() {
            let data: Result<T, _> = data.json::<T>().await;
            if let Ok(data) = data {
                log::debug!("Response: {:?}", data);
                Ok(data)
            } else {
                Err(AppError::DeserializeError)
            }
        } else {
            match data.status().as_u16() {
                401 => Err(AppError::Unauthorized),
                403 => Err(AppError::Forbidden),
                404 => Err(AppError::NotFound),
                500 => Err(AppError::InternalServerError),
                422 => Err(AppError::DeserializeError),
                _ => Err(AppError::RequestError),
            }
        }
    } else {
        Err(AppError::RequestError)
    }
}
