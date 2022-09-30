
// https://adventofcode.com/2019/day/3
use std::io;
// use std::convert::TryInto;
use std::str::FromStr;

fn main() -> io::Result<()> {
    // let p1_str = getline().unwrap();
    // let p2_str = getline().unwrap();
    let p1 = getpath().unwrap();
    let p2 = getpath().unwrap();
    println!("{:?}", p1);
    println!("{:?}", p2);

    Ok(())
}

fn getline() -> Result<String, io::Error> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok( line.trim().to_string() )
}

fn getpath() -> Result<Vec<String>, io::Error> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok( line.trim().split(',').map(|s| s.to_string()).collect() )
}

fn getsteps(path: Vec<String>) -> Vec<(i32, i32)> {
    let mut steps  = Vec::new();
    
    let mut pos: (i32,i32) = (0,0);
    for p in path {
        let c = p.chars();
        let direction = c.next().unwrap();
        let distance: i32 = FromStr::from_str(c.as_str()).unwrap();
        steps.push(move_pos(direction, distance));
        pos = steps[-1];
    }
}

fn move_pos(pos: (i32, i32), dist: i32) -> Vec<(i32,i32)> {

}
