#![feature(format_args_capture)]

// use std::fs;



mod open_api_type;
mod open_api_spec;
mod error_process;
mod parse_spec;
mod parse_type;
mod run_gen;

pub use error_process::ErrorProcess;
pub use run_gen::run_gen;

pub fn process(dir_spec: String, dir_target: String, base_url: String, target_spec: String) {

    println!("basic ...");

    // let contents = fs::read_to_string("examples/cms.json").unwrap();
    // let contents = fs::read_to_string("examples/bonuses.json").unwrap();
    // let contents = fs::read_to_string("examples/wallet.json").unwrap();
    // let res = parse_spec::parse_spec(contents);

    run_gen::run_gen(dir_spec, dir_target, base_url, target_spec).unwrap();

    // println!("aaaaa ==> {:#?}", res);
}
