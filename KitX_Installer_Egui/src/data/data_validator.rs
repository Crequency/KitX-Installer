use regex::Regex;

pub fn is_path_legal_in_windows(path: &str) -> bool {
    let regex = Regex::new("^[a-zA-Z]:\\\\([^\\\\/:*?\"<>|]|\\\\)+\\\\?$").unwrap();
    regex.is_match(path)
}
