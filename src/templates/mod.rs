use super::wave;
use super::headers;

pub struct WAVTemplate {
    wave_header : headers::WaveHeader,
    wave_format : headers::WaveFmt,
    wave_data : headers::WaveData,
}

impl WAVTemplate {
    pub fn new() -> WAVTemplate {
        Self {
            wave_header : headers::WaveHeader::new(),
            wave_format : headers::WaveFmt::new(),
            wave_data : headers::WaveData::new(),
        }
    }
}
