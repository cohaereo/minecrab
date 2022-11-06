pub trait FixedPoint {
    fn to_f64(&self) -> f64;
    fn from_f64(v: f64) -> Self;
}

impl FixedPoint for i32 {
    fn to_f64(&self) -> f64 {
        (*self as f64) / 32.0
    }

    fn from_f64(v: f64) -> Self {
        (v * 32.0) as Self
    }
}

impl FixedPoint for i8 {
    fn to_f64(&self) -> f64 {
        (*self as f64) / 32.0
    }

    fn from_f64(v: f64) -> Self {
        (v * 32.0) as Self
    }
}
