use reqwest::StatusCode;
use reqwest::Response;
use reqwest;
use common::ArgParsable;
use getopts::{Options, Matches};

#[derive(FromForm)]
pub struct WgetOptions {
    url: String
}

impl ArgParsable for WgetOptions {
    fn options_defs() -> Options {
        // only positional for now
        Options::new()
    }

    fn parse_matches(matches: Matches) -> Result<Self, String> {
        if matches.free.len() == 1 {
            Ok(WgetOptions { url: matches.free[0].clone() })
        } else {
            Err("requires (only) URL".to_string())
        }
    }

    fn usage_brief() -> String {
        "Usage: wget [options] URL".to_string()
    }
}

pub fn wget(options: WgetOptions) -> Result<Response, String> {
    let resp = reqwest::get(&options.url)
        .map_err(|e| e.to_string())?;

    match resp.status() {
        StatusCode::Ok => Ok(resp),
        _ => Err(format!("Error: {}", resp.status()))
    }
}

pub fn wget_client(arguments: &str) -> Result<Response, String> {
    wget(WgetOptions::from_args(arguments)?)
}