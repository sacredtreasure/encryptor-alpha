use std::{fs, env, io::Write, vec};

fn byte_teleport(text: Vec<u8>, shift_by: u8, backwards: bool) -> Vec<u8> {
    text.iter()
    .map(|byte| {
        if backwards {
            byte.wrapping_sub(shift_by)
        } else {
            byte.wrapping_add(shift_by)
        }
    })
    .collect()
}

fn main() {
   let args: Vec<String> = env::args().collect()::<Vec<String>>();

    if args.len() != 3 || (args[2].clone() != *"false" && args[2].clone() != *"true") {
        println!("Usage: {} <file> <decrypting?>", args[0].clone());
    }
}