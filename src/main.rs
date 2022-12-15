extern crate serde_json;

use std::fs;
use template_parser;

fn main() {
    let config = template_parser::Config {
        filename: "example.txt",
        json_filename: "example.json",
        opening_tag: "<%",
        closing_tag: "%>",
    };
    let content =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    match template_parser::read_json_file(&config) {
        Ok(data) => {
            match template_parser::get_parsed_content(content, &data, &config) {
                Ok(v) => println!("{}", v),
                Err(err) => panic!("could not parse: {}", err),
            };
        }
        Err(err) => panic!("couldn't parse json: {}", err),
    }
}
