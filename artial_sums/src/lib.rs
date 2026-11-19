
pub fn parts_sums(arr: &[u64]) -> Vec<u64> {
     let mut total =  arr.iter().sum();
     let mut res =  Vec::new();
     res.push(total);
     
     for &val in arr.iter().rev() {
        total -= val ;
        res.push(total)
     }
     res
}
