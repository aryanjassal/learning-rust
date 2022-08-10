use std::{env, fs};

const CHARACTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-_ ";

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let rotation = &args[2].parse().unwrap_or(1);
    let file_content = fs::read_to_string(filename).unwrap();

    let encoded_text = cypher_encode(&file_content, *rotation);

    println!();
    println!("{}", &encoded_text);
    println!();
    println!("{}", cypher_encode(&encoded_text, -(*rotation)));
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
