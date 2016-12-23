extern crate gcc;
extern crate libbindgen;

use std::env;
use std::path::Path;

fn main() {
    gcc::compile_library("libsmhasher.a",
                         &["src/smhasher/City.cpp",
                           "src/smhasher/farmhash-c.c",
                           // "src/smhasher/metrohash64.cpp",
                           // "src/smhasher/metrohash64crc.cpp",
                           // "src/smhasher/metrohash128.cpp",
                           // "src/smhasher/metrohash128crc.cpp",
                           "src/smhasher/MurmurHash1.cpp",
                           "src/smhasher/MurmurHash2.cpp",
                           "src/smhasher/MurmurHash3.cpp",
                           "src/smhasher/Spooky.cpp"]);

    let out_dir = env::var("OUT_DIR").unwrap();
    let _ = libbindgen::builder()
        .header("src/fasthash.hpp")
        .no_unstable_rust()
        .generate()
        .unwrap()
        .write_to_file(Path::new(&out_dir).join("fasthash.rs"))
        .expect("Couldn't write bindings!");
}