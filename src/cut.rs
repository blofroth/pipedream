use transform::{LinesTransformer, LinesTransform, TfResult};
use std::io::{Read};
use itertools::Itertools;

use common::ArgParsable;
use getopts::{Options, Matches};

#[derive(FromForm)]
pub struct CutOptions {
    /// delimiter
    d: Option<String>,
    /// fields
    f: String
}

impl ArgParsable for CutOptions {
    fn options_defs() -> Options {
        let mut opts = Options::new();
        opts.reqopt("f", "", "fields - comma separated set of numbers", "list");
        opts.optopt("d", "", "delimiter", "delim");

        opts
    }

    fn parse_matches(matches: Matches) -> Result<Self, String> {
        Ok(CutOptions {
            f: matches.opt_str("f").unwrap(),
            d: matches.opt_str("d")
        })
    }
}

pub fn cut_tf<I: Read>(input: I, options: CutOptions) -> Result<LinesTransformer<CutTransform, I>, String> {
    Ok(LinesTransformer::new(input, CutTransform::new(options)?))
}

pub fn cut_client<I: Read>(input: I, arguments: &str) -> Result<LinesTransformer<CutTransform, I>, String> {
    cut_tf(input, CutOptions::from_args(arguments)?)
}

pub struct CutTransform {
    delimiter: String,
    fields: Vec<usize>
}

fn parse_fields(fields_arg: String) -> Result<Vec<usize>, String> {

    let (oks, fails): (Vec<_>, Vec<_>) = fields_arg
        .split(",")
        .map(|s| s.parse::<usize>())
        .partition(Result::is_ok);
    
    if !fails.is_empty() {
        return Err(format!("could not parse some field indices: {:?} ({})", fails, fields_arg))
    }
    Ok(oks.into_iter()
        .filter_map(Result::ok)
        .collect())
}

impl CutTransform {
    fn new(options: CutOptions) -> Result<CutTransform, String> {
        let fields = parse_fields(options.f)?;
        Ok(CutTransform {
            delimiter: options.d.unwrap_or("\t".to_string()),
            fields: fields
        })
    }
}

impl LinesTransform for CutTransform {
    fn transform(&mut self, line: &str) -> TfResult {
        let enumerated = line
            .split(&self.delimiter)
            .enumerate();

        let mut wanted_parts = enumerated
            // indices start at 1
            .filter( |&(i, _)| self.fields.contains(&(i + 1)) )
            .map( |(_, part)| part);

        let put_together = wanted_parts.join(&self.delimiter);

        TfResult::Yield(put_together)
    }
}