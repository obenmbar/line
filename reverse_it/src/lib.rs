
pub fn reverse_it(v: i32) -> String {
   let vstr =  v.unsigned_abs().to_string();
   let mut rev: String= vstr.chars().rev().collect();
   if v < 0 {
    rev.insert(0,'-');
   }
   rev + &vstr
}
