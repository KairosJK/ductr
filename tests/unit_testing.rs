
#[cfg(test)]
mod tests {

    use ductr::*;

    #[test]
    fn make_pbm_binary() {
        let pixel_map = vec![1; 100*100];
        let x = AnymapImage::pbm(pixel_map, 100, 100).unwrap();
        x.write_as_binary("tests/images/make_pbm_binary.pbm").expect("Error");
    }

    #[test]
    fn make_pbm_ascii() {
        let pixel_map = vec![1; 100*100];
        let x = AnymapImage::pbm(pixel_map, 100, 100).unwrap();
        x.write_as_ascii("tests/images/make_pbm_ascii.pbm").expect("Error");
    }

    #[test]
    fn make_pgm_binary() {
        let pixel_map = vec![150; 100*100];
        let x = AnymapImage::pgm(pixel_map, 255, 100, 100).unwrap();
        x.write_as_binary("tests/images/make_pgm_binary.pgm").expect("Error");
    }

    #[test]
    fn make_pgm_ascii() {
        let pixel_map = vec![150; 100*100];
        let x = AnymapImage::pgm(pixel_map, 255, 100, 100).unwrap();
        x.write_as_ascii("tests/images/make_pgm_ascii.pgm").expect("Error");
    }

    #[test]
    fn make_ppm_binary() {
        let mut pixel_map = vec![0; 100*3*100];
        for x in 0..pixel_map.len() {
            if x % 3 == 0 {
                pixel_map[x] = 225;
            }
        }
        let x = AnymapImage::ppm(pixel_map, 255, 100, 100).unwrap();
        x.write_as_binary("tests/images/make_ppm_binary.ppm").expect("Error");
    }

    #[test]
    fn make_ppm_ascii() {
        let mut pixel_map = vec![0; 100*3*100];
        for x in 0..pixel_map.len() {
            if x % 3 == 0 {
                pixel_map[x] = 225;
            }
        }
        let x = AnymapImage::ppm(pixel_map, 255, 100, 100).unwrap();
        x.write_as_ascii("tests/images/make_ppm_ascii.ppm").expect("Error");
    }

    #[test]
    fn check_dimensions() {
        let pixel_map = vec![0; 20*3*20];
        let x = AnymapImage::ppm(pixel_map, 255, 20, 20).unwrap();
        assert_eq!((20,20), x.dimensions());
    }

    #[test]
    fn check_buffer() {
        let pixel_map = vec![0; 5*5];
        let x = AnymapImage::pbm(pixel_map, 5, 5).unwrap();
        assert_eq!(vec![0; 5*5], x.get_buffer());
    }

    #[test]
    fn read_file_binary() {
        let x = AnymapImage::read_from_binary("tests/images/cat_binary.ppm").unwrap();
        x.write_as_binary("tests/images/read_file_binary.pnm").unwrap();
    }

    #[test]
    fn make_greyscale() {
        let mut x = AnymapImage::read_from_ascii("tests/images/cat.ppm").unwrap();
        x.greyscale();
        x.write_as_binary("tests/images/make_greyscale.pnm").unwrap();
    }

    #[test]
    fn add_filter() {
        let mut red = AnymapImage::read_from_ascii("tests/images/red.ppm").unwrap();
        let green = AnymapImage::read_from_ascii("tests/images/green.ppm").unwrap();
        assert_eq!((), red.add_filter(green).unwrap());
        assert_eq!((), red.write_as_ascii("tests/images/add_filter.pnm").unwrap());
    }

    #[test]
    fn read_file_ascii() {
        let x = AnymapImage::read_from_ascii("tests/images/cat.ppm").unwrap();
        x.write_as_ascii("tests/images/read_file_ascii.pnm").unwrap();
    }

}
