pub fn parts_sums(arr: &[u64]) -> Vec<u64> {
 let mut totale: u64 = arr.iter().sum();
 let mut res =  vec![totale];
 for &num in arr.iter().rev() {
    totale-= num;
    res.push(totale);

 }
 res
}
pub fn parts_sums(arr: &[u64]) -> Vec<u64> {
let mut sum : u64 = 0;
let mut res = vec[0];
for num in arr.iter(){
    sum += *num;
    res.push(sum);
}
res.reverse()
res
}