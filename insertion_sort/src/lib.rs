pub fn insertion_sort(slice: &mut [i32], steps: usize) {
    if slice.len() <= 1 || steps == 0 {
        return;
    }
    
     let limit = steps.min(slice.len()-1);

     for i  in 1..=limit {
        let key = slice[i];
        let mut j = i ;
        while j >0 && slice[j-1] > key{
            slice[j] = slice[j-1];
            j-=1;
        }
        slice[j] = key ;
     }
     
}