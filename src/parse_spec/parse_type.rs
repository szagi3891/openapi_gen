use crate::{open_api_type::OpenApiType, utils::OrderHashMap};
use serde_json::Value;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use crate::utils::ErrorProcess;

pub fn parse_type(data: Value, all_spec: &Value) -> Result<OpenApiType, ErrorProcess> {
    let data = filter_null(&data);

    if let Some(data) = parse_type_one_of_with_discriminator(&data, all_spec)? {
        return Ok(data);
    }

    if let Some(data) = parse_type_one_of(&data, all_spec)? {
        return Ok(data);
    }

    if let Some(type_value) = get_type(&data) {
        if type_value == "string" {
            return parse_type_string(&data);
        }

        if type_value == "array" {
            return parse_type_array(&data, all_spec);
        }

        if type_value == "object" {
            if let Some(data) = parse_type_object_additional_properties(&data, all_spec)? {
                return Ok(data);
            }

            if let Some(data) = parse_type_object(&data, all_spec)? {
                return Ok(data);
            }

            println!("Data ... {data:#?}");
            panic!("Not match object");
        }

        if type_value == "integer" {
            return parse_type_integer(&data);
        }

        if type_value == "number" {
            return parse_type_number(&data);
        }

        if type_value == "boolean" {
            return parse_type_boolean(&data);
        }
    }

    if let Some(data) = parse_type_content(&data, all_spec)? {
        return Ok(data);
    }

    if let Some(data) = parse_type_schema(&data, all_spec)? {
        return Ok(data);
    }

    if let Some(data) = parse_type_ref(&data, all_spec)? {
        return Ok(data);
    }

    if let Some(data) = parse_type_empty_object(&data)? {
        return Ok(data);
    }

    if let Some(data) = parse_type_only_description(&data)? {
        return Ok(data);
    }

    println!("Data ... {data:#?}");
    panic!("Not match");
}


fn parse_type_content(data: &Value, all_spec: &Value) -> Result<Option<OpenApiType>, ErrorProcess> {
    #[derive(Debug, Serialize, Deserialize)]
    struct ContentSpec {
        content: HashMap<String, Value>,
        // description: Option<Value>,
        // r#in: Option<Value>,
        // name: Option<Value>,
        // required: Option<Value>,
        // headers: Option<Value>,
    }

    if let Ok(data) = serde_json::from_value::<ContentSpec>(data.clone()) {
        let mut list = Vec::new();
        let mut content_name = Vec::new();

        for (name, item) in data.content {
            content_name.push(name);
            list.push(item);
        }

        let content_value = list.pop();

        if list.len() > 0 {
            let content_message = content_name.join(", ");
            return Err(ErrorProcess::message(format!("ContentSpec: one parameter was expected, received {content_message}")));
        }

        let content_value = match content_value {
            Some(schema) => schema,
            None => {
                return Err(ErrorProcess::message("Schema is missing"));
            }
        };

        let result = parse_type(content_value, all_spec)?;
        return Ok(Some(result));
    }

    Ok(None)
}

