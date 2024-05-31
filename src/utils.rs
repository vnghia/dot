pub fn parse_addition(s: &str) -> Result<(String, String), &'static str> {
    s.split_once('=')
        .map(|(k, v)| (k.to_owned(), v.to_owned()))
        .ok_or("--addition must have format k=v")
}
