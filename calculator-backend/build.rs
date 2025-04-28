fn main() {
    println!("cargo:rerun-if-changed=c_library/calculator.c");
    
    cc::Build::new()
        .file("c_library/calculator.c")
        .compile("calculator");
} 