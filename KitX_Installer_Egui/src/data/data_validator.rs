use regex::Regex;

pub fn is_path_legal_in_windows(path: &str ) -> bool {
    let regex = Regex::new("[a-zA-z]:(\\[\\u4E00-\\u9FA5A-Za-z0-9_\\s]*)*").unwrap();
    regex.is_match(path)
}
