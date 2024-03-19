use super::intervalo::Intervalo;

#[derive(Debug, Copy, Clone)]
pub struct Iteracion {
    pub intervalo: Intervalo,
    pub numero: u64,
    pub aproximacion: f64,
    pub error: f64,
}
