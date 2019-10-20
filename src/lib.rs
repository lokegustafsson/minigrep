pub struct Config {
    pub expr: String,
    pub path: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new<T>(mut args: T) -> Result<Config, &'static str> 
        where T: ExactSizeIterator<Item = String>
    {
        if args.len() != 3 {
            return Err("Usage:\n$ minigrep <string> <file>");
        }
        args.next();
        Ok(Config {
            expr: args.next().unwrap(),
            path: args.next().unwrap(),
            case_sensitive: true
        })
    }
}

pub fn search<'a>(text: &'a str, expr: &str) -> Vec<&'a str> {
    text.lines()
        .filter(|line| {line.contains(expr)})
        .collect()
}

#[cfg(tests)]
mod tests {
    #[test]
    fn config_pass() {
        let config = Config::new("minigrep expr filepath".split(' ')).unwrap();
        assert_eq!(config.path, "filepath");
        assert_eq!(config.expr, "expr");
        assert_eq!(config.case_sensitive, true);
    }
    #[test]
    fn config_fail() {
        Config::new("minigrep expr filepath extra".split(' ')).unwrap_err();
        Config::new("too short").unwrap_err();
    }
    #[test]
    fn search1() {
        let text = "First line
second line
third!";
        let expr = "line";
        assert_eq!(search(text, expr), vec!["First line", "second line"]);
    }
}