pub fn inv_pyramid(v: String, i: usize) -> Vec<String> {
    let mut res = Vec::new();
    for j  in 1..=i {
    let s = " ".repeat(j) +&v.repeat(j);
    res.push(s);
    }
    for j in (1..i).rev() {
    let s = " ".repeat(j) +&v.repeat(j);
    res.push(s);
    }
    res
}
