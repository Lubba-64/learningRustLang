use crate::my_io;
use crate::general;

pub fn run(){
    println!("input your word: ");
    let word: String = my_io::getln(true);
    my_io::cls_with_new_lines();
    let mut current_guessed_word: Vec<char> = Vec::new();

    for _char in word.chars(){
        current_guessed_word.push('_');
    }
    while current_guessed_word.contains(&'_')
    {
        println!("currently revealed letters: {}", general::parse_iter(current_guessed_word.iter(),","));
        break;
    }
}
