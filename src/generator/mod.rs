use super::templates;

#[derive(Clone)]
pub enum OutputType {
    WAV
}

#[derive(Clone)]
pub struct GeneratorConfig {
    output_type : Option<OutputType>
}

#[derive(Clone)]
pub struct Generator {
    config: Option<GeneratorConfig>,
    bytes : Option<Vec<u8>>
}

impl Generator {
    fn new(config: GeneratorConfig) -> Generator {
        Self {
            config : Some(config.clone()),
            bytes : None,
        }
    }

    fn generate_wav(&mut self) {
        println!("Hello");
    }

    pub fn generate(&mut self) {
       match &self.config {
           Some(_i) => { 
               match _i.output_type {
                   Some(OutputType::WAV) => self.generate_wav(),
                   _=> println!("Unknown Encoding") 
               }
           },
           _=> println!("No config provided")
       }
    }
}
