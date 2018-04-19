extern crate rand;

use std::io::prelude::*;
use std::net::TcpStream;
use rand::Rng;
use std::thread;
use std::time;


fn run(start_x: i32, start_y: i32, max_x: i32, max_y: i32) {
    let mut stream = TcpStream::connect("192.168.178.74:1337").unwrap();

    let mut rng = rand::thread_rng();

    let hex = vec!['1','2','3','4','5','6','7','8','9','0','A','B','C','D','E','F'];
    let mut iterate = 0;

    loop {
        let mut y = start_y + (iterate % 2);
        let x = rng.gen_range(start_x, max_x);
        let mut msg = format!("");

        /*let color = format!(
            "{}{}{}{}{}{}", 
            hex[rng.gen_range(0, 15)], hex[rng.gen_range(0, 15)], 
            hex[rng.gen_range(0, 15)], hex[rng.gen_range(0, 15)], 
            &hex[rng.gen_range(0, 15)], hex[rng.gen_range(0, 15)]);*/

        while y < max_y {

            msg =  format!("{msg}PX {x} {y} f442eb\n", msg = msg, x = x, y = y);
            y = y + 2;
        }
        stream.write(msg.as_bytes());
        iterate = iterate + 1;
    }
}

fn main() {

    let width = 1300;
    let offset_x = 250;
    let height = 750;
    let offset_y = 0;

    for x in 0..12 {
        let start_x = (x % 4) * ((width - offset_x) / 4) + offset_x;
        let start_y = (x / 4) * ((height - offset_y) / 3) + offset_y;

        let stop_x = start_x + ((width - offset_x) / 4) + offset_x; 
        let stop_y = start_y + ((height - offset_y) / 3) + offset_y; 

        println!("sx:{} sy:{} mx:{} my:{}", start_x, start_y, stop_x, stop_y);

        thread::spawn(move|| { run(start_x, start_y, stop_x, stop_y); });
    }

    let millis = time::Duration::from_millis(1000);
    loop {
        thread::sleep(millis);
    }
}