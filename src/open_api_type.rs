use crate::utils::OrderHashMap;


#[derive(Debug, Clone)]
pub enum OpenApiType {
    String {
        required: bool,
    },
    Number {
        required: bool,
    },
    Boolean {
        required: bool,
    },
    Array {
        required: bool,
        items: Box<OpenApiType>,
    },
    Object {
        required: bool,
        props: OrderHashMap<String, OpenApiType>,
    },
    Record {
        required: bool,
        value: Box<OpenApiType>,
    },
    Union {
        required: bool,
        list: Vec<OpenApiType>,
    },
    Unknown
}

impl OpenApiType {
    pub fn set_required(self, required: bool) -> OpenApiType {
        match self {
            Self::String { required: _required } => Self::String { required },
            Self::Number { required: _required } => Self::Number { required },
            Self::Boolean { required: _required } => Self::Boolean { required },
            Self::Array { required: _required, items } => Self::Array { required, items },
            Self::Object { required: _required, props } => Self::Object { required, props },
            Self::Record { required: _required, value } => Self::Record { required, value },
            Self::Union { required: _required, list } => Self::Union { required, list },
            Self::Unknown => Self::Unknown
        }
    }
}
