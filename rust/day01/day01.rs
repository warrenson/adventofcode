// https://adventofcode.com/2019/day/1
use std::io;
use std::io::prelude::*;


fn main() -> io::Result<()> {
    let stdin = io::stdin();
    // we may need to handle large sums, use i64
    let fuel_mass_sum: i64 = stdin.lock().lines().map(
        |m| total_fuel_mass(m.unwrap().parse::<i64>().unwrap())
    ).sum();
    println!("{}", fuel_mass_sum);
    Ok(())
}

fn total_fuel_mass(f: i64) -> i64 {
    let fuel_mass = |m: i64| -> i64 { m / 3 - 2 };
    let mut m = fuel_mass(f);
    let mut r = fuel_mass(m);

    while r > 0 {
        m += r;
        r = fuel_mass(r);
    }
    return m;
}

