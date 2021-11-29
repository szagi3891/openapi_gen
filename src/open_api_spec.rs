use std::{collections::HashMap, fmt::Display};
use crate::{open_api_type::OpenApiType, utils::{ErrorProcess, OrderHashMap}};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone)]
pub enum ParamIn {
    Path,
    Body,
    Query,
    Header,
}

#[derive(Debug, Clone)]
pub struct ParametersType {
    pub where_in: ParamIn,    //'path' | 'body' | 'query' | 'header',
    pub name: String,
    pub api_type: OpenApiType
}
#[derive(Debug, Clone)]
pub struct SpecHandlerType {                                    //TODO SpecHandlerType -> OpenApiHandler
    pub parameters: Vec<ParametersType>,
    pub responses: OrderHashMap<u16, OpenApiType>,
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub enum OpenApiMethod {
    #[serde(rename = "get")]
    Get,
    #[serde(rename = "post")]
    Post,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "put")]
    Put,
    #[serde(rename = "patch")]
    Patch
}


#[derive(Debug)]
pub struct SpecOpenApi {
    pub paths: HashMap<String, HashMap<OpenApiMethod, SpecHandlerType>>
}



impl Display for OpenApiMethod {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Get => "get",
            Self::Post => "post",
            Self::Delete => "delete",
            Self::Put => "put",
            Self::Patch => "path",
        };

        write!(fmt, "{name}")
    }
}

impl OpenApiMethod {
    pub fn from_string(name: String) -> Result<OpenApiMethod, ErrorProcess> {
        let name = name.to_lowercase();

        Ok(match name.as_str().as_ref() {
            "get" => OpenApiMethod::Get,
            "post" => OpenApiMethod::Post,
            "delete" => OpenApiMethod::Delete,
            "put" => OpenApiMethod::Put,
            "patch" => OpenApiMethod::Patch,
            _ => {
                return Err(ErrorProcess::message(format!("unknown method = {name}")));
            }
        })
    }

    pub fn to_upper_case(&self) -> String {
        format!("{}", self).to_uppercase()
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
                return Err(ErrorProcess::message(format!("unknown ParamIn = {name}")));
            }
        })
    }
}

impl SpecHandlerType {
    pub fn new() -> SpecHandlerType {
        SpecHandlerType {
            parameters: Vec::new(),
            responses: OrderHashMap::new(),
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
        self.responses.expect_insert(code, api_type)?;
        Ok(())
    }
}
