pub mod manipulation {

    use crate::netpbm::AnymapImage;

    #[allow(dead_code)]
    impl AnymapImage {

        /// # Pixel color invert
        /// Properly inverts AnymapImage struct into a suitable inverted image
        /// 
        /// Example:
        /// ```
        /// use ductr::netpbm::AnymapImage;
        /// 
        /// // create pixel buffer
        /// let buffer = vec![1; 100*100];
        /// 
        /// // create black 100x100 PBM AnymapImage object
        /// let mut pbm_black_to_white = AnymapImage::pbm(buffer, 100, 100).unwrap();
        /// 
        /// pbm_black_to_white.invert();
        /// 
        /// ``` 
        pub fn invert(&mut self) {
            match self.magic_number.as_str() {
                "pbm" => self.buffer = self.buffer.iter().map(|x| 1 - x).collect::<Vec<u8>>(),
                _ => self.buffer = self.buffer.iter().map(|x| !x).collect::<Vec<u8>>()
            }
        }

        /// # Pixel filter application
        /// Properly adds filter layer to every pixel of a given AnymapImage
        /// 
        /// Rules for arguments:
        /// - `filter` buffer length must be lesser than or equal to self buffer length
        /// - AnymapImage formats must match (ie. (PPM & PPM), (PGM & PGM))
        /// - PBM formats are not accepted
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
        /// let filter_red = 
        ///  
        /// ``` 
        pub fn add_filter(&mut self, filter: AnymapImage) -> Result<(), String> {
            if self.buffer.len() < filter.buffer.len() { return Err("Error: filter buffer is larger than self".to_string()) }
            if self.magic_number != filter.magic_number { return Err("Error: filter image format differs to self".to_string()) }
            if self.magic_number == "pbm" { return Err("Error: pbm images cannot have a filter applied".to_string()) }

            for idx in 0..self.buffer.len() {
                self.buffer[idx] = self.buffer[idx].wrapping_add(*filter.buffer.get(idx)
                                                   .unwrap_or(&0));
            }
            Ok(())
        }

        /// # Pixel color to greyscale
        /// Properly greyscales all pixels in the AnymapImage struct
        /// 
        /// Rules for arguments:
        /// - while it will take any AnymapImage format without error, it only performs greyscaling on PPM color images
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
        /// let mut ppm_red_greyscale = AnymapImage::ppm(buffer, 255, 100, 100).unwrap();
        /// 
        /// // convert red square to greyscale
        /// ppm_red_greyscale.greyscale();
        /// ``` 
        pub fn greyscale(&mut self) {
            match self.magic_number.as_str() {
                "ppm" => {
                    for idx in (0..(self.buffer.len()-3)).step_by(3) {
                        let greyscale_pixel: usize = (self.buffer[idx] as usize + 
                                                     self.buffer[idx+1] as usize +
                                                     self.buffer[idx+2] as usize) / 3;

                        self.buffer[idx]    = greyscale_pixel as u8;
                        self.buffer[idx+1]  = greyscale_pixel as u8;
                        self.buffer[idx+2]  = greyscale_pixel as u8;
                    }
                },
                _ => ()
            }
        }
    }
}