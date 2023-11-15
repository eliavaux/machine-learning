## Description

A very simple machine learning project using backpropagation built from the ground up.

The neural network is trained on the MNIST database and can recognize handwritten digits fairly well after about two minutes.
It is possible to save and load the neural network data inside the `saves` directory.

In the `data` directory lie the training images and labels without the header data of the file. There is also a `create_img()`
function that converts the image data from the MNIST database into an image. Inside the `images` directory I have printed the
first 100 images already.

The `Matrix` struct is saved as the amount of columns and a one-dimensional vector of the data with column-major order. 


## Thank You

3Blue1Brown for his informative and straightforward [Video Series](https://youtube.com/playlist?list=PLZHQObOWTQDNU6R1_67000Dx_ZCJB-3pi) on the basics of neural networks and
backpropagation.

MathleteDev's [Matrices and Neural Networks Tutorial](https://github.com/mathletedev/rust-ml) which has been a great guide throughout the project.


## Thoughts

This is not meant to be practical code for a neural network, but more of a learning project on the basic concepts of machine
learning and how to implement them into Rust 
