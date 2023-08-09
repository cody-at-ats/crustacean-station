pub struct DrumSegment
{
    beat: u32,
    kick: bool,
    snare: bool,
    hi_hat_closed: bool,
    hi_hat_open:bool, 
    floor_tom: bool,
    ride:bool,
}

pub struct DrumSequencer
{
    segments: Vec<DrumSegment>
}

impl default for DrumSequencer{
    fn default () -> Self{
        Self{
            segments: Default::default(),
        }
    }
}

impl DrumSequencer{
    pub fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::Window::new("Drum Pattern").show(ctx, |ui|{
            ui.horizontal_wrapped(|ui| {
                for _ in 0..64 {
                    ui.checkbox(&mut true, "");
                }
            });
            ui.checkbox(&mut true, "checkbox");
        });
    })
}