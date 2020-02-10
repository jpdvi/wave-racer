use std::f64;

#[derive(Clone)]
pub struct SineWave {
    pub frequency : f64,
    pub sample_rate : u64, 
    pub total_seconds : u64,
    pub samples : Vec<i16>,     
}

fn calculate_sample(current_sample: i16, samples_per_cycle: f64, amplitude: i16) -> f64 {
    (current_sample as f64 / samples_per_cycle * 2.0 * f64::consts::PI).sin() * amplitude as f64
}

impl SineWave {
    // TODO: Replace frequency with configuration
    pub fn new(frequency: f64, sample_rate: u64, total_seconds: u64) -> Self {
        Self {
            frequency: frequency,
            sample_rate: sample_rate,
            total_seconds: total_seconds,
            samples : vec![]
        }.build().clone()
    }

    fn build(&mut self) -> &mut Self {
        let total_samples = self.total_seconds * self.sample_rate;
        let amplitude : i16 = 32760;
        let samples_per_cycle = self.sample_rate as f64 / self.frequency;
        for i in 0..total_samples as i16 {
            let sample_value = calculate_sample(i, samples_per_cycle, amplitude);
            self.samples.push(sample_value as i16);
        }
        println!("{}", self.samples.len());
        self
    }
}

#[cfg(test)]
mod test {
    use super::SineWave;
    #[test]
    fn test_sine_amplitude_range() {
        let wv : SineWave = SineWave::new(261.626,44100,60);
        for sample in wv.samples {
            assert!(sample <= 32760 && sample >= -32760);
        }
    }
}
