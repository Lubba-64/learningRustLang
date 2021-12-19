use crate::my_io;

pub fn run(){
    println!("input your word: ");
    let word: String = my_io::getln(true);
    my_io::cls();
    let mut current_guessed_word: Vec<char> = Vec::new();

    for _char in word.chars(){
        current_guessed_word.push('_');
    }
    while current_guessed_word.contains(&'_')
    {
        println!("currently revealed letters: {}", my_io::parse_iter(current_guessed_word.iter(),","));
        break;
    }
}
