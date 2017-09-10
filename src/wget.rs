use reqwest::StatusCode;
use reqwest::Response;
use reqwest;

#[derive(FromForm)]
pub struct WgetOptions {
    url: String
}

pub fn wget(options: WgetOptions) -> Result<Response, String> {
    let resp = reqwest::get(&options.url)
        .map_err(|e| e.to_string())?;

    match resp.status() {
        StatusCode::Ok => Ok(resp),
        _ => Err(format!("Error: {}", resp.status()))
    }
}

pub fn wget_client(arguments: Option<&str>) -> Result<Response, String> {
    let url = arguments.ok_or("no url provided")?;
    wget(WgetOptions { url: url.to_string() })
}