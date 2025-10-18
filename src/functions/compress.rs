
fn find_period(text: &str) -> (usize, usize, &str) {
    for i in 1..(text.len() / 2) + 1 {
        if text[0..i] == text[i..(i*2)] {
            //print!("{i} {0} - {1} ", &text[0..i], &text[i..(i*2)]);
            //print!("true");
            let mut j = 0;
            let mut counter = 0;

            while j + i <= text.len() {
                //println!("{j} {0}", &text[j..(j + i)]);
                if text[0..i] != text[j..(j + i)] {
                    break;
                }
                counter += 1;
                j += i;
            }
            return (counter, i, &text[0..i]); 
        }
        //println!();
    }
    return (1, 1, &text[0..1]);  
}

pub fn new(text: &str) -> String {
    let mut i = 0;

    let mut string_builder = String::from("");

    while i < text.len() {
        let (counter, buffer_size, str) = find_period(&text[i..]);
        println!("{str} ");
        string_builder = format!("{string_builder}-{str}{counter}");  
        i += counter * buffer_size;
        print!("{string_builder}");
    }

    string_builder = string_builder[1..].to_string(); 

    print!("\nFinal Result: {string_builder}");
    return string_builder;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_singleton() {
        let res = new("a");
        let expect = "a1";
        if  res != expect  {
            panic!("Unexpected Result {res} != {expect}");
        }
    }

    #[test]
    fn check_simple() {
        let res = new("abcdfabcdfabcdfabcdf");
        let expect = "abcdf4";
        if  res != expect  {
            panic!("Unexpected Result {res} != {expect}");
        }
    }

    #[test]
    fn check_2predicate() {
        let res = new("ab");
        let expect = "a1-b1";
        if  res != expect  {
            panic!("Unexpected Result {res} != {expect}");
        }
    }
    #[test]
    fn check_advance() {
        let res = new("abcdfabcdfabcdfabcdfcaabbabab");
        let expect = "abcdf4-c1-a2-b2-ab2";
        if  res != expect  {
            panic!("Unexpected Result {res} != {expect}");
        }
    }

    #[test]
    fn teste_basic() {
        let res = new("aaaaaaabbbbbaaaaaaaaaa");
        let expect = "a7-b5-a10";
        if  res != expect  {
            panic!("Unexpected Result {res} != {expect}");
        }
    }

    #[test]
    fn teste_basile1() {
        let res = new("ababababababababababab");
        let expect = "ab11";
        if  res != expect  {
            panic!("Unexpected Result {res} != {expect}");
        }
    }
}