use std::num::ParseIntError;

#[derive(Debug)]
pub enum ErrorProcess {
    Message(String),
    DeserializeError(serde_json::Error),
    ParseError(ParseIntError),
    StdError(std::io::Error),
}

impl From<serde_json::Error> for ErrorProcess {
    fn from(err: serde_json::Error) -> Self {
        ErrorProcess::DeserializeError(err)
    }
}

impl From<ParseIntError> for ErrorProcess {
    fn from(err: ParseIntError) -> Self {
        ErrorProcess::ParseError(err)
    }
}

impl From<std::io::Error> for ErrorProcess {
    fn from(err: std::io::Error) -> Self {
        ErrorProcess::StdError(err)
    }
}

impl ErrorProcess {
    pub fn message(message: impl Into<String>) -> ErrorProcess {
        let message: String = message.into();
        ErrorProcess::Message(message)
    }
}
