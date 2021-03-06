
use bitmap::{Bitmap};
use zoom::{Zoom, ZoomList};
use rgb::{RGB};
use mandelbrot;

use std::error::Error;
use std::fs::File;
use std::io;

use serde_json;

use crossbeam;
use std::sync::mpsc;

use hprof::{Profiler};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RangeColor {
    range_end : f64,
    color : RGB
}

impl RangeColor {
    pub fn new(range_end: f64, color : RGB) -> RangeColor {
        RangeColor{ 
            range_end : range_end, 
            color : color
            }
    }
}

#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct Fractal {
	width : i32,
	height : i32,
	histogram : Vec<i32>,
    fractal : Vec<u32>,
	zoom_list : ZoomList,
	total : i32,
	ranges_colors: Vec<RangeColor>,
	range_totals : Vec<i32>,
	got_first_range : bool
}


impl Fractal {
    pub fn new(width : i32, height: i32) -> Fractal {
        let mut f = Fractal {
            width: width,
            height: height,
            histogram : vec![0;mandelbrot::MAX_ITERATIONS as usize],
            fractal : vec![0;(width * height) as usize],
            zoom_list : ZoomList::new(width,height),
            total : 0,
            ranges_colors : vec![],
            range_totals : vec![],
            got_first_range : false
        };
        f.zoom_list.add(Zoom::new(f.width / 2, f.height / 2, 4.0 / f.width as f64));
        f
    }

    #[allow(dead_code)]
    pub fn get_range(&self, iterations: i32) -> Option<i32> {
       let mut range : usize = 0;
       let mut found = false;

        for i in 0..self.ranges_colors.len() {
            range = i;

            assert!(range < self.ranges_colors.len());
            if (self.ranges_colors[range].range_end * mandelbrot::MAX_ITERATIONS as f64) as i32 > iterations {
                found = true;
                break;
            }
        }
        
        
        if found {
            Some(range as i32)
        }
        else {
            None
        }
   }

	
	pub fn add_range(&mut self, range_end: f64, rgb: RGB) {
        self.ranges_colors.push( RangeColor::new(range_end, rgb));

        if self.got_first_range {
            self.range_totals.push(0);
        }

        self.got_first_range = true;

    }

	pub fn add_zoom(&mut self, zoom: Zoom) {
        self.zoom_list.add(zoom);
    }

    fn calculate_iteration(&mut self, num_threads: u32) {
        
        let ys : Vec<_> = (0..self.height).collect();
        let (tx, rx) = mpsc::channel();
        let width = self.width.clone();
        let chunk_size : usize = (self.height as u32/num_threads) as usize;
        crossbeam::scope(|scoped| {
            for y_chunk in ys.chunks(chunk_size) {
                let tx_clone = tx.clone();
                let zoom_list = self.zoom_list.clone();
                scoped.spawn(move || {
                    for ity in y_chunk {
                        let y = *ity;
                        for x in 0..width {
                            let coords = zoom_list.do_zoom(x, y);

                            let iterations : u32 = mandelbrot::get_iterations(coords.0,coords.1);

                            let i = (y * width + x) as usize;
                            
                            
                            tx_clone.send((i, iterations)).unwrap();
                        }
                    }
                });
            }
        });
        drop(tx); //unused

        for received in rx {
            let i = received.0;
            let iterations = received.1;
            assert!(i < self.fractal.capacity(), "i : {} , cap: {}", i, self.fractal.capacity());
            self.fractal[i] = iterations;
            if iterations != mandelbrot::MAX_ITERATIONS {
                self.histogram[iterations as usize] += 1;
            }
        }

    }

	fn calculate_total_iterations(&mut self, num_threads: u32) {
        
        let mut handles = vec![];
        let chunk_size = (mandelbrot::MAX_ITERATIONS / num_threads) as usize;
        crossbeam::scope(|scope| {
            for chunk in self.histogram.chunks(chunk_size) {
                let handle = scope.spawn(move || -> i32 {
                    let mut subtotal : i32 = 0;
                    for h in chunk {
                        subtotal += h;
                    }
                    subtotal
                });
                handles.push(handle);
            }
         });

        for handle in handles {
            self.total += handle.join();
        }
    }

	fn calculate_range_totals(&mut self) {
        
        let mut range_index : usize = 0;

        for i in 0..mandelbrot::MAX_ITERATIONS {
            let pixels : i32 = self.histogram[i as usize];

            if i >= (self.ranges_colors[range_index+1].range_end * mandelbrot::MAX_ITERATIONS as f64) as u32  {
                range_index += 1;
            }

            self.range_totals[range_index] += pixels;
        }
    }

	fn draw_fractal(&self, bitmap: &mut Bitmap, num_threads : u32) {

        let (tx, rx) = mpsc::channel();
        let ys : Vec<_> = (0..self.height).collect();
        let chunk_size : usize = (self.height as u32 / num_threads) as usize;
        crossbeam::scope(|scoped| {      
            for y_chunk in ys.chunks(chunk_size) {
                let tx_clone = tx.clone();
                let width = self.width;
        
                scoped.spawn(move || {
                    let start_color = RGB::new(0.0, 0.0, 0.0);
                    let end_color = RGB::new(0.0, 0.0, 255.0);
                    let color_diff = end_color - start_color.clone();

                    for ity in y_chunk {
                        let y =  *ity;
                        for x in 0..width {
                            let mut red : u8 = 0;
                            let mut green : u8 = 0;
                            let mut blue : u8 = 0;
                            
                            
                            let iterations : u32 = self.fractal[(y * width + x) as usize];

                            if iterations != mandelbrot::MAX_ITERATIONS {

                                let mut hue : f64 = 0.0;

                                for i in 0..iterations {
                                    hue += self.histogram[i as usize] as f64 / self.total as f64;
                                }

                                red = (start_color.r + color_diff.r * hue) as u8;
                                green = (start_color.g + color_diff.g * hue) as u8;
                                blue = (start_color.b + color_diff.b * hue) as u8;
                                
                            }
                            tx_clone.send((x, y, red,green,blue)).unwrap();
                        }
                    }
                });
            }
        });
        
        drop(tx);

        for received in rx {
            let color = received;
            bitmap.set_pixel(color.0, color.1, color.2, color.3, color.4);
        }
    }

