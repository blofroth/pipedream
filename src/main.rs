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
use rocket::response::Stream;
use wget::{WgetOptions};
use head::{HeadOptions, HeadTransform};
use transform::{LinesTransformer};


#[get("/")]
fn index() -> &'static str {
    "This is a dream of pipes"
}

#[get("/wget?<options>")]
fn wget(options: WgetOptions) -> 
    Result<Stream<reqwest::Response>, String> {
    wget::wget(options)
}



#[post("/head?<options>", data = "<input>")]
fn head(input: Data, options: HeadOptions) -> 
    Result<Stream<LinesTransformer<HeadTransform>>, String> {
    head::head_tf(input, options)
}

#[post("/cat", data = "<input>")]
fn cat(input: Data) -> Result<Stream<DataStream>, String> {
    Ok(Stream::from(input.open()))
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