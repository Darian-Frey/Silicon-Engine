use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

#[link(name = "silicon_core", kind = "static")]
extern "C" {
    #[allow(dead_code)]
    pub fn get_core_version() -> c_int;
    pub fn identify_system(path: *const c_char) -> c_int;
    pub fn get_system_name(buffer: *mut c_char);
    pub fn get_rom_title(buffer: *mut c_char);
    pub fn calculate_entropy_map(buffer: *mut f32, chunks: c_int);
    pub fn get_rom_size() -> c_int;
    pub fn read_rom_range(offset: u32, length: u32, buffer: *mut u8);
    pub fn decode_sega_tile(offset: u32, buffer: *mut u8);
}

#[allow(dead_code)]
pub fn analyze_rom(path: &str) {
    let c_path = CString::new(path).unwrap();
    
    unsafe {
        let id = identify_system(c_path.as_ptr());
        
        let mut sys_buf = [0u8; 64];
        let mut title_buf = [0u8; 64];
        
        get_system_name(sys_buf.as_mut_ptr() as *mut c_char);
        get_rom_title(title_buf.as_mut_ptr() as *mut c_char);
        
        let system = CStr::from_ptr(sys_buf.as_ptr() as *const c_char).to_string_lossy();
        let title = CStr::from_ptr(title_buf.as_ptr() as *const c_char).to_string_lossy();
        
        println!("--- ANALYSIS RESULTS ---");
        println!("System ID: {}", id);
        println!("Hardware:  {}", system);
        println!("ROM Title: {}", title.trim());
        println!("------------------------");
    }
}

// New function to visualize entropy in the terminal
#[allow(dead_code)]
pub fn test_entropy_scan(path: &str) {
    let c_path = CString::new(path).unwrap();
    unsafe {
        identify_system(c_path.as_ptr());
        
        let chunks = 20; // Let's take 20 samples of the ROM
        let mut entropy_data = vec![0.0f32; chunks as usize];
        
        calculate_entropy_map(entropy_data.as_mut_ptr(), chunks);
        
        println!("--- ENTROPY SCAN (Top 20 Segments) ---");
        for (i, val) in entropy_data.iter().enumerate() {
            let bar_len = (val * 20.0) as usize;
            let bar = "█".repeat(bar_len);
            println!("Block {:02}: [{:<20}] {:.2}", i, bar, val);
        }
    }
}