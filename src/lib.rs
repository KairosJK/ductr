#! Produce a .ppm (P3) image formatted file given a byte array and width and height

mod image;

#[cfg(test)]
mod tests {

    use crate::image::Image;

    #[test]
    fn make_pbm_binary() {
        let pixel_map = vec![1; 100*100];
      let x = Image::pbm(pixel_map, 100, 100).unwrap();
        x.write_as_binary("test/black_binary.pbm").expect("Error");
    }  

    #[test]
    fn make_pbm_ascii() {
        let pixel_map = vec![1; 100*100];
        let x = Image::pbm(pixel_map, 100, 100).unwrap();
        x.write_as_ascii("test/black_ascii.pbm").expect("Error");
    }

    #[test]
    fn make_pgm_binary() {
        let pixel_map = vec![150; 100*100];
        let x = Image::pgm(pixel_map, 255, 100, 100).unwrap();
        x.write_as_binary("test/grey_binary.pgm").expect("Error");
    }

    #[test]
    fn make_pgm_ascii() {
        let pixel_map = vec![150; 100*100];
        let x = Image::pgm(pixel_map, 255, 100, 100).unwrap();
        x.write_as_ascii("test/grey_ascii.pgm").expect("Error");
    }

    #[test]
    fn make_ppm_binary() {
        let mut pixel_map = vec![0; 100*3*100];
        for x in 0..pixel_map.len() {
            if x % 3 == 0 {
                pixel_map[x] = 225;
            }
        }
        let x = Image::ppm(pixel_map, 255, 100, 100).unwrap();
        x.write_as_binary("test/red_binary.ppm").expect("Error");
    }

    #[test]
    fn make_ppm_ascii() {
        let mut pixel_map = vec![0; 100*3*100];
        for x in 0..pixel_map.len() {
            if x % 3 == 0 {
                pixel_map[x] = 225;
            }
        }
        let x = Image::ppm(pixel_map, 255, 100, 100).unwrap();
        x.write_as_ascii("test/red_ascii.ppm").expect("Error");
    }

    #[test]
    fn check_dimensions() {
        let pixel_map = vec![0; 20*3*20];
        let x = Image::ppm(pixel_map, 255, 20, 20).unwrap();
        assert_eq!((20,20), x.dimensions());
    }

    #[test]
    fn check_buffer() {
        let pixel_map = vec![0; 5*5];
        let x = Image::pbm(pixel_map, 5, 5).unwrap();
        assert_eq!(vec![0; 5*5], x.get_buffer());
    }

    #[test]
    fn read_file() {
        let mut x = Image::read_from_binary("test/shinji.ppm").unwrap();
        x.invert_image();
        x.write_as_binary("test/newoutput.pnm").unwrap();
    }

}
