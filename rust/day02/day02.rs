// https://adventofcode.com/2019/day/2
use std::io;
use std::convert::TryInto;

fn main() -> io::Result<()> {
    // First line is input string
    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str)?;
    // println!("{}", input_str);

    let data: Vec<i32> = input_str.split(',').map(|s| s.trim().parse().unwrap()).collect();

    // part 1
    let mut x = data.to_vec();
    x[1] = 12;
    x[2] = 2;
    println!("part 1 result: {:?}", eval_program(x));


    let nv = bf_search(data, 19690720);
    println!("part 2 result: {} {:?}", 100 * nv.0 + nv.1, nv);
    Ok(())
}

fn bf_search (a: Vec<i32>, t: i32) -> (i32, i32) {
    let mut i = 0;
    for n in 0..=99 {
        for v in 0..=99 {
            i += 1;
            let mut x = a.to_vec();
            x[1] = n;
            x[2] = v;
            if eval_program(x) == t {
                return (n, v);
            }
        }
    }
    return (-1, i);
}

fn eval_program(mut p: Vec<i32>) -> i32 {
    let mut i = 0;
    while i < p.len() {
        // exit
        if p[i] == 99 { break; }
        // get params
        let a: usize = p[i+1].try_into().unwrap();
        let b: usize = p[i+2].try_into().unwrap();
        let c: usize = p[i+3].try_into().unwrap();
        // op1
        if p[i] == 1 {
            p[c] = p[a] + p[b];
        }
        // op2
        if p[i] == 2 {
            p[c] = p[a] * p[b];
        }
        // increment
        i += 4;
    }
    return p[0]; 
}
