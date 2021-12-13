use crate::utils::OrderHashMap;


#[derive(Debug, Clone)]
pub enum OpenApiType {
    LiteralString {
        value: String,
        required: bool,
    },
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
            Self::LiteralString { required: _required, value } => Self::LiteralString { required, value },
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

    pub fn object_try_add_literal_field(&mut self, name: impl Into<String>, value: impl Into<String>) {
        let name = name.into();
        let value = value.into();

        match self {
            Self::Object { props, .. } => {
                props.expect_insert(name, OpenApiType::LiteralString { required: true, value }).unwrap();
            },
            _ => {
                panic!("A new property can only be added to an object");
            }
        }
    }
}
