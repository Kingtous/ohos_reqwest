use std::{collections::HashMap, fmt::format};

use napi_ohos::{JsString, Task};
use ohos_hilog_binding::hilog_debug;
use reqwest::{blocking::Body, Method};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::get_default_secure_client;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OHResponse {
    status_code: u16,
    response_body: String,
}

pub struct OHRequest<T: Into<Body>> {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: T,
    // ssl
    ignore_ssl: bool,
}

pub struct OHRequestBuilder {
    inner: OHRequest<String>,
}
impl OHRequestBuilder {
    pub fn new() -> Self {
        OHRequestBuilder {
            inner: OHRequest {
                method: "GET".to_string(),
                url: "".to_string(),
                headers: HashMap::new(),
                body: "".to_string(),
                ignore_ssl: false,
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

    pub fn body(mut self, body: String) -> Self {
        self.inner.body = body;
        self
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.inner
            .headers
            .insert(key.to_string(), value.to_string());
        self
    }

    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.inner.headers = headers;
        self
    }

    pub fn build(self) -> OHRequest<String> {
        self.inner
    }

    pub fn ignore_ssl(mut self, ignore_ssl: bool) -> Self {
        self.inner.ignore_ssl = ignore_ssl;
        self
    }
}

impl<T: Into<Body> + Send + Clone> Task for OHRequest<T> {
    type Output = String;

    type JsValue = JsString;

    fn compute(&mut self) -> napi_ohos::Result<Self::Output> {
        let client = get_default_secure_client(self.ignore_ssl);
        let mut builder = client
            .request(
                Method::from_bytes(self.method.as_bytes()).unwrap_or(Method::GET),
                self.url.clone(),
            )
            .body(self.body.clone())
            .header("x-remote-unlock-client", "openharmony");
        for (key, value) in self.headers.iter() {
            builder = builder.header(key, value);
        }
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
                    hilog_debug!(format!("request failed: {}", err));
                    return Err(napi_ohos::Error::from_reason(err.to_string()));
                }
            },
            Err(err) => {
                hilog_debug!(format!("request builder failed: {}", err));
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
