use std::f64::consts::E;
use std::fs::File;
use crate::matrix::Matrix;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Network {
    pub weights: Vec<Matrix>,
    pub biases: Vec<Matrix>,
    #[serde(skip)]
    last_run: Vec<Matrix>,
    #[serde(skip)]
    learning_rate: f64
}

impl Network {
    pub fn new(layers: &[usize], learning_rate: f64) -> Self {
        let mut weights = Vec::new();
        let mut biases = Vec::new();

        for i in 0..layers.len() - 1 {
            weights.push(Matrix::random(layers[i], layers[i] * layers[i + 1]));
            biases.push(Matrix::random(1, layers[i + 1]));
        }

        Self {
            weights, biases, learning_rate,
            last_run: vec![]
        }
    }

    pub fn feed_forward(&mut self, inputs: &[f64]) -> Vec<f64> {
        self.last_run = vec![Matrix::from(1, inputs)];

        for i in 0..self.weights.len() {
            self.last_run.push(self.weights[i]
                .mul(&self.last_run[i])
                .add(&self.biases[i])
                .map(|x| 1.0 / (1.0 + E.powf(-x)))
            );
        }

        self.last_run.last().unwrap().transpose().data
    }

    pub fn back_propagate(&mut self, outputs: &[f64], targets: &[f64]) {
        let parsed = Matrix::from(1, outputs);
        let mut errors = Matrix::from(1, targets).sub(&parsed);
        let mut gradients = parsed.map(|x| x * (1.0 - x));

        for i in (0..self.weights.len()).rev() {
            gradients = gradients
                .dot_product(&errors)
                .map(|x| x * self.learning_rate);

            self.weights[i] = self.weights[i].add(&gradients.mul(&self.last_run[i].transpose()));
            self.biases[i] = self.biases[i].add(&gradients);

            errors = self.weights[i].transpose().mul(&errors);
            gradients = self.last_run[i].map(|x| x * (1.0 - x));
        }
    }

    pub fn train(&mut self, inputs: &[Vec<f64>], targets: &[Vec<f64>], gens: u32) {
        for i in 1..=gens {
            if gens < 100 || i % (gens / 100) == 0 {
                println!("Generation {i} of {gens}");
            }

            let start = rand::thread_rng().gen_range(0..59900);
            let inputs = &inputs[start..start +100];
            let targets = &targets[start..start +100];

            for j in 0..inputs.len() {
                let outputs = self.feed_forward(&inputs[j]);
                self.back_propagate(&outputs, &targets[j]);
            }
        }
    }

    pub fn save(&self, file: &str) -> Result<(), std::io::Error> {
        let file = File::create(file)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }

    pub fn load(file: &str) -> Result<Self, std::io::Error> {
        let file = File::open(file)?;
        let network: Network = serde_json::from_reader(file)?;
        Ok(network)
    }
}
