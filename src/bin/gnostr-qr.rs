use gnostr_qr::*;

fn main() {
    use std::env;
    let _package_name = env!("CARGO_PKG_NAME");
    let _crate_name = env!("CARGO_CRATE_NAME");
    let _version = env!("CARGO_PKG_VERSION");
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if args.len() == 1 {
        for arg in &args {
            if arg == "-h" || arg == "--help" {}
            if arg == "-v" || arg == "--version" {}
            if arg == "-p" || arg == "--png" {}
            gnostr_qr::render(&arg);
            print!("{}\n", &arg);
        }
        std::process::exit(0);
    }
    if args.len() > 1 {
        for arg in &args {
            if arg == "-h" || arg == "--help" {}
            if arg == "-v" || arg == "--version" {}
            if arg == "-p" || arg == "--png" {}
            gnostr_qr::render(&arg);
            print!("{}\n", &arg);
        }
        std::process::exit(0);
    } else {
        help();
    }
}
