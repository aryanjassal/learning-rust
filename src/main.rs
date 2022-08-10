use std::{env, fs};

const CHARACTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-_ ";

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = &args[1];
    let filename = &args[2];
    let rotation = &args[3].parse().unwrap_or(1);
    let file_content = fs::read_to_string(filename).unwrap();

    println!();
    if mode == "encrypt" || mode == "e" {
        println!("{}", cypher_encode(&file_content, *rotation));    
    } else if mode == "decrypt" || mode == "d" {
        println!("{}", cypher_encode(&file_content, -(*rotation)));
    } else {
        println!("ERROR: mode can only be `encrypt` (e) or `decrypt` (d)")
    }
    println!();
}

fn cypher_encode(input: &str, rotation: isize) -> String {
    let rotation = rotation as isize % CHARACTERS.len() as isize;
    input
        .chars()
        .map(|ch| {
            if !CHARACTERS.contains(ch) {
                ch
            } else {
                CHARACTERS
                    .chars()
                    .nth(
                        ((CHARACTERS.len() as isize
                            + (CHARACTERS.find(ch).unwrap() as isize - (rotation)))
                            % CHARACTERS.len() as isize) as usize,
                    )
                    .unwrap()
            }
        })
        .collect::<String>()
}
