use clap::Parser as ClapParser;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// A simple application to parse a custom key-value file.
#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(required = true)]
    file_path: PathBuf,
}

#[derive(Parser)]
#[grammar = "key_value_pairs.pest"]
struct KeyValuePairsParser;

fn parse_key_value_pairs(text: &str) -> Result<HashMap<String, Vec<i32>>, String> {
    let mut key_value_pairs = HashMap::new();

    let pairs = KeyValuePairsParser::parse(Rule::file, text)
        .map_err(|e| e.to_string())?;

    for pair in pairs.flatten() {
        if pair.as_rule() == Rule::pair {
            let mut inner_rules = pair.into_inner();
            let identifier = inner_rules.next().unwrap();
            let value_list = inner_rules.next().unwrap();

            let key = identifier.as_str().to_string();

            let values_result: Result<Vec<i32>, _> = value_list
                .into_inner()
                .map(|num_pair| num_pair.as_str().trim().parse())
                .collect();

            match values_result {
                Ok(nums) => {
                    key_value_pairs.insert(key, nums);
                }
                Err(e) => {
                    return Err(format!("Failed to parse number for key '{}': {}", key, e));
                }
            }
        }
    }

    Ok(key_value_pairs)
}


fn main() {
    let cli = Cli::parse();

    println!("Reading file: {}", cli.file_path.display());
    let content = match fs::read_to_string(&cli.file_path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", cli.file_path.display(), e);
            std::process::exit(1);
        }
    };

    match parse_key_value_pairs(&content) {
        Ok(parsed_data) => {
            println!("\nSuccessfully parsed data:");
            println!("{:#?}", parsed_data);
        }
        Err(e) => {
            eprintln!("\nError parsing data: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_successful_parsing() {
        let data = r#"
# Asset holdings data
SomeValue: [1, 2, 3, 4]
AnotherKey: [100, -50, 25]
EmptyList: []
SpacedOut : [ 5,    6, 7 ]
        "#;

        let result = parse_key_value_pairs(data).unwrap();

        let mut expected = HashMap::new();
        expected.insert("SomeValue".to_string(), vec![1, 2, 3, 4]);
        expected.insert("AnotherKey".to_string(), vec![100, -50, 25]);
        expected.insert("EmptyList".to_string(), vec![]);
        expected.insert("SpacedOut".to_string(), vec![5, 6, 7]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parsing_fails_on_malformed_number() {
        let data = "BadData: [1, two, 3]";
        let result = parse_key_value_pairs(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_parsing_fails_on_bad_syntax() {
        let data = "MissingColon [1, 2, 3]";
        let result = parse_key_value_pairs(data);
        assert!(result.is_err());
        let error_message = result.unwrap_err();
        assert!(error_message.contains("expected"));
    }
}