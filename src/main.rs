mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

fn main() {
    let x: Vec<u8> = vec![
        104, 105, 115, 32, 105, 115, 32, 119, 104, 101, 114, 101, 32, 121, 111, 117, 114, 32, 115,
        101, 99, 114, 101, 116, 32, 109, 101, 115, 115, 97, 103, 101, 32, 119, 105, 108, 108, 32,
        98, 101, 33,
    ];

    println!("{}", String::from_utf8(x).unwrap());
}
