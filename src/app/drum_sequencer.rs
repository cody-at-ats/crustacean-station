use egui::{FontId, Label, RichText};

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

impl Default for DrumSequencer {
    fn default() -> Self {
        Self { segments: vec![] }
    }
}

impl DrumSequencer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) {
        let Self { segments } = self;
        const beats: u32 = 4;
        const bars: u32 = 4;

        if segments.len() == 0 {
            for i in 0..beats * bars {
                segments.push(DrumSegment {
                    beat: (i),
                    kick: (false),
                    snare: (false),
                    hi_hat_closed: (false),
                    hi_hat_open: (false),
                    floor_tom: (false),
                    ride: (false),
                })
            }
        }

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("Kick");
                let mut cnt = 0;
                for _ in 0..bars {
                    for _ in 0..beats {
                        ui.checkbox(&mut segments[cnt].kick, "");
                        cnt += 1;
                    }
                    ui.label("|");
                }
            });

            ui.horizontal(|ui| {
                ui.label("Snare");
                let mut cnt = 0;
                for _ in 0..bars {
                    for _ in 0..beats {
                        ui.checkbox(&mut segments[cnt].snare, "");
                        cnt += 1;
                    }
                    ui.label("|");
                }
            });

            ui.horizontal(|ui| {
                ui.label("Hi Hat Closed");
                let mut cnt = 0;
                for _ in 0..bars {
                    for _ in 0..beats {
                        ui.checkbox(&mut segments[cnt].hi_hat_closed, "");
                        cnt += 1;
                    }
                    ui.label("|");
                }
            });

            ui.horizontal(|ui| {
                ui.label("Hi Hat Open");
                let mut cnt = 0;
                for _ in 0..bars {
                    for _ in 0..beats {
                        ui.checkbox(&mut segments[cnt].hi_hat_open, "");
                        cnt += 1;
                    }
                    ui.label("|");
                }
            });
        });
    }
}
