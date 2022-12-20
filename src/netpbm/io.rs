pub mod io {

    use std::{fs::{File, self}, path::Path, io::Write};
    use crate::netpbm::AnymapImage;

    #[allow(dead_code)]
    impl AnymapImage {
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
    
        pub fn read_from_binary(path: &str) -> Result<AnymapImage, String> {
    
            // open file
            let file = match fs::read(Path::new(path)) {
                Ok(file) => file,
                Err(e) => return Err(format!("Error: could not read file: {}", e)),
            };
    
            let (header, byte_vector) = parse_header_to_slice(&file);
    
            let mut parsed_header: Vec<usize> = Vec::new();
            for x in &header[1..] {
                let parse_val = match x.parse::<usize>() {
                    Ok(parse_val) => parse_val,
                    Err(_) => return Err("The header format is not valid".to_string()), 
                };
                parsed_header.push(parse_val);
            }
    
            let parsed_image: Result<AnymapImage, String>;
            match header[0] {
                "P4" => parsed_image = AnymapImage::pbm(byte_vector, parsed_header[1], parsed_header[0]),
                "P5" => parsed_image = AnymapImage::pgm(byte_vector, parsed_header[2], parsed_header[1], parsed_header[0]),
                "P6" => parsed_image = AnymapImage::ppm(byte_vector, parsed_header[2], parsed_header[1], parsed_header[0]),
                _ => return Err("Error: no valid magic number detected".to_string()),
            }
            parsed_image
        }
    
        pub fn read_from_ascii(path: &str) -> Result<AnymapImage, String> {
            // open file
            let mut file = match fs::read_to_string(Path::new(path)) {
                Ok(file) => file,
                Err(e) => return Err(format!("Error: could not create file: {:?}", e)),
            };
    
            let delim_vec = file.split_ascii_whitespace().collect::<Vec<&str>>();

            Err("f u".to_string())
        }
    }

    fn parse_header_to_slice(byte_vec: &Vec<u8>) -> (Vec<&str>, Vec<u8>) {
        let mut start_idx = 0;
        let mut i = 0;
        let mut delim_vector: Vec<&str> = Vec::new();
        while i < byte_vec.len() {
            if byte_vec[i] == '\n' as u8 || byte_vec[i] == ' ' as u8 {
                match std::str::from_utf8(&byte_vec[start_idx..i]) {
                    Ok(header_data) => delim_vector.push(header_data.trim()),
                    Err(_) => break
                }
                start_idx = i;
            }
            i += 1;
        }
        (delim_vector, byte_vec[(start_idx+1)..].to_vec())
    }

}