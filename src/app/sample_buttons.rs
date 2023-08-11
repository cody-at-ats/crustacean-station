use crate::app::sample_import::play_wav_from_base64_string;
use crate::play_wav_from_assets;

fn create_audio_button(ui: &mut egui::Ui, label: &str, play_fn: fn()) {
    if ui.button(label).clicked() {
        play_fn();
    }
}

pub fn show_sample_buttons(ctx: &egui::Context) {
    egui::Window::new("Sample Buttons").show(ctx, |ui| {
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
