use std::path::{PathBuf};
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::env;

/// should not be considered secure
lazy_static! {
    pub static ref CAT_WRITE_ENABLED: bool = env::var("PIPEDREAM_CAT_WRITE_ENABLED")
        .map(|s| s.parse())
        .unwrap_or(Ok(false))
        .unwrap();
}
lazy_static! {
    pub static ref CAT_FILES_PATH: String = env::var("PIPEDREAM_CAT_FILES_PATH")
        .unwrap_or("files/".to_string());
}

pub fn cat_write<I: Read>(dest_file: PathBuf, input_stream: &mut I) -> Result<(), String> {
    if *CAT_WRITE_ENABLED {
        let mut dest_path = PathBuf::from(CAT_FILES_PATH.to_string());
        dest_path.push(&dest_file);
        let mut file = File::create(dest_path)
                    .map_err(|e| format!("Could not open file: {:?}", e))?;

        let mut buf = [0u8; 512];
        loop {
            let num_read = input_stream.read(&mut buf)
                .map_err(|e| format!("Could not read input stream: {:?}", e))?;
            if num_read == 0 {
                break;
            }
            file.write(&buf[..num_read])
                .map_err(|e| format!("Could not write to file: {:?}", e))?;;
        }
        Ok(())
    } else {
        Err("cat writing is disabled".to_string())
    }
}