fn parse_type_schema(data: &Value, all_spec: &Value) -> Result<Option<OpenApiType>, ErrorProcess> {
    #[derive(Debug, Serialize, Deserialize)]
    struct SchemaSpec {
        schema: Value,
    }

    if let Ok(data) = serde_json::from_value::<SchemaSpec>(data.clone()) {
        let schema_type = parse_type(data.schema, all_spec)?;
        return Ok(Some(schema_type));
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

fn parse_type_integer(data: &Value) -> Result<OpenApiType, ErrorProcess> {
    #[derive(Debug, Serialize, Deserialize)]
    struct StringSpec {
        r#type: String,                 //ignore
    }

    let _spec = serde_json::from_value::<StringSpec>(data.clone())?;

    Ok(OpenApiType::Number {
        required: true
    })
}


fn parse_type_number(data: &Value) -> Result<OpenApiType, ErrorProcess> {
    #[derive(Debug, Serialize, Deserialize)]
    struct StringSpec {
        r#type: String,                 //ignore
        // format: Option<Value>,          //ignore
        // description: Option<Value>,     //ignore
    }

    let _spec = serde_json::from_value::<StringSpec>(data.clone())?;

    Ok(OpenApiType::Number {
        required: true
    })
}


fn parse_type_boolean(data: &Value) -> Result<OpenApiType, ErrorProcess> {
    #[derive(Debug, Serialize, Deserialize)]
    struct StringSpec {
        r#type: String,                 //ignore
        // format: Option<Value>,          //ignore
        // description: Option<Value>,     //ignore
    }

    let _spec = serde_json::from_value::<StringSpec>(data.clone())?;

    Ok(OpenApiType::Boolean {
        required: true
    })
}

fn convert_required(required: Option<Vec<String>>) -> Result<HashSet<String>, ErrorProcess> {
    let mut out = HashSet::new();

    if let Some(required) = required {
        let required_message = required.join(", ");

        for item in required {
            let is_set = out.insert(item);

            if is_set == false {
                return Err(ErrorProcess::message(format!("duplicate values {required_message}")));
            }
        }
    }

    Ok(out)
}

fn parse_type_object_additional_properties(data: &Value, all_spec: &Value) -> Result<Option<OpenApiType>, ErrorProcess> {
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct ObjectSpec {
        r#type: String,
        #[serde(rename = "additionalProperties")]
        additional_properties: Value,
    }

    if let Ok(spec) = serde_json::from_value::<ObjectSpec>(data.clone()) {
        let value = parse_type(spec.additional_properties, all_spec)?;

        return Ok(Some(OpenApiType::Record {
            required: true,
            value: Box::new(value),
        }));
    }

    Ok(None)
}

fn parse_type_object(data: &Value, all_spec: &Value) -> Result<Option<OpenApiType>, ErrorProcess> {
    #[derive(Debug, Serialize, Deserialize)]
    struct ObjectSpec {
        r#type: String,                             //ignore
        properties: Option<HashMap<String, Value>>,
        required: Option<Vec<String>>,
    }

    if let Ok(spec) = serde_json::from_value::<ObjectSpec>(data.clone()) {

        let mut props_all: OrderHashMap<String, OpenApiType> = OrderHashMap::new();

        let required = convert_required(spec.required)?;

        for (prop_name, prop_spec) in spec.properties.unwrap_or(HashMap::new()) {
            let prop_type = parse_type(prop_spec, all_spec)?;
            let is_required = required.contains(&prop_name);

            props_all.expect_insert(prop_name.clone(), prop_type.set_required(is_required))?;
        }

        return Ok(Some(OpenApiType::Object {
            required: true,
            props: props_all
        }))
    }

    Ok(None)
}



fn parse_type_array(data: &Value, all_spec: &Value) -> Result<OpenApiType, ErrorProcess> {

    #[derive(Debug, Serialize, Deserialize)]
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


fn parse_type_empty_object(data: &Value) -> Result<Option<OpenApiType>, ErrorProcess> {
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Spec {
    }

    if let Ok(_spec) = serde_json::from_value::<Spec>(data.clone()) {
        Ok(Some(OpenApiType::Unknown))
    } else {
        Ok(None)
    }
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

fn get_sorted_map(mapping: HashMap<String, String>) -> Vec<(String, String)> {
    let mut list = Vec::new();

    for (name, value) in mapping {
        list.push((name, value));
    }

    use std::cmp::Ordering;

    list.sort_by(|(akey, _), (bkey, _)| -> Ordering {
        akey.cmp(bkey)
    });

    list
}

fn parse_type_one_of_with_discriminator(data: &Value, all_spec: &Value) -> Result<Option<OpenApiType>, ErrorProcess> {

    #[derive(Debug, Serialize, Deserialize)]
    struct DiscriminatorInner {
        #[serde(rename="propertyName")]
        property_name: String,
        mapping: HashMap<String, String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Spec {
        discriminator: DiscriminatorInner
    }

    if let Ok(spec) = serde_json::from_value::<Spec>(data.clone()) {
        let mut union = Vec::<OpenApiType>::new();
        
        for (ref_name, ref_spec) in get_sorted_map(spec.discriminator.mapping).iter() {
            let ref_spec = go_to_spec(all_spec, ref_spec)?;
            let mut item_type = parse_type(ref_spec.clone(), all_spec)?;

            item_type.object_try_add_literal_field(&spec.discriminator.property_name, ref_name);

            union.push(item_type);
        }

        return Ok(Some(OpenApiType::Union {
            required: true,
            list: union,
        }));
    } else {
        return Ok(None);
    }
}

fn parse_type_one_of(data: &Value, all_spec: &Value) -> Result<Option<OpenApiType>, ErrorProcess> {

    #[derive(Debug, Serialize, Deserialize)]
    struct Spec {
        #[serde(rename = "oneOf")]
        one_of: Vec<Value>,
    }

    if let Ok(spec) = serde_json::from_value::<Spec>(data.clone()) {
        let mut one_of = Vec::<OpenApiType>::new();
        
        for item in spec.one_of {
            let item_type = parse_type(item, all_spec)?;
            one_of.push(item_type);
        }

        if let Some((first, rest)) = one_of.as_slice().split_first() {
            if rest.len() == 0 {
                let first = (*first).clone();
                return Ok(Some(first));
            }

        } else {
            log::error!("error parse {data:#?}");
            return Err(ErrorProcess::message("Incorrect data in section 'oneOf'"));
        }

        return Ok(Some(OpenApiType::Union {
            required: true,
            list: one_of,
        }));
    } else {
        return Ok(None);
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


fn filter_null<'a>(spec: &'a Value) -> Value {
    if let Value::Object(data) = spec {
        let mut new_data = data.clone();

        new_data.retain(|_key: &String, value: &mut Value| -> bool {
            *value != Value::Null
        });

        return Value::Object(new_data);
    }

    spec.clone()
}