#![allow(unused)]
use rusty_ppm::canvas::*;
use rusty_ppm::ppm_writer::write_to_file;
use rusty_ppm::ppm_reader::read_to_canvas;
use cgmath::vec3;
use std::time::Instant;
use std::path::Path;

const W: usize = 1920;
const H: usize = 1080;
fn main() {
    let tt_1 = Instant::now();
    test_01();
    let tt_1 = tt_1.elapsed();
    println!("Test 1 time: {:?}", tt_1);
    let tt_2 = Instant::now();
    test_02();
    let tt_2 = tt_2.elapsed();
    println!("Test 2 time: {:?}", tt_2);
    //TODO Test 2 is consistantly faster. Convert the logic to only use the test 2 logic
    //Pretty sure its the preallocated, pre-filled vector
}

fn test_01() {
    let folder_path = Path::new("./");
    let mut image = Canvas::new_empty("test_01", W, H);
    for col in 0..image.width {
        for row in 0..image.height {
            image.push(&vec3((row*255/H) as u8, (col*255/W) as u8, 0));
        }
    }
    write_to_file(&image, folder_path);
}

fn test_02() {
    let folder_path = Path::new("./");
    let mut image = Canvas::new("test_02", W, H);
    for col in 0..image.width {
        for row in 0..image.height {
            *image.get_mut(row, col).unwrap() = (vec3((row*255/H) as u8, (col*255/W) as u8, 0));
        }
    }
    write_to_file(&image, folder_path);
}
