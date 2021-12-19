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

pub fn getln_char() -> Option<char> {
    let mut result: Option<char> = None;
    let ln: String = getln(true);
    if ln.len() != 0 {
        result = ln.chars().next();
    }
    return result;
}

pub fn getln_int<T: std::str::FromStr>() -> Option<T>{
    let mut result: Option<T> = None;
    let ln: String = getln(true);
    if ln.len() != 0 {
        let parse_res = ln.parse::<T>();
        if !parse_res.is_err(){
            result = None;
        }
        else{
            result = parse_res.ok();
        }
    }
    return result;
}


pub fn cls() {
    print!("{esc}c", esc = 27 as char);
}

const ALOT_OF_NEWLINES: &str = "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n
\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n
\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n
\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n
\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n
\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n
\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n
\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n
\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n
\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n";

pub fn cls_with_new_lines(){
    print!("{}", ALOT_OF_NEWLINES);
}