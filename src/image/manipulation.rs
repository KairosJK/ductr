pub mod manipulation {
    
    use crate::image::Image;

    #[allow(dead_code)]
    impl Image {
        pub fn invert_image(&mut self) {
            self.buffer = self.buffer.iter().map(|x| !x).collect::<Vec<u8>>()
        }
    }
}