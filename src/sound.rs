// NOTE: This section was taken fram the sdl2 example for audio-wav.rs
// TODO: investigate using the mixer feature
pub struct SoundAsset {
    pub data: Vec<u8>
}

// This is a Wav
impl SoundAsset {
    // TODO: can this use the actual file?
    pub fn from_wav(wav_file: &std::path::Path) -> SoundAsset {
        let audio_spec_wav = sdl2::audio::AudioSpecWAV::load_wav(wav_file)
            .expect("Could not load test WAV file");
        SoundAsset {
            data: audio_spec_wav.buffer().to_vec(),
        }
    }
}


pub struct Sound {
    pub data: Vec<u8>,
    pub volume: f32,
    pub pos: usize,
}


impl sdl2::audio::AudioCallback for Sound {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        for dst in out.iter_mut() {
            // With channel type u8 the "silence" value is 128 (middle of the 0-2^8 range) so we need
            // to both fill in the silence and scale the wav data accordingly. Filling the silence
            // once the wav is finished is trivial, applying the volume is more tricky. We need to:
            // * Change the range of the values from [0, 255] to [-128, 127] so we can multiply
            // * Apply the volume by multiplying, this gives us range [-128*volume, 127*volume]
            // * Move the resulting range to a range centered around the value 128, the final range
            //   is [128 - 128*vlume, 128 + 127*volume] – scaled and correctly positioned
            //
            // Using value 0 instead of 128 would result in clicking. Scaling by simply multiplying
            // would not give correct results.
            let pre_scale = *self.data.get(self.pos).unwrap_or(&128);
            let scaled_signed_float = (pre_scale as f32 - 128.0) * self.volume;
            let scaled = (scaled_signed_float + 128.0) as u8;
            *dst = scaled;
            self.pos += 1;
        }
        println!("running audio");
    }
}

