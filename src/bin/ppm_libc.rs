extern crate libc;

use std::os::raw::{c_char,c_int};

extern  {
    pub fn ppma_read(input_name: *const c_char, xsize : &mut c_int, ysize : &mut c_int, rgb_max : &mut c_int, r : *mut *mut c_int,
          g : *mut *mut c_int,  b : *mut *mut c_int);
    
    pub fn ppma_write(file_out_name : *const c_char, xsize : c_int, ysize : c_int, r : *mut c_int, g : *mut c_int, b : *mut c_int) -> c_int;
}

#[cfg(test)]
mod test{
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_ppm_read(){
        let mut xsize : c_int = 0;
        let mut ysize : c_int = 0;
        let mut rgb_max : c_int = 0;
        let mut r : *mut c_int = std::ptr::null_mut();
        let mut g : *mut c_int = std::ptr::null_mut();
        let mut b : *mut c_int = std::ptr::null_mut();

        let mut r_write = vec![7, 14, 23];
        let mut b_write = vec![91, 32, 43];
        let mut g_write = vec![43, 56, 32];

        unsafe {ppma_write("test_read_ppm.ppm\0".as_ptr() as *const c_char, 3 as c_int, 1 as c_int,
        r_write.as_mut_ptr() as *mut c_int, g_write.as_mut_ptr() as *mut c_int, b_write.as_mut_ptr() as *mut c_int);}

        unsafe { ppma_read("test_read_ppm.ppm\0".as_ptr() as *const c_char, &mut xsize,
        &mut ysize , &mut rgb_max, &mut r, &mut g, &mut b) };
    }

    #[test]
    fn test_ppm_write(){
        let mut r = vec![7, 14, 23];
        let mut b = vec![91, 32, 43];
        let mut g = vec![43, 56, 32];

        unsafe {ppma_write("test_write_ppm.ppm\0".as_ptr() as *const c_char, 3 as c_int, 1 as c_int,
         r.as_mut_ptr() as *mut c_int, g.as_mut_ptr() as *mut c_int, b.as_mut_ptr() as *mut c_int);}

         fs::remove_file(Path::new("test_write_ppm.ppm"));
    }


}