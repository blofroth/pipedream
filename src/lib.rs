#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate rocket;
extern crate reqwest;
extern crate itertools;
extern crate regex;
extern crate getopts;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

extern crate serde;
extern crate serde_urlencoded;

pub mod transform;
pub mod head;
pub mod wget;
pub mod cut;
pub mod grep;
pub mod pipe;
pub mod common;
pub mod remote;