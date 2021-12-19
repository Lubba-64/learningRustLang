mod hangman;
mod sayback;
pub mod my_io;

const MY_PROGS: [&str; 2] = ["hangman","sayback"];



fn main(){
    println!("this is a test application to try and figure out rust as a language.\nhere are some test programs that you can run:");
    println!("{}", my_io::parse_iter(MY_PROGS.iter(), "\n"));
    println!("please select a module: ");
    let prog = my_io::getln(true).to_lowercase();
    if prog == String::from("hangman") {
        hangman::run();
    }
    else if prog == String::from("sayback") {
        sayback::run();
    }
    else{
        println!("no module by that name was found.\nrerun the application to run one of the included modules");
    }
    
}
