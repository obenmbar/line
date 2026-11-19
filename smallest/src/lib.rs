use std::collections::HashMap;
 pub fn smallest(h: HashMap<&str, i32>) -> i32 {
//    if h.len() == 0 {
//     return i32::MAX;
//    }

*h.values().min().unwrap_or(&i32::MAX)
//   let mut min = i32::MAX;

//    for &val in h.values() {
//     if val <min {
//         min = val;
//     }
//    } 
//    min
}
