//! a small struct crate for reading, writing, and simple manipulation of the portable pixelmap format family
//! 
//! ## Design
//! This crate was made with the focus of quick and easy debugging as its main goal. Printing byte arrays quickly to a visual file can be great when debugging basic graphics and image manipulation algorithms, and the Portable pixelmap family format is one of the quickest to set up.
//! 
//! In the future this crate may extend to other raster file formats, such as JPEG or BMP
//! 
//! ## Quick Examples
//! ```no_run
//! use ductr::AnymapImage;
//!
//! // Creating a 100x100 black pbm image
//! 
//! // Prepare buffer to be written to pbm format
//! let buffer = vec![1; 100*100];
//! 
//! // Create AnymapImage object with pbm constructor
//! let pbm_black = AnymapImage::pbm(buffer, 100, 100).unwrap();
//! 
//! // Write pbm as binary file 
//! pbm_black.write_as_binary("pbm_black_binary.pbm").expect("Error: could not to binary file.");
//! ```
//! ```no_run
//! use ductr::AnymapImage;
//! 
//! // Inverting the colors of a given ppm image
//! 
//! // Create AnymapImage object from binary ppm image file
//! let mut cat = AnymapImage::read_from_binary("tests/images/cat_binary.ppm").expect("Error: could not read from binary file");
//! 
//! // Invert the image
//! cat.invert();
//! 
//! // Write ppm as binary file
//! cat.write_as_binary("tests/images/cat_inverted.pnm").expect("Error: could not write to binary file");
//!```
//!
//! ## Other Information on the format
//! - [@Tsoding - "Procedural Graphics in C"](https://www.youtube.com/watch?v=kT-Mz87-HcQ) for highlighting the utility of a simple format such as the ppm family for easier graphics debugging
//! - [PPM Format Description](https://oceancolor.gsfc.nasa.gov/staff/norman/seawifs_image_cookbook/faux_shuttle/pbm.html)

#[derive(Debug, Clone)]
#[allow(dead_code)]
/// Image struct for PBM, PGM, and PPM formats
pub struct AnymapImage {
    buffer: Vec<u8>,
    magic_number: String,
    saturation: usize,
    height: usize,
    width: usize,
}

mod io;
mod manipulation;

impl AnymapImage {

    /// # Constructor for the PBM format
    /// Properly encodes the AnymapImage struct into a suitable PBM image
    /// 
    /// # Rules for arguments:
    /// - length of `buffer` must equal the product of the width and height inputs
    /// - `buffer` must consist only of bytes equal to 0 or 1
    /// 
    /// # Example:
    /// ```
    /// use ductr::AnymapImage;
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
    /// # Rules for arguments:
    /// - length of `buffer` must equal the product of the width and height inputs
    /// - `saturation` argument must fall in the range of `0..255`
    /// 
    /// # Example:
    /// ```
    /// use ductr::AnymapImage;
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
    /// # Rules for arguments:
    /// - `buffer` must hold 3 bytes for each pixel (8 bits per color channel)
    /// - length of `buffer` must equal the product of the width and height inputs multiplied by 3 (h*w*3)
    /// - `saturation` argument must fall in the range of `0..255`
    /// 
    /// # Example:
    /// ```
    /// use ductr::AnymapImage;
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

    /// Clones and returns byte vector `buffer` from AnymapImage struct
    /// 
    /// # Example:
    /// ```
    /// use ductr::AnymapImage;
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

    /// returns dimensions as (height, width) tuple
    /// 
    /// # Example:
    /// ```
    /// use ductr::AnymapImage;
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
