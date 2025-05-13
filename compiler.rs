use std::path::{Path};
use std::fs::File;
use std::io::{Read, Write};
use std::{io};
use std::process::exit;
use std::env;
use std::time::{Duration, Instant};
use std::sync::{Mutex};

// Define print variables
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const ITALIC: &str = "\x1b[3m";

const FG_BRIGHT_RED: &str = "\x1b[91m";
const FG_BRIGHT_GREEN: &str = "\x1b[92m";
const FG_BRIGHT_YELLOW: &str = "\x1b[93m";
const FG_BRIGHT_BLUE: &str = "\x1b[94m";
const FG_BRIGHT_CYAN: &str = "\x1b[96m";

static START_TIME: Mutex<Option<Instant>> = Mutex::new(None);

fn start_timer() {
    let mut start_time = START_TIME.lock().unwrap();
    *start_time = Some(Instant::now());
}

fn get_elapsed_time() -> Option<Duration> {
    let start_time = START_TIME.lock().unwrap();
    start_time.map(|start| start.elapsed())
}

fn read_file(file_path: &str) -> Result<String, io::Error> {
    let path = Path::new(file_path);
    let mut file = File::open(path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn write_file(file_path: &str, contents: &str) -> Result<(), io::Error> {
    let path = Path::new(file_path);
    let mut file = File::create(path)?;

    file.write_all(contents.as_bytes())?;

    Ok(())
}

struct Compiler {
    file_content: String,
    path: String,
}

impl Compiler {
    pub fn new(path: &str) -> Result<Self, io::Error> {
        let file_contents = read_file(path)?.replace(";", "\n");

        println!("{}", file_contents);

        Ok(Compiler {
            file_content: file_contents,
            path: path.to_string(),
        })
    }

    pub fn run(&self) -> Result<(), String> {
        let lines = self.file_content.split("\n");
        let mut new_file: String = String::new();

        let mut element_stack: Vec<String> = Vec::new();

        for raw_line in lines {
            let line = raw_line.trim();
            let mut new_line: String = String::new();

            // Possible line suffixes: ';' (property), '{' (element), '}' element end

            let suffix = match line.chars().last() {
                Some(val) => val,
                None => {
                    continue;
                }
            };

            if suffix == '{' {
                if !element_stack.is_empty() {
                    new_line += ">\n"
                }

                let mut split_line = line.split(" "); // Mapping it to an owned string, consuming line

                // Locate prefix, and if no prefix is found then simply default to element
                let prefix = split_line.next().unwrap();

                element_stack.push(prefix.to_string());

                // Extract size and position data

                let mut classes = String::new();
                let mut id = String::new();
                
                let mut i = 0;
                for raw_part in split_line {
                    let first_char = match raw_part.chars().next() {
                        Some(val) => val,
                        None => {
                            continue;
                        }
                    };

                    if first_char == '{' {
                        continue;
                    }

                    if first_char == '.' {
                        classes += raw_part;
                        continue;
                    }

                    if first_char == '#' && id == "" {
                        id += raw_part;
                        continue;
                    }
                   
                    println!("ERROR");

                    i += 1; 
                }

                new_line += format!("<{} class=\"{}\" id=\"{}\" style=\"position: absolute; padding: 0px; margin: 0px; ", prefix, classes, id).as_str()
            }

            else if suffix == '}' {
                if let Some(element) = element_stack.pop() {
                    new_line = format!("</{}>\n", element);
                }
                else {
                    println!("ERROR");
                }
            }

            new_file += &new_line;
        }

        println!("{}", new_file);

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    start_timer();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 { // Rust args include the program name itself
        eprintln!(
            "{}{:?}{}: {}{}Argument error:{} Not enough arguments {}(Must have 1: file path){}",
            FG_BRIGHT_CYAN, get_elapsed_time(), RESET, BOLD, FG_BRIGHT_RED, RESET, FG_BRIGHT_BLUE, RESET
        );
        exit(1);
    } else if args.len() > 2 {
        eprintln!(
            "{}{:?}{}: {}Too many arguments{}{} {}(Should have 1: file path){}",
            FG_BRIGHT_CYAN, get_elapsed_time(), RESET, BOLD, FG_BRIGHT_YELLOW, RESET, FG_BRIGHT_BLUE, RESET
        );
    }

    let path: &str = &args[1].clone(); // Clone the path to own it.

    let app = Compiler::new(path)?;
    app.run()?;
    Ok(())
}