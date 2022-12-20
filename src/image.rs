#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Image {
    buffer: Vec<u8>,
    magic_number: String,
    saturation: usize,
    height: usize,
    width: usize,
}

impl Image {
        
    pub fn pbm(buffer: Vec<u8>, height: usize, width: usize) -> Result<Self, String> {
        if buffer.len() != height*width { return Err(format!("Error: could not create PBM object: byte vector does not fit given dimensions: (buffer length: {}) != (given dimensions [h*w]: {})", buffer.len(), height*width)) }
        if buffer.iter()
                 .any(|&b| b > 1)   { return Err(format!("Error: could not create PBM object: values must only consist of 1 or 0"))}
        Ok(Image {
            buffer,
            magic_number: "pbm".to_string(),
            saturation: 0, 
            height, 
            width,
        })
    }

    pub fn pgm(buffer: Vec<u8>, saturation: usize, height: usize, width: usize) -> Result<Self, String>{
        if buffer.len() != height*width { return Err(format!("Error: could not create PGM object: byte vector does not fit given dimensions: (buffer length: {}) != (given dimensions [h*w]: {})", buffer.len(), height*width)) }
        if saturation > 255             { return Err(format!("Error: could not create PGM object: saturation too great for PGM format ({})", saturation))} 

        Ok(Image {
            buffer,
            magic_number: "pgm".to_string(),
            saturation,
            height,
            width,
        })
    }

    pub fn ppm(buffer: Vec<u8>, saturation: usize, height: usize, width: usize) -> Result<Self, String> {
        if buffer.len() != 3*height*width { return Err(format!("Error: could not create PPM object: byte vector does not fit given dimensions: (buffer length: {}) != (given dimensions [h*w*3]: {})", buffer.len(), 3*height*width)) }
        if saturation > 255               { return Err(format!("Error: could not create PPM object: saturation too great for PPM format ({})", saturation))} 

        Ok(Image {
            buffer,
            magic_number: "ppm".to_string(),
            saturation,
            height,
            width,
        })
    }

    pub fn get_buffer(&self) -> Vec<u8> {
        self.buffer.clone()
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.height, self.width)
    }
}


pub mod io;
pub mod manipulation;