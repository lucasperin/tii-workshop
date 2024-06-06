use std::env;
use std::path::PathBuf;

// Custom build
fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let gen = cbindgen::generate(crate_dir);
    if gen.is_ok() {
        gen.unwrap()
            .write_to_file(out_path.join("src/api/crypto_api.h"));
    } else {
        println!("CBINDGEN Error: {:?},", Some(gen.err()));
        panic!("Bindgen error");
    }
}
