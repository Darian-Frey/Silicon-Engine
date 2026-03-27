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
            let mut visuals = egui::Visuals::dark();
            visuals.panel_fill = egui::Color32::from_rgb(10, 10, 11);
            visuals.override_text_color = Some(egui::Color32::from_rgb(200, 200, 200));
            visuals.selection.bg_fill = egui::Color32::from_rgb(255, 69, 0);
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
                        unsafe {
                            use std::ffi::{CString, CStr};
                            use std::os::raw::c_char;

                            let c_path = CString::new(path_str).unwrap();
                            let _id = bridge::identify_system(c_path.as_ptr());
                            
                            let mut sys_buf = [0u8; 64];
                            let mut title_buf = [0u8; 64];
                            bridge::get_system_name(sys_buf.as_mut_ptr() as *mut c_char);
                            bridge::get_rom_title(title_buf.as_mut_ptr() as *mut c_char);
                            
                            self.system_name = CStr::from_ptr(sys_buf.as_ptr() as *const c_char).to_string_lossy().into_owned();
                            self.rom_title = CStr::from_ptr(title_buf.as_ptr() as *const c_char).to_string_lossy().trim().to_owned();
                            self.entropy_data = vec![0.0f32; 128];
                            bridge::calculate_entropy_map(self.entropy_data.as_mut_ptr(), 128);
                            self.rom_size = bridge::get_rom_size();
                            self.scroll_offset = 0;
                        }
                        self.rom_path = path_str.to_string();
                        self.is_loaded = true;
                    }
                }
            }

            // --- MOUSE WHEEL SCROLL LOGIC ---
            if self.is_loaded {
                let delta = i.smooth_scroll_delta.y;
                if delta != 0.0 {
                    let scroll_lines = (delta / 10.0) as i32;
                    let byte_delta = scroll_lines * 16;
                    let new_offset = self.scroll_offset as i32 - byte_delta;
                    let max_offset = (self.rom_size.max(512) - 512) as i32;
                    let clamped = new_offset.clamp(0, max_offset) as u32;
                    self.scroll_offset = clamped - (clamped % 16);
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
                    let r = (val * 255.0) as u8;
                    let g = ((1.0 - val) * 150.0 + 50.0) as u8;
                    let b = ((1.0 - val) * 255.0) as u8;
                    let color = egui::Color32::from_rgb(r, g, b);
                    let top = rect.top() + (i as f32 * segment_h);
                    painter.rect_filled(
                        egui::Rect::from_min_size(egui::pos2(rect.left(), top), egui::vec2(40.0, segment_h)),
                        0.0,
                        color
                    );
                }

                if self.rom_size > 0 {
                    let top_y = rect.top() + (self.scroll_offset as f32 / self.rom_size as f32) * rect.height();
                    let bottom_y = rect.top() + ((self.scroll_offset + 512) as f32 / self.rom_size as f32) * rect.height();
                    painter.rect_stroke(
                        egui::Rect::from_min_max(egui::pos2(rect.left(), top_y), egui::pos2(rect.right(), bottom_y)),
                        0.0,
                        egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 69, 0))
                    );
                }

                let response = ui.allocate_rect(rect, egui::Sense::click_and_drag());
                if response.clicked() || response.dragged() {
                    if let Some(pos) = response.interact_pointer_pos() {
                        let pct = ((pos.y - rect.top()) / rect.height()).clamp(0.0, 1.0);
                        let max_offset = (self.rom_size.max(512) - 512) as u32;
                        let target = (pct * self.rom_size as f32) as u32;
                        let clamped_target = target.min(max_offset);
                        self.scroll_offset = clamped_target - (clamped_target % 16);
                    }
                }
            });

        // --- RIGHT PANEL (Tile Scraper) ---
        if self.is_loaded {
            egui::SidePanel::right("scraper")
                .default_width(200.0)
                .show(ctx, |ui| {
                    ui.add_space(10.0);
                    ui.colored_label(egui::Color32::from_rgb(255, 69, 0), " [ TILE PREVIEW ] ");
                    ui.separator();

                    let mut tile_pixels = vec![0u8; 8 * 8 * 4];
                    unsafe {
                        bridge::decode_sega_tile(self.scroll_offset, tile_pixels.as_mut_ptr());
                    }

                    let color_image = egui::ColorImage::from_rgba_unmultiplied([8, 8], &tile_pixels);
                    let texture = ctx.load_texture("tile_preview", color_image, Default::default());

                    ui.vertical_centered(|ui| {
                        ui.add_space(10.0);
                        ui.add(egui::Image::from_texture(&texture).fit_to_exact_size(egui::vec2(160.0, 160.0)));
                        ui.add_space(10.0);
                        ui.label(format!("Offset: {:08X}", self.scroll_offset));
                        ui.label("Format: Sega 4bpp");
                        ui.add_space(20.0);
                        ui.separator();
                        ui.add_space(10.0);
                        ui.label("Patterns found in higher entropy blocks often represent graphical assets.");
                    });
                });
        }

        // --- CENTRAL AREA ---
        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.is_loaded {
                ui.vertical_centered(|ui| {
                    ui.add_space(ctx.available_rect().height() / 3.0);
                    ui.heading("SILICON ENGINE // DATA STREAM INACTIVE");
                    ui.label("Drop a .bin or .sms file to begin forensic analysis.");
                });
            } else {
                ui.horizontal(|ui| {
                    ui.colored_label(egui::Color32::from_rgb(0, 255, 255), "OFFSETS");
                    ui.separator();
                    ui.label(format!("SIZE: {} KB", self.rom_size / 1024));
                    ui.label(format!("| OFFSET: {:08X}", self.scroll_offset));
                });
                ui.add_space(10.0);

                ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
                let line_count = 32;
                let bytes_per_line = 16;
                let mut buffer = vec![0u8; (line_count * bytes_per_line) as usize];
                
                unsafe {
                    bridge::read_rom_range(self.scroll_offset, buffer.len() as u32, buffer.as_mut_ptr());
                }

                egui::Grid::new("hex_grid").striped(true).spacing([10.0, 4.0]).show(ui, |ui| {
                    for line in 0..line_count {
                        let line_offset = self.scroll_offset + (line * bytes_per_line) as u32;
                        ui.colored_label(egui::Color32::from_rgb(0, 255, 255), format!("{:08X}", line_offset));

                        let mut hex_str = String::new();
                        let mut ascii_str = String::new();
                        for i in 0..bytes_per_line {
                            let b = buffer[(line * bytes_per_line + i) as usize];
                            hex_str.push_str(&format!("{:02X} ", b));
                            ascii_str.push(if b >= 32 && b <= 126 { b as char } else { '.' });
                        }
                        ui.label(hex_str);
                        ui.separator();
                        ui.colored_label(egui::Color32::from_rgb(150, 150, 150), ascii_str);
                        ui.end_row();
                    }
                });
            }
        });
    }
}