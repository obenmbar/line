use std::collections::HashMap;
pub fn counting_words(words: &str) -> HashMap<String, u32> {
let mut map =  HashMap::new();
for word in words.to_lowercase().split(|c: char| !c.is_alphanumeric() && c != '\'') {
    let clear =  word.trim_matches('\'');
    if !clear.is_empty() {
        *map.entry(word.to_string()).or_insert(0) += 1;
    }
} 
map
}
