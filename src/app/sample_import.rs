use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/play_wav_file.js")]
extern "C" {
    fn selectAndPlayWavFile();
}

#[wasm_bindgen]
pub fn play_wav() {
    selectAndPlayWavFile();
}
