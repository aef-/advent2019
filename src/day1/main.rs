use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_gas(mass: i32) -> i32 {
    let fuel = (mass / 3) - 2;
    if fuel <= 0 {
        return 0;
    }

    return fuel + find_gas(fuel);
}

fn main() {
    let filename = "src/input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let masses: Vec<i32> = reader.lines().into_iter().map({ |line|
        line.unwrap().parse::<i32>().unwrap()
    }).collect();

    let sum_fuel = masses.into_iter().fold(0, |acc, m| acc + find_gas(m));
    println!("{}", sum_fuel.to_string())
}
