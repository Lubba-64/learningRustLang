use crate::my_io;
use crate::general;
use itertools::Itertools;

pub fn run(){
    println!("input your word: ");
    let word_to_guess: String = my_io::getln(true);
    my_io::cls_with_new_lines();
    let mut current_guessed_word: Vec<char> = Vec::new();
    // set up the number of guesses, takes the input from console, and handles edge cases
    println!("number of guesses: ");
    let guess_amount: usize = 
    match my_io::getln_int::<usize>(){
        None =>{general::unique_chars_in_str(word_to_guess.as_str()).len() + 3}
        Some(x) => {x}
    };
    let mut number_of_guesses: usize = 0;

    for _char in word_to_guess.chars(){
        current_guessed_word.push('_');
    }
    println!("currently revealed letters: {}", general::parse_iter(current_guessed_word.iter(),","));
    while current_guessed_word.contains(&'_')
    {
        let guess = my_io::getln_char();
        if guess.is_none() {
            println!("you did not enter a letter to guess, please enter a letter that you would like to guess");
            continue;
        }

        if number_of_guesses > guess_amount{
            println!("you have exceeded the maximum allowed guesses, the word was {}", word_to_guess);
            return;
        }

        let guess_ch: char = guess.unwrap();
        println!("you guessed: {}", guess_ch);

        if word_to_guess.chars().contains(&guess_ch){
            let number_of_results = word_to_guess.chars().filter(|x| *x == guess_ch).count();
            println!("The guess was correct! this word contains {} {}'s",number_of_results, guess_ch);
            for i in 0..current_guessed_word.len(){
                current_guessed_word[i] = match word_to_guess.chars().nth(i){
                    None => {0 as char}
                    Some(x) => {
                        if x == guess_ch || current_guessed_word[i] != '_'{
                            x
                        }
                        else{
                            '_'
                        }
                    }
                };
            }
        }
        else{
            number_of_guesses += 1;
        }
        println!("currently revealed letters: {}", general::parse_iter(current_guessed_word.iter(),","));
    }
    println!("good job! you guessed the word! the word was {}", word_to_guess)
}