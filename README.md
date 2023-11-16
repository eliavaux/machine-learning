## Description

A very simple machine learning project using backpropagation built from the ground up.

The neural network is trained on the [MNIST database](https://en.wikipedia.org/wiki/MNIST_database) and can recognize handwritten digits fairly well after about two minutes of training.
Trained neural networks can be saved to a JSON file inside the `saves/` directory with `Network::save()` and loaded in with
`Network::load()`. 

Inside the `data/` folder lie the training data of the MNIST database with their headers removed for simplicity. The `parser::create_img()`
function can convert the training data into an PNG of size 28x28. The first 100 images are already inside the `images/` folder.

The `Matrix` struct is written as the amount of columns and a one-dimensional vector of the data with column-major order. 


## Thoughts

This is not meant to be practical code for a neural network, but more of a learning project on the basic concepts of machine
learning and my implementation of them into Rust 


## Thank You

3Blue1Brown for his informative and straightforward [Video Series](https://youtube.com/playlist?list=PLZHQObOWTQDNU6R1_67000Dx_ZCJB-3pi) on the basics of neural networks and
backpropagation.

MathleteDev's [Matrices and Neural Networks Tutorial](https://github.com/mathletedev/rust-ml) which has been a great guide throughout the project.
