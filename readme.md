# 🎨 Ductr

a small struct crate for reading, writing, and simple manipulation of the portable pixelmap format family

## 🧠 Design

This crate was made with the focus of quick and easy debugging as its main goal. Printing byte arrays quickly to a visual file can be great when debugging basic graphics and image manipulation algorithms, and the Portable pixelmap family format is one of the quickest to set up.

In the future this crate may extend to other raster file formats, such as JPEG or BMP

## 🖥️ How to use:

The following crate can be used by entering the following in your project's `Cargo.toml` file:

```toml
[dependencies]
ductr = "0.0.1"
```

## ⚙️ Examples
```rust
use ductr::AnymapImage;

// Creating a 100x100 black pbm image

// Prepare buffer to be written to pbm format
let buffer = vec![1; 100*100];
 
// Create AnymapImage object with pbm constructor
let pbm_black = AnymapImage::pbm(buffer, 100, 100).unwrap();

// Write pbm as binary file 
pbm_black.write_as_binary("pbm_black_binary.pbm").expect("Error: could not to binary file.");
```
```rust
use ductr::AnymapImage;

// Inverting the colors of a given ppm image

// Create AnymapImage object from binary ppm image file
let mut cat = AnymapImage::read_from_binary("tests/images/cat_binary.ppm").expect("Error: could not read from binary file");

// Invert the image
cat.invert();

// Write ppm as binary file
cat.write_as_binary("tests/images/cat_inverted.pnm").expect("Error: could not write to binary file");
```

## 📌 Other Information on the format

- [@Tsoding - "Procedural Graphics in C"](https://www.youtube.com/watch?v=kT-Mz87-HcQ) for highlighting the utility of a simple format such as the ppm family for easier graphics debugging

- [PPM Format Description](https://oceancolor.gsfc.nasa.gov/staff/norman/seawifs_image_cookbook/faux_shuttle/pbm.html)

## 👤 Authored by 

Jonathan Kocevar

## 📝 License

This project is licensed under the terms of the [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.en.html).