export async function playWavFile(wavFile) {
    const reader = new FileReader();
    const arrayBuffer = await new Promise((resolve) => {
        reader.onload = () => resolve(reader.result);
        reader.readAsArrayBuffer(wavFile);
    });
    const audioContext = new AudioContext();
    const audioBuffer = await audioContext.decodeAudioData(arrayBuffer);
    const source = audioContext.createBufferSource();
    source.buffer = audioBuffer;
    source.connect(audioContext.destination);
    source.start();
}

// Other functions in this file might just be deadcode eventually
export async function play_wav_from_base64_string(base64String) {
    // Decode the base64 string into an ArrayBuffer
    const binaryString = atob(base64String);
    const len = binaryString.length;
    const bytes = new Uint8Array(len);
    for (let i = 0; i < len; i++) {
        bytes[i] = binaryString.charCodeAt(i);
    }
    const arrayBuffer = bytes.buffer;

    // Create an AudioContext and decode the audio data
    const audioContext = new AudioContext();
    const audioBuffer = await audioContext.decodeAudioData(arrayBuffer);

    // Create a BufferSource and play the audio
    const source = audioContext.createBufferSource();
    source.buffer = audioBuffer;
    source.connect(audioContext.destination);
    source.start();
}

// Could use a version of this to save the "imports" -- do that later
export async function selectAndPlayWavFile() {
    // Create an input element and set its type to "file"
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.wav';

    // Wait for the user to select a file
    const file = await new Promise((resolve) => {
        input.addEventListener('change', () => resolve(input.files[0]));
        input.click();
    });

    // Pass the file to the playWavFile function
    playWavFile(file);
}

export async function playWavFileFromAssets(relativePath) {
    // Construct the absolute URL to the .wav file
    const url = new URL(relativePath, window.location.href);

    // Fetch the .wav file
    const response = await fetch(url);
    const blob = await response.blob();

    // Pass the Blob to the playWavFile function
    playWavFile(blob);
}
