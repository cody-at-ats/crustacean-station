use core::panic;

use log::info;

#[derive(Clone)]
pub struct DrumSequencer {
    pub segments: Vec<DrumSegment>,
    active_segment: usize,
}

#[derive(Copy, Clone)]
pub struct DrumSegment {
    beat: u32,
    pub kick: bool,
    pub snare: bool,
    pub hi_hat_closed: bool,
    pub hi_hat_open: bool,
    pub floor_tom: bool,
    pub ride: bool,
}

pub static mut ACTIVE_STEP: usize = 0;

impl Default for DrumSequencer {
    fn default() -> Self {
        Self {
            segments: vec![],
            active_segment: 0,
        }
    }
}

impl DrumSequencer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn SetActiveStep(step: usize) {
        unsafe {
            ACTIVE_STEP = step;
        }
    }

    pub fn GetActiveStep() -> usize {
        unsafe { ACTIVE_STEP }
    }

    pub fn get_slice(&mut self, iteration: usize) -> &DrumSegment {
        if iteration < self.segments.len() {
            self.active_segment = iteration;
            &self.segments[iteration]
        } else {
            panic!("NO")
        }
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) {
        let Self {
            segments,
            active_segment,
        } = self;

        let mut active_index: usize = 0;
        unsafe {
            active_index = ACTIVE_STEP.clone();
        }

        const BEATS: u32 = 4;
        const BARS: u32 = 4;

        if segments.len() == 0 {
            for i in 0..BEATS * BARS {
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
            const KICK_LABEL: &str = "Kick----------";
            const SNARE_LABEL: &str = "Snare---------";
            const HI_HAT_CLOSED_LABEL: &str = "Hi Hat Closed-";
            const HI_HAT_OPEN_LABEL: &str = "Hi Hat Open---";
            const FLOOR_TOM_LABEL: &str = "Floor Tom-----";

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(KICK_LABEL).text_style(egui::TextStyle::Monospace));

                let mut cnt = 0;
                for _ in 0..BARS {
                    for _ in 0..BEATS {
                        ui.checkbox(&mut segments[cnt].kick, "");

                        let text = if active_index == cnt { "<" } else { " " };
                        ui.label(egui::RichText::new(text).text_style(egui::TextStyle::Monospace));

                        cnt += 1;
                    }
                    ui.label("|");
                }
            });

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(SNARE_LABEL).text_style(egui::TextStyle::Monospace));
                let mut cnt = 0;
                for _ in 0..BARS {
                    for _ in 0..BEATS {
                        ui.checkbox(&mut segments[cnt].snare, "");

                        let text = if active_index == cnt { "<" } else { " " };
                        ui.label(egui::RichText::new(text).text_style(egui::TextStyle::Monospace));

                        cnt += 1;
                    }
                    ui.label("|");
                }
            });

            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(HI_HAT_CLOSED_LABEL).text_style(egui::TextStyle::Monospace),
                );
                let mut cnt = 0;
                for _ in 0..BARS {
                    for _ in 0..BEATS {
                        ui.checkbox(&mut segments[cnt].hi_hat_closed, "");

                        let text = if active_index == cnt { "<" } else { " " };
                        ui.label(egui::RichText::new(text).text_style(egui::TextStyle::Monospace));

                        cnt += 1;
                    }
                    ui.label("|");
                }
            });

            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(HI_HAT_OPEN_LABEL).text_style(egui::TextStyle::Monospace),
                );
                let mut cnt = 0;
                for _ in 0..BARS {
                    for _ in 0..BEATS {
                        ui.checkbox(&mut segments[cnt].hi_hat_open, "");

                        let text = if active_index == cnt { "<" } else { " " };
                        ui.label(egui::RichText::new(text).text_style(egui::TextStyle::Monospace));

                        cnt += 1;
                    }
                    ui.label("|");
                }
            });

            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(FLOOR_TOM_LABEL).text_style(egui::TextStyle::Monospace),
                );
                let mut cnt = 0;
                for _ in 0..BARS {
                    for _ in 0..BEATS {
                        ui.checkbox(&mut segments[cnt].floor_tom, "");

                        let text = if active_index == cnt { "<" } else { " " };
                        ui.label(egui::RichText::new(text).text_style(egui::TextStyle::Monospace));

                        cnt += 1;
                    }
                    ui.label("|");
                }
            });

            ui.horizontal(|ui| {
                if ui.button("<").clicked() {
                    if active_index > 0 {
                        *active_segment -= 1;
                    }
                }
                if ui.button(">").clicked() {
                    if active_index < segments.len() - 1 {
                        *active_segment += 1;
                    }
                };
            });
        });
    }
}
