use transform::{LinesTransformer, LinesTransform, TfResult};
use std::io::{Read};
use regex::Regex;

#[derive(FromForm)]
pub struct GrepOptions {
    /// pattern to match against lines
    pattern: String,
    /// ignore case (default: false)
    i: Option<bool>
}

pub fn grep_tf<I: Read>(input: I, options: GrepOptions) -> Result<LinesTransformer<GrepTransform, I>, String> {
    Ok(LinesTransformer::new(input, GrepTransform::new(options)?))
}

pub fn grep_client<I: Read>(input: I, arguments: Option<&str>) -> Result<LinesTransformer<GrepTransform, I>, String> {
    let pattern_arg = arguments.ok_or("pattern is not specified")?;
                    
    grep_tf(input, GrepOptions { i: None,  pattern: pattern_arg.to_string() })
}

pub struct GrepTransform {
    re: Regex,
    ignore_case: bool
}

impl GrepTransform {
    fn new(options: GrepOptions) -> Result<GrepTransform, String> {
        Ok(GrepTransform {
            re: Regex::new(&options.pattern).map_err(|e| format!("{:?}", e))?,
            ignore_case: options.i.unwrap_or(false)
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