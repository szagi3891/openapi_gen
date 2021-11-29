use std::collections::HashMap;
use crate::{open_api_spec::{OpenApiMethod, ParamIn, ParametersType, SpecHandlerType, SpecOpenApi}, open_api_type::OpenApiType, read_wanted_spec::WantedMethod};

pub fn fix_url_param(spec: &mut SpecOpenApi, methods: &mut HashMap<String, WantedMethod>, param_from: String, param_to: String) {
    spec.paths = correc_spec_paths(&spec.paths, &param_from, &param_to);

    for (_, wanted_method) in methods.iter_mut() {
        wanted_method.url = fix_url(&wanted_method.url, &param_from, &param_to);
    }
}

fn fix_url(url: &String, param_from: &String, param_to: &String) -> String {
    url
        .split("/")
        .map(move |item| {
            if param_from == item {
                let left = '{';
                let right = '}';
                return format!("{left}{param_to}{right}");
            }

            item.to_string()
        })
        .collect::<Vec<String>>()
        .join("/")
}

fn correct_handlers(
    handlers: &HashMap<OpenApiMethod, SpecHandlerType>,
    param_to: &String
) -> HashMap<OpenApiMethod, SpecHandlerType> {

    let mut handlers = handlers.clone();

    for (_, handler) in handlers.iter_mut() {
        handler.parameters.push(ParametersType {
            where_in: ParamIn::Path,
            name: param_to.clone(),
            api_type: OpenApiType::String { required: true },
        })
    }

    handlers
}

fn correc_spec_paths (
    paths: &HashMap<String, HashMap<OpenApiMethod, SpecHandlerType>>,
    param_from: &String,
    param_to: &String
) -> HashMap<String, HashMap<OpenApiMethod, SpecHandlerType>> {

    let mut out = HashMap::new();

    for (path, body) in paths.into_iter() {
        let new_path = fix_url(&path, param_from, param_to);

        if new_path == *path {
            out.insert(path.clone(), body.clone());
        } else {
            out.insert(new_path, correct_handlers(body, param_to));
        }
    }

    out
}


#[test]
fn test_fix_url() {
    let url = "/fixed/session/register";
    assert_eq!(
        fix_url(&url.to_string(), &"fixed".to_string(), &"rrrrrr".to_string()),
        "/{rrrrrr}/session/register".to_string()
    );
}