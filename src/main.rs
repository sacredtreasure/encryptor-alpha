use std::{fs, env, io::Write};

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
   let args: Vec<String> = env::args().collect::<Vec<String>>();

    if args.len() != 3 || (args[2].clone() != *"false" && args[2].clone() != *"true") {
        println!("Usage: {} <file> <decrypting?>", args[0].clone());
        println!("Example: {} important.txt true", args[0].clone());
        println!("Example: {} important.txt false", args[0].clone());
        return;
    }
    let decrypt_init = args[2] == *"true";

    match fs::read(args[1].clone()) {

       Ok(contents) => {

        //2 here can be any number
        let new_content = byte_teleport(contents, 2, decrypt_init);

        let mut file = fs::OpenOptions::new()
        .write(true)
        .open(args[1].clone())
        .unwrap();

    if let Err(e) = file.write_all(&new_content) {
        println!("Error: {:?}", e);
    }
       }
       Err(e) => {
        println!("Could not open file `{}`:`{}`", args[1], e);
       }
    }

    println!("Encrypted Successfully, Fuck you XD");

}