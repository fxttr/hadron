fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-arg=-Tkernel/linker.ld");
    println!("cargo:rerun-if-changed=kernel/linker.ld");
}
