use reqwest::StatusCode;
use reqwest::Response;
use rocket::response::Stream;
use reqwest;

#[derive(FromForm)]
pub struct WgetOptions {
    url: String
}

pub fn wget(options: WgetOptions) -> Result<Stream<Response>, String> {
    let resp = reqwest::get(&options.url)
        .map_err(|e| e.to_string())?;

    match resp.status() {
        StatusCode::Ok => {
            Ok(Stream::from(resp))
        }
        _ => Err(format!("Error: {}", resp.status()))
    }
}