    fn render(&mut self, bitmap: &mut Bitmap, num_threads: u32) {
        let profiler = Profiler::new("render");
        
        {   
            let _p = profiler.enter("calculate_iteration");
            self.calculate_iteration(num_threads);
        }
        
        {
            let _p = profiler.enter("calculate_total_iterations");
            self.calculate_total_iterations(num_threads);
        }

        {
            let _p = profiler.enter("calculate_range_totals");
            self.calculate_range_totals();
        }
        {
            let _p = profiler.enter("draw_fractal");
            self.draw_fractal(bitmap, num_threads);
        }
        
        profiler.print_timing();
    }
}

#[test]
fn test_get_ranges() {
    let mut fractal = Fractal::new(800, 600);

    assert!(fractal.get_range(999).is_none());

	fractal.add_range(0.0, RGB::new(0.0, 0.0, 0.0));
	fractal.add_range(0.3, RGB::new(255.0, 0.0, 0.0));
	fractal.add_range(0.5, RGB::new(255.0, 255.0, 0.0));
	fractal.add_range(1.0, RGB::new(255.0, 255.0, 255.0));


    assert_eq!(fractal.get_range(299), Some(1));
    assert_eq!(fractal.get_range(499), Some(2));
    assert_eq!(fractal.get_range(999), Some(3));
}

pub struct FractalCreator {

}

impl FractalCreator {
    pub fn new() -> FractalCreator {
        FractalCreator{}
    }

    pub fn generate_fractal(&self, fractal: &mut Fractal, output_file_name: String, num_threads: u32) -> Result<(), io::Error> {
        let mut bitmap = Bitmap::new(fractal.width,fractal.height);
   
        fractal.render(&mut bitmap, num_threads);
        match bitmap.write(output_file_name) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
        }
    }
}


#[derive(Debug, Clone,Serialize, Deserialize, PartialEq)]
struct FractalFile {
    width: i32,
    height: i32,
    ranges : Vec<RangeColor>,
    zooms : Vec<Zoom>
}

impl FractalFile {
    #[allow(dead_code)]
    pub fn new() -> FractalFile {
        FractalFile {
            width : 0,
            height : 0,
            ranges : vec![],
            zooms: vec![]
        }
    }
}
pub fn fractal_from_file(filename: String, override_width: u32, override_height: u32) -> Result<Fractal, Box<Error>> {
    
    let file = File::open(filename)?;

    let mut fractal_file : FractalFile = serde_json::from_reader(file)?;

    if override_width !=0 && override_height != 0{
        fractal_file.width = override_width as i32;
        fractal_file.height = override_height as i32;
    }

    let mut fractal = Fractal::new(fractal_file.width, fractal_file.height);
    
    for range in fractal_file.ranges {
        fractal.add_range(range.range_end, range.color);
    }

    for zoom in fractal_file.zooms {
        fractal.add_zoom(zoom);
    }
    Ok(fractal)
}

#[test]
fn test_fractal_file() {
    let mut fractal_file = FractalFile::new();

    fractal_file.width = 800;
    fractal_file.height = 600;
    fractal_file.ranges.push(RangeColor::new(0.0, RGB::new(0.0, 0.0, 0.0)));
    fractal_file.ranges.push(RangeColor::new(0.3, RGB::new(255.0, 0.0, 0.0)));
    fractal_file.ranges.push(RangeColor::new(0.5, RGB::new(255.0, 255.0, 0.0)));
    fractal_file.ranges.push(RangeColor::new(1.0, RGB::new(255.0, 255.0, 255.0)));

    fractal_file.zooms.push(Zoom::new(295, 202, 0.1));
    fractal_file.zooms.push(Zoom::new(312, 304, 0.1));

    let json = serde_json::to_string(&fractal_file).unwrap();

    let ref_json = "{\"width\":800,\"height\":600,\"ranges\":[{\"range_end\":0.0,\"color\":{\"r\":0.0,\"g\":0.0,\"b\":0.0}},{\"range_end\":0.3,\"color\":{\"r\":255.0,\"g\":0.0,\"b\":0.0}},{\"range_end\":0.5,\"color\":{\"r\":255.0,\"g\":255.0,\"b\":0.0}},{\"range_end\":1.0,\"color\":{\"r\":255.0,\"g\":255.0,\"b\":255.0}}],\"zooms\":[{\"x\":295,\"y\":202,\"scale\":0.1},{\"x\":312,\"y\":304,\"scale\":0.1}]}";
    assert_eq!(json, String::from(ref_json));


    let fractal_parse_file : FractalFile = serde_json::from_str(ref_json).unwrap();
    assert_eq!(fractal_parse_file, fractal_file);
}