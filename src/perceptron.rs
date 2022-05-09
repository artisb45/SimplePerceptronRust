use rand::prelude::*;
use rand::seq::SliceRandom;

// trait Sigmoid {
//     fn sigmoid(&self) -> f64;
// }

// impl Sigmoid for f64 {
//     fn sigmoid(&self) -> f64 {
//         1.0 / (1.0 + std::f64::consts::E.powf(-self))
//     }
// }

trait Heaviside {
    fn heaviside(&self) -> f64;
}

// Implement heaviside() for f64
impl Heaviside for f64 {
    fn heaviside(&self) -> f64 {
        let r = (*self >= 0.0) as i8;
        r as f64
    }
}

fn dot(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

pub struct TrainingData {
    pub input: (f64, f64, f64),
    pub expected: f64,
}

#[derive(Debug)]
pub struct Perceptron {
    size: i32,
    weights: Vec<f64>,
}

impl Perceptron {
    pub fn new(size: i32) -> Perceptron {
        let mut rng = rand::thread_rng();
        let weights: Vec<f64> = (0..size + 1).map(|_| rng.gen()).collect();
        Perceptron { size, weights }
    }

    pub fn guess(&self, input: &Vec<f64>) -> f64 {
        let result = dot(input, &self.weights).heaviside();
        result
    }

    pub fn train(&mut self, iterations: &i32, learning_rate: &f64, data: &Vec<TrainingData>) {
        let mut rng = rand::thread_rng();
        for _ in 0..*iterations {
            let &TrainingData { input: x, expected } = data.choose(&mut rng).unwrap();
            let mut inputs: Vec<f64> = Vec::new();
            inputs.push(x.0);
            inputs.push(x.1);
            inputs.push(x.2);

            let result = self.guess(&inputs);
            let error = expected - result;

            for i in 0..self.size + 1 {
                self.weights[i as usize] += learning_rate * error * inputs[i as usize];
            }
        }
    }
}
