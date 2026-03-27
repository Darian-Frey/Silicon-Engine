fn main() {
    cc::Build::new()
        .cpp(true)
        .file("core/src/engine.cpp")
        .include("core/include")
        .compile("silicon_core");

    println!("cargo:rerun-if-changed=core/src/engine.cpp");
    println!("cargo:rerun-if-changed=core/include/silicon_core.h");
}