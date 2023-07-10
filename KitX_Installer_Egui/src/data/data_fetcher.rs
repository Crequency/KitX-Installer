extern crate reqwest;

pub async fn _fetch_string_async(url: String) -> String {
    let resp = reqwest::get(url);
    resp.await.unwrap().text().await.unwrap()
}

pub fn fetch_string(url: String) -> Option<String> {
    let response = reqwest::blocking::get(url);
    if response.is_err() {
        None
    } else {
        Some(response.unwrap().text().unwrap())
    }
}

pub fn _fetch_binary(_url: String) {
    // TODO: implement this
}
