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
    // Note the added 'mut' here
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
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
                    // Low Entropy (Code/Tables) = Dark Blue/Cyan
                    // Mid Entropy (Tiles) = Green/Yellow
                    // High Entropy (Compressed) = Bright Orange/Red
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
            ui.vertical_centered(|ui| {
                ui.add_space(200.0);
                ui.heading("SILICON ENGINE // DATA STREAM ACTIVE");
                ui.label("Drop a .bin or .sms file to begin forensic analysis.");
            });
        });
    }
}