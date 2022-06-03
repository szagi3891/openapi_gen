use std::env;

mod utils;
mod open_api_type;
mod open_api_spec;
mod parse_spec;
mod run_gen;
mod read_wanted_spec;
mod generate_js;

use utils::ErrorProcess;


#[tokio::main]
async fn main() -> Result<(), ErrorProcess> {
    log::set_max_level(log::LevelFilter::Warn);
    env_logger::init();
    log::set_max_level(log::LevelFilter::Warn);

    let mut args= env::args();
    
    let _ = args.next();
    let dir_spec = args.next();
    let dir_target = args.next();
    let base_url = args.next();
    let target_spec = args.next();

    if let (Some(dir_spec), Some(dir_target), Some(base_url), Some(target_spec)) = (dir_spec, dir_target, base_url, target_spec) {
        run_gen::run_gen(dir_spec, dir_target, base_url, target_spec).await?;
    } else {
        return Err(ErrorProcess::message("Incorrect parameters"));
    }

    Ok(())
}
