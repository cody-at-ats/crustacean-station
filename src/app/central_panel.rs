use crate::app::sample_import::play_wav_from_base64_string;
use crate::play_wav_from_assets;

fn create_audio_button(ui: &mut egui::Ui, label: &str, play_fn: fn()) {
    if ui.button(label).clicked() {
        play_fn();
    }
}

pub fn show_central_panel(ctx: &egui::Context, logo: &egui::TextureHandle) {
    egui::CentralPanel::default().show(ctx, |ui| {
        // The central panel the region left after adding TopPanel's and SidePanel's
        ui.heading("Crustacean STATION 📻");

        // show our logo
        ui.image(logo, logo.size_vec2());

        ui.hyperlink("https://media.tenor.com/oB3o62J9hjkAAAAC/dancing-ferris.gif");
        egui::warn_if_debug_build(ui);

        play_wav_from_assets!(play_bass_drum, "../../assets/Bass-Drum-2.wav");
        play_wav_from_assets!(play_snare_drum, "../../assets/Hip-Hop-Snare-1.wav");
        play_wav_from_assets!(play_closed_hat, "../../assets/Closed-Hi-Hat-1.wav");
        play_wav_from_assets!(play_open_hat, "../../assets/Open-Hi-Hat-1.wav");
        play_wav_from_assets!(play_floor_tom, "../../assets/Floor-Tom-1.wav");

        create_audio_button(ui, "Play bass drum sample", play_bass_drum);
        create_audio_button(ui, "Play snare drum sample", play_snare_drum);
        create_audio_button(ui, "Play closed hat sample", play_closed_hat);
        create_audio_button(ui, "Play open hat sample", play_open_hat);
        create_audio_button(ui, "Play floor tom sample", play_floor_tom);
    });
}
