use std::collections::{VecDeque};

use crate::generate_js::fix_to_camel_case::fix_to_camel_case;
use crate::open_api_spec::{OpenApiMethod, ParamIn, SpecHandlerType, ParametersType};
use crate::utils::ErrorProcess;
use crate::utils::OrderHashMap;
use crate::open_api_type::OpenApiType;

use self::generate_params_type::generate_type_ts;

mod generate_ident;
mod generate_params_type;
mod generate_response_io;
mod fix_to_camel_case;

fn add_import_query_string(spec: &SpecHandlerType) -> &str {
    for param in spec.parameters.iter() {
        if param.where_in == ParamIn::Query {
            return "import qs from 'query-string';";
        }
    }

    return "";
}

fn word_first_letter_to_lowercase(word: &str) -> String {
    let mut chars = word.chars().collect::<Vec<char>>();
    if let Some(char) = chars.get_mut(0) {
        *char = char.to_lowercase().nth(0).unwrap();
    }

    chars.iter().collect::<String>()
}

#[test]
fn test_word_first_letter_to_lowercase() {
    assert_eq!(word_first_letter_to_lowercase("Openapi"), "openapi");
    assert_eq!(word_first_letter_to_lowercase("O"), "o");
    assert_eq!(word_first_letter_to_lowercase(""), "");
}

fn word_first_letter_to_upper(word: &str) -> String {
    let mut chars = word.chars().collect::<Vec<char>>();
    if let Some(char) = chars.get_mut(0) {
        *char = char.to_uppercase().nth(0).unwrap();
    }

    chars.iter().collect::<String>()
}

#[test]
fn test_word_first_letter_to_upper() {
    assert_eq!(word_first_letter_to_upper("openapi"), "Openapi");
    assert_eq!(word_first_letter_to_upper("o"), "O");
    assert_eq!(word_first_letter_to_upper(""), "");
}

fn to_big_camel_case(name: &str) -> String {
    name.split("_")
        .map(|word| word_first_letter_to_upper(word))
        .collect::<String>()
}

#[test]
fn test_to_big_camel_case() {
    assert_eq!(to_big_camel_case("openapi_socket_get_market"), "OpenapiSocketGetMarket");
}

/*
{
    status: 200,
    body: Response200Type,
} | {
    status: 300,
    body: Response300Type,
} | {
    status: 400,
    body: Response400Type,
} 
*/

