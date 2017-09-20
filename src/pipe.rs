
use {wget, head, cut, grep};
use std::io::{Cursor};
use std::io::{Read};

pub fn pipe(input: String) -> Result<Box<Read>, String> {
    let mut prev_response: Box<Read> = empty_stream();

    for line in input.lines() {
        let mut parts = line.split(" ");
        let command = parts.next().ok_or("missing command on line")?;
        let args_parts: Vec<_> = parts.collect();
        let args = args_parts.join(" ");

        println!("{:?}", command);

        let new_response: Box<Read> = match command {
            "wget" => Box::new(wget::wget_client(&args)?),
            "head" => Box::new(head::head_client(prev_response, &args)?),
            "cut" => Box::new(cut::cut_client(prev_response, &args)?),
            "grep" => Box::new(grep::grep_client(prev_response, &args)?),
            _ => return Err(format!("Unknown command: {:?}", command)) 
        };
        prev_response = new_response;
    }

    Ok(prev_response)
}

// TODO: impl LinesTransform for pipe

fn empty_stream() -> Box<Read> {
    Box::new(Cursor::new(Vec::new()))
}