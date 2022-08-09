use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

const CHARACTERS: &str = "ABCDEFGHIJKLMNOPQRSTYUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=!@#$%^&*()_+`~[]{};:\'\"\\|/.,<> ?";

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let rotation = &args[2];
    let lines = read_from_file(filename);

    println!();
    lines.iter().for_each(|line| {
        println!("{:?}", cypher_encode(line, rotation.parse().unwrap_or(1)));
    });
    println!();
    lines.iter().for_each(|line| {
        println!("{:?}", cypher_decode(cypher_encode(line, 1).as_str(), rotation.parse().unwrap_or(1)));
    });
}

fn read_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("ERROR: file does not exist");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("ERROR: could not parse line"))
        .collect()
}

fn cypher_encode(input: &str, rotation: usize) -> String {
    let mut buffer = String::new();

    input.chars().for_each(|ch| {
        if !CHARACTERS.contains(ch) {
            buffer.push(ch);
        } else {
            buffer.push(
                CHARACTERS
                    .chars()
                    .nth((CHARACTERS.find(ch).unwrap() + rotation) % (CHARACTERS.len() - 1))
                    .unwrap(),
            );
        }
    });
    buffer
}

fn cypher_decode(input: &str, rotation: usize) -> String {
    let mut buffer = String::new();

    input.chars().for_each(|ch| {
        if !CHARACTERS.contains(ch) {
            buffer.push(ch);
        } else {
            buffer.push(
                CHARACTERS
                    .chars()
                    .nth((CHARACTERS.find(ch).unwrap() + CHARACTERS.len() - rotation - 1) % (CHARACTERS.len() - 1))
                    .unwrap(),
            );
        }
    });
    buffer
}
