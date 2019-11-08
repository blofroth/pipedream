use transform::{LinesTransformer, LinesTransform, TfResult, CharStream, Command};
use rocket::request::FromForm;
use common::ArgParsable;
use getopts::{Options, Matches};

#[derive(FromForm, Serialize)]
pub struct HeadOptions {
    /// number of lines to keep
    n: u64
}

impl ArgParsable for HeadOptions {
    fn options_defs() -> Options {
        let mut opts = Options::new();
        opts.reqopt("n", "", "number of lines to keep", "count");

        opts
    }

    fn parse_matches(matches: Matches) -> Result<Self, String> {
        Ok(HeadOptions {
            n: matches.opt_str("n").unwrap().parse()
                .map_err(|e| format!("bad integer: {:?}", e))?
        })
    }
}

impl Command for HeadOptions {
    fn name(&self) -> String {
        "head".to_string()
    }

    fn execute_local(&self, input: CharStream) -> Result<CharStream, String> {
        Ok(Box::new(LinesTransformer::new(input, HeadTransform::new(&self))))
    }
}

pub fn head_tf(input: CharStream, options: HeadOptions) -> CharStream {
    Box::new(LinesTransformer::new(input, HeadTransform::new(&options)))
}

pub fn head_client(input: CharStream, arguments: &str) -> Result<CharStream, String> {
    let options = HeadOptions::from_args(arguments)?;
    Ok(head_tf(input, options))
}

pub struct HeadTransform {
    num_processed: u64,
    limit: u64
}

impl HeadTransform {
    fn new(options: &HeadOptions) -> HeadTransform {
        HeadTransform {
            num_processed: 0,
            limit: options.n
        } 
    }
}

impl LinesTransform for HeadTransform {
    fn transform(&mut self, line: &str) -> TfResult {
        self.num_processed += 1;
        if self.num_processed > self.limit {
            TfResult::Stop
        } else {
            TfResult::Yield(String::from(line))
        }
    }
}