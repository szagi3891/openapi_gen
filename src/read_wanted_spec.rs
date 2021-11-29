use std::{collections::{HashMap}, fs, path::PathBuf, str::FromStr};
use serde::{Deserialize, Serialize};

use crate::{open_api_spec::OpenApiMethod, utils::{get_file_name, ErrorProcess}};


pub fn read_wanted_spec(dir_spec: &String) -> Result<HashMap<String, WantedSpec>, ErrorProcess> {

    let mut out = HashMap::new();

    let dir = std::fs::read_dir(dir_spec)?;

    for entry in dir {
        let entry = entry?;

        let path = entry.path();
        
        if path.is_file() {
            if let Some(spec_name) = parse_spec_name(&path)? {

                let content = fs::read_to_string(path)?;
                let wanted_spec = parse_wanted_spec(content)?;

                out.insert(spec_name, wanted_spec);
                continue;
            }
        } else {
            log::error!("I ignore reading is not a file: {path:?}");
        }
    }

    Ok(out)
}

fn parse_spec_name(path: &PathBuf) -> Result<Option<String>, ErrorProcess> {
    let file_name = get_file_name(path)?;

    let mut file_name_chunks: Vec<&str> = file_name.split('.').collect();

    let const_json = file_name_chunks.pop();
    let const_spec = file_name_chunks.pop();
    
    if let (Some(const_spec), Some(const_json)) = (const_spec, const_json) {
        if const_spec == "spec" && const_json == "json" {
            let file_name = file_name_chunks.join(".");
            return Ok(Some(file_name));
        }
    }

    Ok(None)
}

#[test]
fn test_parse_spec_name() {
    let path = PathBuf::from_str("rrr/dddd/ttrree.spec.json").unwrap();
    let name = parse_spec_name(&path).unwrap().unwrap();

    assert_eq!(name, "ttrree");
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WantedSource {
    #[serde(rename = "url")]
    Url {
        url: String,
        fix_url_param: Option<Vec<FixUrlParamItem>>,
    },
    #[serde(rename = "file")]
    File {
        file: String,
        fix_url_param: Option<Vec<FixUrlParamItem>>,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WantedMethod {
    pub method: OpenApiMethod,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FixUrlParamItem {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WantedSpec {
    pub source: WantedSource,
    pub methods: HashMap<String, WantedMethod>,
}

fn parse_wanted_spec(content: String) -> Result<WantedSpec, ErrorProcess> {
    let spec = serde_json::from_str::<WantedSpec>(&content)?;
    Ok(spec)
}

/*
{
    "source": {
        "type": "url",
        "url": "/lottery-integration/meta/open-api"
    },
    "methods": {
        "getActualLottery": {
            "url": "/lottery/draws/current/{universe}/{accountId}",
            "method": "get"
        },
        "getPreviousDrawsLottery": {
            "url": "/lottery/draws/previous/{universe}",
            "method": "get"
        },
        "postLotteryNotificationsConfirmed": {
            "url": "/lottery/notifications/confirmed/{universe}/{accountId}",
            "method": "post"
        },
        "getLotteryNotifications": {
            "url": "/lottery/notifications/{universe}/{accountId}",
            "method": "get"
        },
        "postOrderLottery": {
            "url": "/lottery/orders/{universe}/{accountId}",
            "method": "post"
        },
        "getPendingLottery": {
            "url": "/lottery/tickets/pending/{universe}/{accountId}",
            "method": "get"
        },
        "getSettledLottery": {
            "url": "/lottery/tickets/settled/{universe}/{accountId}",
            "method": "get"
        }
    }
}
*/