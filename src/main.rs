use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

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

            // Collect the results of parsing
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
    let data = r#"
# Key-value pairs data
SomeValue: [1, 2, 3, 4]
AnotherKey: [100, -50, 25]
# Will parse even the below guy
EmptyList: []
SpacedOut : [ 5,    6, 7 ]

# An invalid line for testing:
# BadLine: [1, two, 3]
    "#;

    match parse_key_value_pairs(data) {
        Ok(parsed_data) => {
            println!("{:#?}", parsed_data);
        }
        Err(e) => {
            // Pest provides beautiful, human-readable error messages!
            eprintln!("Error parsing data:\n{}", e);
        }
    }
}

