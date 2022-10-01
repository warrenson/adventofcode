
// https://adventofcode.com/2019/day/3
use std::io;
// use std::convert::TryInto;
use std::str::FromStr;

fn main() -> io::Result<()> {
    let p1 = getpath().unwrap();
    let p2 = getpath().unwrap();
    println!("{:?}", p1);
    println!("{:?}", p2);
    let p1_steps = getsteps(p1);
    let p2_steps = getsteps(p2);
    println!("{:?}", p1_steps);
    println!("{:?}", p2_steps);

    Ok(())
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
        let mut c = p.chars();
        let direction = c.next().unwrap();
        let distance: i32 = FromStr::from_str(c.as_str()).unwrap();
        steps.append(&mut move_pos(pos, direction, distance));
        // println!("{:?}", steps);
        pos = steps[steps.len() - 1];
    }
    return steps;
}

// this one is nasty :(
fn move_pos(start_pos: (i32, i32), dir: char, dist: i32) -> Vec<(i32,i32)> {
    // vec![(1,0),(2,0),(3,0)]
    let mut pos = start_pos;
    let dir_base = match dir {
        'U' => (0,1),
        'D' => (0,-1),
        'R' => (1,0),
        'L' => (-1,0),
        _ => panic!("WTF direction is that?"),
    };
    let mut move_steps = Vec::new();
    for _i in 1..=dist {
        move_steps.push( (pos.0 + dir_base.0, pos.1 + dir_base.1) );
        pos = move_steps[move_steps.len() - 1];
    }
    return move_steps;
}
