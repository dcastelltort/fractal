use std::ops::{Add, Sub};
use std::clone::Clone;
use std::f64;

#[derive(Debug, Clone)]
pub struct RGB {
    r : f64,
    g : f64,
    b : f64
}

impl PartialEq for RGB {

    fn eq(&self, other: &RGB) -> bool {
        let rdiff = (self.r - other.r).abs();
        let gdiff = (self.g - other.g).abs();
        let bdiff = (self.b - other.b).abs();

        ( (rdiff < f64::EPSILON) && (gdiff < f64::EPSILON) && (bdiff < f64::EPSILON))
    }

    fn ne(&self, other: &RGB) -> bool { 
        !self.eq(other)
    }
}

impl RGB {
    pub fn new(r: f64, g: f64, b: f64) -> RGB{
        RGB{r : r, g : g, b : b}
    }
}

impl Sub<RGB> for RGB {
    type Output = RGB;

    fn sub(self, other: RGB) -> RGB {
        RGB {r: self.r - other.r, g: self.g - other.g, b: self.b - other.b}
    }
}

impl Add<RGB> for RGB {
    type Output = RGB;

    fn add(self, other: RGB) -> RGB {
        RGB {r: self.r + other.r, g: self.g + other.g, b: self.b + other.b}
    }
}

#[test]
fn test_creation() {
    let rgb = RGB::new(1.1,2.2,3.3);
    assert!(rgb.r == 1.1 && rgb.g == 2.2 && rgb.b == 3.3);
}

#[test]
fn test_ops() {
    let rgb1 = RGB::new(1.1,2.2,3.3);
    let rgb2 = RGB::new(1.0,2.0,3.0);
    let rgb3 = RGB::new(0.1,0.2,0.3);

    assert!((rgb1.clone() - rgb2.clone()) == rgb3);
    assert!(rgb2.clone() + rgb3.clone() == rgb1);
    assert!(rgb1 == RGB{r : 1.1, g : 2.2, b : 3.3});
    assert!(rgb1 != rgb2);
}