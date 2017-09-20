use transform::{LinesTransformer, LinesTransform, TfResult};
use std::io::{Read};
use regex::{Regex, RegexBuilder};

use common::ArgParsable;
use getopts::{Options, Matches};

#[derive(FromForm)]
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

pub fn grep_tf<I: Read>(input: I, options: GrepOptions) -> Result<LinesTransformer<GrepTransform, I>, String> {
    Ok(LinesTransformer::new(input, GrepTransform::new(options)?))
}

pub fn grep_client<I: Read>(input: I, arguments: &str) -> Result<LinesTransformer<GrepTransform, I>, String> {
    grep_tf(input, GrepOptions::from_args(arguments)? )
}

pub struct GrepTransform {
    re: Regex
}

impl GrepTransform {
    fn new(options: GrepOptions) -> Result<GrepTransform, String> {
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