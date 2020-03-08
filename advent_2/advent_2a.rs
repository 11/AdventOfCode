use std::fs::File;
use std::io::prelude::*;


/**
 * *OPCODES:
 * op1: addition (+)
 *  - 1,10,20,30 => reg[30] = reg[10] + reg[20]
 *
 * op2: multiplication (*)
 *  - 2,10,20,30 => reg[30] = reg[10] * reg[20]
 *
 * op99: end program
 *
 * *NOTE:
 * - a register refers to the nth comma-delimited position
 * - need to step forward 4 positions after reading each opcode - except for op99.
 *
 */
fn main() -> Result {
    let mut file = File::open("./advent_2.txt")?;
    let mut file_buf:Vec<u32> = Vec::new();
    file.read_to_end(&mut file_buf)?

    while op < program.len() {

    }
}
