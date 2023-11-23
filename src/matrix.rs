use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Matrix {
    pub cols: usize,
    pub data: Vec<f64>,
}

impl Matrix {
    pub fn new(cols: usize, len: usize) -> Self {
        Self {
            cols,
            data: vec![0.0; len],
        }
    }

    pub fn from(cols: usize, data: &[f64]) -> Self {
        Matrix {
            cols,
            data: data.to_vec(),
        }
    }

    pub fn random(cols: usize, len: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut data = Vec::new();

        for _ in 0..len {
            data.push(rng.gen_range(-1.0..1.0));
        }

        Self { cols, data }
    }

    pub fn map<F>(&self, func: F) -> Self
        where
            F: Fn(f64) -> f64,
    {
        Self {
            cols: self.cols,
            data: self.data.iter().map(|&x| func(x)).collect(),
        }
    }

    pub fn transpose(&self) -> Self {
        let cols = self.data.len() / self.cols;
        let mut data = Vec::new();

        for i in 0..self.data.len() {
            data.push(self.data[(i%self.cols) * cols + i/self.cols]);
        }

        Self {
            cols,
            data
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        let data = self.data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a + b)
            .collect();

        Self {
            cols: self.cols,
            data,
        }
    }

    pub fn sub(&self, other: &Self) -> Self {
        let data = self.data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a - b)
            .collect();

        Self {
            cols: self.cols,
            data,
        }
    }

    pub fn dot_product(&self, other: &Self) -> Self {
        let data = self.data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a * b)
            .collect();

        Self {
            cols: self.cols,
            data,
        }
    }

    pub fn mul(&self, other: &Self) -> Self {
        assert_eq!(self.cols, other.data.len() / other.cols,
        "\nMatrix dimensions are incompatible for multiplication");

        let mut data = Vec::new();

        for i in 0..other.cols {
            for j in 0..self.data.len() / self.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[j + self.data.len() / self.cols * k] * other.data[i * self.cols + k];
                }
                data.push(sum);
            }
        }

        Self {
            cols: other.cols,
            data,
        }
    }
}