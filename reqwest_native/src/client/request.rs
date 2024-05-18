use std::{collections::HashMap, fmt::format, hash::Hash};

use napi_ohos::{JsString, Task};
use ohos_hilog_binding::hilog_debug;
use reqwest::{blocking::Body, Method};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::client;

use super::{get_request_client, ReqwestOptions};

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct OHResponse {
    status_code: u16,
    response_body: String,
}

pub struct OHRequest {
    method: String,
    url: String,
    // ssl
    options: Option<ReqwestOptions>,
}

pub struct OHRequestBuilder {
    inner: OHRequest,
}

impl OHRequestBuilder {
    pub fn new() -> Self {
        OHRequestBuilder {
            inner: OHRequest {
                method: "GET".to_string(),
                url: "".to_string(),
                options: None,
            },
        }
    }

    pub fn method(mut self, method: &str) -> Self {
        self.inner.method = method.to_string();
        self
    }

    pub fn method_std(mut self, method: String) -> Self {
        self.inner.method = method;
        self
    }

    pub fn url(mut self, url: &str) -> Self {
        self.inner.url = url.to_string();
        self
    }

    pub fn url_std(mut self, url: String) -> Self {
        self.inner.url = url;
        self
    }

    pub fn build(self) -> OHRequest {
        self.inner
    }

    pub fn options(mut self, options: ReqwestOptions) -> Self {
        self.inner.options = Some(options);
        self
    }
}

impl Task for OHRequest {
    type Output = String;

    type JsValue = JsString;

    fn compute(&mut self) -> napi_ohos::Result<Self::Output> {
        let client =
            get_request_client(self.options.as_ref());
        if let Err(err) = client {
            return Err(napi_ohos::Error::from_reason(format!("{:?}", err)));
        }
        let client = client.unwrap();
        // method, url, body
        hilog_debug!(format!("method: {}, url: {}", self.method, self.url));
        let mut builder = client
            .request(
                Method::from_bytes(self.method.as_bytes()).unwrap_or(Method::GET),
                self.url.clone(),
            )
            .header("x-remote-unlock-client", "openharmony");
        hilog_debug!(format!("applying options: {:?}", self.options));
        // options
        if let Some(option) = self.options.as_ref() {
            // timeout
            if let Some(timeout) = option.timeout {
                builder = builder.timeout(std::time::Duration::from_millis(timeout));
            }
            // headers
            let hders = HashMap::new();
            for (key, value) in option.headers.as_ref().unwrap_or(&hders).iter() {
                builder = builder.header(key, value);
            };
            // body
            if let Some(body) = option.body.as_ref() {
                builder = builder.body(body.clone());
            } else if let Some(data) = option.json_body.as_ref() {
                builder = builder.json(data);
            } else if let Some(data) = option.form_body.as_ref() {
                builder = builder.form(data);
            }
        }
        hilog_debug!(format!("sending request: {:?}", builder));
        // send request
        match builder.build() {
            Ok(request) => match client.execute(request) {
                Ok(resp) => {
                    let status_code = resp.status().as_u16();
                    hilog_debug!(format!("status_code: {}", status_code));
                    match resp.text() {
                        Ok(text) => {
                            return Ok(json!(OHResponse {
                                status_code: status_code,
                                response_body: text
                            })
                            .to_string());
                        }
                        Err(err) => {
                            return Ok(json!(OHResponse {
                                status_code: status_code,
                                response_body: err.to_string()
                            })
                            .to_string());
                        }
                    }
                }
                Err(err) => {
                    hilog_debug!(format!("request failed: {:?}", err));
                    return Err(napi_ohos::Error::from_reason(err.to_string()));
                }
            },
            Err(err) => {
                hilog_debug!(format!("request builder failed: {:?}", err));
                return Err(napi_ohos::Error::from_reason(err.to_string()));
            }
        }
    }

    fn resolve(
        &mut self,
        env: napi_ohos::Env,
        output: Self::Output,
    ) -> napi_ohos::Result<Self::JsValue> {
        env.create_string_from_std(output)
    }

    fn reject(
        &mut self,
        _env: napi_ohos::Env,
        err: napi_ohos::Error,
    ) -> napi_ohos::Result<Self::JsValue> {
        hilog_debug!(format!("request reject: {}", err.to_string()));
        Err(err)
    }
}
