use std::{collections::HashMap, hash::Hash, time::Duration};

use ohos_hilog_binding::{hilog_debug, hilog_warn};
use reqwest::{tls::Version, Certificate, Result};
use serde::{Deserialize, Serialize};

pub mod request;
#[derive(Debug, Serialize, Deserialize)]
pub struct ReqwestCert {
    ty: String,
    cert: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ReqwestOptions {
    #[serde(rename = "responseType")]
    pub response_type: Option<String>,
    // ms
    #[serde(rename = "connectTimeout")]
    pub connect_timeout: Option<u64>,
    // ms
    #[serde(rename = "timeout")]
    pub timeout: Option<u64>,
    // ca
    #[serde(rename = "caCert")]
    pub ca_cert: Option<Vec<ReqwestCert>>,
    // headers
    #[serde(rename = "headers")]
    pub headers: Option<HashMap<String, String>>,
    // body和form_body不能两个都传，如果都传，那么按body->form_body->json_body的顺序处理，匹配到一个就停止
    #[serde(rename = "body")]
    pub body: Option<String>,
    #[serde(rename = "formBody")]
    pub form_body: Option<HashMap<String, String>>,
    #[serde(rename = "jsonBody")]
    pub json_body: Option<HashMap<String, String>>,
    // misc
    #[serde(rename = "noProxy")]
    pub no_proxy: Option<bool>,
    #[serde(rename = "ignoreSsl")]
    pub ignore_ssl: Option<bool>,
    #[serde(rename = "forceRustlsSsl")]
    pub force_rustls_ssl: Option<bool>,
}

impl ReqwestOptions {
    pub fn new() -> Self {
        ReqwestOptions {
            response_type: None,
            ignore_ssl: None,
            connect_timeout: None,
            timeout: None,
            ca_cert: None,
            no_proxy: None,
            body: None,
            form_body: None,
            json_body: None,
            headers: None,
            force_rustls_ssl: None,
        }
    }
}

pub fn get_request_client(option: Option<&ReqwestOptions>) -> Result<reqwest::blocking::Client> {
    let default_opt = ReqwestOptions::new();
    let option = option.unwrap_or(&default_opt);
    // ssl check
    let mut builder = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(option.ignore_ssl.unwrap_or(false));
    if let Some(timeout) = option.connect_timeout {
        builder = builder.connect_timeout(Duration::from_millis(timeout));
    }
    // CA
    let default_ca = vec![];
    for cert in option.ca_cert.as_ref().unwrap_or(&default_ca).iter() {
        hilog_debug!(format!("cert: {:?}", cert));
        if cert.ty.to_lowercase() == "pem" {
            if let Ok(cert) = Certificate::from_pem(cert.cert.as_bytes()) {
                hilog_debug!(format!("add pem root cert: {:?}", cert));
                builder = builder.add_root_certificate(cert);
                hilog_debug!(format!("added pem root cert"));
            } else {
                hilog_warn!(format!("Failed to parse pem cert: {}", cert.cert));
            }
        } else if cert.ty.to_lowercase() == "der" {
            if let Ok(cert) = Certificate::from_der(cert.cert.as_bytes()) {
                hilog_debug!(format!("add der root cert: {:?}", cert));
                builder = builder.add_root_certificate(cert);
                hilog_debug!(format!("added der root cert"));
            } else {
                hilog_warn!(format!("Failed to parse der cert: {}", cert.cert));
            }
        }
    }
    // rustls
    if option.force_rustls_ssl.unwrap_or(false) {
        builder = builder.use_rustls_tls();
    }
    // proxy
    if option.no_proxy.unwrap_or(false) {
        builder = builder.no_proxy();
    }
    builder.build()
}
