extern crate rand;

use std::io::prelude::*;
use std::net::TcpStream;
use rand::Rng;
use std::thread;
use std::time;


fn run(start_x: i32, start_y: i32, max_x: i32, max_y: i32) {
    let mut stream = TcpStream::connect("192.168.178.122:1234").unwrap();

    let mut rng = rand::thread_rng();

    let hex = vec!['1','2','3','4','5','6','7','8','9','0','A','B','C','D','E','F'];

    let color = format!(
        "{}{}{}{}{}{}", 
        hex[rng.gen_range(0, 15)], hex[rng.gen_range(0, 15)], 
        hex[rng.gen_range(0, 15)], hex[rng.gen_range(0, 15)], 
        &hex[rng.gen_range(0, 15)], hex[rng.gen_range(0, 15)]);

    loop {
        let mut y = start_y;
        let x = rng.gen_range(start_x, max_x);
        let mut msg = format!("");

        while y < max_y {
            msg =  format!("{msg}PX {x} {y} {c}\n", msg = msg, x = x, y = y, c = color);
            y = y + 1;
        }
        stream.write(msg.as_bytes());
    }
}

fn main() {

    let width = 1920;
    let offset_x = 250;
    let height = 1080;
    let offset_y = 0;

    let num_threads = 10;

    for x in 0..num_threads {
        let w = (width - offset_x) / num_threads;
        let sx = x * ((width - offset_x) / num_threads) + offset_x;
        thread::spawn(move|| { run(sx, offset_y, sx + w, height); });
    }

    let millis = time::Duration::from_millis(1000);
    loop {
        thread::sleep(millis);
    }
}