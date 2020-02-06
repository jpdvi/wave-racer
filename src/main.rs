use std::io::prelude::*;
use std::fs::File; 
//use byteorder::{BigEndian, ReadBytesExt};


#[derive(Clone)]
pub enum ByteOrder {
    Big = 0,
    Little = 1,
}

#[derive(Clone)]
pub struct Chunk<T> {
    byte_count : Option<u8>,
    offset : Option<u8>,
    name : String,
    byte_order :  ByteOrder,
    bytes : Option<Vec<T>>, 
}

impl<T> Chunk<T> {
    fn write_bytes() {
        println!("Hello");
    }
}

pub struct WaveHeader<T> {
    chunk_id : Chunk<T>,
    chunk_size : Chunk<T>,
    format : Chunk<T>,
}

pub struct WaveFmt<T> {
    subchunk1id : Chunk<T>,
    subchunk1size: Chunk<T>,
    audio_format : Chunk<T>,
    num_channels : Chunk<T>,
    sample_rate : Chunk<T>,
    byte_rate : Chunk<T>,
    block_align : Chunk<T>,
    bits_per_sample : Chunk<T>,
    extra_param_size : Option<Chunk<T>>,
    extra_params : Option<Chunk<T>>,
}

pub struct WaveData<T> {
    subchunk2id : Chunk<T>,
    subchunk2size : Chunk<T>,
    data : Chunk<T>,
}

pub struct WaveTemplate<T> {
    wave_header : WaveHeader<T>,
    wave_format : WaveFmt<T>,
    wave_data : WaveData<T>,
}

fn main() -> std::io::Result<()> {
    let _riff = b"RIFF";
//    let mut pos = 0;
//    let mut buffer = File::create("foo.txt")?;
//
//    while pos < _riff.len() {
//        let bytes_written = buffer.write(&_riff[pos..])?;
//        pos += bytes_written;
//    }
    Ok(())
}
