pub static mut ACTIVE_STEP: usize = 0;

#[derive(Clone)]
pub struct DrumSequencer {
    pub segments: Vec<DrumSegment>,
}

#[derive(Copy, Clone)]
pub struct DrumSegment {
    pub kick: bool,
    pub snare: bool,
    pub hi_hat_closed: bool,
    pub hi_hat_open: bool,
    pub floor_tom: bool,
    pub ride: bool,
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

    pub fn set_active_step(step: usize) {
        unsafe {
            ACTIVE_STEP = step;
        }
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) {
        let Self { segments } = self;

        let active_index: usize;
        unsafe {
            active_index = ACTIVE_STEP.clone();
        }

        const BEATS: u32 = 4;
        const BARS: u32 = 4;

        if segments.len() == 0 {
            for _ in 0..BEATS * BARS {
                segments.push(DrumSegment {
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

                        let text = if active_index == cnt { "⇜" } else { " " };
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

                        let text = if active_index == cnt { "⇜" } else { " " };
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

                        let text = if active_index == cnt { "⇜" } else { " " };
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

                        let text = if active_index == cnt { "⇜" } else { " " };
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

                        let text = if active_index == cnt { "⇜" } else { " " };
                        ui.label(egui::RichText::new(text).text_style(egui::TextStyle::Monospace));

                        cnt += 1;
                    }
                    ui.label("|");
                }
            });
        });
    }
}
