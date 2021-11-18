extern crate minifb;
extern crate rand;

use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use std::time::Duration;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

fn randomise(buffer: &mut Vec<u32>) {
    let mut rng = rand::thread_rng();

    for i in 0..buffer.len() {
        let r: u32 = rng.gen_range(0..255) << 24;
        let g: u32 = rng.gen_range(0..255) << 16;
        let b: u32 = rng.gen_range(0..255) << 8;

        buffer[i] = r + g + b + 0xFF;
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "sus",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();
    window.limit_update_rate(Some(Duration::from_micros(16600)));

    randomise(&mut buffer);

    loop {
        if window.is_key_down(Key::Space) {
            randomise(&mut buffer);
        };

        if window.is_open() {
            window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        } else {
            break;
        };
    }
}
