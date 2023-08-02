pub struct Host {
    pub host_key: String,
    pub host_url: String,
    pub host_latest_url: String,
    pub host_descr: String,
}

impl Clone for Host {
    fn clone(&self) -> Self {
        Host {
            host_key: self.host_key.clone(),
            host_url: self.host_url.clone(),
            host_latest_url: self.host_latest_url.clone(),
            host_descr: self.host_descr.clone(),
        }
    }
}
