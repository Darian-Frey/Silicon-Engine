pub struct SiliconApp {
    pub rom_path: String,
    pub system_name: String,
    pub rom_title: String,
    pub entropy_data: Vec<f32>,
    pub rom_size: i32,
    pub scroll_offset: u32,
    pub is_loaded: bool,
}

impl Default for SiliconApp {
    fn default() -> Self {
        Self {
            rom_path: "No ROM Loaded".to_owned(),
            system_name: "Unknown".to_owned(),
            rom_title: "---".to_owned(),
            entropy_data: vec![0.0; 128], // 128 vertical segments for the ribbon
            rom_size: 0,
            scroll_offset: 0,
            is_loaded: false,
        }
    }
}