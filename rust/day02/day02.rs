// https://adventofcode.com/2019/day/2
use std::io;
// use std::io::prelude::*;


fn main() -> io::Result<()> {
    // First line is input string
    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str)?;
    println!("{}", input_str);

    for x in input_str.split(',') {
        // println!("{}", x);
    }


    Ok(())
}
