#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate rocket;
extern crate reqwest;
extern crate itertools;
extern crate regex;

pub mod transform;
pub mod head;
pub mod wget;
pub mod cut;
pub mod grep;