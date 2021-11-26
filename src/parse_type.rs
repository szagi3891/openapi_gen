use crate::open_api_type::OpenApiType;
use serde_json::Value;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::error_process::ErrorProcess;

pub fn parse_type(data: Value, all_spec: &Value) -> Result<OpenApiType, ErrorProcess> {

    if let Some(data) = parse_type_content(&data, all_spec)? {
        return Ok(data);
    }

    if let Some(data) = parse_type_ref(&data, all_spec)? {
        return Ok(data);
    }

    if let Some(data) = parse_type_only_description(&data)? {
        return Ok(data);
    }

    if let Some(type_value) = get_type(&data) {
        if type_value == "string" {
            return parse_type_string(&data);
        }

        if type_value  == "array" {
            return parse_type_array(&data, all_spec);
        }
    }

    println!("");
    println!("");
    println!("prsujemy wartość {data:#?}");
    println!("");
    println!("");

    todo!()
}

fn parse_type_content(data: &Value, all_spec: &Value) -> Result<Option<OpenApiType>, ErrorProcess> {
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct ContentSpec {
        content: HashMap<String, ContentSchemaSpec>,
        description: Option<Value>,
        r#in: Option<Value>,
        name: Option<Value>,
        required: Option<Value>,
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct ContentSchemaSpec {
        schema: Value,
    }

    if let Ok(data) = serde_json::from_value::<ContentSpec>(data.clone()) {
        let mut list = Vec::new();
        let mut content_name = Vec::new();

        for (name, item) in data.content {
            content_name.push(name);
            list.push(item);
        }

        let schema = list.pop();

        if list.len() > 0 {
            let content_message = content_name.join(", ");
            return Err(ErrorProcess::message(format!("ContentSpec: one parameter was expected, received {content_message}")));
        }

        let schema = match schema {
            Some(schema) => schema,
            None => {
                return Err(ErrorProcess::message("Schema is missing"));
            }
        };

        let result = parse_type(schema.schema, all_spec)?;

        return Ok(Some(result));
    }

    Ok(None)
}


fn get_type(data: &Value) -> Option<String> {

    #[derive(Debug, Serialize, Deserialize)]
    struct TypeSpec {
        r#type: String,
    }

    if let Ok(data) = serde_json::from_value::<TypeSpec>(data.clone()) {
        Some(data.r#type.to_lowercase())
    } else {
        None
    }
}

fn parse_type_string(data: &Value) -> Result<OpenApiType, ErrorProcess> {

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct StringSpec {
        r#type: String,                 //ignore
        format: Option<Value>,          //ignore
        r#enum: Option<Value>,          //todo - add
        description: Option<Value>,     //ignore
    }

    let _spec = serde_json::from_value::<StringSpec>(data.clone())?;

    Ok(OpenApiType::String {
        required: true
    })
}

fn parse_type_array(data: &Value, all_spec: &Value) -> Result<OpenApiType, ErrorProcess> {

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct ArraySpec {
        r#type: String,
        items: Value,
    }

    let spec = serde_json::from_value::<ArraySpec>(data.clone())?;
    let items = parse_type(spec.items, all_spec)?;

    Ok(OpenApiType::Array {
        required: true,
        items: Box::new(items)
    })
}


fn parse_type_only_description(data: &Value) -> Result<Option<OpenApiType>, ErrorProcess> {
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Spec {
        description: String,
    }

    if let Ok(_spec) = serde_json::from_value::<Spec>(data.clone()) {
        Ok(Some(OpenApiType::Unknown))
    } else {
        Ok(None)
    }
}



fn parse_type_ref(data: &Value, all_spec: &Value) -> Result<Option<OpenApiType>, ErrorProcess> {
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct RefSpec {
        #[serde(rename = "$ref")]
        r#ref: String,
    }

    if let Ok(spec) = serde_json::from_value::<RefSpec>(data.clone()) {
        let ref_spec = go_to_spec(all_spec, &spec.r#ref)?;

        let spec = parse_type(ref_spec.clone(), all_spec)?;

        Ok(Some(spec))
    } else {
        Ok(None)
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

fn parse_ref_path(path: &str) -> Result<Vec<String>, ErrorProcess> {
    if let Some((_, path)) = path.split_once("#/") {
        Ok(path.split("/").map(|item| item.to_string()).collect())        
    } else {
        Err(ErrorProcess::message(format!("invalid ref {path}")))
    }
}

#[test]
fn test_parse_ref() {
    let result: Vec<String> = vec!("components".into(), "schemas".into(), "WithdrawalViewForAccount".into());

    assert_eq!(
        parse_ref_path("#/components/schemas/WithdrawalViewForAccount").unwrap(),
        result
    );
}