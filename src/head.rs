use transform::{LinesTransformer, LinesTransform, TfResult};
use std::io::{Read};

use common::ArgParsable;
use getopts::{Options, Matches};

#[derive(FromForm)]
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

pub fn head_tf<I: Read>(input: I, options: HeadOptions) -> LinesTransformer<HeadTransform, I> {
    LinesTransformer::new(input, HeadTransform::new(options))
}

pub fn head_client<I: Read>(input: I, arguments: &str) -> Result<LinesTransformer<HeadTransform, I>, String> {
    Ok(head_tf(input, HeadOptions::from_args(arguments)?))
}

pub struct HeadTransform {
    num_processed: u64,
    limit: u64
}

impl HeadTransform {
    fn new(options: HeadOptions) -> HeadTransform {
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