use super::filters::{filter_deref, filter_null};
use super::parse_type::parse_type;
use serde_json::Value;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::utils::{expect_json};
use crate::open_api_spec::{SpecHandlerType, OpenApiMethod, SpecOpenApi};


#[derive(Debug, Serialize, Deserialize)]
struct Spec {
    paths: HashMap<String, HashMap<String, Option<Value>>>,
}

fn parse_file(data: String) -> Value {
    if let Ok(data) = serde_json::from_str::<Value>(&data) {
        return data;
    }

    if let Ok(data) = serde_yaml::from_str::<Value>(&data) {
        return data;
    }

    println!("\n\n");
    println!("Data: {data}");
    println!("\n\n");
    
    panic!("Problem with parsing the specification");
}

pub fn parse_spec(data: String) -> SpecOpenApi {
    let spec_raw = parse_file(data);

    let spec_raw = filter_deref(spec_raw);
    let spec_raw = filter_null(spec_raw);

    let spec = expect_json::<Spec>(&spec_raw);

    let mut paths: HashMap<String, HashMap<OpenApiMethod, SpecHandlerType>> = HashMap::new();

    for (path, path_body) in spec.paths {
        let mut path_methods: HashMap<OpenApiMethod, SpecHandlerType> = HashMap::new();

        
        for (method_name, method_body) in path_body {
            if let Some(method_body) = method_body {

                let method_name = OpenApiMethod::from_string(method_name).unwrap();
                let method_body = parse_handler(method_body, &spec_raw);

                path_methods.insert(method_name, method_body);
            }
        }

        paths.insert(path, path_methods);
    }

    SpecOpenApi {
        paths
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct HandlerSpec {
    parameters: Option<Vec<Value>>,
    #[serde(rename = "requestBody")]
    request_body: Option<Value>,
    responses: Option<HashMap<String, Value>>,      //200 -> typ, 300 -> typ
}

#[derive(Debug, Serialize, Deserialize)]
struct ParameterSpec {
    name: String,
    r#in: String,
    required: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RequestBody {
    required: Option<bool>,
}

pub fn parse_handler(body_raw: Value, spec: &Value) -> SpecHandlerType {

    let mut result = SpecHandlerType::new();

    let body = expect_json::<HandlerSpec>(&body_raw);

    if let Some(parameters) = body.parameters {
        for param in parameters {
            let param_decode = expect_json::<ParameterSpec>(&param);
            let param_type = parse_type(param, spec).unwrap();
            let required: bool = param_decode.required.unwrap_or(false);

            result.add_param(param_decode.name, param_decode.r#in, param_type, required).unwrap();
        }
    }

    if let Some(request_body) = body.request_body {
        let request_decode = expect_json::<RequestBody>(&request_body);
        let param_type = parse_type(request_body, spec).unwrap();
        let required: bool = request_decode.required.unwrap_or(false);

        result.add_param("requestBody", "body", param_type, required).unwrap();
    }

    if let Some(responses) = body.responses {
        for (code, code_response_spec) in responses {
            let code_response_type = parse_type(code_response_spec, spec).unwrap();
            result.add_response(code, code_response_type).unwrap();
        }
    }

    result
}
