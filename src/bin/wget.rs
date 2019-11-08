#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

extern crate pipedream;
extern crate reqwest;


use pipedream::transform;
use pipedream::wget;
use pipedream::wget::{WgetOptions};
use rocket::response::{Stream};
use rocket::request::{Form, FromFormValue};

#[get("/wget?<options..>")]
fn wget(options: Form<WgetOptions>) ->
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