// ...new file...
fn main() {
    println!("cargo:rerun-if-changed=src/file.cpp");
    cc::Build::new()
        .cpp(true)
        .file("src/file.cpp")
        .compile("file_cpp");
}