use std::{fs, path::Path};

fn main() {
    let input_path = Path::new("input/02.in");
    let input = fs::read_to_string(input_path).expect("Couldnt open the file");
    let solution = worker(input);
    println!("Solution one: {:?}, Solution two: {:?}", solution.0, solution.1);
}

fn worker(input: String) -> (usize, usize) {
    let red = 12;
    let green = 13;
    let blue = 14;
    
    let t: usize = input.lines().filter_map(|a| {
        let data: Vec<&str> = a.split(": ").collect();
    }).sum();
    let p1 = 0;
    let p2 = 0;
    (p1, p2)
}