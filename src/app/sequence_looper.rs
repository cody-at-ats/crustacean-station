use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use std::time::Duration;
use wasm_bindgen_futures::spawn_local;

use super::DrumSequencer;

use crate::app::sample_import::play_wav_from_base64_string;
use crate::{app::drum_sequencer::DrumSegment, play_wav_from_assets};

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
    bpm: Arc<Mutex<u32>>,
) -> () {
    let sequence_clone = sequence.clone();
    let bpm_clone = bpm.clone();

    spawn_local(async move {
        loop {
            if should_stop.load(Ordering::SeqCst) {
                break;
            }

            let sequence_copy;

            if let Ok(full_sequence) = sequence_clone.try_lock() {
                sequence_copy = full_sequence.clone();
            } else {
                sequence_copy = vec![];
            }

            // let mut count = 0;
            for segment in sequence_copy.iter() {
                // DrumSequencer::set_active_step(count);
                // count += 1;

                spawn_local({
                    let segment = segment.clone();
                    async move {
                        run_drum_segment(&segment);
                    }
                });

                // convert the bpm value to a millisecond
                // value that represents the amount of time between
                // each beat.
                let bpm_copy;
                if let Ok(actual_bpm) = bpm_clone.try_lock() {
                    bpm_copy = actual_bpm.clone();
                } else {
                    // couldn't get lock, use default bpm
                    bpm_copy = 110;
                }

                let bpm_in_millis = 60000 / bpm_copy / 4;
                let _ = async_std::task::sleep(Duration::from_millis(bpm_in_millis as u64)).await;
            }
        }
        sequence_running.store(false, Ordering::SeqCst);
    });
}
