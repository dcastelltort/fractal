extern crate byteorder;

mod bitmap;
mod rgb;

fn main() {
    
    
    let rgb1 = rgb::RGB::new(1.1,2.2,3.3);
    let rgb2 = rgb::RGB::new(1.0,2.0,3.0);
    let rgb3 = rgb::RGB::new(0.1,0.2,0.3);

    let rgb4 = rgb1.clone() - rgb2.clone();
    let rgb5 = rgb2 + rgb3;
    
    
    
    let w = 1024;
    let h = 780;
    let mut b = bitmap::Bitmap::new(w, h);
 
   for x in 0..w  {
        for y in 0..h {
            b.set_pixel(x, y, 0,0,255);
        }
    }

    b.set_pixel(0, 0, 128,128,128);
    b.set_pixel(w/2, h/2, 255,0,0);
    println!("{:?}", b.get_pixel(w/2, h/2));

    b.write(String::from("test.bmp") );
}
