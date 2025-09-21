fn main() {
    // Rebuild if the native source changes
    println!("cargo:rerun-if-changed=src_lib/lib.cpp");

    // Tell rustc where to find the native libraries
    println!("cargo:rustc-link-search=native=src_lib");

    // Link libadd.so (libadd.so -> name "add")
    println!("cargo:rustc-link-lib=dylib=add");
}