
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
            histogram : vec![],
            fractal : vec![],
            bitmap : Bitmap::new(0,0),
            zoomList : ZoomList::new(0,0),
            total : 0,
            ranges : vec![],
            colors : vec![],
            rangeTotals : vec![],
            got_first_range : false
        };
        f.zoomList.add(Zoom::new(f.width / 2, f.height / 2, 4.0 / f.width as f64));
        f
    }

   pub fn get_range(&self, iterations: i32) -> i32 {
       let mut range : i32 = 0;

        for it in self.ranges.iter() {
            range = *it;
       
            if self.ranges[range as usize] > iterations {
                break;
            }
        }

        range -= 1;

        assert!(range > -1);
        assert!(range < self.ranges.len() as i32);

        range
   }

	
	pub fn add_range(&mut self, range_end: f64, rgb: RGB) {
        self.ranges.push( (range_end as i32 * mandelbrot::MAX_ITERATIONS));
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
        self.calculateIteration();
        self.calculateTotalIterations();
        self.calculateRangeTotals();
        self.drawFractal();
        self.writeBitmap(name);
    }


    fn calculateIteration(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let coords = self.zoomList.do_zoom(x, y);

                let iterations : i32 = mandelbrot::get_iterations(coords.0,coords.1);

                self.fractal[(y * self.width + x) as usize] = iterations;

                if iterations != mandelbrot::MAX_ITERATIONS {
                    self.histogram[iterations as usize] += 1;
                }

            }
	    }
    }

	fn calculateTotalIterations(&mut self) {
        
        for i in 0..mandelbrot::MAX_ITERATIONS {
            self.total += self.histogram[i as usize];
        }
    }

	fn calculateRangeTotals(&mut self) {
        
        let mut rangeIndex : usize = 0;

        for i in 0..mandelbrot::MAX_ITERATIONS {
            let pixels : i32 = self.histogram[i as usize];

            if i >= self.ranges[rangeIndex+1]  {
                rangeIndex += 1;
            }

            self.rangeTotals[rangeIndex] += pixels;
        }
    }

	fn drawFractal(&mut self) {
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

	fn writeBitmap(&self, name : String) {
       self.bitmap.write(name);
    }

}