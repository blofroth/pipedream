use transform::{LinesTransformer, LinesTransform, TfResult};
use std::io::{Read};

#[derive(FromForm)]
pub struct HeadOptions {
    /// number of lines to keep
    n: u64
}

pub fn head_tf<I: Read>(input: I, options: HeadOptions) -> LinesTransformer<HeadTransform, I> {
    LinesTransformer::new(input, HeadTransform::new(options))
}

pub fn head_client<I: Read>(input: I, arguments: Option<&str>) -> Result<LinesTransformer<HeadTransform, I>, String> {
    let n: u64 = arguments.ok_or("n not specified")?
                    .parse().map_err(|e| format!("{:?}", e))?;

    Ok(head_tf(input, HeadOptions { n: n }))
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
    fn transform(&mut self, line: &String) -> TfResult {
        self.num_processed += 1;
        if self.num_processed > self.limit {
            TfResult::Stop
        } else {
            TfResult::Yield(line.clone())
        }
    }
}