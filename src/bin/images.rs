use pixels::Pixels;
#[path = "pixels.rs"]
mod pixels;

use std::fs::File;
use std::io::{self, BufReader, Error, Lines};
use std::io::prelude::*;
use std::path::Path;
use std::os::raw::{c_char,c_int};

#[derive(Clone, Debug)]
pub struct Image{
    pub pixels : Vec<Pixels>,
    pub heigth : usize,
    pub width : usize,
    pub fileType : String,
    pub maxValue : usize,
}

impl  Image {

    /// create a new image
    /// 
    /// # Arguments
    /// 
    /// * `pixels` - a vec contain the pixel in the image
    /// * `heigth` - heigth of the image
    /// * `width` - width of the image
    /// * `fileType` - the type of the image
    /// * `maxValue` - the max value of the pixels in the image
    /// 
    /// # Example
    /// 
    /// ```
    /// let mut pixels = Vec::new();
    /// pixels.push(Pixels::new(7, 91, 43));
    /// pixels.push(Pixels::new(14, 32, 56));
    /// pixels.push(Pixels::new(23, 43, 32));
    /// let image = Image::new(pixels, 1, 3, "P3".to_string(), 91);
    /// ```
    pub fn new(pixels : Vec<Pixels>, heigth : usize, width : usize, fileType : String, maxValue : usize) -> Image{
        Image{pixels, heigth, width, fileType, maxValue}
    }

    ///load the image from the file ppm
    /// 
    /// # Arguments
    /// 
    /// * `filename` - the name of the file
    /// 
    /// # Example
    /// 
    /// ```
    /// let mut pixels = Vec::new();
    /// pixels.push(Pixels::new(7, 91, 43));
    /// pixels.push(Pixels::new(14, 32, 56));
    /// pixels.push(Pixels::new(23, 43, 32));
    /// let image = Image::new(pixels, 1, 3, "P3".to_string(), 91);
    /// image.save(Path::new("test_image.ppm"))?;
    /// let image_load = Image::new_with_file(Path::new("test_image.ppm"));
    /// assert_eq!(image, image_load?);
    ///
    /// fs::remove_file(Path::new("test_image.ppm"))?;
    /// ```
    pub fn new_with_file(filename: &Path) -> std::io::Result<Image>{
        let f = File::open(filename)?;
        let f = BufReader::new(f);
        //let mut contents = fs::read_to_string(filename).expect("files not found");
        let mut pixels = Vec::new();

        let mut lines = f.lines();
        let lineType = find_line_not_commentaire(&mut lines);
        let mut lineSize = find_line_not_commentaire(&mut lines);
        let mut lineMaxNumber = find_line_not_commentaire(&mut lines);

        let heigth = find_number(&mut lineSize).unwrap();
        let width = find_number(&mut lineSize).unwrap();
        let maxValue = find_number(&mut lineMaxNumber).unwrap();

        for line in lines {
            let mut line = line.unwrap();
            while &line != ""{
                if &(*line)[0..1] == "#"{
                    break
                }
                let pixel = find_pixels(&mut line);
                pixels.push(pixel);
            }
        }

        Ok(Image{ pixels , heigth, width, fileType : lineType, maxValue})
    }

    ///save the image in a file of ppm
    /// 
    /// # Arguments
    /// 
    /// * `filename` - the name of the file
    /// 
    /// # Example
    /// 
    /// ```
    /// let mut pixels = Vec::new();
    /// pixels.push(Pixels::new(7, 91, 43));
    /// pixels.push(Pixels::new(14, 32, 56));
    /// pixels.push(Pixels::new(23, 43, 32));
    /// let image = Image::new(pixels, 1, 3, "P3".to_string(), 91);
    /// image.save(Path::new("test_image.ppm"))?;
    /// let image_load = Image::new_with_file(Path::new("test_image.ppm"));
    /// assert_eq!(image, image_load?);
    ///
    /// fs::remove_file(Path::new("test_image.ppm"))?;
    /// ```
    pub fn save(&self, filename : &Path) -> std::io::Result<()>{
        let mut f = File::create(filename)?;
        let mut buf = String::new();
        buf = buf + &self.fileType + "\r\n";
        buf = buf + "#" + &filename.to_str().unwrap() + "\r\n";
        buf = buf + &format!("{} {}",self.heigth, self.width) + "\r\n"; 
        buf = buf + &self.maxValue.to_string() + "\r\n";
        let mut index = 0;
        let mut pixel_string = String::new();
        for pixel in &self.pixels{
            pixel_string = pixel_string + &pixel.display() + " ";
            index += 1;
            if index > 2{
                pixel_string += "\r\n";
                index = 0;
            }
        }
        buf += &pixel_string;

        f.write_all(buf.as_bytes()).unwrap();
        Ok(())
    }

