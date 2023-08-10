use async_std::stream::StreamExt;
use futures::executor::block_on;

use futures::Future;
use log::info;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use std::time::Duration;
use wasm_bindgen_futures::spawn_local;
use wasm_timer::Delay;

use crate::app::sample_import::play_wav_from_base64_string;
use crate::{app::drum_sequencer::DrumSegment, play_wav_from_assets};

use super::{drum_sequencer, DrumSequencer};

pub fn run_drum_segment(drum_segment: &DrumSegment) {
    play_wav_from_assets!(play_bass_drum, "../../assets/Bass-Drum-2.wav");
    play_wav_from_assets!(play_snare_drum, "../../assets/Hip-Hop-Snare-1.wav");
    play_wav_from_assets!(play_closed_hat, "../../assets/Closed-Hi-Hat-1.wav");
    play_wav_from_assets!(play_open_hat, "../../assets/Open-Hi-Hat-1.wav");
    play_wav_from_assets!(play_floor_tom, "../../assets/Floor-Tom-1.wav");

    if drum_segment.kick == true {
        play_bass_drum();
    }
    if drum_segment.snare == true {
        play_snare_drum();
    }
    if drum_segment.hi_hat_closed == true {
        play_closed_hat();
    }
    if drum_segment.hi_hat_open == true {
        play_open_hat();
    }
    if drum_segment.floor_tom == true {
        play_floor_tom();
    }
}

// Write a function that takes a boolean "is_playing" and a vector "sequence"
// and starts a new thread that loops through the sequence and calls the function
// run_drum_segment from the sample_runner module.
//
// The function should return a handle to the thread that was started.
//
// The thread should be stopped when the boolean "is_playing" is set to false.
//
// The thread should sleep for 1 second between each iteration.
// The returned handle should be used to join the thread in the main thread when
// playing is stopped.
pub fn start_looping_sequence(
    should_stop: Arc<AtomicBool>,
    sequence: &mut Arc<Mutex<Vec<DrumSegment>>>,
    sequence_running: Arc<AtomicBool>,
) -> () {
    info!("Called start_looping_sequence!!");

    let sequence_clone = sequence.clone();

    spawn_local(async move {
        loop {
            if should_stop.load(Ordering::SeqCst) {
                break;
            }

            let sequence_copy;
            {
                let full_sequence = sequence_clone.lock().unwrap();
                sequence_copy = full_sequence.clone();
            }

            info!("Length of sequence: {}", sequence_copy.len());
            for segment in sequence_copy.iter() {
                spawn_local({
                    let segment = segment.clone();
                    async move {
                        run_drum_segment(&segment);
                    }
                });

                // run_drum_segment(&segment);
                let _ = Delay::new(Duration::from_millis(200)).await;
            }
        }
        sequence_running.store(false, Ordering::SeqCst);
    });
}
