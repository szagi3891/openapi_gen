use std::{collections::HashMap, fs, path::PathBuf};
use crate::{ErrorProcess, open_api_spec::OpenApiMethod};


pub fn run_gen(dir_spec: String, dir_target: String, base_url: String, target_spec: String) -> Result<(), ErrorProcess> {

    println!("run gen");
    println!("{} {} {} {}", dir_spec, dir_target, base_url, target_spec);

    let wanted_spec = read_wanted_spec(dir_spec);

    //TODO - tutaj trzeba dalej przetworzyc te specyfikacje

    Ok(())
}

fn read_wanted_spec(dir_spec: String) -> Result<HashMap<String, WantedSpec>, ErrorProcess> {

    let mut out = HashMap::new();

    let dir = std::fs::read_dir(dir_spec)?;

    for entry in dir {
        let entry = entry?;

        let path = entry.path();
        
        if path.is_file() {
            let spec_name = parse_spec_name(&path)?;

            let content = fs::read_to_string(path)?;
            let wanted_spec = parse_wanted_spec(content)?;

            out.insert(spec_name, wanted_spec);
        } else {
            log::error!("I ignore reading is not a file: {path:?}");
        }
    }

    Ok(out)
}

fn parse_spec_name(path: &PathBuf) -> Result<String, ErrorProcess> {
    println!("Trzeba przetworzyć tą nazwe specyfikacji {:?}", path);
    todo!()
}

enum WantedSource {
    Url {
        url: String,
    },
    File {
        file: String,
    }
}

struct WantedMethod {
    method: OpenApiMethod,
    url: String,
}

struct WantedSpec {
    source: WantedSource,
    methods: HashMap<String, WantedMethod>,
}

fn parse_wanted_spec(content: String) -> Result<WantedSpec, ErrorProcess> {

    todo!()
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