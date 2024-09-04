use std::hint::black_box;
use zune_jpeg::JpegDecoder;

const JPEG: &[u8] = include_bytes!("../data/photo.jpg");

fn main() {
    let data = JpegDecoder::new(black_box(JPEG)).decode().unwrap();
    println!("{}", data.len());
}
