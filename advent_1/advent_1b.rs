use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    let mut file = File::open("./advent_1.txt").expect("Can't open file");
    let mut file_content: String = String::new();
    file.read_to_string(&mut file_content).expect("Could not read file");

    let module_masses: Vec<&str> = file_content.split('\n').collect();
    let mut running_sum = 0;
    for &mass in module_masses.iter() {
        match mass.parse::<i32>() {
            Err(_e) => { },
            Ok(n) => {
                let mut fuel_total = 0;
                let mut fuel = n;
                while fuel > 0 {
                    fuel /= 3;
                    fuel -= 2;
                    if fuel > 0 {
                        fuel_total += fuel;
                    }
                }

                running_sum += fuel_total;
            }
        }
    }

    println!("total fuel: {}", running_sum);
}
