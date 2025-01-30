use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut input = reader.lines();
    loop {
        let mut current_number: i64 = input.next().unwrap().unwrap().parse().unwrap();

        if current_number == 0 {
            break;
        }

        let mut is_negative: bool = current_number < 0;
        current_number = current_number.abs();
        let sqrt_of_current_number = (current_number as f32).ceil() as i64;
        let mut pth_power = 1;
        for p in ( 2..32).rev() {
            let p_root = f64::powf(current_number as f64, 1.0 / p as f64).round() as i64;

            if p_root.pow(p as u32) == current_number && (!is_negative || p % 2 == 1) {
                pth_power = p;
                break;
            }
        }
        println!("{}", pth_power);
    }
} 
