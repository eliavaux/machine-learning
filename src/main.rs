mod matrix;
mod network;
mod parser;

use crate::network::Network;


fn main() {
    let inputs = parser::get_inputs();
    let targets = parser::get_targets();

    let mut network = Network::new(&[784, 16, 16, 10], 0.3);

    network.train(&inputs, &targets, 1000);
    network.save("saves/test").unwrap();

    let mut network = Network::load("saves/test").unwrap();
    network.train(&inputs, &targets, 10);

    println!("image 50: {:#?}, target: {:?}", network.feed_forward(&inputs[50]), targets[50]);
    println!("image 51: {:#?}, target: {:?}", network.feed_forward(&inputs[51]), targets[51]);
    println!("image 52: {:#?}, target: {:?}", network.feed_forward(&inputs[52]), targets[52]);
    println!("image 53: {:#?}, target: {:?}", network.feed_forward(&inputs[53]), targets[53]);
}