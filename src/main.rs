#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate rocket;
extern crate reqwest;
use rocket::{Data};
use rocket::data::DataStream;
use rocket::response::Stream;
use reqwest::StatusCode;
use std::io::{BufReader, Read, BufRead};
use std::io;
use std::cmp;

#[get("/")]
fn index() -> &'static str {
    "This is a dream of pipes"
}

#[derive(FromForm)]
struct WgetOptions {
    url: String
}

#[get("/wget?<options>")]
fn wget(options: WgetOptions) -> 
    Result<Stream<reqwest::Response>, String> {

    let resp = reqwest::get(&options.url)
        .map_err(|e| e.to_string())?;

    match resp.status() {
        StatusCode::Ok => {
            Ok(Stream::from(resp))
        }
        _ => Err(format!("Error: {}", resp.status()))
    }
}

#[derive(FromForm)]
struct HeadOptions {
    n: u64
}

#[post("/head?<options>", data = "<input>")]
fn head(input: Data, options: HeadOptions) -> 
    Result<Stream<LinesTransformer<HeadTransform>>, String> {
    let tf = LinesTransformer::new(input, 
                HeadTransform::new(options.n));
    Ok(Stream::from(tf))
}

#[post("/cat", data = "<input>")]
fn cat(input: Data) -> Result<Stream<DataStream>, String> {
    Ok(Stream::from(input.open()))
}

enum TfResult {
    Yield(String),
    Skip,
    Stop
}

trait LinesTransform {
    fn transform(&mut self, line: &String) -> TfResult;
}

struct HeadTransform {
    num_processed: u64,
    limit: u64
}

impl HeadTransform {
    fn new(n: u64) -> HeadTransform {
        HeadTransform {
            num_processed: 0,
            limit: n
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

struct LinesTransformer<T: LinesTransform> {
    reader: BufReader<DataStream>,
    finished: bool,
    curr_line: String,
    num_read: usize,
    transform: T
}

impl<T: LinesTransform> LinesTransformer<T> {
    fn new(data: Data, transform: T) -> LinesTransformer<T> {
        LinesTransformer {
            reader: BufReader::new(data.open()),
            finished: false,
            curr_line: String::new(),
            num_read: 0,
            transform: transform
        }
    }

    fn refill(&mut self) -> io::Result<()> {
        self.curr_line.clear();
        match self.reader.read_line(&mut self.curr_line) {
            Ok(read) => {       
                match self.transform.transform(&self.curr_line) {
                    TfResult::Yield(line) => {
                        self.num_read = 0;
                        self.curr_line = line;
                    }
                    TfResult::Skip => {
                        // leave num read and curr_line as is
                    },
                    TfResult::Stop => {
                        self.finished = true;
                    }
                }
                Ok(())
            },
            Err(e) => Err(e)
        }
    }
}

impl <T: LinesTransform> Read for LinesTransformer<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        while !self.finished && self.num_read == self.curr_line.len() {
            self.refill()?;
        }

        if self.finished {
            return Ok(0);
        }

        let str_bytes = self.curr_line.as_bytes();
        let remaining_bytes = &str_bytes[self.num_read..];
        let num_to_write = cmp::min(buf.len(), remaining_bytes.len());
        let (to_write, _) = remaining_bytes.split_at(num_to_write);
        buf[0..num_to_write].copy_from_slice(&to_write[0..num_to_write]);
        self.num_read += num_to_write;
        Ok(num_to_write)
    }
}



fn main() {
    rocket::ignite().mount("/", 
        routes![
            index, 
            wget, 
            head,
            cat
        ]
    ).launch();
}