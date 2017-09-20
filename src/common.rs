
use getopts::{Options, Matches};
use regex::Regex;

pub trait ArgParsable : Sized {
    fn options_defs() -> Options;
    fn parse_matches(matches: Matches) -> Result<Self, String>;

    fn from_args(args: &str) -> Result<Self, String> {
        let args = parse_arg_str(args);

        let matches = Self::options_defs().parse(args)
            .map_err(|e| format!("{}\n{}", e, Self::usage()))?;

        Self::parse_matches(matches)
            .map_err(|e| format!("{}\n{}", e, Self::usage()))
    }

    fn usage_brief() -> String {
        "Usage:".to_string()
    }

    fn usage() -> String {
        Self::options_defs().usage(&Self::usage_brief())
    }
}

fn parse_arg_str(arg_str: &str) -> Vec<&str> {
    let re = Regex::new(r"\s+").unwrap();
    re.split(arg_str).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct MyOptions {
        a: String,
        b: u32
    }

    impl ArgParsable for MyOptions {
        fn options_defs() -> Options {
            let mut opts = Options::new();
            opts.reqopt("a", "", "a string", "STRING");
            opts.reqopt("b", "", "an int", "NUMBER");
            opts
        }

        fn parse_matches(matches: Matches) -> Result<Self, String> {
            Ok(MyOptions {
                a: matches.opt_str("a").unwrap(),
                b: matches.opt_str("b").unwrap().parse()
                    .map_err(|e| format!("Could not parse numerical field: {:?}", e))?
            })
        }
    }

    #[test]
    fn test_from_args() {
        let s =  "-a x -b 1";

        assert_eq!(MyOptions { a: "x".to_string(), b: 1}, 
            MyOptions::from_args(s).unwrap());
    }

    
}