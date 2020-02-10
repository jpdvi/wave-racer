pub trait Compile {
    fn compile(&mut self) -> Self;
}

#[derive(Clone)]
pub enum ByteOrder {
    Big = 0,
    Little = 1,
}

#[derive(Clone)]
pub struct Chunk {
    byte_count : Option<u8>,
    offset : Option<u8>,
    name : String,
    byte_order :  ByteOrder,
    data : Option<String>,
    bytes : Option<Vec<u8>>, 
}

impl Chunk {
    pub fn new (byte_count: Option<u8>, 
        offset : Option<u8>, 
        name: &str, 
        byte_order: ByteOrder,
        data : Option<String>,
        bytes : Option<Vec<u8>>) -> Chunk {
        Self {
            offset : offset,
            name : name.to_string(),
            byte_count: byte_count,
            byte_order : byte_order,
            data: data,
            bytes : None
        }
    }
}

impl Compile for Chunk {
    fn compile(&mut self) -> Chunk {
        if let Some(i) = &self.data {
            self.bytes = Some(self.data
                .clone().unwrap().as_bytes().to_vec());
        };
        return self.clone()
    }
}
