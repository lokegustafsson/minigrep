pub struct Config {
    pub expr: String,
    pub path: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new<T>(mut args: T) -> Result<Config, &'static str> 
        where T: Iterator<Item = String>
    {
        let command = args.next();
        let expr = args.next();
        let path = args.next();
        if command == None || expr == None || path == None || args.next() != None {
            return Err("Usage:\n$ minigrep <string> <file>");
        }
        Ok(Config {
            expr: expr.unwrap(),
            path: path.unwrap(),
            case_sensitive: true
        })
    }
}

pub fn search<'a>(text: &'a str, expr: &str) -> Vec<&'a str> {
    text.lines()
        .filter(|line| {line.contains(expr)})
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec::IntoIter;

    fn iterify(inp: &str) -> IntoIter<String> {
        let tmp: Vec<String> = inp.split(' ').map(|s: &str| s.to_string()).collect();
        tmp.into_iter()
    }
    #[test]
    fn config_pass() {
        let config = Config::new(iterify("minigrep expr filepath")).unwrap();
        assert_eq!(config.path, "filepath");
        assert_eq!(config.expr, "expr");
        assert_eq!(config.case_sensitive, true);
    }
    #[test]
    fn config_fail() {
        if let Ok(_) = Config::new(iterify("minigrep expr filepath extra")) {
            panic!();
        }
        if let Ok(_) = Config::new(iterify("too short")) {
            panic!();
        }
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