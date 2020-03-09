use std::fs::File;
use std::error::Error;
use std::io::prelude::*;


/**
 * OPCODES:
 * op1: addition (+)
 *  - 1,10,20,30 => reg[30] = reg[10] + reg[20]
 *
 * op2: multiplication (*)
 *  - 2,10,20,30 => reg[30] = reg[10] * reg[20]
 *
 * op99: end program
 *
 * NOTE:
 * - a register refers to the nth comma-delimited position
 * - need to step forward 4 positions after reading each opcode - except for op99.
 *
 */
fn main() -> Result<(), Box<dyn Error + 'static>> {
    let mut file = File::open("./src/bin/advent_2.txt")?;
    let mut file_buf = String::with_capacity(294usize.next_power_of_two());
    file.read_to_string(&mut file_buf)?;
    let mut input = file_buf
        .split(',')
        .map(|s| {
            let x = s.trim().parse::<i64>()?;
            Ok(x)
        })
        .collect::<Result<Vec<i64>, Box<dyn Error>>>()?;

    input[1] = 12;
    input[2] = 2;


    let mut i = 0;
    loop {
        if i == input.len() {
            break;
        }

        let op = input[i];
        match op {
            99 => break,
            1 | 2 => {
                let reg1 = input[i+1];
                let reg2 = input[i+2];
                let store = input[i+3];
                let result = if op == 1 {
                    input[reg1 as usize] + input[reg2 as usize]
                } else {
                    input[reg1 as usize] * input[reg2 as usize]
                };

                input[store as usize] = result;
                i += 4;
            }
            _ => panic!("Something broke")
        }
    }
    println!("{}", input[0]);
    Ok(())
}
