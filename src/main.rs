mod hangman;
mod sayback;
pub mod my_io;
pub mod general;
mod cuss;
use std::*;

struct Module<'a>{
    name:&'a str,
    _fn: fn()
}

fn main(){
    const STARTING_MODULE:&str = "cuss";
    let modules: Vec<Module> = vec![
        Module{name:"hangman",_fn:hangman::run},
        Module{name:"sayback",_fn:sayback::run},
        Module{name:"cuss",_fn:cuss::run}
        ];

    if STARTING_MODULE == "user_choice" {
        start_module(choose(&modules).as_str(),&modules)
    }
    else{
        start_module(STARTING_MODULE, &modules)
    }
}

fn start_module(module_name:&str, module_options:&Vec<Module>){
    for module in module_options.into_iter(){
        if module.name == module_name{
            (module._fn)();
            break
        }
    }
}

fn choose(module_options:&Vec<Module>) -> String{
    println!("this is a test application to try and figure out rust as a language.\nhere are some test programs that you can run:");
    fn get_name<'a>(x:&'a Module,) -> &'a str{
        x.name
    }
    println!("{}", general::concat_iter(module_options.into_iter().map(get_name).into_iter(), "\n"));
    println!("please select a module: ");
    let selected = my_io::getln(true).to_lowercase();
    my_io::cls_with_new_lines();
    selected
}