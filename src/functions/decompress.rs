pub fn new(text: &str) -> String {
    let mut str_repeat_num = String::from("");
    let mut buffer = String::from("");

    let mut str_builder = String::from("");

    for c in text.chars() {
        if c.is_numeric() {
            str_repeat_num.push(c); 
        }
        else if c != '-' && c != '\n'{
            buffer.push(c);
        }
        else{ 
            for _ in 0..str_repeat_num.parse::<u32>().unwrap(){
                str_builder += &buffer;
            }
            buffer = String::from("");
            str_repeat_num = String::from("");
        }
        
    }

    return str_builder;
}
