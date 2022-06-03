use serde_json::{Value, Map};

use crate::utils::ErrorProcess;

use super::get_sorted_map::get_sorted_map;

pub fn deref(value: Value, all: &Value) -> Value {
    match value.clone() {
        Value::Object(props) => {
            if let Some(path) = props.get("$ref") {
                if props.len() > 1 {
                    panic!("There may be only $ref as the only attribute of the object");
                }

                if let Value::String(path) = path {
                    let target_value = go_to_spec(all, path.as_str());

                    let target_value = match target_value {
                        Ok(value) => value,
                        Err(err) => {
                            dbg!(value);
                            println!("{err:?}");
                            panic!("Problem with finding ref: {path}");
                        }
                    };

                    return deref(target_value.clone(), all);

                } else {
                    panic!("The attribute pointed to by $ref can only be a string");
                }
            }

            let mut new_map = Map::new();

            for (key, value) in get_sorted_map(props.into_iter()) {
                new_map.insert(key, deref(value, all));
            }

            Value::Object(new_map)
        },
        Value::Array(list) => {
            let mut new_list = Vec::new();

            for item in list.into_iter() {
                new_list.push(deref(item, all));
            }

            Value::Array(new_list)
        },
        rest => rest
    }
}

pub fn filter_deref(value: Value) -> Value {
    let all = value.clone();
    deref(value, &all)
}


fn parse_ref_path(path: &str) -> Result<Vec<String>, ErrorProcess> {
    if let Some((_, path)) = path.split_once("#/") {
        Ok(path.split("/").map(|item| item.to_string()).collect())        
    } else {
        Err(ErrorProcess::message(format!("invalid ref {path}")))
    }
}

fn go_to_spec_property<'a>(spec: &'a Value, property: &str, ref_path: &str) -> Result<&'a Value, ErrorProcess> {
    match spec {
        Value::Object(data) => {
            match data.get(property) {
                Some(value) => {
                    Ok(value)
                },
                None => {
                    let message = format!("invalid ref {ref_path}, Error with reference to property = {property}, no value");
                    Err(ErrorProcess::message(message))
                }
            }
        },
        _ => {
            let message = format!("invalid ref {ref_path}, Error with reference to property = {property}, no map");
            Err(ErrorProcess::message(message))
        }
    }
}

fn go_to_spec<'a>(all_spec: &'a Value, ref_path: &str) -> Result<&'a Value, ErrorProcess> {
    let ref_path_list = parse_ref_path(&ref_path)?;

    let mut current = all_spec;

    for property in ref_path_list {
        current = go_to_spec_property(current, &property, ref_path)?;
    }

    Ok(current)
}


#[test]
fn test_parse_ref() {
    let result: Vec<String> = vec!("components".into(), "schemas".into(), "WithdrawalViewForAccount".into());

    assert_eq!(
        parse_ref_path("#/components/schemas/WithdrawalViewForAccount").unwrap(),
        result
    );
}
