mod client;

use std::{collections::HashMap, primitive};

use client::{request::OHRequestBuilder, ReqwestOptions};
use napi_derive_ohos::napi;
use napi_ohos::{bindgen_prelude::FromNapiValue, AsyncWorkPromise, Env, JsFunction, JsObject, Result};
use ohos_hilog_binding::{hilog_debug, hilog_warn};


#[napi(ts_return_type = "Promise<string>")]
pub fn request(
    env: Env,
    url: String,
    method: String,
    options: String, // will serialize to ReqwestOptions
) -> Result<JsObject> {
    let options: ReqwestOptions = serde_json::from_str(&options).unwrap_or_else(|error| {
        hilog_warn!(format!("parse options failed: {:?}, your options: {} (length={})", error, options.as_str(), options.len()));
        ReqwestOptions::new()
    });
    hilog_debug!(format!("url: {}, method: {}, options: {:?}", url, method, options));
    let task = OHRequestBuilder::new()
        .url_std(url)
        .method_std(method)
        .options(options);
    let promise = env.spawn(task.build()).unwrap();
    return Ok(promise.promise_object());
}
