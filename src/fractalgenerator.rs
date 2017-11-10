
use bitmap::{Bitmap};
use zoom::{Zoom, ZoomList};
use rgb::{RGB};

struct FractalCreator {

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

	got_first_fange : bool
}

impl FractalCreator {
    pub fn new() -> FractalCreator {
        FractalCreator {
            width: 0,
            height: 0,
            histogram : vec![],
            fractal : vec![],
            bitmap : Bitmap::new(0,0),
            zoomList : ZoomList::new(0,0),
            total : 0,
            ranges : vec![],
            colors : vec![],
            rangeTotals : vec![],
            got_first_fange : false
        }
    }

   pub fn get_range(&self, iterations: i32) -> i32 {
       unimplemented!();
       0
   }

	
	pub fn add_range(&mut self, rangeEnd: f64, rgb: &RGB) {
        unimplemented!();
    }

	pub fn add_zoom(&mut self, zoom: &Zoom) {

    }

	pub fn run(&mut self, name: &String) {
        unimplemented!();
    }


    fn calculateIteration() {
        unimplemented!();
    }

	fn calculateTotalIterations() {
        unimplemented!();
    }

	fn calculateRangeTotals() {
        unimplemented!();
    }

	fn drawFractal() {
        unimplemented!();
    }

	fn writeBitmap(name : &String) {
        unimplemented!();
    }

}