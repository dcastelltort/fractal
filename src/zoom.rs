
#[derive(Clone)]
struct Zoom {
	x : i32, 
	y: i32,
	scale: f64
}

impl Zoom {
    pub fn new(x: i32, y: i32, scale: f64) -> Zoom {
        Zoom {x : x, y: y, scale : scale}
    }
}

struct ZoomList {

	x_center : f64,
	y_center: f64, 
	scale : f64, 
    width : i32, 
	height: i32, 
	zooms : Vec<Zoom>
}

impl ZoomList {
    pub fn new(width: i32, height: i32) -> ZoomList {
        ZoomList{x_center : 0.0, y_center: 0.0, width: width, height: height, scale: 1.0, zooms: vec![]}
    }

	pub fn add(self: &mut ZoomList, zoom : &Zoom) {
        self.zooms.push(zoom.clone());

        self.x_center += (zoom.x - self.width / 2) as f64 * self.scale;
        self.y_center += -(zoom.y - self.height / 2) as f64 * self.scale;

        self.scale *= zoom.scale;
    }

	pub fn do_zoom(self: &ZoomList, x: i32, y: i32) -> (f64, f64) {
 
        let x_fractal = (x - self.width / 2) as f64 * self.scale + self.x_center;
	    let y_fractal = (y - self.height / 2) as f64 * self.scale + self.y_center;
        (x_fractal, y_fractal)

    } 
}
