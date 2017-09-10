#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate rocket;
extern crate reqwest;

mod transform;
mod head;
mod wget;

use rocket::{Data};
use rocket::data::DataStream;
use rocket::response::{Stream, NamedFile};
use wget::{WgetOptions};
use head::{HeadOptions, HeadTransform};
use transform::{LinesTransformer};
use std::io::{Cursor};
use std::io::{Read};
use std::path::{PathBuf, Path};

const BASE_URL: &'static str = "http://localhost:8000/";

#[get("/")]
fn index() -> &'static str {
    "This is a dream of pipes"
}

#[get("/files/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("files/").join(file)).ok()
}

#[get("/wget?<options>")]
fn wget(options: WgetOptions) -> 
    Result<Stream<reqwest::Response>, String> {
    wget::wget(options).map(|r| Stream::from(r))
}

#[post("/head?<options>", data = "<input>")]
fn head(input: Data, options: HeadOptions) -> 
    Result<Stream<LinesTransformer<HeadTransform, DataStream>>, String> {
    Ok(Stream::from(head::head_tf(input.open(), options)))
}

#[post("/cat", data = "<input>")]
fn cat(input: Data) -> Result<Stream<DataStream>, String> {
    Ok(Stream::from(input.open()))
}

#[post("/pipe", data = "<input>")]
fn pipe(input: String) -> Result<Stream<Box<Read>>, String> {
    let mut prev_response: Option<Box<Read>> = None;
    for line in input.lines() {
        if line.contains("?") {
            let mut parts = line.split("?");
            let command = parts.next();
            let new_response: Box<Read> = match command {
                Some("head") => {
                    let input = prev_response.ok_or("no previous response to pipe")?;
                    Box::new(head::head_client(input, parts.next())?)
                },
                Some("wget") => Box::new(wget::wget_client(parts.next())?),
                _ => return Err(format!("Unknown command: {:?}", command)) 
            };
            prev_response = Some(new_response);
        }
    }

    match prev_response {
        Some(readable) => Ok(Stream::from(readable)),
        None => {
            let empty_vec: Vec<u8> = Vec::new();
            let read: Box<Read> = Box::new(Cursor::new(empty_vec));
            Ok(Stream::from(read))
        }
    }
}

fn main() {
    rocket::ignite().mount("/", 
        routes![
            index, 
            files,
            wget, 
            head,
            cat,
            pipe
        ]
    ).launch();
}