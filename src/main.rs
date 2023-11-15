/// Markdown Parser Command-Line Interface Implementation

use markdown_parser::*;
use std::env;
use std::fs;

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => print_help(),
        2 => {
            let command = &args[1];
            match command.as_str() {
                "help" => print_help(),
                _ => {
                    eprintln!("Unknown command: {}. Please read the following instructions:", command);
                    print_help();
                }
            }
        }
        3 => {
            let command = &args[1];
            let input_path = &args[2];

            match command.as_str() {
                "to_html" => {
                    let markdown_content = read_file(input_path)?;
                    let html_output = to_html(&markdown_content)?;
                    println!("{}", html_output);
                }
                _ => {
                    eprintln!("Unknown command: {}. Please read the following instructions:", command);
                    print_help();
                }
            }
        }
        _ => {
            eprintln!("Invalid number of arguments. Please read the following instructions:");
            print_help();
        }
    }

    Ok(())
}

fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}

fn print_help() {
    println!("Hi! Here is a little guide on my available interface:\n");
    println!("  - cargo run to_html <markdown_file> -> reforms given single markdown-styled string from file into html-styled\n");
    println!("  - cargo run help -> returns you to this guide\n");
    println!(" - cargo doc --open -> opens documenation for this project\n");
    println!("Stay tuned and follow the updates – more functionality is about to appear!")
}



