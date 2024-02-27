#[derive(Debug, Copy, Clone)]
pub struct Parameters {
    pub tolerance: f64,
    pub dimension: usize,
    pub population: usize,
    pub iterations: usize,
    pub selection: usize,
    pub elitism: usize,
    pub mutation: f64,
    pub crossover: f64,
    pub verbosity: usize,
    pub interval: usize,
}
