use log::{debug, error, info};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;
use std::time::Instant;

// This function is an exercise of modularising duplicated.
// Using the heap is not ideal but I wanted to try Boxes. I think defining a
// and b as 'static vars would be more performant
fn _set_nums(num: u32, a: &mut Box<Option<u32>>, b: &mut u32) {
    if (**a).is_none() {
        **a = Some(num);
    }
    *b = num;
}

// searches ony for digits
fn read_digits(file: &str) -> Result<u32> {
    let file = File::open(file)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    let mut sum = 0;
    let mut a = Box::new(None);
    let mut b;

    while buf_reader.read_line(&mut contents).unwrap() > 0 {
        *a = None;
        b = 0;
        for char in contents.chars() {
            if let Some(d) = char.to_digit(10) {
                _set_nums(d, &mut a, &mut b);
            }
        }
        sum += a.unwrap_or(0) * 10 + b;
        contents.clear();
    }
    Ok(sum)
}

enum TokenParsedResult {
    Matched(u32),
    Incomplete,
    NoMatch,
}

fn _match_number_word(str: &str) -> TokenParsedResult {
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for (i, word) in words.iter().enumerate() {
        if word == &str {
            debug!("Match: {} ({})", str, i + 1);
            // +1 because words array starts with "one"
            return TokenParsedResult::Matched(i as u32 + 1);
        }
        if word.starts_with(str) {
            debug!("Incomplete: {}", str);
            return TokenParsedResult::Incomplete;
        }
    }
    debug!("NoMatch: {}", str);
    TokenParsedResult::NoMatch
}

// searches for digits and number names
fn read_alpha(file: &str) -> Result<u32> {
    let file = File::open(file)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    let mut sum = 0;
    let mut a = Box::new(None);
    let mut b;
    while buf_reader.read_line(&mut contents).unwrap() > 0 {
        *a = None;
        b = 0;
        let mut acc = String::new();
        for char in contents.chars() {
            // skip non alpha chars
            if !char.is_alphanumeric() {
                continue;
            }
            // easy if its a digit
            if let Some(d) = char.to_digit(10) {
                _set_nums(d, &mut a, &mut b);
                acc.clear();
                continue;
            }
            // if not we parse tokens
            acc.push(char);
            loop {
                match _match_number_word(&acc) {
                    TokenParsedResult::Matched(d) => {
                        _set_nums(d, &mut a, &mut b);
                        acc.remove(0);
                    }
                    TokenParsedResult::Incomplete => {
                        break;
                    }
                    TokenParsedResult::NoMatch => {
                        acc.remove(0);
                    }
                };
                // break loop
                if acc.is_empty() {
                    break;
                }
            }
        }
        debug!("{} => {} + {}", contents.trim_end(), a.unwrap_or(0), b);
        sum += a.unwrap_or(0) * 10 + b;
        contents.clear();
        // press enter to continue
        // std::io::stdin().read_line(&mut String::new()).unwrap();
    }
    Ok(sum)
}

// executes a function and prints the result and the elapsed time
// TODO: move this to a place to be reused
fn _exec(fn_name: String, f: fn(&str) -> Result<u32>, file_path: &str) {
    let now = Instant::now();
    match f(file_path) {
        Ok(result) => {
            println!("Result {}: {}", fn_name, result);
            info!("took: {}ms", now.elapsed().as_millis());
        }
        Err(err) => {
            error!("Error: {}", err);
            std::process::exit(1);
        }
    };
}

fn main() {
    // remove timestamp from logs
    env_logger::builder().format_timestamp(None).init();

    // read CLI args
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).unwrap_or_else(|| {
        println!("Usage: RUST_LOG=<info|bebug|error> {} <file>", args[0]);
        std::process::exit(1);
    });

    // execute functions
    _exec(String::from("digits"), read_digits, file_path);
    _exec(String::from("alpha"), read_alpha, file_path);

    std::process::exit(0);
}
