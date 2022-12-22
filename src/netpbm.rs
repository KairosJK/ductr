#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AnymapImage {
    buffer: Vec<u8>,
    magic_number: String,
    saturation: usize,
    height: usize,
    width: usize,
}

impl AnymapImage {
    
    /// # Constructor for the PBM format
    /// Properly encodes the AnymapImage struct into a suitable PBM image
    /// 
    /// Rules for arguments:
    /// - length of `buffer` must equal the product of the width and height inputs
    /// - `buffer` must consist only of bytes equal to 0 or 1
    /// 
    /// Example:
    /// ```
    /// use ductr::netpbm::AnymapImage;
    /// 
    /// // create pixel buffer
    /// let buffer = vec![1; 100*100];
    /// 
    /// // create black 100x100 PBM AnymapImage object
    /// let pbm_black = AnymapImage::pbm(buffer, 100, 100).unwrap();
    /// ``` 
    pub fn pbm(buffer: Vec<u8>, height: usize, width: usize) -> Result<Self, String> {
        if buffer.len() != height*width { return Err(format!("Error: could not create PBM object: byte vector does not fit given dimensions: (buffer length: {}) != (given dimensions [h*w]: {})", buffer.len(), height*width)) }
        if buffer.iter()
                 .any(|&b| b > 1)   { return Err(format!("Error: could not create PBM object: values must only consist of 1 or 0"))}
        Ok(AnymapImage {
            buffer,
            magic_number: "pbm".to_string(),
            saturation: 0, 
            height, 
            width,
        })
    }

    /// # Constructor for the PGM format
    /// Properly encodes the AnymapImage struct into a suitable PGM image
    /// 
    /// Rules for arguments:
    /// - length of `buffer` must equal the product of the width and height inputs
    /// - `saturation` argument must fall in the range of `0..255`
    /// 
    /// Example:
    /// ```
    /// use ductr::netpbm::AnymapImage;
    /// 
    /// // create pixel buffer
    /// let buffer = vec![150; 100*100];
    /// 
    /// // create grey 100x100 PGM AnymapImage object
    /// let pgm_grey = AnymapImage::pgm(buffer, 255, 100, 100).unwrap();
    /// ``` 
    pub fn pgm(buffer: Vec<u8>, saturation: usize, height: usize, width: usize) -> Result<Self, String>{
        if buffer.len() != height*width { return Err(format!("Error: could not create PGM object: byte vector does not fit given dimensions: (buffer length: {}) != (given dimensions [h*w]: {})", buffer.len(), height*width)) }
        if saturation > 255             { return Err(format!("Error: could not create PGM object: saturation too great for PGM format ({})", saturation))} 

        Ok(AnymapImage {
            buffer,
            magic_number: "pgm".to_string(),
            saturation,
            height,
            width,
        })
    }

    /// # Constructor for the PPM format
    /// Properly encodes the AnymapImage struct into a suitable PPM image
    /// 
    /// Rules for arguments:
    /// - `buffer` must hold 3 bytes for each pixel (8 bits per color channel)
    /// - length of `buffer` must equal the product of the width and height inputs multiplied by 3 (h*w*3)
    /// - `saturation` argument must fall in the range of `0..255`
    /// 
    /// Example:
    /// ```
    /// use ductr::netpbm::AnymapImage;
    /// 
    /// // create pixel buffer
    /// let mut buffer = vec![0; 100*100*3];
    /// 
    /// // set red value in every pixel to max
    /// for x in 0..buffer.len() {
    ///     if x % 3 == 0 {
    ///     buffer[x] = 225; 
    ///     }
    /// }
    /// 
    /// // create red 100x100 PPM AnymapImage object
    /// let ppm_red = AnymapImage::ppm(buffer, 255, 100, 100).unwrap();
    /// ``` 
    pub fn ppm(buffer: Vec<u8>, saturation: usize, height: usize, width: usize) -> Result<Self, String> {
        if buffer.len() != 3*height*width { return Err(format!("Error: could not create PPM object: byte vector does not fit given dimensions: (buffer length: {}) != (given dimensions [h*w*3]: {})", buffer.len(), 3*height*width)) }
        if saturation > 255               { return Err(format!("Error: could not create PPM object: saturation too great for PPM format ({})", saturation))} 

        Ok(AnymapImage {
            buffer,
            magic_number: "ppm".to_string(),
            saturation,
            height,
            width,
        })
    }

    /// # Accessor for buffer data
    /// Clones and returns byte vector `buffer` from AnymapImage struct
    /// 
    /// Example:
    /// ```
    /// use ductr::netpbm::AnymapImage;
    /// 
    /// // create pixel buffer
    /// let buffer = vec![150; 100*100];
    /// 
    /// // create grey 100x100 PGM AnymapImage object
    /// let pgm_grey = AnymapImage::pgm(buffer.clone(), 255, 100, 100).unwrap();
    /// 
    /// // recieve returned buffer
    /// let new_buffer = pgm_grey.get_buffer();
    /// 
    /// // assert original buffer and returned buffer are equal
    /// assert!(buffer.iter()
    ///               .zip(new_buffer.iter())
    ///               .all(|(a,b)| a == b));
    /// ``` 
    pub fn get_buffer(&self) -> Vec<u8> {
        self.buffer.clone()
    }

    /// # Accessor for buffer data
    /// Clones and returns byte vector `buffer` from AnymapImage struct
    /// 
    /// Example:
    /// ```
    /// use ductr::netpbm::AnymapImage;
    /// 
    /// // create pixel buffer
    /// let buffer = vec![150; 100*100];
    /// 
    /// // create grey 100x100 PGM AnymapImage object
    /// let pgm_grey = AnymapImage::pgm(buffer.clone(), 255, 100, 100).unwrap();
    /// 
    /// // recieve returned buffer
    /// let dimensions: (usize, usize) = pgm_grey.dimensions();
    /// 
    /// assert_eq!(dimensions, (100, 100));
    /// ``` 
    pub fn dimensions(&self) -> (usize, usize) {
        (self.height, self.width)
    }
}

mod io;
mod manipulation;