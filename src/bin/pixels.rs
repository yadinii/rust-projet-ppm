
#[derive(Clone, Copy, Debug)]
pub struct Pixels{
    pub red : u8,
    pub green : u8,
    pub blue : u8,
}


impl Pixels {

    /// create a new pixel
    /// 
    /// # Arguments
    /// 
    /// * `red` - the number of red for pixel
    /// * `green` - the number of green for pixel
    /// * `blue` - the number of blue for pixel
    /// 
    /// # Example
    /// 
    /// ```
    /// let pixel = Pixels::new(12, 11, 9);
    /// ```
    pub fn new(red: u8, green: u8, blue: u8) -> Pixels {
        Pixels{red, green, blue}
    }

    /// display the pixel
    /// 
    /// # Example
    /// 
    /// ```
    /// let pixel = Pixels::new(12, 11, 9);
    /// assert_eq!("12 11 9".to_string(), pixel.display());
    /// ```
    pub fn display(self) -> String{
        format!("{} {} {}",self.red,self.green,self.blue)
    }

    fn invert(&mut self){

    }

    /// transform the RGB pixel to gray pixel
    /// 
    /// # Example
    /// 
    /// ```
    /// let pixel = Pixels::new(34, 56, 102);
    /// pixel.grayscale()
    /// ```
    pub fn grayscale(&self) -> Self{
        let grep = ((0.299 as u8) * self.red) + ((0.587 as u8) * &self.green) + ((0.114 as u8)* &self.blue);
        Pixels::new(grep, grep, grep)
    }
}

impl PartialEq for Pixels {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red &&
        self.green == other.green &&
        self.blue == other.blue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_disply(){
        let pixel = Pixels::new(12, 11, 9);
        assert_eq!("12 11 9".to_string(), pixel.display());
    }

    #[test]
    fn test_pixel_eq(){
        let pixel1 = Pixels::new(12, 11, 9);
        let pixel2 = Pixels::new(12, 11, 9);
        let pixel3 = Pixels::new(11, 3, 9);

        assert_eq!(pixel1, pixel2);
        assert_ne!(pixel1, pixel3);
    }

    #[test]
    fn test_pixel_grayscale(){
        let pixels_origin = Pixels::new(34, 56, 102);
        let pixels_gray = Pixels::new(34 * (0.299 as u8), 56 * (0.587 as u8), 102 * (0.114 as u8));
        assert_eq!(pixels_gray, pixels_origin.grayscale());
    }


}


