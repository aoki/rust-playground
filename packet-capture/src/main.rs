use std::env;
use log::error;
use pnet::datalink;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let args: Vec<String> =env::args().collect();
    if args.len() != 2 {
        error!("Please specify target interface name");
        std::process::exit(1);
    }
    let interface_name = &args[1];
    let interfaces = datalink::interfaces();
}
