use std::io;
use std::process;
mod perceptron;

fn main() {
    let mut p = perceptron::Perceptron::new(2);

    // NAND
    // let training_data = vec![
    //     perceptron::TrainingData {
    //         input: (0.0, 0.0, 1.0),
    //         expected: 1.0,
    //     },
    //     perceptron::TrainingData {
    //         input: (0.0, 1.0, 1.0),
    //         expected: 1.0,
    //     },
    //     perceptron::TrainingData {
    //         input: (1.0, 0.0, 1.0),
    //         expected: 1.0,
    //     },
    //     perceptron::TrainingData {
    //         input: (1.0, 1.0, 1.0),
    //         expected: 0.0,
    //     },
    // ];

    //AND
    // let training_data = vec![
    //     perceptron::TrainingData {
    //         input: (0.0, 0.0, 1.0),
    //         expected: 0.0,
    //     },
    //     perceptron::TrainingData {
    //         input: (0.0, 1.0, 1.0),
    //         expected: 0.0,
    //     },
    //     perceptron::TrainingData {
    //         input: (1.0, 0.0, 1.0),
    //         expected: 0.0,
    //     },
    //     perceptron::TrainingData {
    //         input: (1.0, 1.0, 1.0),
    //         expected: 1.0,
    //     },
    // ];

    // OR
    let training_data = vec![
        perceptron::TrainingData {
            input: (0.0, 0.0, 1.0),
            expected: 0.0,
        },
        perceptron::TrainingData {
            input: (0.0, 1.0, 1.0),
            expected: 1.0,
        },
        perceptron::TrainingData {
            input: (1.0, 0.0, 1.0),
            expected: 1.0,
        },
        perceptron::TrainingData {
            input: (1.0, 1.0, 1.0),
            expected: 1.0,
        },
    ];

    println!("Training!");
    p.train(&1000, &0.02, &training_data);

    let v: Vec<f64> = vec![0.0, 0.0, 1.0];
    println!("Input: {:?} Guess: {}", v, p.guess(&v));

    let v: Vec<f64> = vec![1.0, 0.0, 1.0];
    println!("Input: {:?} Guess: {}", v, p.guess(&v));

    let v: Vec<f64> = vec![0.0, 1.0, 1.0];
    println!("Input: {:?} Guess: {}", v, p.guess(&v));

    let v: Vec<f64> = vec![1.0, 1.0, 1.0];
    println!("Input: {:?} Guess: {}", v, p.guess(&v));

    println!("{:?}", p);
}
