use transform::{LinesTransformer, LinesTransform, TfResult};
use rocket::response::Stream;
use rocket::{Data};

#[derive(FromForm)]
pub struct HeadOptions {
    n: u64
}

pub fn head_tf(input: Data, options: HeadOptions) -> Result<Stream<LinesTransformer<HeadTransform>>, String>  {
    let tf = LinesTransformer::new(input, 
                HeadTransform::new(options));
    Ok(Stream::from(tf))
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