use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("./advent_1.txt").expect("Can't open file");
    let mut file_content: String = String::new();
    file.read_to_string(&mut file_content).expect("Could not read file");

    let module_masses: Vec<&str> = file_content.split('\n').collect();
    let mut running_sum = 0;
    for &mass in module_masses.iter() {
        match mass.parse::<i32>() {
            Ok(n) => {
                let fuel = n / 3 - 2;
                running_sum += fuel;
            },
            Err(e) => {}
        }
    }

    println!("total fuel: {}", running_sum);
}
