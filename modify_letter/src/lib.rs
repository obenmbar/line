pub fn remove_letter_sensitive(s: &str, letter: char) -> String {
    s.replace(letter,"")
}

pub fn remove_letter_insensitive(s: &str, letter: char) -> String {
    s.replace(letter.to_ascii_uppercase(), "").replace(letter.to_ascii_lowercase(),"")
}

pub fn swap_letter_case(s: &str, letter: char) -> String {
    let lower =  letter.to_ascii_lowercase();
    let upper =  letter.to_ascii_uppercase();
    let mut res  =  String::new();
 for c in s.chars() {
    if c ==  lower {
        res.push(upper);
    }else if c == upper {
        res.push(lower);
    }else {
        res.push(c)
    }
 }
 res
}