    ///transform the RGB image to the gray image
    /// 
    /// # Example
    /// 
    /// ```
    /// let image_gray = image.grayscale();
    /// ```
    pub fn grayscale(&self) -> Image{
        let mut ret_pixels = Vec::new();
        let pixels = self.pixels.clone();

        for pixel in pixels{
            ret_pixels.push(pixel.grayscale());
        }

        let fileType = self.fileType.to_string();

        Image::new(ret_pixels, self.heigth, self.width, fileType, self.maxValue)
    }
}

/// remove the espace in the start of the string
/// 
/// # Arguments
/// 
/// `contents` - the string for remove
/// 
/// # Example
/// ```
/// let mut test_str = "   a".to_string();
/// filter_start_espace(&mut test_str);
/// assert_eq!("a", test_str);
/// ```
fn filter_start_espace(contents : &mut String){
    let mut index = 0;
    for c in contents.chars() {
        if c != ' ' {
            *contents = contents[index..].to_string();
            return;
        }
        index += 1;
    }
    *contents = "".to_string();
}

/// find next line from the file that not only contain the commentaire
/// 
/// # Arguments
/// 
/// `line` - a line read by file
/// 
/// # Example
/// ```
/// let mut f_w = File::create(Path::new("test.txt"))?;
/// f_w.write_all("first\n#sathey\nhello!".as_bytes())?;
///
/// let f_r = File::open(Path::new("test.txt"));
/// let f_r = BufReader::new(f_r.unwrap());
///
/// let mut lines = f_r.lines();
///
/// assert_eq!("first".to_string(), find_line_not_commentaire(&mut lines));
/// assert_eq!("hello!".to_string(), find_line_not_commentaire(&mut lines));
///
/// fs::remove_file(Path::new("test.txt"))?;
/// ```
fn find_line_not_commentaire(lines : &mut Lines<BufReader<File>>) -> String{
    let mut line = lines.next();
    while !line.is_none() {

        let res_string = filter_commentaire(&line.unwrap().unwrap());
        if res_string != ""{
            return res_string;
        }
        line = lines.next();
    }
    "".to_string()
}

/// remove the commentaire of the string
/// 
/// # Arguments
/// 
/// `contents` - the string for remove
/// 
/// # Example
/// ```
/// let mut test_str = "   a".to_string();
/// filter_start_espace(&mut test_str);
/// assert_eq!("a", test_str);
/// ```
fn filter_commentaire(contents : &String) -> String{
    let mut index = 0;
    for c in contents.chars() {
        if c == '#'{
            return contents[..index].to_string();
        }
        index += 1;
    }
    contents.clone()
}

/// find the first number from the string
/// 
/// # Arguments
/// 
/// `contents` - the string for find
/// 
/// # Example
/// 
/// ```
/// let mut test_str = "3 41 5".to_string();
/// assert_eq!(3, find_number(&mut test_str)?);
/// assert_eq!(41, find_number(&mut test_str)?);
/// ```
fn find_number(contents : &mut String) -> Result<usize, std::num::ParseIntError>{
    let mut index = 0;
    for c in contents.chars(){
        if c == ' ' {
            let n = &contents[..index].to_string().parse::<usize>()?;
            *contents = contents[index..].to_string();
            filter_start_espace(contents);
            return Ok(*n);
        }
        index += 1;
    }
    let ret = contents.parse::<usize>().unwrap();
    *contents = "".to_string();
    Ok(ret)
}

