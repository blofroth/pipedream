use transform::{LinesTransformer, LinesTransform, TfResult, CharStream, Command};
use itertools::Itertools;
use rocket::request::FromForm;
use common::ArgParsable;
use getopts::{Options, Matches};

#[derive(FromForm, Serialize)]
pub struct SortOptions {
    /// key
    k: Option<String>
    /// field-separator
    t: Option<String>,
    /// numeric 
    n: Option<bool>
    /// reverse
    r: Option<bool>
    /// ignore case
    f: Option<bool>
}

impl ArgParsable for SortOptions {
    fn options_defs() -> Options {
        let mut opts = Options::new();
        opts.optopt("k", "key", "start a key at POS1, end it at POS2 (origin 1)", "POS1[,POS2]");
        opts.optopt("t", "field-separator", "use SEP instead of non-blank to blank transition", "SEP");
        opts.flagopt("n", "numeric-sort", "compare according to string numerical value");
        opts.flagopt("r", "reverse", "reverse the result of comparisons");
        opts.flagopt("f", "ignore-case", "fold lower case to upper case characters");
        opts
    }

    fn parse_matches(matches: Matches) -> Result<Self, String> {
        Ok(SortOptions {
            k: matches.opt_str("k"),
            t: matches.opt_str("t"),
            n: Some(matches.opt_present("n")),
            r: Some(matches.opt_present("r")),
            f: Some(matches.opt_present("f"))
        })
    }
}

impl Command for SortOptions {
    fn name(&self) -> String {
        "sort".to_string()
    }

    fn execute_local(&self, input: CharStream) -> Result<CharStream, String> {
        Ok(Box::new(LinesTransformer::new(input, SortTransform::new(&self)?)))
    }
}

pub struct SortTransform {
    extract_key: FnMut(&str) -> &str,
    compare_key: FnMut(&str, &str) -> Ordering,
    reverse: bool
}

impl SortTransform {
    fn new(options: &SortOptions) -> Result<SortTransform, String> {
        let keys = options.k.map(|keys| parse_fields(&keys)?);
        let extract_key = if let Some(keys) = keys {
            let field_separator_o = options.t.clone();
            let field_separator = field_separator_o.unwrap_or(r"\s+".to_string());
            unimplemented!()
        } else {
            // use whole line
            |x| x
        }

        let compare_key = |k1, k2| k1.cmp(k2);
        /*
        let compare_key = if let Some(true) = options.n {
            // numeric 
            |key1, key2| {
                
            }
        }*/
        
        Ok(SortTransform {
            extract_key: extract_key,
            compare_key: fields,
            reverse: options.r.unwrap_or(false)
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