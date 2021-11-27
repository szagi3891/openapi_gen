#![feature(format_args_capture)]

use std::env;
use openapi_gen::process;

fn main(){
    println!("odpalam main openapi");

    let mut args= env::args();
    
    let _ = args.next();
    let dir_spec = args.next();
    let dir_target = args.next();
    let base_url = args.next();
    let target_spec = args.next();

    if let (Some(dir_spec), Some(dir_target), Some(base_url), Some(target_spec)) = (dir_spec, dir_target, base_url, target_spec) {
        process(dir_spec, dir_target, base_url, target_spec);
    } else {
        panic!("Incorrect parameters");
    }
}
