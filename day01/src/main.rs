use std::{fs, path::Path};

fn main() {
    let input_path = Path::new("input/01.in");
    let input = fs::read_to_string(input_path).expect("Couldnt open the file");
    let solution = worker(input);
    println!("Solution one: {:?}, Solution two: {:?}", solution[0], solution[1]);
}

fn worker(input: String) -> Vec<&'static str>{
    print!("{input}");
    vec!["1", "2"]
}
