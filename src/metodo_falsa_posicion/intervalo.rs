use core::f64;

#[derive(Debug, Copy, Clone)]
pub struct Intervalo(pub f64, pub f64);

impl Intervalo {
    pub fn punto_medio(&self) -> f64 {
        (self.0 + self.1) / 2.0
    }
}

