use core::panic;

pub struct DrumSequencer {
    segments: Vec<DrumSegment>,
    active_segment: usize,
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

    pub fn get_slice(&mut self, iteration: usize) -> &DrumSegment {
        if iteration < self.segments.len() {
            self.active_segment = iteration;
            &self.segments[iteration]
        } else {
            panic!("NO")
        }
    }

    // pub fn layout_drum_row(label: &str, segmentProp: &mut bool, ui: &mut egui::Ui) {
    //     ui.horizontal(|ui| {
    //         ui.label(segmentProp);
    //         let mut cnt = 0;
    //         for _ in 0..bars {
    //             for _ in 0..beats {
    //                 ui.checkbox(ui, "");
    //                 cnt += 1;
    //             }
    //             ui.label("|");
    //         }
    //     });
    // }

    pub fn draw(&mut self, ui: &mut egui::Ui) {
        let Self {
            segments,
            active_segment: _,
        } = self;
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
            const kick_label: &str = "Kick----------";
            const snare_label: &str = "Snare---------";
            const hi_hat_closed_label: &str = "Hi Hat Closed-";
            const hi_hat_open_label: &str = "Hi Hat Open---";
            const floor_tom_label: &str = "Floor Tom-----";
            const ride_label: &str = "Ride----------";

            // self::layout_drum_row("Kickz", &mut segments[0].kick, ui);
            // self::layout_drum_row("Snare", &mut segments[0].snare, ui);
            // self::layout_drum_row("Hi Hat Closed", &mut segments[0].hi_hat_closed, ui);
            // self::layout_drum_row("Hi Hat Open", &mut segments[0].hi_hat_open, ui);
            // self::layout_drum_row("Floor Tom", &mut segments[0].floor_tom, ui);

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(kick_label).text_style(egui::TextStyle::Monospace));

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
                ui.label(egui::RichText::new(snare_label).text_style(egui::TextStyle::Monospace));
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
                ui.label(
                    egui::RichText::new(hi_hat_closed_label).text_style(egui::TextStyle::Monospace),
                );
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
                ui.label(
                    egui::RichText::new(hi_hat_open_label).text_style(egui::TextStyle::Monospace),
                );
                let mut cnt = 0;
                for _ in 0..bars {
                    for _ in 0..beats {
                        ui.checkbox(&mut segments[cnt].hi_hat_open, "");
                        cnt += 1;
                    }
                    ui.label("|");
                }
            });

            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(floor_tom_label).text_style(egui::TextStyle::Monospace),
                );
                let mut cnt = 0;
                for _ in 0..bars {
                    for _ in 0..beats {
                        ui.checkbox(&mut segments[cnt].floor_tom, "");
                        cnt += 1;
                    }
                    ui.label("|");
                }
            });
        });
    }
}
