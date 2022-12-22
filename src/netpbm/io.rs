pub mod io {

    use std::{fs::{File, self}, path::Path, io::Write};
    use crate::netpbm::AnymapImage;

    #[allow(dead_code)]
    impl AnymapImage {

        /// # Write AnymapImage to ascii file
        /// Writes given image to file argument in standard ascii format
        /// 
        /// Rules for arguments:
        /// - Must be a valid path to write to
        /// - Can take any PNM formatted file (PBM, PGM, PPM)
        /// 
        /// Example:
        /// ```
        /// use ductr::netpbm::AnymapImage;
        /// 
        /// // create pixel buffer
        /// let mut buffer = vec![0; 100*100];
        /// 
        /// // create white 100x100 PBM AnymapImage object
        /// let ppm_white = AnymapImage::pbm(buffer, 100, 100).unwrap();
        /// 
        /// ppm_white.write_as_ascii("tests/images/white.ppm").expect("Could not create file");
        /// 
        /// ``` 
        pub fn write_as_ascii(&self, path: &str) -> Result<(), String> {
            // open file
            let mut file = match File::create(Path::new(path)) {
                Ok(file) => file,
                Err(e) => return Err(format!("Error: could not create file: {:?}", e)),
            };
    
            // create metadata header
            let mut pixel_width: usize = 1;  // holds width of single pixel (ie. 1 for PBM & PGM, 3 for PPM)
            let mut header = String::new(); // holds header metadata
            match self.magic_number.as_str() {
                "pbm" => header = format!("P1\n{} {}\n", self.width, self.height),
                "pgm" => header = format!("P2\n{} {}\n{}\n", self.width, self.height, self.saturation), // add saturation metadata if format is PGM
                "ppm" => {
                    header = format!("P3\n{} {}\n{}\n", self.width, self.height, self.saturation); // add saturation metadata if format is PPM
                    pixel_width = 3; // increase pixel width value to 3 for all three color channels
                }
                _ => (),
            }                  
    
            // create whitespaced pixel buffer
            let mut pixel_buffer = String::new();                       // make string holding single pixel buffer to write
            for (idx, elem) in self.buffer.iter().enumerate() {
                
                pixel_buffer.push_str(format!("{}", elem).as_str());    // push next byte to write
                
                if (idx+1) != (self.width*self.height*pixel_width) {            // check if on last line, if true, do not add newline
                    if (idx+1) % (self.width*pixel_width) == 0 {                // check if index has wrote entire horizontal slice of image
                        pixel_buffer.push_str("\n")                     // write next line to new line
                    } else {
                        pixel_buffer.push_str(" ")                      // add space delim for next byte
                    }
                }
            }
    
            // write header information
            if let Err(e) = file.write_all(header.as_bytes()) {
                return Err(format!("Error: could not write to file: {:?}", e));
            }
    
            // write pixel buffer
            if let Err(e) = file.write_all(pixel_buffer.as_bytes()) {
                return Err(format!("Error: could not write to file {:?}", e));
            }
    
            Ok(())
        }
    
        /// # Write AnymapImage to binary file
        /// Writes given image to file argument in standard binary format
        /// 
        /// Rules for arguments:
        /// - Must be a valid path to write to
        /// - Can take any PNM formatted file (PBM, PGM, PPM)
        /// 
        /// Example:
        /// ```
        /// use ductr::netpbm::AnymapImage;
        /// 
        /// // create pixel buffer
        /// let mut buffer = vec![1; 100*100];
        /// 
        /// // create black 100x100 PBM AnymapImage object
        /// let ppm_black = AnymapImage::pbm(buffer, 100, 100).unwrap();
        /// 
        /// ppm_black.write_as_binary("tests/images/black.ppm").expect("Could not create file");
        /// 
        /// ``` 
        pub fn write_as_binary(&self, path: &str) -> Result<(), String> {
            
            // open file
            let mut file = match File::create(Path::new(path)) {
                Ok(file) => file,
                Err(e) => return Err(format!("Error: could not create file: {:?}", e)),
            };
            
            // create metadata header and pixel buffer
            let mut header = String::new();                                   // holds header metadata
            let mut pixel_buffer: Vec<u8> = self.buffer.clone();                      // set new buffer to be written
    
            match self.magic_number.as_str() {
                "pbm" => {
                    header = format!("P4\n{} {}\n", self.width, self.height);
                    pixel_buffer = Vec::new();
                    for x in self.buffer                                // delimiter byte array into width based chunks
                                        .chunks(self.width)
                                        .collect::<Vec<&[u8]>>() {
                        for y in x.to_vec().chunks(8) {             // delimiter chunk into 8 byte based chunks
                            let mut byte: u8 = 0;                                     // set new byte 
                            for j in y {
                                byte = (byte << 1) + j;                               // set given u8 to bit inside new byte
                            }
                            for _ in 0..(8-y.len()) {                                 // pad byte on right if new byte holds less than 8 image bits
                                byte = (byte << 1) + 0;                               // pad with zeros (ie. [0xFF, 0xFF] becomes C0 instead of 02)
                            }
                            pixel_buffer.push(byte);                                  // push byte to new buffer
                        }
                    }
                },
                "pgm" => {
                    header = format!("P5\n{} {}\n{}\n", self.width, self.height, self.saturation); // add saturation metadata if format is PGM
                },
                "ppm" => {
                    header = format!("P6\n{} {}\n{}\n", self.width, self.height, self.saturation); // add saturation metadata if format is PPM
                }
                _ => (),
            }
    
            // write header information
            if let Err(e) = file.write_all(header.as_bytes()) {
                return Err(format!("Error writing to file: {:?}", e));
            }
    
            // write pixel buffer
            if let Err(e) = file.write_all(&pixel_buffer) {
                return Err(format!("Error writing to file {:?}", e));
            }
    
            Ok(())
        }
    
        /// # Read AnymapImage from binary file
        /// Read given file to new image in standard binary format
        /// 
        /// Rules for arguments:
        /// - Must be a valid path to read from
        /// - File at path must be in valid binary format
        /// - Can take any PNM formatted file (PBM, PGM, PPM)
        /// 
        /// Example:
        /// ```
        /// use ductr::netpbm::AnymapImage;
        /// 
        /// // create black PBM AnymapImage object from file
        /// let ppm_black = AnymapImage::read_from_binary("tests/images/black.ppm").expect("Could not read file");
        /// ``` 
        pub fn read_from_binary(path: &str) -> Result<AnymapImage, String> {
    
            // open file
            let file = match fs::read(Path::new(path)) {
                Ok(file) => file,
                Err(e) => return Err(format!("Error: could not read file: {}", e)),
            };
    
            let magic_num = std::str::from_utf8(&file[..2]).map_err(|_| "Error: could not read file: magic number was not detected".to_string())?;
            let header_args: usize;
            match magic_num {
                "P4" => header_args = 3,
                "P5" | "P6" => header_args = 4,
                _ => return Err("Error: could not read file: valid magic number was not detected".to_string()),
            }

            let (header, byte_vector) = parse_header_to_slice(&file, header_args);
    
            let mut parsed_header: Vec<usize> = Vec::new();
            for x in &header[1..] {
                let parse_val = match x.parse::<usize>() {
                    Ok(parse_val) => parse_val,
                    Err(_) => return Err("The header format is not valid".to_string()), 
                };
                parsed_header.push(parse_val);
            }
    
            let parsed_image: Result<AnymapImage, String>;
            match magic_num {
                "P4" => parsed_image = AnymapImage::pbm(realign_byte_buffer(byte_vector, parsed_header[0]), parsed_header[1], parsed_header[0]),
                "P5" => parsed_image = AnymapImage::pgm(byte_vector, parsed_header[2], parsed_header[1], parsed_header[0]),
                "P6" => parsed_image = AnymapImage::ppm(byte_vector, parsed_header[2], parsed_header[1], parsed_header[0]),
                _ => unreachable!(),
            }
            parsed_image
        }
    
        /// # Read AnymapImage from ascii file
        /// Read given file to new image in standard ascii format
        /// 
        /// Rules for arguments:
        /// - Must be a valid path to read from
        /// - File at path must be in valid ascii format
        /// - Can take any PNM formatted file (PBM, PGM, PPM)
        /// 
        /// Example:
        /// ```
        /// use ductr::netpbm::AnymapImage;
        /// 
        /// // create white PBM AnymapImage object from file
        /// let ppm_white = AnymapImage::read_from_ascii("tests/images/white.ppm").expect("Could not read file");
        /// ``` 
        pub fn read_from_ascii(path: &str) -> Result<AnymapImage, String> {
            
            // open file
            let file = match fs::read_to_string(Path::new(path)) {
                Ok(file) => file,
                Err(e) => return Err(format!("Error: could not read file: {:?}", e)),
            };
    
            let delim_vec = file.split_ascii_whitespace().collect::<Vec<&str>>();

            let header_args: usize;
            match delim_vec.get(0) {
                Some(&"P1") => header_args = 3,
                Some(&"P2" | &"P3") => header_args = 4,
                _ => return Err("Error: could not read file: magic number was not detected".to_string()),
            }

            let mut parsed_header: Vec<usize> = Vec::new();
            let mut header_args_count: usize = 1;
            let mut idx = 1;
            while header_args_count < header_args && idx < delim_vec.len() {
                if delim_vec[idx].chars().all(|x| x.is_ascii_digit()) {
                    let parsed_header_info = delim_vec[idx].parse::<usize>()
                                                                  .map_err(|_| "Error: header holds non-standard arguments".to_string())?;
                    header_args_count += 1;
                    parsed_header.push(parsed_header_info);
                }
                idx += 1;
            }

            let mut byte_vector: Vec<u8> = Vec::new();
            for byte in delim_vec[idx..].iter() {
                let parsed_byte = byte.parse::<u8>()
                                          .map_err(|_| "Error: byte array holds non-standard elements".to_string())?;
                byte_vector.push(parsed_byte);
            }

            let parsed_image: Result<AnymapImage, String>;
            match delim_vec.get(0) {
                Some(&"P1") => parsed_image = AnymapImage::pbm(byte_vector, parsed_header[1], parsed_header[0]),
                Some(&"P2") => parsed_image = AnymapImage::pgm(byte_vector, parsed_header[2], parsed_header[1], parsed_header[0]),
                Some(&"P3") => parsed_image = AnymapImage::ppm(byte_vector, parsed_header[2], parsed_header[1], parsed_header[0]),
                _ => unreachable!(),
            }
            parsed_image
        }
    }

    /// Helper function for parsing header from binary file
    /// Returns a string vector containing header info, and a byte vector containing the buffer
    fn parse_header_to_slice(byte_vec: &Vec<u8>, mut arg_count: usize) -> (Vec<&str>, Vec<u8>) {
        let mut start_idx = 0;
        let mut i = 0;
        let mut delim_vector: Vec<&str> = Vec::new();
        while i < byte_vec.len() {
            if byte_vec[i] == '\n' as u8 || byte_vec[i] == ' ' as u8 {
                match std::str::from_utf8(&byte_vec[start_idx..i]) {
                    Ok(header_data) => delim_vector.push(header_data.trim()),
                    Err(_) => break
                }
                if arg_count == 0 { break } else { arg_count -= 1; }
                start_idx = i;
            }
            i += 1;
        }
        (delim_vector, byte_vec[(start_idx+1)..].to_vec())
    }

    // Helper function for parsing PBM misaligned bytes
    // Converts given buffer from 8 bits a byte to 1 bit a byte
    // Returns new vector which is formatted to a byte per pixel
    fn realign_byte_buffer(byte_buffer: Vec<u8>, width: usize) -> Vec<u8> {
        let mut new_buffer: Vec<u8> = Vec::new();
        let byte_alignment: (usize, usize) = ((width / 8) + 1, width % 8);
        for (idx, byte) in byte_buffer.iter().enumerate() {
            let mut bits_to_grab = 8;
            if (idx +1) % byte_alignment.0 == 0 { bits_to_grab = byte_alignment.1 }
            for idy in ((8 - bits_to_grab)..8).rev() {
                if byte & (1 << idy) != 0 {
                    new_buffer.push(1);
                } else {
                    new_buffer.push(0);
                }
            }
        }
        new_buffer
    }

}