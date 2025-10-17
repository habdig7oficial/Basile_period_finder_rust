use std::{env::*, *};

mod functions;
use functions::{compress, decompress};

fn main() {
    let arguments: Vec<String> = args().collect(); 


    for i in 1..arguments.len(){
        if arguments[i] == "-c" || arguments[i] == "--compress" {
            println!("Compact {0}", &arguments[i + 1]);
            let file = fs::read_to_string(&arguments[i + 1])
                .expect("Coud not read file");
            println!("{file}\n");
            compress::new(&file);
        }
        else if arguments[i] == "-d" || arguments[i] == "--descompress"  {
            println!("Descompact {0}", &arguments[i + 1]);
            let file = fs::read_to_string(&arguments[i + 1])
                .expect("Coud not read file");
            print!("{0}", decompress::new(&file));
        }
    }


}
