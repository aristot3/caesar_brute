/* Usage : caesar_brute <path/to/binary>  */
use std::env;
use std::fs;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path/to/file>", args[0]);
        std::process::exit(1);
    }

    let mut file = match fs::File::open(&args[1]) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error when opening the file : {}", err);
            std::process::exit(1);
        }
    };

    let mut contents = Vec::new();
    if let Err(err) = file.read_to_end(&mut contents) {
        eprintln!("Error when reading the file : {}", err);
        std::process::exit(1);
    }

    // Bruteforcing here
    for i in 0..125 {
        for c in &contents {
            print!("{}", (((*c as u8) + i) % 128) as char);
        }
        print!("\n\n");
    }
}


