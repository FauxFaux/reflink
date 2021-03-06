extern crate reflink;

use std::time::{Instant};
use std::io;
use std::io::Read;
use std::fs;

fn main() {
    let mut base_file = fs::File::create("base.txt").unwrap();
    let mut src = io::repeat(65).take(100 * 1024 * 1024); // 100 MB
    io::copy(&mut src, &mut base_file).unwrap();
    base_file.sync_all().unwrap();

    let before_reflink = Instant::now();
    reflink::reflink("base.txt", "reflinked.txt").unwrap();
    println!("Time to reflink: {:?}", Instant::now() - before_reflink);

    let before_copy = Instant::now();
    fs::copy("base.txt", "copied.txt").unwrap();
    println!("Time to copy: {:?}", Instant::now() - before_copy);

    fs::remove_file("base.txt").unwrap();
    fs::remove_file("reflinked.txt").unwrap();
    fs::remove_file("copied.txt").unwrap();
}
