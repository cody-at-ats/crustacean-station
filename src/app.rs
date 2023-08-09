use egui::{Frame, vec2, Rect, emath, Pos2, pos2, Shape, Stroke, Ui};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct CrustaceanStationApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
}

impl Default for CrustaceanStationApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl CrustaceanStationApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for CrustaceanStationApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { label, value } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui


        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Crustacean STATION");
            
            ui.hyperlink("https://media.tenor.com/oB3o62J9hjkAAAAC/dancing-ferris.gif");
            egui::warn_if_debug_build(ui);
        });


        // DrumSequencer::show(ctx,|ui|);
        egui::Window::new("Drum Sequencer").show(ctx, |ui|{
            ui.horizontal_wrapped(|ui| {
                for _ in 0..64 {
                    ui.checkbox(&mut false, "");
                }
            });
            ui.checkbox(&mut false, "checkbox");
        });

        egui::Window::new("Scope").show(ctx, |ui| {
            let color = if ui.visuals().dark_mode {
                egui::Color32::from_additive_luminance(196)
            } else {
                egui::Color32::from_black_alpha(240)
            };
    
            Frame::canvas(ui.style()).show(ui, |ui| {
                ui.ctx().request_repaint();
                let time = ui.input(|i| i.time);
    
                let desired_size = ui.available_width() * vec2(1.0, 0.35);
                let (_id, rect) = ui.allocate_space(desired_size);
    
                let to_screen =
                    emath::RectTransform::from_to(Rect::from_x_y_ranges(0.0..=1.0, -1.0..=1.0), rect);                    
    
                let mut shapes = vec![];
    
                for &mode in &[2, 3, 5] {
                    let mode = mode as f64;
                    let n = 120;
                    let speed = 1.5;
    
                    let points: Vec<Pos2> = (0..=n)
                        .map(|i| {
                            let t = i as f64 / (n as f64);
                            let amp = (time * speed * mode).sin() / mode;
                            let y = amp * (t * std::f64::consts::TAU / 2.0 * mode).sin();
                            to_screen * pos2(t as f32, y as f32)
                        })
                        .collect();
    
                    let thickness = 10.0 / mode as f32;
                    shapes.push(Shape::line(points, Stroke::new(thickness, color)));
                }
    
                ui.painter().extend(shapes);
            });
     
        });
    }
}
