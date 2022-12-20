pub mod manipulation {
    
    use crate::netpbm::AnymapImage;

    #[allow(dead_code)]
    impl AnymapImage {
        pub fn invert_image(&mut self) {
            self.buffer = self.buffer.iter().map(|x| !x).collect::<Vec<u8>>()
        }
    }
}