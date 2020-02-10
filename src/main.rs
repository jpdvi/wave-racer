use std::io::prelude::*;
use std::fs::File; 
mod headers;
mod common;
mod wave;
use wave::SineWave;
mod generator;
mod templates;
//use byteorder::{BigEndian, ReadBytesExt};

fn main() -> std::io::Result<()> {
    let _riff = b"RIFF";
    //let wv : SineWave = SineWave::new(261.626,44100,60);
    Ok(())
}
