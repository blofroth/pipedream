use reqwest::{Response, Client, Body, StatusCode};
use reqwest;
use common::ArgParsable;
use getopts::{Options, Matches};
use transform::{Command, CharStream};
use rocket::request::FromForm;

pub const DEFAULT_LOCAL_BASE_URL: &str = "http://localhost:8000/";

#[derive(FromForm, Serialize)]
pub struct WgetOptions {
    /// url to call
    pub url: String,
    /// whether to use stdin as post data, and make a post request
    pub post_data: Option<bool>
}

impl ArgParsable for WgetOptions {
    fn options_defs() -> Options {
        let mut opts = Options::new();
        opts.optflag("", "post-data", "whether to use stdin as post data");
        opts
    }

    fn parse_matches(matches: Matches) -> Result<Self, String> {
        if matches.free.len() == 1 {
            Ok(WgetOptions { 
                url: matches.free[0].clone(), 
                post_data: Some(matches.opt_present("post-data"))
            })
        } else {
            Err("requires (only) URL".to_string())
        }
    }

    fn usage_brief() -> String {
        "Usage: wget [options] URL".to_string()
    }
}

impl Command for WgetOptions {
    fn name(&self) -> String {
        "wget".to_string()
    }

    fn execute_local(&self, input: CharStream) -> Result<CharStream, String> {
        Ok(Box::new(wget_tf(input, self)?))
    }
}

pub fn wget_tf(input: CharStream, options: &WgetOptions) -> Result<Response, String> {
    let resp = if options.post_data.is_some() && options.post_data.unwrap() {
        let client = Client::new();
                        
        client.post(&options.url)
            .body(Body::new(input))
            .send()
                .map_err(|e| e.to_string())?
    } else {
        reqwest::get(&options.url)
            .map_err(|e| e.to_string())?
    };

    match resp.status() {
        StatusCode::OK => Ok(resp),
        _ => Err(format!("Error: {}", resp.status()))
    }
}

pub fn wget_client(input: CharStream, arguments: &str) -> Result<Response, String> {
    wget_tf(input, &WgetOptions::from_args(arguments)?)
}