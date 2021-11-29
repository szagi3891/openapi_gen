use crate::{generate_js::fix_to_camel_case::fix_to_camel_case, open_api_spec::{ParamIn, ParametersType, SpecHandlerType}, open_api_type::OpenApiType};
use super::generate_ident::generate_ident;

fn add_require(require: bool, type_param: impl Into<String>) -> String {
    let type_param: String = type_param.into();
    match require {
        true => type_param,
        false => format!("null | undefined | {type_param}")
    }
}

pub fn generate_type_ts(ident: u32, type_param: &OpenApiType) -> String {
    let right = '}';
    let next_ident = ident + 4;

    match type_param {
        OpenApiType::String { required } => add_require(*required, "string"),
        OpenApiType::Number { required } => add_require(*required, "number"),
        OpenApiType::Boolean { required } => add_require(*required, "boolean"),
        OpenApiType::Unknown => "unknown".into(),
        OpenApiType::Array { required, items } => {
            let items = generate_type_ts(ident, items);
            let result = format!("Array<{items}>");
            add_require(*required, result)
        },
        OpenApiType::Object { required, props } => {
            let mut out = Vec::<String>::new();

            out.push("{".into());

            for (key, value) in props.get_sorted() {
                let ident_str = generate_ident(next_ident);
                let value_std = generate_type_ts(next_ident, value);
                out.push(format!("{ident_str}{key}: {value_std},"));
            }

            let end_iden = generate_ident(ident);
            out.push(format!("{end_iden}{right}"));

            add_require(*required,out.join("\n".into()))
        },
        OpenApiType::Union { required, list } => {
            let mut result_types = Vec::<String>::new();

            for list_item in list {
                result_types.push(generate_type_ts(0, list_item));
            }

            let union_type = result_types.join("\n | \n");
            add_require(*required, union_type)
        }
        OpenApiType::Record { required, value } => {
            let inner_type = generate_type_ts(next_ident, value);
            let value_srt = format!("Record<string, {inner_type}>");
            add_require(*required, value_srt)
        }
    }
}

pub fn generate_params_type(spec: &SpecHandlerType) -> String {
    let left = '{';
    let right = '}';

    let mut out: Vec<String> = vec![format!("export interface ParamsType {left}")];

    let generate_str = |param: &ParametersType| -> String {
        let out1 = generate_ident(4);
        let out2 = fix_to_camel_case(&param.name);
        let out3 = generate_type_ts(4, &param.api_type);
        format!("{out1}{out2}: {out3},")
    };

    for param in spec.parameters.iter() {
        match param.where_in {
            ParamIn::Body => {
                out.push(generate_str(param));
            },
            ParamIn::Path => {
                out.push(generate_str(param));
            },
            ParamIn::Query => {
                out.push(generate_str(param));
            },
            ParamIn::Header => {},
        };
    }

    out.push(format!("{right}"));

    return out.join("\n".into());
}
