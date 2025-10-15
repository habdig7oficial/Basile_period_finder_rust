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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1(){
        let res = new("ab3\n");
        let expect = "ababab";
        if res != expect{
            panic!("Unexpected Result {res} != {expect}");
        }
    }

    #[test]
    fn teste_basile1() {
        let res = new("a7-b5-a10\n");
        let expect = "aaaaaaabbbbbaaaaaaaaaa";
        if res != expect {
            panic!("Unexpected Result {res} != {expect}");
        }
    }

    #[test]
    fn teste_basile2() {
        let res = new("a6-b15-c1-d1\n");
        let expect = "aaaaaabbbbbbbbbbbbbbbcd";
        if res != expect {
            panic!("Unexpected Result {res} != {expect}");
        }
    }
} 