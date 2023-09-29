extern crate cmake;
use cmake::Config;

fn main() {
    //println!(r"cargo:rustc-link-search=../c++/build");
     let dst = Config::new("libverona")
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=verona");
    println!("cargo:rustc-link-lib=static=stdc++");
}
