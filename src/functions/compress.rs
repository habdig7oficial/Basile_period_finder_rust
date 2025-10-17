fn find_period(text: &str) -> Option<(u32, &str)> {
    for i in 1..(text.len() / 2) {
        if text[0..i] == text[i..(i*2)] {
            //print!("{i} {0} - {1} ", &text[0..i], &text[i..(i*2)]);
            print!("true");
            let mut j = 0;
            print!("bg");
            while ((i*j) * 2) < text.len() {
                println!("{j} {0}", &text[(i*j)..((i*j)*2)]);
                j += 1
            }
            print!("end");

        }else {
            //print!("false");
        }
        //println!();
    }
    None
}

pub fn new(text: &str){
    let mut i = 0;
    while i < text.len() {
        find_period(&text[i..]);
        print!("hello");
        i += 1;
    }
}