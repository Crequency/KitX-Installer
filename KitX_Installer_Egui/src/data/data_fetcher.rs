extern crate reqwest;

pub async fn fetch_string_async(url: String) -> String {
    let resp = reqwest::get(url);
    resp.await.unwrap().text().await.unwrap()
}

pub fn fetch_string(url: String) -> String {
    reqwest::blocking::get(url).unwrap().text().unwrap()
}
