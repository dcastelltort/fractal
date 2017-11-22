use std::mem;
use std::io;
use std::io::Write;
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::BufWriter;
use std::fs::File;

#[derive(Debug)]
pub struct BitmapFileHeader {
    header: [u8; 2],
    file_size: u32,
    reserved: i32,
    data_offset: u32
}
impl BitmapFileHeader {
   pub fn new() -> BitmapFileHeader {
        BitmapFileHeader{
            header : ['B' as u8, 'M' as u8],
            file_size: 0,
            reserved: 0,
            data_offset: 0
        }
    }
}

#[test]
fn test_bitmap_file_header() {
    let bh = BitmapFileHeader::new();
    assert!((bh.header[0] == 'B' as u8) && (bh.header[1] == 'M' as u8));
    assert!(bh.file_size == 0);
    assert!(bh.reserved == 0);
    assert!(bh.data_offset == 0);
    assert!(mem::size_of::<BitmapFileHeader>() == 16);
}

pub struct BitmapInfoHeader {
    header_size: u32,
    width : i32,
    height: i32,
    planes: i16,
    bits_per_pixel: i16,
    compression: i32,
    data_size: u32,
    horizontal_resolution: u32,
    vertical_resolution: u32,
    colors: i32,
    important_colors: i32
}

impl BitmapInfoHeader {
    pub fn new(width: i32, heigth: i32) -> BitmapInfoHeader {
        BitmapInfoHeader{
            header_size : mem::size_of::<BitmapInfoHeader>() as u32,
            width : width,
            height: heigth,
            planes : 1,
            bits_per_pixel : 24,
            compression : 0,
            data_size : 0,
            horizontal_resolution : 2400,
            vertical_resolution : 2400,
            colors : 0,
            important_colors : 0
            }
    }
}

#[test]
fn test_bitmap_info_header() {
    let bi = BitmapInfoHeader::new(1024, 780);
    assert!(bi.width == 1024);
    assert!(bi.height == 780);
    assert!(bi.header_size == 40);
    assert!(bi.planes == 1);
    assert!(bi.bits_per_pixel == 24);
    assert!(bi.compression == 0);
    assert!(bi.data_size == 0);
    assert!(bi.horizontal_resolution == 2400);
    assert!(bi.vertical_resolution == 2400);
    assert!(bi.colors == 0);
    assert!(bi.important_colors == 0);
}


pub struct Bitmap {
    width: i32,
    height: i32,
    pixels: Vec<u8>
}

impl Bitmap {
    pub fn new(width: i32, height: i32) -> Bitmap {
        let size = (width * height * 3) as usize;
        Bitmap{
            width : width,
            height: height,
            pixels: vec![0; size]
        }
    }

    pub fn set_pixel(self: &mut Bitmap, x: i32, y: i32, red: u8, green: u8,blue: u8) {
        let pos = self.index_from_coord(x,y) as usize;
        self.pixels[pos] = blue;
        self.pixels[pos+1] = green;
        self.pixels[pos+2] = red;
    }

    #[allow(dead_code)]
    pub fn get_pixel(self: &Bitmap, x: i32, y: i32) -> (u8, u8, u8) {
        let index = self.index_from_coord( x, y);
        (self.pixels[index+2], self.pixels[index+1], self.pixels[index])
    }

    fn index_from_coord(self: &Bitmap, x: i32, y: i32) -> usize {
        let index = ((y * 3) * self.width + (x * 3)) as usize;
        index
    }

    pub fn write(self: &Bitmap, filename: String) -> io::Result<()> {
        
        let mut bfh = BitmapFileHeader::new();
        let mut bih = BitmapInfoHeader::new(self.width, self.height);

        bih.width = self.width;
        bih.height = self.height;
        bih.data_size = (self.width * self.height * 3) as u32;

        bfh.data_offset = 14 + 40; //file header size + bitmap info header size;
        bfh.file_size = bfh.data_offset + bih.data_size;

        let f = File::create(filename)?;
        {
            let mut writer = BufWriter::new(f);

            //write file header
            let mut bfh_wtr = vec![];
            bfh_wtr.write_u8(bfh.header[0])?;
            bfh_wtr.write_u8(bfh.header[1])?;
            bfh_wtr.write_u32::<LittleEndian>(bfh.file_size)?;
            bfh_wtr.write_i32::<LittleEndian>(bfh.reserved)?;
            bfh_wtr.write_u32::<LittleEndian>(bfh.data_offset)?;
            writer.write(&bfh_wtr)?;
            
            //write DIB header
            let mut bih_wtr = vec![];
            bih_wtr.write_u32::<LittleEndian>(bih.header_size)?;
            bih_wtr.write_i32::<LittleEndian>(bih.width)?;
            bih_wtr.write_i32::<LittleEndian>(bih.height)?;
            bih_wtr.write_i16::<LittleEndian>(bih.planes)?;
            bih_wtr.write_i16::<LittleEndian>(bih.bits_per_pixel)?;
            bih_wtr.write_i32::<LittleEndian>(bih.compression)?;
            bih_wtr.write_u32::<LittleEndian>(bih.data_size)?;
            bih_wtr.write_u32::<LittleEndian>(bih.horizontal_resolution)?;
            bih_wtr.write_u32::<LittleEndian>(bih.vertical_resolution)?;
            bih_wtr.write_i32::<LittleEndian>(bih.colors)?;
            bih_wtr.write_i32::<LittleEndian>(bih.important_colors)?;
            writer.write(&bih_wtr)?;

            //write data
            writer.write(&self.pixels)?;
            
        } 

        Ok(())
    }
  
}

#[test]
fn test_bitmap() {
    let mut b = Bitmap::new(1024, 780);
    assert!(b.pixels.len() == 1024 * 780 * 3);
    
    b.set_pixel(0, 0, 255,255,255);
    assert!(b.get_pixel(0,0) == (255,255,255));

    b.set_pixel(1023, 779, 128, 128, 128);
    assert!(b.get_pixel(1023,779) == (128,128,128));

}
