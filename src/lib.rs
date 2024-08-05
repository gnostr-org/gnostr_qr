use sha2::{Digest, Sha256};
use std::env;
use std::str;

pub fn sha256_string(data: &str) -> Result<String, String> {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    let hex_result = hex::encode(result);
    Ok(hex_result.to_string())
}
pub fn help() {
    use std::process;
    let package_name = env!("CARGO_PKG_NAME");
    print!("{} <data>\n", package_name.replace("_", "-"));
    process::exit(0);
}
pub fn version() {
    use std::process;
    let version = env!("CARGO_PKG_VERSION");
    print!("v{}", version);
    process::exit(0);
}
pub fn default() {
    print!("\ndefault");
    help();
}

use image::Luma;
use qrcode::QrCode;
pub fn render(data: &str) {
    let code = QrCode::new(&data).unwrap();
    let hash = sha256_string(&data).unwrap();
    let image = code.render::<Luma<u8>>().build();
    let location = format!("{}.png", hash);
    image.save(location).unwrap();
    let string = code
        .render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();
    println!("{}", string); //prints blocks to terminal
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    ///cargo test -- --show-output

    #[test]
    fn test_gnostr_tagline() {
        let gnostr_tagline = "gnostr-qr:part of the git+nostr workflow utility";
        render(&gnostr_tagline);
        eprintln!("{:?}", sha256_string(&gnostr_tagline).unwrap());
    }
    #[test]
    fn test_gnostr_github() {
        let gnostr_github = "https://github.com/gnostr-org/gnostr.git";
        render(&gnostr_github);
        eprintln!("{:?}", sha256_string(&gnostr_github).unwrap());
    }
}
