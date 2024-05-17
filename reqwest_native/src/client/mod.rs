pub mod request;

pub fn get_default_secure_client(ignore_ssl: bool) -> reqwest::blocking::Client {
    reqwest::blocking::Client::builder().danger_accept_invalid_certs(ignore_ssl).build().unwrap()
}
