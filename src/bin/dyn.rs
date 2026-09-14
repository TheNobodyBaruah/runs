use std::io::{self, Write};
// This is a file to learn how dyn works
// Let's write some trait 
trait SomeTrait<T: Display> {
    // add code here
    fn some_fun(&self);
    fn some_other_fun(&self, var: T);
}

// Some structs which follow these traits
struct SomeStruct {
    // Some code here
    field_a : i32,
    field_b : u32
}

impl SomeTrait for SomeStruct {
    // Some code
    fn some_fun(&self) {
        println!("This is some function");
    }

    fn some_other_fun(&self, var: i32) -> String { // We'll need something
                                                             // to replace T
                                                             // which means, we'll have to 
                                                             // have a type which will have Display
                                                             // trait, or create one with an impl
                                                             // Display.
        println!("Something,something and whatever the {} is", var);
        format!("Something something!")
    }
}

impl SomeStruct {
    //Some code 
    fn do_anything() {
        // some code 
        println!("Doing something")
    }
}

struct SomeOtherStruct<'a> {
    // Some code 
    field1: String, 
    field2: &'a str
}

impl<'a> SomeTrait for SomeOtherStruct {
    // Some code 
    fn some_fun<'a>(&self) {
        println!("Whatever this will be");
    }

    fn some_other_fun(&self, variable: i32) {

    }
}

impl SomeOtherStruct {
    // Some code 
    fn do_something() {
        // some code here
    }
}

struct AStruct {
    a: String, 
    b: String 
}

impl SomeTrait {
    fn some_fun() {
        // Some code here
    }

    fn some_other_fun() -> String {
        // Some code here
    }
}

fn main() {
    // Some code 
    // We are experimenting with `dyn`,
    // which means that we'll have to use some conditionals 
    // or match statements such that the type of which something 
    // is can only be known at run time

    // Get user input asking yes or no, mark both as a) and b),
    // Any other thing is marked irrelevant. 
    // Create a mutable string to store the input, and match later
    let mut user_input: String = String::new();
    
    // Prompt the user for response
    println!("Type \'a\' for \"Yes\" and \'b\' for \"No\"");

    // Flush the stdout so the prompt prints before waiting for input
    io::stdout().flush().expect("Failed to flush stdout");


    // Read the line from the terminal
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user_input");

    // Remove the enter that the user typed to send the 
    // response to the program
    let cleaned_input = user_input.trim();

    let which_struct: Box<dyn SomeTrait> = match cleaned_input {
        "a" => SomeStruct {},
        "b" => SomeOtherStruct{},
        _ => AStruct{}
    }; 

    println!("You typed {}", cleaned_input);




}
