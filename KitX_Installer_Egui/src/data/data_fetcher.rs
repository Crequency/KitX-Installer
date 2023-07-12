extern crate reqwest;

pub async fn _fetch_string_async(url: String) -> String {
    let resp = reqwest::get(url);
    resp.await.unwrap().text().await.unwrap()
}

pub fn fetch_string(url: String, time_out_milliseconds: i32) -> Option<String> {
    let response = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_millis(
            time_out_milliseconds as u64,
        ))
        .build()
        .unwrap()
        .get(url)
        .send();

    if response.is_err() {
        None
    } else {
        Some(response.unwrap().text().unwrap())
    }
}

pub fn _fetch_binary(_url: String) {
    // TODO: implement this
}
