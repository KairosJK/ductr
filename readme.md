# 🎨 Ductr

a small crate for reading, writing, and simple manipulation of the portable pixelmap format family

## 🖥️ How to use:

The following crate can be used by entering the following in your project's `Cargo.toml` file:

```toml
[dependencies]
vimwiki = "0.1"
```

## ⚙️ Examples

```rust
use crate::netpbm::AnymapImage;

// Creating a 100x100 black pbm image

// Prepare buffer to be written to pbm format
let buffer = vec![1; 100*100];

// Create AnymapImage object with pbm constructor
let pbm_black = AnymapImage::pbm(pixel_map, 100, 100).unwrap();

// Write pbm as binary file 
pbm_black.write_as_binary("pbm_black_binary.pbm").expect("Error: could not to binary file.");
```
```rust
use crate::netpbm::AnymapImage;

// Inverting the colors of a given ppm image

// Create AnymapImage object from binary ppm image file
let mut x = AnymapImage::read_from_binary("test/.ppm").unwrap();

// Invert the image
x.invert_image();

// Write ppm as binary file
x.write_as_binary("test/newoutput.pnm").unwrap();
```

## 📌 Other Information on the format

- [@Tsoding - "Procedural Graphics in C"](https://www.youtube.com/watch?v=kT-Mz87-HcQ) for highlighting the utility of a simple format such as the ppm family for easier graphics debugging

- [PPM Format Description](https://oceancolor.gsfc.nasa.gov/staff/norman/seawifs_image_cookbook/faux_shuttle/pbm.html)

## 👤 Authored by 

Jonathan Kocevar

## 📝 License

This project is licensed under the terms of the [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.en.html).