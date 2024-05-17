mod client;

use std::collections::HashMap;

use client::request::OHRequestBuilder;
use napi_derive_ohos::napi;
use napi_ohos::{Env, JsObject, Result};
use ohos_hilog_binding::hilog_debug;


#[napi(ts_return_type = "Promise<string>")]
pub fn safe_request(
    env: Env,
    url: String,
    headers: HashMap<String, String>,
    method: String,
    body: String,
    ignore_ssl: bool,
) -> Result<JsObject> {
    hilog_debug!(format!("url: {}, headers: {:?}, method: {}, body: {}, ignore_ssl: {}", url, headers, method, body, ignore_ssl));
    let task = OHRequestBuilder::new()
        .url_std(url)
        .method_std(method)
        .headers(headers)
        .body(body)
        .ignore_ssl(ignore_ssl)
        .build();
    let promise = env.spawn(task).unwrap();
    return Ok(promise.promise_object());
}
