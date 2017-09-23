use wget;
use reqwest::{Response};
use std::env;
use serde_urlencoded;
use serde::Serialize;
use transform::CharStream;


lazy_static! {
    static ref REMOTE_BASE_URL: String = env::var("PIPEDREAM_REMOTE_URL")
        .unwrap_or("http://localhost:8000".to_string());
}

pub struct RemoteClient {
    remote_base_url: String
}

impl RemoteClient {
    pub fn new() -> Self {
        RemoteClient { remote_base_url: REMOTE_BASE_URL.to_string() }
    }

    fn get_command_url(&self, command: &str) -> String {
        format!("{}/{}", self.remote_base_url, command)
    }

    pub fn call_remote<O>(&self, input: CharStream, command: &str, options: &O) -> Result<Response, String> 
        where O: Serialize {
        let command_base_url = self.get_command_url(command);
        let full_url = format!("{}?{}", command_base_url, 
            serde_urlencoded::to_string(options)
                .map_err(|e| e.to_string())?);
        
        wget::wget_tf(input, &wget::WgetOptions {
            url: full_url,
            post_data: Some(true)
        })
    }
}



