use std::env;

fn main() {
    let shirt = snazzy::create_large_shirt("White".to_string());
    println!("{:?}", shirt);

    let bytes: Vec<u8> = snazzy::serialize_shirt(&shirt);
    println!("{:?}", bytes);

    let shirt = snazzy::deserialize_shirt(&bytes);
    println!("{:?}", shirt);

    let out_dir = env::var_os("OUT_DIR").unwrap();
    println!("out_dir: {:?}", out_dir);
}
