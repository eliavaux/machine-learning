use std::fs::File;
use std::io::{BufReader, BufWriter, Read};

pub const IMG_COUNT: usize = 60000;
pub const IMG_SIZE: usize = 28 * 28;

pub fn get_inputs() -> Vec<Vec<f64>> {
    let file = File::open("data/images").unwrap();
    let mut reader = BufReader::new(file);

    let mut inputs = Vec::new();

    for _ in 0..IMG_COUNT {
        let mut img = vec![0; IMG_SIZE];
        reader.read_exact(&mut img).unwrap();
        inputs.push(img.iter().map(|&x| x as f64 / 255.0).collect());
    }

    inputs
}

pub fn get_targets() -> Vec<Vec<f64>>{
    let file = File::open("data/labels").unwrap();
    let mut reader_targets = BufReader::new(file);

    let mut buf = [0u8; IMG_COUNT];
    reader_targets.read_exact(&mut buf).unwrap();
    let mut targets = Vec::new();

    for i in 0..IMG_COUNT {
        targets.push(vec![0.0; 10]);
        targets[i][buf[i] as usize] = 1.0;
    }

    targets
}

pub fn create_img(image: &[u8], path: &std::path::Path) -> std::io::Result<()> {
    let file = File::create(path)?;
    let buf = BufWriter::new(file);

    let mut encoder = png::Encoder::new(buf, 28, 28);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header()?;
    writer.write_image_data(image)?;

    Ok(())
}