// ...new file...
fn main() {
    println!("cargo:rerun-if-changed=src/FFI.cpp");
    cc::Build::new()
        .cpp(true)
        .file("src/FFI.cpp")
        .compile("FFI_cpp");
}