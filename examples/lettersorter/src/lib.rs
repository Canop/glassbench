
/// sort the letters in the string
pub fn sort(s: &str) -> String {
    // let's build a string
    let mut s = s.to_string();
    // let's add a 0 because strings should be zero terminated, right ?
    //s.push_str("0");
    // now make a vector: it's easier to sort
    let mut chars: Vec<char> = s.chars().collect();
    // sort in place (so it's faaast!)
    chars.sort();
    // make the string to return
    let mut s: String = chars.iter().collect();
    // wait, I've been told there should not be a zero, where is it ?
    // let zero_idx = s.find('0');
    // remove it before somebody notices it
    //s.remove(zero_idx.unwrap());
    // well, it's done
    s
}


#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        assert_eq!(sort("bac").as_str(), "abc");
        assert_eq!(sort("π/2").as_str(), "/2π");
        assert_eq!(sort("52145729034508").as_str(), "00122344555789");
    }
}
