
use bitmap::{Bitmap};
use zoom::{Zoom, ZoomList};
use rgb::{RGB};
use mandelbrot;
use std::string;

pub struct FractalCreator {

	width : i32,
	height : i32,
	histogram : Vec<i32>,
    fractal : Vec<i32>,
	bitmap : Bitmap,
	zoomList : ZoomList,
	total : i32,

	ranges: Vec<i32>,
	colors : Vec<RGB>,
	rangeTotals : Vec<i32>,

	got_first_range : bool
}

impl FractalCreator {
    pub fn new(width : i32, height: i32) -> FractalCreator {
        let mut f = FractalCreator {
            width: width,
            height: height,
            histogram : vec![0;mandelbrot::MAX_ITERATIONS as usize],
            fractal : vec![0;(width * height) as usize],
            bitmap : Bitmap::new(width,height),
            zoomList : ZoomList::new(width,height),
            total : 0,
            ranges : vec![],
            colors : vec![],
            rangeTotals : vec![],
            got_first_range : false
        };
        f.zoomList.add(Zoom::new(f.width / 2, f.height / 2, 4.0 / f.width as f64));
        f
    }

   pub fn get_range(&self, iterations: i32) -> Option<i32> {
       let mut range : usize = 0;
       let mut found = false;

        for i in 0..self.ranges.len() {
            range = i;
       
            if self.ranges[range] > iterations {
                found = true;
                break;
            }
        }
        
        assert!(range < self.ranges.len());

        if (found) {
            Some(range as i32)
        }
        else {
            None
        }
   }

	
	pub fn add_range(&mut self, range_end: f64, rgb: RGB) {
        self.ranges.push( (range_end * mandelbrot::MAX_ITERATIONS as f64) as i32);
	    self.colors.push(rgb);

        if self.got_first_range {
            self.rangeTotals.push(0);
        }

        self.got_first_range = true;

    }

	pub fn add_zoom(&mut self, zoom: Zoom) {
        self.zoomList.add(zoom);
    }

	pub fn run(&mut self, name: String) {
        self.calculate_iteration();
        self.calculate_total_iterations();
        self.calculate_range_totals();
        self.draw_fractal();
        self.write_bitmap(name);
    }


    fn calculate_iteration(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let coords = self.zoomList.do_zoom(x, y);

                let iterations : i32 = mandelbrot::get_iterations(coords.0,coords.1);

                let i = (y * self.width + x) as usize;
                assert!(i < self.fractal.capacity(), "i : {} , cap: {}", i, self.fractal.capacity());

                self.fractal[i] = iterations;

                if iterations != mandelbrot::MAX_ITERATIONS {
                    self.histogram[iterations as usize] += 1;
                }

            }
	    }
    }

	fn calculate_total_iterations(&mut self) {
        
        for i in 0..mandelbrot::MAX_ITERATIONS {
            self.total += self.histogram[i as usize];
        }
    }

	fn calculate_range_totals(&mut self) {
        
        let mut rangeIndex : usize = 0;

        for i in 0..mandelbrot::MAX_ITERATIONS {
            let pixels : i32 = self.histogram[i as usize];

            if i >= self.ranges[rangeIndex+1]  {
                rangeIndex += 1;
            }

            self.rangeTotals[rangeIndex] += pixels;
        }
    }

	fn draw_fractal(&mut self) {
        let startColor = RGB::new(0.0, 0.0, 0.0);
	    let endColor = RGB::new(0.0, 0.0, 255.0);
	    let colorDiff = endColor - startColor.clone();

        for y in 0..self.height {
            for x in 0..self.width {

                let mut red : u8 = 0;
                let mut green : u8 = 0;
                let mut blue : u8 = 0;

                let iterations : i32 = self.fractal[(y * self.width + x) as usize];

                if iterations != mandelbrot::MAX_ITERATIONS {

                    let mut hue : f64 = 0.0;

                    for i in 0..iterations {
                        hue += self.histogram[i as usize] as f64 / self.total as f64;
                    }

                    red = (startColor.r + colorDiff.r * hue) as u8;
                    green = (startColor.g + colorDiff.g * hue) as u8;
                    blue = (startColor.b + colorDiff.b * hue) as u8;
                }

                self.bitmap.set_pixel(x, y, red, green, blue);
            }
        }
    }

	fn write_bitmap(&self, name : String) {
       self.bitmap.write(name);
    }

}

#[test]
fn test_get_ranges() {
    let mut fractalCreator = FractalCreator::new(800, 600);

    assert!(fractalCreator.get_range(999), None);

	fractalCreator.add_range(0.0, RGB::new(0.0, 0.0, 0.0));
	fractalCreator.add_range(0.3, RGB::new(255.0, 0.0, 0.0));
	fractalCreator.add_range(0.5, RGB::new(255.0, 255.0, 0.0));
	fractalCreator.add_range(1.0, RGB::new(255.0, 255.0, 255.0));


    assert!(fractalCreator.get_range(999), Some(3));
    assert!(fractalCreator.get_range(999), Some(3));
    assert!(fractalCreator.get_range(999), Some(3));
}