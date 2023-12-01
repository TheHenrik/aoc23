use std::{fs, path::Path};

fn main() {
    let input_path = Path::new("input/01.in");
    let input = fs::read_to_string(input_path).expect("Couldnt open the file");
    let solution = worker(input);
    println!("Solution one: {:?}, Solution two: {:?}", solution.0, solution.1);
}

fn worker(input: String) -> (usize, u32) {
    let p1 = input.lines().map(|a| {
            // println!("{a}");
            let first = a.find(char::is_numeric).unwrap();
            let last = a.rfind(char::is_numeric).unwrap();
            // let a = a.as_bytes();
            a.as_bytes()[first] as usize * 10 + a.as_bytes()[last] as usize - 48 * 10 - 48
        }).sum();
    let p2 = input.lines().map(|line| 
        line.to_string()
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine"))
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|vec| 10 * vec.first().unwrap() + vec.last().unwrap())
        .sum();
    (p1, p2)
}
