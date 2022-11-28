pub struct ScaledCanvas {
    pub width: f64,
    pub height: f64,
}

impl ScaledCanvas {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}
