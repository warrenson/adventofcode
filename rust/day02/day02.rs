// https://adventofcode.com/2019/day/2
use std::io;
use std::convert::TryInto;

fn main() -> io::Result<()> {
    // First line is input string
    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str)?;
    println!("{}", input_str);

    let mut data: Vec<i32> = input_str.split(',').map(|s| s.trim().parse().unwrap()).collect();
    let mut i = 0;
    while i < data.len() {
        // exit
        if data[i] == 99 { break; }
        // op1
        if data[i] == 1 {
            let a: usize = data[i+1].try_into().unwrap();
            let b: usize = data[i+2].try_into().unwrap();
            let c: usize = data[i+3].try_into().unwrap();
            data[c] = data[a] + data[b];
        }
        // op2
        if data[i] == 2 {
            let a: usize = data[i+1].try_into().unwrap();
            let b: usize = data[i+2].try_into().unwrap();
            let c: usize = data[i+3].try_into().unwrap();
            data[c] = data[a] * data[b];
        }
        // increment
        i += 4;
    }

    // for x in input_str.split(',') {
        // // println!("{}", x);
    // }


    println!("{:?}", data);
    Ok(())
}
