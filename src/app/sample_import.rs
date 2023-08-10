



use wasm_bindgen::prelude::*;



#[wasm_bindgen(module = "/src/app/javascript/play_wav_file.js")]
extern "C" {

    fn selectAndPlayWavFile();
}

pub fn play_wav() {
    unsafe { selectAndPlayWavFile() };
}

#[wasm_bindgen(module = "/src/app/javascript/play_wav_file.js")]
extern "C" {
    pub fn play_wav_from_base64_string(base64String: &str);
}

#[macro_export]
macro_rules! play_wav_from_assets {
    ($fn_name:ident, $asset_path:literal) => {
        pub fn $fn_name() {
            // Include the .wav file as a byte array
            let wav_file_bytes: &[u8] = include_bytes!($asset_path);

            // Encode the byte array as a base64 string
            let base64_string = base64::encode(wav_file_bytes);

            // Pass the base64 string to a JavaScript function that decodes it and plays the audio
            play_wav_from_base64_string(&base64_string);
        }
    };
}
