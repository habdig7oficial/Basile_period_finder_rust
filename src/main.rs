use std::{env::*, *};

fn main() {
    let arguments: Vec<String> = args().collect(); 

    for i in 1..arguments.len(){
        if arguments[i] == "-c" || arguments[i] == "--compact" {
            println!("{0}", &arguments[i + 1]);
            let file = fs::read_to_string(&arguments[i + 1])
                .expect("Coud not read file");
            println!("{file}\n");
        }
    }
}
