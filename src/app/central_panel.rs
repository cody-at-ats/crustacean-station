use crate::app::sample_import::play_wav_from_base64_string;
use crate::play_wav_from_assets;

pub fn show_central_panel(ctx: &egui::Context, logo: &egui::TextureHandle) {
    egui::CentralPanel::default().show(ctx, |ui| {
        // The central panel the region left after adding TopPanel's and SidePanel's

        // show our logo
        ui.image(logo, logo.size_vec2());

    });
}
