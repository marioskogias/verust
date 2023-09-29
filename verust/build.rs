fn main() {
    println!(r"cargo:rustc-link-search=../c++/build");
    println!("cargo:rustc-link-lib=static=stdc++");
}
