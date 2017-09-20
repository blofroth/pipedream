#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate rocket;
extern crate reqwest;
extern crate pipedream;

use rocket::{Data};
use rocket::data::DataStream;
use rocket::response::{Stream, NamedFile};

use pipedream::{wget, head, cut, grep, pipe};
use pipedream::transform::{LinesTransformer};
use pipedream::wget::{WgetOptions};
use pipedream::head::{HeadOptions, HeadTransform};
use pipedream::cut::{CutOptions, CutTransform};
use pipedream::grep::{GrepOptions, GrepTransform};

use std::io::{Read};
use std::path::{PathBuf, Path};

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

#[post("/cut?<options>", data = "<input>")]
fn cut(input: Data, options: CutOptions) -> 
    Result<Stream<LinesTransformer<CutTransform, DataStream>>, String> {
    Ok(Stream::from(cut::cut_tf(input.open(), options)?))
}

#[post("/grep?<options>", data = "<input>")]
fn grep(input: Data, options: GrepOptions) -> 
    Result<Stream<LinesTransformer<GrepTransform, DataStream>>, String> {
    Ok(Stream::from(grep::grep_tf(input.open(), options)?))
}

#[post("/cat", data = "<input>")]
fn cat(input: Data) -> Result<Stream<DataStream>, String> {
    Ok(Stream::from(input.open()))
}

#[post("/pipe", data = "<input>")]
fn pipe(input: String) -> Result<Stream<Box<Read>>, String> {
    Ok(Stream::from(pipe::pipe(input)?))
}

fn main() {
    rocket::ignite().mount("/", 
        routes![
            index, 
            files,
            wget, 
            head,
            cut,
            grep,
            cat,
            pipe
        ]
    ).launch();
}