use super::common::Chunk;
use super::common::ByteOrder;
use super::common::Compile;

pub struct WaveHeader {
    chunk_id : Chunk,
    chunk_size : Chunk,
    format : Chunk,
}

impl WaveHeader {
    pub fn new () -> WaveHeader {
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
    pub fn new() -> WaveFmt {
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
        }
    }
}

pub struct WaveData {
    subchunk2id : Chunk,
    subchunk2size : Chunk,
    data : Chunk,
}

impl WaveData {
    pub fn new() -> WaveData {
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
}
