fn main() {
    println!("cargo:rustc-link-lib=crypt");
    println!("cargo:rustc-link-lib=c");
}