use std::collections::HashMap;
use std::path::PathBuf;
use core::hash::Hash;
use std::fmt::Debug;

pub fn get_file_name(path: &PathBuf) -> Result<String, ErrorProcess> {
    let file_name = match path.file_name() {
        Some(file_name) => file_name,
        None => {
            return Err(ErrorProcess::message(format!("Problem with path processing {path:?}")));
        }
    };

    let file_name = match file_name.to_str() {
        Some(file_name) => file_name,
        None => {
            return Err(ErrorProcess::message(format!("Problem with path processing {path:?}")));
        }
    };
    
    Ok(file_name.into())
}


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

#[derive(Debug, Clone)]

pub struct OrderHashMap<K, V> {
    data: HashMap<K, V>,
}

impl<K: Eq + Hash + Clone + Debug + Ord, V> OrderHashMap<K, V> {
    pub fn new() -> OrderHashMap<K,V> {
        OrderHashMap {
            data: HashMap::new(),
        }
    }

    pub fn expect_insert(&mut self, k: K, v: V) -> Result<(), ErrorProcess> {
        let result = self.data.insert(k.clone(), v);


        if result.is_some() {
            return Err(ErrorProcess::message(format!("Duplicate key {k:?}")));
        }

        Ok(())
    }

    pub fn get_sorted<'a>(&'a self) -> Vec<(&'a K, &'a V)> {
        let mut result = Vec::<(&K, &V)>::new();

        for (key, value) in self.data.iter() {
            result.push((key, value));
        }

        use core::cmp::Ordering;

        result.sort_by(|(key1, _), (key2, _)| -> Ordering {
            let key1 = key1.clone();
            let key2 = key2.clone();

            key1.cmp(key2)
        });

        result
    }
}