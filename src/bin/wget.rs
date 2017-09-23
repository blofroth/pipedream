#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate pipedream;
extern crate reqwest;


use pipedream::transform;
use pipedream::wget;
use pipedream::wget::{WgetOptions};
use rocket::response::{Stream};

#[get("/wget?<options>")]
fn wget(options: WgetOptions) -> 
    Result<Stream<reqwest::Response>, String> {
    wget::wget_tf(transform::empty_stream(), &options).map(|r| Stream::from(r))
}

fn main() {
    rocket::ignite().mount("/", 
        routes![
            wget
        ]
    ).launch();
}