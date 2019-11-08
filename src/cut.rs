use transform::{LinesTransformer, LinesTransform, TfResult, CharStream, Command};
use itertools::Itertools;
use rocket::request::FromForm;
use common::ArgParsable;
use getopts::{Options, Matches};

#[derive(FromForm, Serialize)]
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

impl Command for CutOptions {
    fn name(&self) -> String {
        "cut".to_string()
    }

    fn execute_local(&self, input: CharStream) -> Result<CharStream, String> {
        Ok(Box::new(LinesTransformer::new(input, CutTransform::new(&self)?)))
    }
}

pub fn cut_tf(input: CharStream, options: CutOptions) -> Result<CharStream, String> {
    Ok(Box::new(LinesTransformer::new(input, CutTransform::new(&options)?)))
}

pub fn cut_client(input: CharStream, arguments: &str) -> Result<CharStream, String> {
    let options = CutOptions::from_args(arguments)?;
    Ok(cut_tf(input, options)?)
}

pub struct CutTransform {
    delimiter: String,
    fields: Vec<usize>
}

impl CutTransform {
    fn new(options: &CutOptions) -> Result<CutTransform, String> {
        let fields = parse_fields(&options.f)?;
        let o = options.d.clone();
        let delim = o.unwrap_or("\t".to_string());
        Ok(CutTransform {
            delimiter: delim,
            fields: fields
        })
    }
}

fn parse_fields(fields_arg: &str) -> Result<Vec<usize>, String> {

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