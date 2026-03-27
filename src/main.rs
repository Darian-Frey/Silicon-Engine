mod bridge;
mod state;

use eframe::egui;
use state::SiliconApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("SILICON ENGINE // FORENSIC UNIT"),
        ..Default::default()
    };

    eframe::run_native(
        "Silicon Engine",
        options,
        Box::new(|_cc| {
            // Setup the "Cyberpunk" visuals
            let mut visuals = egui::Visuals::dark();
            visuals.panel_fill = egui::Color32::from_rgb(10, 10, 11); // Primary Background
            visuals.override_text_color = Some(egui::Color32::from_rgb(200, 200, 200));
            visuals.selection.bg_fill = egui::Color32::from_rgb(255, 69, 0); // Safety Orange
            _cc.egui_ctx.set_visuals(visuals);

            Box::new(SiliconApp::default())
        }),
    )
}

impl eframe::App for SiliconApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        // --- FILE DROP LOGIC ---
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                if let Some(file) = i.raw.dropped_files.first() {
                    if let Some(path) = &file.path {
                        let path_str = path.to_str().unwrap_or("");
                        
                        // 1. Trigger the C++ Bridge
                        unsafe {
                            use std::ffi::{CString, CStr};
                            use std::os::raw::c_char;

                            let c_path = CString::new(path_str).unwrap();
                            let _id = bridge::identify_system(c_path.as_ptr());
                            
                            // 2. Fetch Metadata from Core
                            let mut sys_buf = [0u8; 64];
                            let mut title_buf = [0u8; 64];
                            bridge::get_system_name(sys_buf.as_mut_ptr() as *mut c_char);
                            bridge::get_rom_title(title_buf.as_mut_ptr() as *mut c_char);
                            
                            // Corrected String Handling
                            self.system_name = CStr::from_ptr(sys_buf.as_ptr() as *const c_char)
                                .to_string_lossy()
                                .into_owned();
                            
                            self.rom_title = CStr::from_ptr(title_buf.as_ptr() as *const c_char)
                                .to_string_lossy()
                                .trim()
                                .to_owned();

                            // 3. Recalculate Entropy (128 segments for the ribbon)
                            self.entropy_data = vec![0.0f32; 128];
                            bridge::calculate_entropy_map(self.entropy_data.as_mut_ptr(), 128);
                        }
                        
                        self.rom_path = path_str.to_string();
                        self.is_loaded = true;
                    }
                }
            }
        });
        
        // --- TOP PANEL (Status Bar) ---
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.colored_label(egui::Color32::from_rgb(0, 255, 255), " [ SYSTEM ] ");
                ui.label(&self.system_name);
                ui.separator();
                ui.colored_label(egui::Color32::from_rgb(255, 69, 0), " [ ROM ] ");
                ui.label(&self.rom_title);
            });
            ui.add_space(5.0);
        });

        // --- LEFT PANEL (The Entropy Ribbon) ---
        egui::SidePanel::left("ribbon")
            .resizable(false)
            .default_width(40.0)
            .show(ctx, |ui| {
                let rect = ui.available_rect_before_wrap();
                let height = rect.height();
                let chunks = self.entropy_data.len();
                let segment_h = height / chunks as f32;

                let painter = ui.painter();
                for (i, &val) in self.entropy_data.iter().enumerate() {
                    // Forensic Color Mapping: 
                    // Low Entropy (Code) = Cyan/Blue
                    // Mid Entropy (Tiles) = Green/Yellow
                    // High Entropy (Compressed) = Orange/Red
                    let r = (val * 255.0) as u8;
                    let g = ((1.0 - val) * 150.0 + 50.0) as u8;
                    let b = ((1.0 - val) * 255.0) as u8;
                    
                    let color = egui::Color32::from_rgb(r, g, b);
                    
                    let top = rect.top() + (i as f32 * segment_h);
                    painter.rect_filled(
                        egui::Rect::from_min_size(
                            egui::pos2(rect.left(), top),
                            egui::vec2(40.0, segment_h)
                        ),
                        0.0,
                        color
                    );
                }
            });

        // --- CENTRAL AREA ---
        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.is_loaded {
                ui.vertical_centered(|ui| {
                    ui.add_space(ctx.available_rect().height() / 3.0);
                    ui.heading("SILICON ENGINE // DATA STREAM INACTIVE");
                    ui.label("Drop a .bin or .sms file to begin forensic analysis.");
                });
            } else {
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.colored_label(egui::Color32::from_rgb(0, 255, 255), "SOURCE PATH:");
                    ui.label(&self.rom_path);
                });
                ui.separator();
                
                ui.add_space(20.0);
                ui.heading("FORENSIC ANALYSIS COMPLETE.");
                ui.add_space(10.0);
                ui.label("• The Entropy Ribbon (Left) visualized the binary landscape.");
                ui.label("• Cyan/Blue segments represent structured code or jump tables.");
                ui.label("• Green/Yellow segments indicate graphic tile patterns.");
                ui.label("• Red segments highlight high-entropy compressed or encrypted data.");
            }
        });
    }
}