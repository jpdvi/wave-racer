mod headers;
mod common;
mod wave;
mod generator;
mod templates;
use common::Compile;

fn main() -> std::io::Result<()> {
    let wv : wave::SineWave = wave::SineWave::new(261.626,44100,60);
    let mut wt = templates::WAVTemplate::new(wv).compile();
    Ok(())
}
