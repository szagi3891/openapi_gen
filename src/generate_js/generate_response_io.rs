use crate::generate_js::generate_ident::generate_ident;
use crate::open_api_type::OpenApiType;
use crate::open_api_spec::{OpenApiMethod, SpecHandlerType};
use super::generate_params_type::generate_type_ts;

fn add_require(require: bool, type_param: impl Into<String>) -> String {
    let type_param = type_param.into();
    match require {
        true => type_param,
        false => format!("t.union([t.null, t.undefined, {type_param}])")
    }
}

fn generate_type_io(ident: u32, type_param: &OpenApiType) -> String {
    let left = '{';
    let right = '}';

    match type_param {
        OpenApiType::String { required } => add_require(*required, "t.string"),
        OpenApiType::Number { required } => add_require(*required, "t.number"),
        OpenApiType::Boolean { required } => add_require(*required, "t.boolean"),
        OpenApiType::Unknown => "t.unknown".into(),
        OpenApiType::Array { required, items } => {
            let items = generate_type_io(ident, items);
            let result = format!("t.array({items})");
            add_require(*required, result)
        },
        OpenApiType::Object { required, props } => {
            let next_ident = ident + 4;
            let mut out = Vec::<String>::new();

            out.push(format!("t.interface({left}"));

            for (key, value) in props.get_sorted() {
                let ident_str = generate_ident(next_ident);
                let value_std = generate_type_io(next_ident, value);
                out.push(format!("{ident_str}{key}: {value_std},"));
            }

            let end_iden = generate_ident(ident);
            out.push(format!("{end_iden}{right})"));

            add_require(*required, out.join("\n".into()))
        },
        OpenApiType::Union { required, list } => {
            let mut result_types = Vec::<String>::new();

            for list_item in list {
                result_types.push(generate_type_io(0, list_item));
            }

            let result_str = result_types.join(", ".into());
            let union_type = format!("t.union([{result_str}])");
            add_require(*required, union_type)
        }
        OpenApiType::Record { required, value } => {
            let inner_type = generate_type_io(ident, value);
            let value_srt = format!("t.record(t.string, {inner_type})");
            add_require(*required, value_srt)
        }
    }
}


pub fn generate_response_io(spec: &SpecHandlerType, url: &String, method: &OpenApiMethod) -> String {
    let left = '{';
    let right = '}';
    let mut out = Vec::<String>::new();

    for (code, response) in spec.responses.get_sorted() {
        let type_io = generate_type_io(0, response);
        let type_ts = generate_type_ts(0, response);

        out.push(format!("const Response{code}IO = {type_io};"));
        out.push("".into());
        out.push(format!("export type Response{code}Type = {type_ts};"));
        out.push("".into());
        out.push(format!("export const decodeResponse{code} = (data: unknown): Response{code}Type => {left}"));
        out.push(format!("    const decodeResult = Response{code}IO.decode(data);"));
        out.push(format!("    if (isRight(decodeResult)) {left}"));
        out.push(format!("        return decodeResult.right;"));
        out.push(format!("    {right}"));
        out.push(format!("    throw Error('Response decoding error {url} -> {method} -> {code}');"));
        out.push(format!("{right};"));
        out.push("".into());
        out.push("".into());
        out.push("".into());
        out.push("".into());
    }

    out.join("\n")
}
