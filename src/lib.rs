pub use images::Image;
#[path = "bin/images.rs"]
mod images;

pub use pixels::Pixels;
#[path = "bin/pixels.rs"]
mod pixels;

#[path = "bin/ppm_libc.rs"]
mod ppm_libc;

use std::os::raw::{c_char,c_int};
use std::mem::size_of;

/// unzip the pixel to three color RGB
/// 
/// # Arguments
/// 
/// `pixels` - a vec of the pixel
/// 
/// # Example
/// 
/// ```
/// let mut pixels = Vec::new();
/// pixels.push(Pixels::new(7, 91, 43));
/// pixels.push(Pixels::new(14, 32, 56));
/// pixels.push(Pixels::new(23, 43, 32));
/// 
/// let (r, b, g) = zip_pixel(&pixels);
/// 
/// assert_eq!(r, vec![7, 14, 23]);
/// assert_eq!(b, vec![91, 32, 43]);
/// assert_eq!(g, vec![43, 56, 32]);
/// ```
fn un_zip_pixel(pixels : &Vec<Pixels>) -> (Vec<c_int>,Vec<c_int>,Vec<c_int>){
    let mut r = Vec::<c_int>::new();
    let mut g = Vec::<c_int>::new();
    let mut b = Vec::<c_int>::new();

    for pixel in pixels{
        r.push(pixel.red as c_int);
        g.push(pixel.green as c_int);
        b.push(pixel.blue as c_int);
    }

    (r,g,b)
}

/// zip three color RBG to a pixel
/// 
/// # Arguments
/// 
/// `r` - a range of color red
/// `g` - a range of color green
/// `b` - a range of color blue
/// 
/// # Example
/// 
/// ```
/// let r = vec![7 as c_int, 14 as c_int, 23 as c_int];
/// let g = vec![91 as c_int, 32 as c_int, 43 as c_int];
/// let b = vec![43 as c_int, 56 as c_int, 32 as c_int];
/// 
/// let pixels = zip_pixel(r, g, b);
/// 
/// assert_eq!(pixels[0], Pixels::new(7, 91, 43));
/// assert_eq!(pixels[1], Pixels::new(14, 32, 56));
/// assert_eq!(pixels[2], Pixels::new(23, 43, 32));
/// ```
fn zip_pixel(r : Vec::<c_int>, g : Vec::<c_int>, b : Vec::<c_int>) -> Vec::<Pixels>{
    let mut pixels = Vec::<Pixels>::new();
    for i in 0 .. r.len(){
        pixels.push(Pixels::new(r[i] as u8,g[i] as u8, b[i] as u8));
    }
    pixels
}

/// read the image ppm use the lib c
/// 
/// # Arguments
/// 
/// `file_name` - the name of file to read
/// 
/// # Example
/// 
/// ```
/// let mut pixels = Vec::new();
/// pixels.push(Pixels::new(7, 91, 43));
/// pixels.push(Pixels::new(14, 32, 56));
/// pixels.push(Pixels::new(23, 43, 32));
/// let image = Image::new(pixels, 1, 3, "P3".to_string(), 91);
/// image.save(Path::new("test_read_image_c.ppm")).unwrap();
/// let image_load : Image;
/// 
/// unsafe{
///    image_load = readPPM_libc("test_read_image_c.ppm".to_string());
/// }
/// ```
pub unsafe fn readPPM_libc(file_name : String) -> Image{
    let mut xsize : c_int = 0;
    let mut ysize : c_int = 0;
    let mut rgb_max : c_int = 0;
    let mut r : *mut c_int = std::ptr::null_mut();
    let mut g : *mut c_int = std::ptr::null_mut();
    let mut b : *mut c_int = std::ptr::null_mut();
    let mut red_vec = Vec::<c_int>::new(); 
    let mut green_vec = Vec::<c_int>::new(); 
    let mut blue_vec = Vec::<c_int>::new(); 
    unsafe { 
        ppm_libc::ppma_read((file_name + "\0").as_ptr() as *const c_char, &mut xsize,
            &mut ysize , &mut rgb_max, &mut r, &mut g, &mut b);

        let num_pixels = xsize * ysize;
        red_vec = Vec::from_raw_parts(r, num_pixels as usize, size_of::<c_int>());
        green_vec = Vec::from_raw_parts(g, num_pixels as usize, size_of::<c_int>());
        blue_vec = Vec::from_raw_parts(b, num_pixels as usize, size_of::<c_int>());
    }
    let pixels = zip_pixel(red_vec, green_vec, blue_vec);

    Image::new(pixels, ysize as usize, xsize as usize, "P3".to_string(), rgb_max as usize)
}

