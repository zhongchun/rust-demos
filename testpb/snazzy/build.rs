use std::env;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let mut config = prost_build::Config::new();
    config
        // `OUT_DIR` is set by cargo when executing build scripts, so
        // `out_dir` typically does not need to be configured.
        // .out_dir("src/generated")
        .out_dir(out_dir)
        .compile_protos(&["src/proto/items.proto"], &["src/proto/"])
        .unwrap();
}