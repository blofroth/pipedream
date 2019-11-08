use transform::{LinesTransformer, LinesTransform, TfResult, CharStream, Command};
use regex::{Regex, RegexBuilder};
use rocket::request::FromForm;
use common::ArgParsable;
use getopts::{Options, Matches};

#[derive(FromForm, Serialize)]
pub struct GrepOptions {
    /// pattern to match against lines
    pattern: String,
    /// ignore case (default: false)
    i: Option<bool>
}

impl ArgParsable for GrepOptions {
    fn options_defs() -> Options {
        let mut opts = Options::new();
        opts.optflag("i", "ignore-case", "perform case insensitive matching");

        opts
    }

    fn parse_matches(matches: Matches) -> Result<Self, String> {
        if matches.free.len() == 1 {
            Ok(GrepOptions {
                pattern: matches.free[0].clone(),
                i: Some(matches.opt_present("i"))
            })
        } else {
            Err("requires (only) pattern".to_string())
        }
    }

    fn usage_brief() -> String {
        "Usage: grep [options] pattern".to_string()
    }
}

impl Command for GrepOptions {
    fn name(&self) -> String {
        "grep".to_string()
    }

    fn execute_local(&self, input: CharStream) -> Result<CharStream, String> {
        Ok(Box::new(LinesTransformer::new(input, GrepTransform::new(&self)?)))
    }
}


pub fn grep_tf(input: CharStream, options: GrepOptions) -> Result<CharStream, String> {
    Ok(Box::new(LinesTransformer::new(input, GrepTransform::new(&options)?)))
}

pub fn grep_client(input: CharStream, arguments: &str) -> Result<CharStream, String> {
    let options = GrepOptions::from_args(arguments)?;
    Ok(grep_tf(input, options)?)
}

pub struct GrepTransform {
    re: Regex
}

impl GrepTransform {
    fn new(options: &GrepOptions) -> Result<GrepTransform, String> {
        Ok(GrepTransform {
            re: RegexBuilder::new(&options.pattern)
                    .case_insensitive(options.i.unwrap_or(false))
                    .build()
                        .map_err(|e| format!("{:?}", e))?
        })
    }
}

impl LinesTransform for GrepTransform {
    fn transform(&mut self, line: &str) -> TfResult {
        if self.re.is_match(line) {
            TfResult::Yield(String::from(line))
        } else {
            TfResult::Skip
        }
    }
}