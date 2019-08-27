extern crate interactive_term;
use interactive_term::my_mod;
use interactive_term::my_mod2;

fn main() {
    println!("ex1 main:");
    my_mod::stuff2();
    my_mod2::stuff3();
    interactive_term::stuff();
}