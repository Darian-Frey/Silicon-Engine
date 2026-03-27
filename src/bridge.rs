use std::os::raw::c_int;

#[link(name = "silicon_core", kind = "static")]
extern "C" {
    pub fn get_core_version() -> c_int;
    pub fn add_numbers(a: c_int, b: c_int) -> c_int;
}

pub fn check_handshake() {
    unsafe {
        let v = get_core_version();
        let sum = add_numbers(10, 32);
        println!("--- SILICON ENGINE HANDSHAKE ---");
        println!("Core Version: {}", v);
        println!("Math Test (10+32): {}", sum);
        println!("--------------------------------");
    }
}