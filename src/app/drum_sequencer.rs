pub struct DrumSequencer {
    segments: Vec<DrumSegment>,
}

pub struct DrumSegment {
    beat: u32,
    kick: bool,
    snare: bool,
    hi_hat_closed: bool,
    hi_hat_open: bool,
    floor_tom: bool,
    ride: bool,
}

impl DrumSequencer {
    pub fn draw(ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            for _ in 0..64 {
                ui.checkbox(&mut false, "");
            }
        });
        ui.checkbox(&mut true, "checkbox");
    }
}