fn generate_generic_response(responses: &OrderHashMap<u16, OpenApiType>) -> (String, String) {
    let left = '{';
    let right = '}';

    let mut param_chunks = Vec::<String>::new();
    let mut if_chunks = Vec::<String>::new();

    for (code, _) in responses.get_sorted() {
        param_chunks.push(format!(r#"{left}
    status: {code},
    body: Response{code}Type,
{right}"#));

        if_chunks.push(format!(r#"
    if (status === {code}) {left}
        return {left}
            status: {code},
            body: decodeResponse{code}(bodyParsed.json)
        {right};
    {right}"#));
    }

    (
        param_chunks.join(" | "),
        if_chunks.join("\n\n")
    )
}

fn generate_headers_type(parameters: &Vec<ParametersType>) -> String {
    let left = '{';
    let right = '}';

    let mut headers = Vec::new();

    for param in parameters {
        if param.where_in == ParamIn::Header {
            let name = &param.name;
            let param_type = generate_type_ts(4, &param.api_type);
            if name.contains("-") {
                headers.push(format!("    '{name}': {param_type},"));
            } else {
                headers.push(format!("    {name}: {param_type},"));
            }
        }
    }

    if headers.len() > 0 {
        let headers_join = headers.join("\n");
        return format!("type ExtraHeadersType = {left}\n{headers_join}\n{right};\n");
    }

    String::from("type ExtraHeadersType = Record<string, string>")
}

fn generate_headers_param(parameters: &Vec<ParametersType>) -> String {

    let mut counter = 0;

    for param in parameters {
        if param.where_in == ParamIn::Header {
            counter += 1;
        }
    }
    if counter > 0 {
        return String::from("extraHeaders: ExtraHeadersType");
    }

    String::from("extraHeaders?: ExtraHeadersType")
}

pub fn generate_js(name_in_file: String, url: String, method: OpenApiMethod, handler: &SpecHandlerType) -> Result<String, ErrorProcess> {
    let left = '{';
    let right = '}';

    let import_query_string = add_import_query_string(handler);
    let (generate_params_name, generate_params_type) = generate_params_type::generate_params_type(handler);
    let generate_response_io_data = generate_response_io::generate_response_io(handler, &url, &method);
    let generate_url = generate_url(url, handler);
    let generate_method = get_method(&method);
    let generate_body = get_body(handler);
    let extra_headers_type = generate_headers_type(&handler.parameters);
    let extra_headers = generate_headers_param(&handler.parameters);

    let name_in_file_camelcase_big = to_big_camel_case(name_in_file.as_str());
    let name_in_file_camelcase_small = word_first_letter_to_lowercase(name_in_file_camelcase_big.as_str());
    let (generic_response_types, generic_response_ifs) = generate_generic_response(&handler.responses);

    let content = format!(r#"//The contents of this file have been generated automatically. Do not edit this file.

import * as t from 'io-ts';
import {left} isRight {right} from 'fp-ts/lib/Either';
import {left} fetchGeneralRaw, FetchGeneralRawResponseType {right} from 'src_common/common/fetch';
import {left} ApiTimeLog {right} from 'src_common/server/webDriver/logFormat';
{import_query_string}


{generate_params_type}


{generate_response_io_data}


{extra_headers_type}


/**
 * @deprecated - Please use this method "{name_in_file_camelcase_small}Request"
 */
export const {name_in_file} = async (api_url: string, api_timeout: number, backendToken: string, {generate_params_name}, {extra_headers}): Promise<FetchGeneralRawResponseType> => {left}
    const url = `${left}api_url{right}{generate_url}`;
    const method = {generate_method};
    const paramsFetch = {left}
        url,
        body: {generate_body},
        backendToken,
        timeout: api_timeout,
        extraHeaders,
    {right};

    const apiTime = ApiTimeLog.createWithProcessEnv(method, url);
    const response = await fetchGeneralRaw(method, paramsFetch);
    apiTime.show(response.status);
    return response;
{right};


export type {name_in_file_camelcase_big}ParamsType = ParamsType;

export type {name_in_file_camelcase_big}ResponseType = {generic_response_types};

export type {name_in_file_camelcase_big}Response200Type = Response200Type;

export const {name_in_file_camelcase_small}Request = async (api_url: string, api_timeout: number, backendToken: string, {generate_params_name}, {extra_headers}): Promise<{name_in_file_camelcase_big}ResponseType> => {left}
    const response = await {name_in_file}(api_url, api_timeout, backendToken, params, extraHeaders);
    const {left} status, body {right} = response;

    const parse = (body: string): {left}
        type: 'json',
        json: unknown,
    {right} | {left}
        type: 'text'
    {right} => {left}
        try {left}
            return {left}
                type: 'json',
                json: JSON.parse(body)
            {right};
        {right} catch (_err) {left}
            return {left}
                type: 'json',
                json: {left}{right},
            {right};
        {right}
    {right};
    const bodyParsed = parse(body);

    if (bodyParsed.type === 'text') {left}
        throw Error(`Http status ${left}status{right} - json was expected`);
    {right}

    {generic_response_ifs}

    throw new Error(`{name_in_file_camelcase_small}Request - unhandled response ${left}response.status{right}`);
{right};

"#);

    Ok(content)
}

fn get_method(method: &OpenApiMethod) -> String {
    let method = method.to_upper_case();
    format!("'{method}'")
}

fn generate_url(url: String, spec: &SpecHandlerType) -> String {
    /*
        convert url:
        /website-cms/{universe}/landing/landing_promo_page
        /website-cms/${params.universe}/landing/landing_promo_page
    */

    let base_url = url.split('/').map(generate_url_item).collect::<Vec<String>>().join("/");

    let mut query_params = Vec::<&String>::new();

    for param in spec.parameters.iter() {
        if param.where_in == ParamIn::Query {
            query_params.push(&param.name);
        }
    }

    if query_params.len() > 0 {
        let query = generate_url_query(query_params);
        return format!("{base_url}?{query}");

    } else {
        return base_url;
    }
}

fn generate_url_query(query_params: Vec<&String>) -> String {
    let left = '{';
    let right = '}';

    let mut param_chunks = Vec::<String>::new();

    for param_name in query_params {
        let param_name_camel_case = fix_to_camel_case(&param_name);
        param_chunks.push(format!("'{param_name}': params.{param_name_camel_case}"));
    }

    let param_result = param_chunks.join(", ");

    format!("${left}qs.stringify({left} {param_result} {right}, {left} skipNull: true {right}){right}")
}

fn generate_url_item(url_chunk: &str) -> String {
    /*
    convert chunk:
    {universe}
    ${params.universe}
    */

    let left = '{';
    let right = '}';

    let mut chars = url_chunk.chars().collect::<VecDeque<_>>();

    let first = chars.pop_front();
    let last = chars.pop_back();

    if first == Some('{') && last == Some('}') {
        let inner: String = chars.iter().collect();
        let inner = fix_to_camel_case(&inner);

        return format!("${left}params.{inner}{right}");
    }

    return url_chunk.to_string();
}

fn get_body(spec: &SpecHandlerType) -> String {
    for param in spec.parameters.iter() {
        if param.where_in == ParamIn::Body {
            let name = &param.name;
            return format!("params.{name}");
        }
    }

    "undefined".into()
}