/// write the image ppm use the lib c
/// 
/// # Arguments
/// 
/// `file_name` - the name of file to write
/// `image`  - the image ppm to write
/// 
/// # Example
/// 
/// ```
/// let mut pixels = Vec::new();
/// pixels.push(Pixels::new(7, 91, 43));
/// pixels.push(Pixels::new(14, 32, 56));
/// pixels.push(Pixels::new(23, 43, 32));
/// let image = Image::new(pixels, 1, 3, "P3".to_string(), 91);
/// 
/// unsafe{
///     writePPM_libc("test_write_image.ppm".to_string(), &image);
/// }
/// ```
pub unsafe fn writePPM_libc(file_name : String, image : &Image){
    let (mut r, mut g, mut b) = un_zip_pixel(&image.pixels);
    unsafe {
        ppm_libc::ppma_write((file_name + "\0").as_ptr() as *const c_char, image.width as c_int, image.heigth as c_int,
            r.as_mut_ptr() as *mut c_int, g.as_mut_ptr() as *mut c_int, b.as_mut_ptr() as *mut c_int);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_un_zip_pixel(){
        let mut pixels = Vec::new();
        pixels.push(Pixels::new(7, 91, 43));
        pixels.push(Pixels::new(14, 32, 56));
        pixels.push(Pixels::new(23, 43, 32));

        let (r, b, g) = un_zip_pixel(&pixels);

        assert_eq!(r, vec![7, 14, 23]);
        assert_eq!(b, vec![91, 32, 43]);
        assert_eq!(g, vec![43, 56, 32]);
    }

    #[test]
    fn test_zip_pixel(){
        let r = vec![7 as c_int, 14 as c_int, 23 as c_int];
        let g = vec![91 as c_int, 32 as c_int, 43 as c_int];
        let b = vec![43 as c_int, 56 as c_int, 32 as c_int];

        let pixels = zip_pixel(r, g, b);

        assert_eq!(pixels[0], Pixels::new(7, 91, 43));
        assert_eq!(pixels[1], Pixels::new(14, 32, 56));
        assert_eq!(pixels[2], Pixels::new(23, 43, 32));
    }

    #[test]
    fn test_read_ppm_c(){
        let mut pixels = Vec::new();
        pixels.push(Pixels::new(7, 91, 43));
        pixels.push(Pixels::new(14, 32, 56));
        pixels.push(Pixels::new(23, 43, 32));
        let image = Image::new(pixels, 1, 3, "P3".to_string(), 91);
        image.save(Path::new("test_read_image_c.ppm")).unwrap();
        let image_load : Image;

        unsafe{
            image_load = readPPM_libc("test_read_image_c.ppm".to_string());
        }
        
        assert_eq!(image, image_load);

        fs::remove_file(Path::new("test_read_image_c.ppm")).unwrap();
    }

    #[test]
    fn test_write_ppm_c(){
        let mut pixels = Vec::new();
        pixels.push(Pixels::new(7, 91, 43));
        pixels.push(Pixels::new(14, 32, 56));
        pixels.push(Pixels::new(23, 43, 32));
        let image = Image::new(pixels, 1, 3, "P3".to_string(), 91);

        unsafe{
            writePPM_libc("test_write_image.ppm".to_string(), &image);
        }

        let image_load = Image::new_with_file(Path::new("test_write_image.ppm"));

        assert_eq!(image, image_load.unwrap());

        fs::remove_file(Path::new("test_write_image.ppm")).unwrap();
    }
}