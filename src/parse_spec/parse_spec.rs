use super::parse_type::parse_type;
use serde_json::Value;
use serde::{Serialize, Deserialize};
// use serde::Serialize;
use std::collections::HashMap;

use crate::utils::ErrorProcess;
use crate::open_api_spec::{SpecHandlerType, OpenApiMethod, SpecOpenApi};


#[derive(Debug, Serialize, Deserialize)]
struct Spec {
    paths: HashMap<String, HashMap<String, Option<Value>>>,
}


pub fn parse_spec(data: String) -> Result<SpecOpenApi, ErrorProcess> {
    let spec_raw = match serde_json::from_str::<serde_json::Value>(&data) {
        Ok(data) => data,
        Err(err) => {
            println!("\n\n");
            println!("Data: {data}");
            println!("\n\n");

            return Err(ErrorProcess::message(format!("Problem with decoding {err}")));
        }
    };


    let spec = serde_json::from_value::<Spec>(spec_raw.clone())?;

    let mut paths: HashMap<String, HashMap<OpenApiMethod, SpecHandlerType>> = HashMap::new();

    for (path, path_body) in spec.paths {
        let mut path_methods: HashMap<OpenApiMethod, SpecHandlerType> = HashMap::new();

        
        for (method_name, method_body) in path_body {
            if let Some(method_body) = method_body {
                let method_name = OpenApiMethod::from_string(method_name)?;
                let method_body = parse_handler(method_body, &spec_raw)?;

                path_methods.insert(method_name, method_body);
            }
        }

        paths.insert(path, path_methods);
    }

    Ok(SpecOpenApi {
        paths
    })
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

pub fn parse_handler(body_raw: Value, spec: &Value) -> Result<SpecHandlerType, ErrorProcess> {

    let mut result = SpecHandlerType::new();

    let body = serde_json::from_value::<HandlerSpec>(body_raw.clone())?;

    if let Some(parameters) = body.parameters {
        for param in parameters {
            let param_decode = serde_json::from_value::<ParameterSpec>(param.clone())?;
            let param_type = parse_type(param, spec)?;
            let required: bool = param_decode.required.unwrap_or(false);

            result.add_param(param_decode.name, param_decode.r#in, param_type, required)?;
        }
    }

    if let Some(request_body) = body.request_body {
        let request_decode = serde_json::from_value::<RequestBody>(request_body.clone())?;
        let param_type = parse_type(request_body, spec)?;
        let required: bool = request_decode.required.unwrap_or(false);

        result.add_param("requestBody", "body", param_type, required)?;
    }

    if let Some(responses) = body.responses {
        for (code, code_response_spec) in responses {
            let code_response_type = parse_type(code_response_spec, spec)?;
            result.add_response(code, code_response_type)?;
        }
    }

    Ok(result)
}
