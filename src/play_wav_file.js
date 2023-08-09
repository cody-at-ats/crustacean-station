async function playWavFile(wavFile) {
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
