use std::collections::HashMap;
use crate::{error_process::ErrorProcess, open_api_type::OpenApiType};

#[derive(Debug)]
pub enum ParamIn {
    Path,
    Body,
    Query,
    Header,
}

#[derive(Debug)]
pub struct ParametersType {
    pub where_in: ParamIn,    //'path' | 'body' | 'query' | 'header',
    pub name: String,
    pub api_type: OpenApiType
}
#[derive(Debug)]
pub struct SpecHandlerType {                                    //TODO SpecHandlerType -> OpenApiHandler
    pub parameters: Vec<ParametersType>,
    pub responses: HashMap<u16, OpenApiType>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum OpenApiMethod {
    Get,
    Post,
    Delete,
    Put,
    Path
}

#[derive(Debug)]
pub struct SpecOpenApi {
    pub paths: HashMap<String, HashMap<OpenApiMethod, SpecHandlerType>>
}



impl OpenApiMethod {
    pub fn from_string(name: String) -> Result<OpenApiMethod, ErrorProcess> {
        let name = name.to_lowercase();

        Ok(match name.as_str().as_ref() {
            "get" => OpenApiMethod::Get,
            "post" => OpenApiMethod::Post,
            "delete" => OpenApiMethod::Delete,
            "put" => OpenApiMethod::Put,
            "path" => OpenApiMethod::Path,
            _ => {
                return Err(ErrorProcess::message("dsdsa"))
            }
        })
    }
}

impl ParamIn {
    pub fn from_string(name: String) -> Result<ParamIn, ErrorProcess> {
        let name = name.to_lowercase();

        Ok(match name.as_str().as_ref() {
            "path" => ParamIn::Path,
            "body" => ParamIn::Body,
            "query" => ParamIn::Query,
            "header" => ParamIn::Header,
            _ => {
                return Err(ErrorProcess::message("dsdsa"))
            }
        })
    }
}

impl SpecHandlerType {
    pub fn new() -> SpecHandlerType {
        SpecHandlerType {
            parameters: Vec::new(),
            responses: HashMap::new(),
        }
    }

    pub fn add_param(&mut self, name: impl Into<String>, where_in: impl Into<String>, api_type: OpenApiType, required: bool) -> Result<(), ErrorProcess> {
        let name: String = name.into();
        for item_name in self.parameters.iter() {
            if item_name.name == name {
                return Err(ErrorProcess::message(format!("duplicate parameter {name}")));
            }
        }

        let where_in: String = where_in.into();
        let where_in = ParamIn::from_string(where_in)?;

        self.parameters.push(ParametersType {
            name,
            where_in,
            api_type: api_type.set_required(required),
        });

        Ok(())
    }

    pub fn add_response(&mut self, code: String, api_type: OpenApiType) -> Result<(), ErrorProcess> {
        let code = code.parse::<u16>()?;
        let prev = self.responses.insert(code, api_type);

        if prev.is_some() {
            return Err(ErrorProcess::message(format!("Duplicate response codes {code}")));
        }

        Ok(())
    }
}
