pub trait Compile {
    fn compile(&mut self) -> &mut Self;
}

#[derive(Clone)]
pub enum ByteOrder {
    Big = 0,
    Little = 1,
}

#[derive(Clone)]
pub struct Chunk {
    pub byte_count : Option<u8>,
    pub offset : Option<u8>,
    pub name : String,
    pub byte_order :  ByteOrder,
    pub data : Option<String>,
    pub bytes : Option<Vec<u8>>, 
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
            bytes : None,
        }
    }
   
    pub fn raw_data(&self) -> u32 {
        let raw : u32 = self.data.clone().unwrap().parse().unwrap();
        return raw
    }
}

impl Compile for Chunk {
    fn compile(&mut self) -> &mut Self {
        if let Some(i) = &self.data {
            self.bytes = Some(self.data.clone().unwrap().as_bytes().to_vec());
        };         
        self
    }
}
