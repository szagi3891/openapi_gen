#![feature(format_args_capture)]

use std::fs;



mod open_api_type;
mod open_api_spec;
mod error_process;
mod parse_spec;
mod parse_type;

pub fn parse22() {

    println!("basic ...");

    let contents = fs::read_to_string("examples/wallet.json").unwrap();
    let res = parse_spec::parse_spec(contents);

    println!("aaaaa ==> {:#?}", res);
}
