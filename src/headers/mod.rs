use super::common::Chunk;
use super::common::ByteOrder;
use super::common::Compile;
use super::wave::SineWave;

#[derive(Clone)]
pub struct WaveHeader {
    chunk_id : Chunk,
    chunk_size : Chunk,
    format : Chunk,
}

impl WaveHeader {
    pub fn new () -> Self {
        Self {
            chunk_id : Chunk::new(Some(4), Some(0), "ChunkID",
                ByteOrder::Big,
                Some("RIFF".to_string()),
                None),
            chunk_size : Chunk::new(Some(4),Some(4), "ChunkSize",
                ByteOrder::Little,
                None, 
                None), // Needs to be updated at end of cycle
            format : Chunk::new(Some(4), Some(8), "Format",
                ByteOrder::Big,
                Some("WAVE".to_string()),
                None),
        }
    }
}

impl Compile for WaveHeader {
    fn compile(&mut self) -> &mut Self {
        self.chunk_id.compile();
        self.chunk_size.compile();
        self.format.compile();
        self
    }
}

#[derive(Clone)]
pub struct WaveFmt {
    subchunk1id : Chunk,
    subchunk1size: Chunk,
    audio_format : Chunk,
    num_channels : Chunk,
    sample_rate : Chunk,
    byte_rate : Chunk,
    block_align : Chunk,
    bits_per_sample : Chunk,
    extra_param_size : Option<Chunk>,
    extra_params : Option<Chunk>,
}

impl WaveFmt {
    pub fn new() -> Self {
        Self {
            subchunk1id : Chunk::new(Some(4), Some(12), "Subchunk1ID",
                ByteOrder::Big,
                Some("fmt".to_string()),
                None),
            subchunk1size : Chunk::new(Some(4), Some(16), "Subchunk1Size", 
                ByteOrder::Little, 
                Some("16".to_string()),
                None),
            audio_format : Chunk::new(Some(2), Some(20), "AudioFormat", 
                ByteOrder::Little, 
                Some("1".to_string()),
                None),
            num_channels : Chunk::new(Some(2), Some(22), "NumChannels", 
                ByteOrder::Little, 
                Some("1".to_string()),
                None),
            sample_rate : Chunk::new(Some(4), Some(24), "SampleRate", 
                ByteOrder::Little, 
                Some("8000".to_string()),
                None),
            byte_rate : Chunk::new(Some(4), Some(28), "ByteRate", 
                ByteOrder::Little, 
                Some("1".to_string()), //TODO:  SampleRate * NumChannels * BitsPerSample/8,
                None),
            block_align : Chunk::new(Some(2), Some(32), "BlockAlign", 
                ByteOrder::Little, 
                Some("1".to_string()), // TODO: NumChannels * BitsPerSample / 8,
                None),
            bits_per_sample : Chunk::new(Some(2), Some(34), "BitsPerSample", 
                ByteOrder::Little, 
                Some("8".to_string()),
                None),
            extra_param_size : None,
            extra_params : None,
        }.compile().clone()
    }
}

impl Compile for WaveFmt {
    fn compile(&mut self) -> &mut Self {
        self.subchunk1id.compile();
        self.subchunk1size.compile();
        self.audio_format.compile();
        self.sample_rate.compile();
        self.num_channels.compile();
        self.bits_per_sample.compile();
        let sample_rate_raw: u32 = self.sample_rate.raw_data();
        let num_channels_raw: u32 = self.num_channels.raw_data();
        let bits_per_sample_raw: u32 =  self.bits_per_sample.raw_data();
        let calculated_byte_rate = sample_rate_raw * num_channels_raw as u32 * bits_per_sample_raw / 8;
        self.byte_rate.data = Some(calculated_byte_rate.to_string());
        self.byte_rate.compile();
        self.block_align.data = Some((num_channels_raw as u32 * bits_per_sample_raw / 8).to_string());
        self.block_align.compile();
        self
    }
}

#[derive(Clone)]
pub struct WaveData {
    subchunk2id : Chunk,
    subchunk2size : Chunk,
    data : Chunk,
}

impl WaveData {
    pub fn new() -> Self {
        Self {
            subchunk2id :  Chunk::new(Some(4), Some(36), "Subchunk2ID",
                ByteOrder::Big,
                Some("data".to_string()),
                None),
            subchunk2size : Chunk::new(Some(4), Some(40), "Subchunk2Size",
                ByteOrder::Little,
                Some("1".to_string()),
                None), // TODO : waveFmt.Numsamples * waveFmt.NumChannels, * waveFmt.BitsPerSecond/8
            data : Chunk::new(None, // Needs to be reconciled once bytes are compiled and counted
                Some(44), "Data",
                ByteOrder::Little,
                None,
                None),
        }
    }

    pub fn compile(&mut self, data: &SineWave, wave_fmt :&WaveFmt) -> &mut Self {
        self.subchunk2id.compile();
        self.data.data = Some(data.samples.clone().into_iter()
            .map(|i| { format!("{} ", i.to_string()) })
            .collect());
        self.data.compile();
        self.subchunk2size.byte_count = Some(self.data.bytes.as_ref().clone().unwrap().len() as u8);
        self.subchunk2size.data = Some((wave_fmt.num_channels.raw_data() * wave_fmt.bits_per_sample.raw_data()).to_string());
        self
    }
}
