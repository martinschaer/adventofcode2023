use log::{error, info};
use std::io::Result;
use std::time::Instant;

// Runs a function and prints the result and the elapsed time
pub fn run<T: std::fmt::Debug>(fn_name: String, f: fn(&str) -> Result<T>, file_path: &str) {
    let now = Instant::now();
    match f(file_path) {
        Ok(result) => {
            println!("Result {}: {:?}", fn_name, result);
            info!("took: {}ms", now.elapsed().as_millis());
        }
        Err(err) => {
            error!("Error: {}", err);
            std::process::exit(1);
        }
    };
}
