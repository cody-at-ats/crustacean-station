mod drum_sequencer;
mod sample_import;
mod sequence_looper;

use egui::Slider;
use futures::executor::block_on;

use futures::StreamExt;
use log::info;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use wasm_timer::Instant;

pub use drum_sequencer::DrumSequencer;

mod central_panel;
use central_panel::show_central_panel;

use self::drum_sequencer::DrumSegment;
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
    drum_segments: Arc<Mutex<Vec<DrumSegment>>>,

    #[serde(skip)]
    is_playing: Arc<AtomicBool>,
    // #[serde(skip)]
    // sequence_handle: Future<Output = ()>,
    #[serde(skip)]
    should_stop: Arc<AtomicBool>,

    #[serde(skip)]
    sequence_running: Arc<AtomicBool>,

    #[serde(skip)]
    last_update: Instant,

    bpm: Arc<Mutex<u32>>,
}

impl Default for CrustaceanStationApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            drum_sequencer: DrumSequencer::new(),
            drum_segments: Arc::new(Mutex::new(vec![])),
            is_playing: Arc::new(AtomicBool::new(false)),
            should_stop: Arc::new(AtomicBool::new(false)),
            sequence_running: Arc::new(AtomicBool::new(false)),
            last_update: Instant::now(),
            bpm: Arc::new(Mutex::new(110)),
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
            drum_segments,
            is_playing,
            should_stop,
            sequence_running,
            last_update,
            bpm,
        } = self;

        egui::Window::new("Drum Sequencer").show(ctx, |ui| DrumSequencer::draw(drum_sequencer, ui));

        // Only update the drum segments if at least 100ms have passed since the last update
        if last_update.elapsed() >= Duration::from_millis(100) {
            if let Ok(mut drum_segments) = drum_segments.try_lock() {
                *drum_segments = drum_sequencer.segments.clone();
                *last_update = Instant::now();
            }
        }

        // loads the central panel that contains all the backgroung UI
        show_central_panel(ctx);

        let mut play_clicked = false;
        let mut stop_clicked = false;

        egui::Window::new("Control Buttons").show(ctx, |ui| {
            if ui.button("PLAY LOOP!").clicked() {
                play_clicked = true;
            }

            if ui.button("STOP LOOP").clicked() {
                stop_clicked = true;
            }

            if let Ok(mut bpm) = bpm.try_lock() {
                ui.add(
                    Slider::new(&mut *bpm, 40..=500)
                        .logarithmic(false)
                        .clamp_to_range(true)
                        .smart_aim(true)
                        .orientation(egui::SliderOrientation::Horizontal)
                        .text("BPM Control")
                        .step_by(1.0)
                        .trailing_fill(false),
                );
            }

            if play_clicked && !sequence_running.load(Ordering::SeqCst) {
                is_playing.store(true, Ordering::SeqCst);
                should_stop.store(false, Ordering::SeqCst);
                let sequence_running_clone = sequence_running.clone();
                start_looping_sequence(
                    should_stop.clone(),
                    drum_segments,
                    sequence_running_clone,
                    bpm.clone(),
                );
            }

            if stop_clicked {
                is_playing.store(false, Ordering::SeqCst);
                should_stop.store(true, Ordering::SeqCst);
            }
        });
    }
}
