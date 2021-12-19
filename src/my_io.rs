pub fn getln(trim: bool) -> String{
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input){
        Ok(_) => {
            if trim {
                input = String::from(input.trim());
            }
            return input;
        },
        Err(e) => { return e.to_string(); }
    }
}

pub fn cls() {
    print!("{}[2J", 27 as char);
}

pub fn parse_iter<T>(_iter: T, delim: &str) -> String 
where T:Iterator,
T::Item: std::fmt::Display
{
    let mut _return: String = String::from("");
    let len: usize = _iter.size_hint().0;
    for (i, x) in _iter.enumerate() {

        _return += x.to_string().as_str();
        if i + 1 == len{
            break;
        }
        _return += delim;
    }
    return _return;
}