/// find the first pixel from the string
/// 
/// # Arguments
/// 
/// `contents` - the string for find
/// 
/// # Example
/// 
/// ```
/// extern crate ppm;
/// let mut test_pixel = "4 11 4 12 9 11".to_string();
/// let pixel1 = ppm::Pixels::new(4, 11, 4);
/// let pixel2 = ppm::Pixels::new(12, 9, 11);
/// assert_eq!(pixel1, find_pixels(&mut test_pixel));
/// assert_eq!(pixel2, find_pixels(&mut test_pixel));
/// ```
fn find_pixels(mut contents : &mut String) -> Pixels{
    
    let red = find_number(&mut contents).unwrap();
    let green = find_number(&mut contents).unwrap();
    let blue = find_number(&mut contents).unwrap();

    Pixels::new(red as u8, green as u8, blue as u8)
}

impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        let mut pixels_self = self.pixels.clone();
        let mut pixels_other = other.pixels.clone();
        while !pixels_self.is_empty() {
            if ! (pixels_self.pop() == pixels_other.pop()){
                return false;
            }
        }
        self.fileType == other.fileType &&
        self.maxValue == other.maxValue &&
        self.heigth == self.heigth &&
        self.width == self.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_filter_start_espace(){
        let mut test_str = "   a".to_string();
        filter_start_espace(&mut test_str);
        assert_eq!("a", test_str);
    }

    #[test]
    fn test_find_number()-> Result<(), std::num::ParseIntError>{
        let mut test_str = "3 41 5".to_string();
        assert_eq!(3, find_number(&mut test_str)?);
        assert_eq!(41, find_number(&mut test_str)?);
        
        assert_eq!(5, find_number(&mut test_str)?);

        Ok(())
    }

    #[test]
    fn test_find_pixel(){
        let mut test_pixel = "4 11 4 12 9 11".to_string();
        let pixel1 = Pixels::new(4, 11, 4);
        let pixel2 = Pixels::new(12, 9, 11);
        assert_eq!(pixel1, find_pixels(&mut test_pixel));
        assert_eq!(pixel2, find_pixels(&mut test_pixel));
    }

    #[test]
    fn test_filter_commentaire(){
        let test_str = "hello#sayhay".to_string();
        assert_eq!("hello", filter_commentaire(&test_str));
    }

    #[test]
    fn test_find_line_not_commentaire() -> std::io::Result<()>{
        let mut f_w = File::create(Path::new("test_find_line.txt"))?;
        f_w.write_all("first\n#sathey\nhello!".as_bytes())?;

        let f_r = File::open(Path::new("test_find_line.txt"));
        let f_r = BufReader::new(f_r.unwrap());

        let mut lines = f_r.lines();

        assert_eq!("first".to_string(), find_line_not_commentaire(&mut lines));
        assert_eq!("hello!".to_string(), find_line_not_commentaire(&mut lines));

        fs::remove_file(Path::new("test_find_line.txt"))?;

        Ok(())
    }

    #[test]
    fn test_save() -> std::io::Result<()>{
        let mut pixels = Vec::new();
        pixels.push(Pixels::new(7, 91, 43));
        pixels.push(Pixels::new(14, 32, 56));
        pixels.push(Pixels::new(23, 43, 32));
        let image = Image::new(pixels, 1, 3, "P3".to_string(), 91);
        image.save(Path::new("test_save_image.ppm"))?;
        let image_load = Image::new_with_file(Path::new("test_save_image.ppm"));
        assert_eq!(image, image_load?);

        fs::remove_file(Path::new("test_save_image.ppm"))?;

        Ok(())
    }

    #[test]
    fn test_grayscale(){
        let mut pixels = Vec::new();
        pixels.push(Pixels::new(7, 91, 43));
        pixels.push(Pixels::new(14, 32, 56));
        pixels.push(Pixels::new(23, 43, 32));
        let image = Image::new(pixels, 1, 3, "P3".to_string(), 91);

        let mut pixels_grayscale = Vec::new();
        for pixel in &image.pixels{
            pixels_grayscale.push(pixel.grayscale());
        }
        let image_grayscale = Image::new(pixels_grayscale, 1, 3, "P3".to_string(), 91);
        let image_compare = image.grayscale();

        assert_eq!(image_grayscale, image_compare);
    }
    
}

