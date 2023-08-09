use js_sys::Array;
use js_sys::ArrayBuffer;
use js_sys::Uint8Array;
use log::info;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Blob, BlobPropertyBag};

#[wasm_bindgen(module = "/src/play_wav_file.js")]
extern "C" {
    fn selectAndPlayWavFile();
}

pub fn play_wav() {
    unsafe { selectAndPlayWavFile() };
}

#[wasm_bindgen(module = "/src/play_wav_file.js")]
extern "C" {
    fn play_wav_from_base64_string(base64String: &str);
}

pub fn play_wav_from_assets() {
    // Include the .wav file as a byte array
    let wav_file_bytes: &[u8] = include_bytes!("../../assets/Bass-Drum-2.wav");

    // Encode the byte array as a base64 string
    let base64_string = base64::encode(wav_file_bytes);

    // Pass the base64 string to a JavaScript function that decodes it and plays the audio
    unsafe { play_wav_from_base64_string(&base64_string) };
}
