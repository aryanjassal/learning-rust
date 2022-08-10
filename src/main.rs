use std::{env, fs};

fn main() {
    let base_charset: &String = &"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()-_<>/[]{}+=., ".to_string();
    let args: Vec<String> = env::args().collect();
    let mode = &args[1];
    let filename = &args[2];
    let rotation = &args[3].parse().unwrap_or(1);
    let charset = &args.get(4).unwrap_or(base_charset);
    let file_content = fs::read_to_string(filename).unwrap();

    println!();
    if mode == "encrypt" || mode == "e" {
        println!("{}", cypher_encode(&file_content, *rotation, charset));    
    } else if mode == "decrypt" || mode == "d" {
        println!("{}", cypher_encode(&file_content, -(*rotation), charset));
    } else {
        println!("ERROR: mode can only be `encrypt` (e) or `decrypt` (d)")
    }
    println!();
}

fn cypher_encode(input: &str, rotation: isize, charset: &String) -> String {
    let rotation = rotation as isize % charset.len() as isize;
    input
        .chars()
        .map(|ch| {
            if !charset.contains(ch) {
                ch
            } else {
                charset
                    .chars()
                    .nth(
                        ((charset.len() as isize
                            + (charset.find(ch).unwrap() as isize - (rotation)))
                            % charset.len() as isize) as usize,
                    )
                    .unwrap()
            }
        })
        .collect::<String>()
}
