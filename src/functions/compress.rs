fn find_period(text: &str) -> Option<(usize, &str)> {
    for i in 1..(text.len() / 2) {
        if text[0..i] == text[i..(i*2)] {
            //print!("{i} {0} - {1} ", &text[0..i], &text[i..(i*2)]);
            print!("true");
            let mut j = 0;
            let mut counter = i; // The first element is already counted
            while j < text.len() {
                println!("{j} {0}", &text[j..(j + i)]);

                counter += 1;
                j += i
            }
            return Some((i, &text[0..i])); 
        }else {
            //print!("false");
        }
        //println!();
    }
    None
}

pub fn new(text: &str){
    let mut i = 0;
    find_period(&text[i..]);
    
}