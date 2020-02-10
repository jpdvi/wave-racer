use super::wave;
use super::headers;
use super::common::Compile;

#[derive(Clone)]
pub struct WAVTemplate {
    pub wave_header : headers::WaveHeader,
    pub wave_format : headers::WaveFmt,
    pub wave_data : headers::WaveData,
    pub _wave : wave::SineWave,
}

impl WAVTemplate {
    pub fn new(_wave: wave::SineWave) -> WAVTemplate {
        Self {
            wave_header : headers::WaveHeader::new(),
            wave_format : headers::WaveFmt::new(),
            wave_data : headers::WaveData::new(),
            _wave : _wave.clone(),
        }
    }
}

impl Compile for WAVTemplate {
    fn compile(&mut self) -> &mut Self {
        self.wave_header.compile();
        self.wave_format.compile();
        self.wave_data.compile(&self._wave, &self.wave_format);
        self
    }
}


#[cfg(test)]
mod test {
    use super::WAVTemplate;
    use crate::common::Compile;
    use crate::wave::SineWave;

    #[test]
    fn test_basic(){
        let wv : SineWave = SineWave::new(261.626,44100,60);
        let wt = WAVTemplate::new(wv).compile();
    }
}
