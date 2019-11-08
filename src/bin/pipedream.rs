#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate reqwest;
extern crate pipedream;

use rocket::{Data};
use rocket::data::DataStream;
use rocket::response::{Stream, NamedFile};
use rocket::request::{Form, FromFormValue};

use pipedream::{wget, head, cut, grep, pipe, cat};
use pipedream::transform::{empty_stream, CharStream};
use pipedream::wget::{WgetOptions};
use pipedream::head::{HeadOptions};
use pipedream::cut::{CutOptions};
use pipedream::grep::{GrepOptions};

use std::path::{PathBuf, Path};

#[get("/")]
fn index() -> &'static str {
    "This is a dream of pipes"
}

#[get("/wget?<options..>")]
fn wget(options: Form<WgetOptions>) ->
    Result<Stream<reqwest::Response>, String> {
    wget::wget_tf(empty_stream(), &options).map(|r| Stream::from(r))
}

#[post("/head?<options..>", data = "<input>")]
fn head(input: Data, options: Form<HeadOptions>) ->
    Result<Stream<CharStream>, String> {
    Ok(Stream::from(head::head_tf(Box::new(input.open()), options.into_inner())))
}

#[post("/cut?<options..>", data = "<input>")]
fn cut(input: Data, options: Form<CutOptions>) ->
    Result<Stream<CharStream>, String> {
    Ok(Stream::from(cut::cut_tf(Box::new(input.open()), options.into_inner())?))
}

#[post("/grep?<options..>", data = "<input>")]
fn grep(input: Data, options: Form<GrepOptions>) ->
    Result<Stream<CharStream>, String> {
    Ok(Stream::from(grep::grep_tf(Box::new(input.open()), options.into_inner())?))
}

#[get("/cat/<file..>")]
fn cat_read(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(&*cat::CAT_FILES_PATH).join(file)).ok()
}

#[post("/cat/<dest_file..>", data = "<input>")]
fn cat_write(dest_file: PathBuf, input: Data) -> Result<(), String> {
    cat::cat_write(dest_file, &mut input.open())
}

#[post("/cat", data = "<input>")]
fn cat_echo(input: Data) -> Result<Stream<DataStream>, String> {
    Ok(Stream::from(input.open()))
}

#[post("/pipe", data = "<input>")]
fn pipe(input: String) -> Result<Stream<CharStream>, String> {
    Ok(Stream::from(pipe::pipe(input)?))
}

fn main() {
    rocket::ignite().mount("/", 
        routes![
            index,
            wget, 
            head,
            cut,
            grep,
            cat_read,
            cat_write,
            cat_echo,
            pipe
        ]
    ).launch();
}