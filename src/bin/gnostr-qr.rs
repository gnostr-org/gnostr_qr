use gnostr_qr::*;

fn main() {
    use std::env;
    let package_name = env!("CARGO_PKG_NAME");
    let crate_name = env!("CARGO_CRATE_NAME");
    let version = env!("CARGO_PKG_VERSION");
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
        let render_help = r"gnostr-qr <data>";
        //render(&render_help);
        help();
    }
}
