mod drum_sequencer;
pub use drum_sequencer::DrumSequencer;

pub fn run_drum_segment (drum_segment: &DrumSegment) {
    if drum_segment.kick == True {
        play_wav_from_assets!(play_bass_drum, "../../assets/Bass-Drum-2.wav");
        play_bass_drum();
    }
    if drum_segment.snare == True {
        play_wav_from_assets!(play_snare_drum, "../../assets/Hip-Hop-Snare-1.wav");
        play_snare_drum();
    }
    if drum_segment.hi_hat_closed == True {
        play_wav_from_assets!(play_closed_hat, "../../assets/Closed-Hi-Hat-1.wav");
        play_closed_hat();
    }
    if drum_segment.hi_hat_open == True {
        play_wav_from_assets!(play_open_hat, "../../assets/Open-Hi-Hat-1.wav");
        play_open_hat();
    }
    if drum_segment.floor_tom == True {
        play_wav_from_assets!(play_floor_tom, "../../assets/Floor-Tom-1.wav");
        play_floor_tom();
    }
}