mod drum_sequencer;
mod sample_import;
mod sequence_looper;

use futures::executor::block_on;
use futures::lock::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub use drum_sequencer::DrumSequencer;

mod central_panel;
use central_panel::show_central_panel;

use self::sequence_looper::start_looping_sequence;

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

    #[serde(skip)]
    is_playing: Arc<AtomicBool>,
    // #[serde(skip)]
    // sequence_handle: Future<Output = ()>,
    #[serde(skip)]
    should_stop: Arc<AtomicBool>,
}

impl Default for CrustaceanStationApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            drum_sequencer: DrumSequencer::new(),
            is_playing: Arc::new(AtomicBool::new(false)),
            should_stop: Arc::new(AtomicBool::new(false)),
            // sequence_handle: None,
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
            drum_sequencer,
            is_playing,
            should_stop,
        } = self;

        egui::Window::new("Drum Sequencer").show(ctx, |ui| DrumSequencer::draw(drum_sequencer, ui));

        // loads the central panel that contains all the backgroung UI
        show_central_panel(ctx);

        let mut play_clicked = false;
        let mut stop_clicked = false;

        egui::Window::new("Play Buttons").show(ctx, |ui| {
            if ui.button("PLAY LOOP!").clicked() {
                play_clicked = true;
            }

            if ui.button("STOP LOOP").clicked() {
                stop_clicked = true;
            }
        });

        if play_clicked {
            is_playing.store(true, Ordering::SeqCst);
            should_stop.store(false, Ordering::SeqCst);
            start_looping_sequence(drum_sequencer.to_owned(), should_stop.clone());
        }

        if stop_clicked {
            is_playing.store(false, Ordering::SeqCst);
            should_stop.store(true, Ordering::SeqCst);
        }

        // if *block_on(self.is_playing.lock()) && self.sequence_handle.is_none() {
        //     self.sequence_handle =
        //         Box::<start_looping_sequence>(self.drum_sequencer.segments.clone());
        // } else if !*block_on(self.is_playing.lock()) {
        //     if let Some(handle) = self.sequence_handle.take() {
        //         handle.join();
        //     }
        // }
    }
}
