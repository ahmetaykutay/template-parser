extern crate serde_json;

use std::fs;

pub type Data = serde_json::Value;

pub struct Config<'a> {
    pub filename: &'a str,
    pub json_filename: &'a str,
    pub opening_tag: &'a str,
    pub closing_tag: &'a str,
}

pub fn get_parsed_content(
    content: String,
    data: &Data,
    config: &Config,
) -> Result<String, &'static str> {
    let mut parsed = String::new();
    let mut remaining = content.clone();

    loop {
        let chars = remaining.chars();
        // find var
        let start_pos = match remaining.find(config.opening_tag) {
            Some(v) => v,
            None => {
                parsed.push_str(remaining.as_str());
                break;
            }
        };
        let end_pos = match remaining.find(config.closing_tag) {
            Some(v) => v,
            None => {
                parsed.push_str(remaining.as_str());
                break;
            }
        };
        let until_var: String = chars.clone().take(start_pos).collect();
        parsed.push_str(until_var.as_str());
        let slice: String = chars
            .clone()
            .skip(start_pos)
            .take((end_pos + 2) - start_pos)
            .collect();
        // replace var
        let without_beginning: String = String::from(slice.replace("<%", ""));
        let without_ending: String = String::from(without_beginning.replace("%>", ""));
        let key = without_ending.trim();
        let var_string = match &data[key] {
            serde_json::Value::String(v) => v.clone(),
            _ => {
                let v = String::new().to_owned();
                v.clone()
            }
        };
        parsed.push_str(&var_string);
        remaining = chars
            .skip(end_pos + 2)
            .take(remaining.len() - end_pos)
            .collect();
    }

    Ok(parsed)
}

pub fn read_json_file(config: &Config) -> serde_json::Result<serde_json::Value> {
    let content = fs::read_to_string(config.json_filename)
        .expect("Something went wrong while reading json file");

    let v: serde_json::Value = serde_json::from_str(content.as_str())?;

    Ok(v)
}

#[cfg(test)]
mod test {
    use super::*;
    static CONFIG: Config = Config {
        filename: "text.txt",
        json_filename: "",
        opening_tag: "<%",
        closing_tag: "%>",
    };

    fn parse_json_data(raw_data: &str) -> serde_json::Value {
        match serde_json::from_str(raw_data) {
            Ok(v) => v,
            Err(err) => panic!(err),
        }
    }

    #[test]
    fn get_parsed_content_correct() {
        let content = String::from("Hello <% greet %>");
        let data = parse_json_data(r#"{"greet": "World"}"#);

        match get_parsed_content(content, &data, &CONFIG) {
            Ok(v) => assert_eq!(v, String::from("Hello World")),
            Err(err) => panic!("could not parse: {}", err),
        }
    }

    #[test]
    fn can_parse_multiple_vars() {
        let content = String::from("Hello <% greet %>. My name is <% name %>.");
        let data = parse_json_data(r#"{"greet": "World", "name": "Aykut"}"#);

        match get_parsed_content(content, &data, &CONFIG) {
            Ok(v) => assert_eq!(v, String::from("Hello World. My name is Aykut.")),
            Err(err) => panic!("could not parse: {}", err),
        }
    }

    #[test]
    fn missing_data() {
        let content = String::from("Hello <% greet %>. My name is <% name %>.");
        let data = parse_json_data(r#"{}"#);

        match get_parsed_content(content, &data, &CONFIG) {
            Ok(v) => assert_eq!(v, String::from("Hello . My name is .")),
            Err(err) => panic!("could not parse: {}", err),
        };
    }
}
