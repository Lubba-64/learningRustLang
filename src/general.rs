pub fn concat_iter<T>(_iter: T, delim: &str) -> String 
where T:Iterator,
T::Item: std::fmt::Display
{
    // concatanates whatever is thrown in as a string and returns it
    let mut result: String = String::from("");
    let len: usize = _iter.size_hint().0;
    for (i, x) in _iter.enumerate() {

        result += x.to_string().as_str();
        if i + 1 == len{
            break;
        }
        result += delim;
    }
    return result;
}

pub fn unique_chars_in_str(_str: &str) -> Vec<char>{
    if _str.len() == 0 {
        return Vec::new();
    }
    let mut unique = Vec::new();
    for (_i, x) in _str.chars().enumerate(){
        if !unique.iter_mut().any(|&mut y| y == x) {
            unique.push(x);
        }
    }
    return unique;
}