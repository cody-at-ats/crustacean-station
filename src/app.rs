mod drum_sequencer;
mod sample_import;

pub use drum_sequencer::DrumSequencer;


mod central_panel;
use central_panel::show_central_panel;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state

pub struct CrustaceanStationApp {
    // Example stuff: I.e the app state
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,

    #[serde(skip)]
    drum_sequencer: DrumSequencer,
}

impl Default for CrustaceanStationApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            drum_sequencer: DrumSequencer::new(),
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
        let Self {
            label: _,
            value: _,
            drum_sequencer: _,
        } = self;

        egui::Window::new("Drum Sequencer")
            .show(ctx, |ui| DrumSequencer::draw(&mut self.drum_sequencer, ui));

        // loads the central panel that contains all the backgroung UI
        show_central_panel(ctx);
    }
}
