// I need to pay attention to the loops and the variables 
// that we'd use to break the loop or continue, 
// if they are located outside the loop then we'd never exit the
// loop since that'd never be read.
use std::io::{Write};
// I need something which has two states at least such that 
// we can 
trait Quality {
    fn action(&self) -> String;
}

struct DoCse {
    learn_ml: bool, 
    learn_ai: bool, 
    learn_rust: bool, 
}

impl Quality for DoCse {
    fn action(&self) -> String {
        // I want all the fields which are true and print them
        match self {
            DoCse{
                learn_ml: true, 
                learn_ai: true, 
                learn_rust: true,
            } => String::from("You'll be on top!"), 
            DoCse {
                learn_ml: true, 
                learn_ai: true, 
                learn_rust: false, 
            } => String::from("You'll succeed, but why'd you 
                    not do what you love?"),
            DoCse {
                learn_ml: true, 
                learn_ai: false, 
                learn_rust: false, 
            } => String::from("Okay, your probability of success decreases considerably.
                    You should reconsider some shit!"),
            DoCse {
                learn_ml: false, 
                learn_ai: false, 
                learn_rust: false,
            } => String::from("If you don't do anything, you definitely know what's 
                    going to happen, remember NEET"),
            DoCse {
                learn_ml: false, 
                learn_ai: true, 
                learn_rust: true,
            } => String::from("You'd definitely succeed, but it'd be smart to learn some ML"), 
            DoCse {
                learn_ml: false, 
                learn_ai: false, 
                learn_rust: true,
            } => String::from("There is some possibility for you to succeed, but do AI"), 
            DoCse {
                learn_ml: false, 
                learn_ai: true, 
                learn_rust: false, 
            } => String::from("Probability of success decreases"), 
            DoCse {
                learn_ml: true, 
                learn_ai: false, 
                learn_rust: true, 
            } => String::from("Not very smart to do so, you'd be unique, I'd give you that!")
        }
    }
}

impl DoCse {
    fn build() -> Self {
        // We'll have to get user input for what bool 
        // to put into `DoCse`
        // Using something like a list would be more efficient, 
        // since, everything would be stored in the stack
        // let's do it in an inefficient way and later we'd refine
        // it by storing the parsed args in a list in the stack
        // without the heap overhead. 

        // ML
        // let mut ml_input = String::new();
        let mut list_args = [String::new(), String::new(), String::new()];
        loop {
            println!("Are you learning Machine Learning?"); // Prompting user to respond
            println!("Type 1 if you are and 2 if you aren't");
            std::io::stdout()
                .flush()
                .expect("Failed to flush stdout!");
            std::io::stdin()
                //.read_line(&mut ml_input)
                .read_line(&mut list_args[0])
                .expect("Failed to read lines");

            if list_args[0].as_str() == "1" || list_args[1].as_str() == "2" {
                break;
            } else {
                println!("Type 1 or 2 and nothing else");
                continue
            }
        }
        
        // let cleaned_ml_input: &str = ml_input.trim();

        // AI
        //let mut ml_
        loop {
            println!("Are you learning AI Engineering?"); // Prompting user to respond
            println!("Type 1 if you are and 2 if you aren't");
            std::io::stdout()
                .flush()
                .expect("Failed to flush stdout!");
            std::io::stdin()
                .read_line(&mut list_args[1])
                .expect("Failed to read line");

            if list_args[0].as_str() == "1" || list_args[1].as_str() == "2" {
                break;
            } else {
                println!("Type 1 or 2 and nothing else");
                continue
            }

        }
        
        // Rust
        //let mut ml_
        loop {
            println!("Are you learning Rust?"); // Prompting user to respond
            println!("Type 1 if you are and 2 if you aren't");
            std::io::stdout()
                .flush()
                .expect("Failed to flush stdout!");
            std::io::stdin()
                .read_line(&mut list_args[2])
                .expect("Failed to read line");

            if list_args[0].as_str() == "1" || list_args[1].as_str() == "2" {
                break;
            } else {
                println!("Type 1 or 2 and nothing else");
                continue
            }

        }
        // Since our list is filtered to have only 1s and 2s
        // now we can populate the DoCse I'm building.
        let ml: bool = match list_args[0].as_str() {
            "1" => true, 
            "2" => false,
            _ => false, // This is for the the compiler to 
                        // not cause any problems
        };
        let ai: bool = match list_args[1].as_str() {
            "1" => true, 
            "2" => false,
            _ => false, // This is for the the compiler to 
                        // not cause any problems
        };
        let rust: bool = match list_args[2].as_str() {
            "1" => true, 
            "2" => false,
            _ => false, // This is for the the compiler to 
                        // not cause any problems
        };
        
        // Get the value for each and send it to the fields of the struct
        // We'll have to convert the Strings from the user input
        // to boolean

        Self {
            learn_ml: ml,
            learn_ai: ai,
            learn_rust: rust
        }
    }
}

struct DoPhar {
    learn_medchem: bool
}
impl Quality for DoPhar {
    fn action(&self) -> String {
        match self {
            DoPhar{ learn_medchem: true } => String::from("Only reading medchem could be difficult, try compsci!"), 
            DoPhar{ learn_medchem: false } => String::from("No medchem and not reading medchem, well you should"),
        }

    }
} 

impl DoPhar {
    fn build() -> Self {
        // Since, there is just one field, a variable would do
        let mut medchem_learn = String::new();
        loop {
            // Get user input as you did for DoCse
            println!("Are you learning Medicinal Chemistry");
            println!("Type 1 if you are and 2 if you are not");
            std::io::stdout()
                .flush()
                .expect("Failed to flush stdout");
            std::io::stdin()
                .read_line(&mut medchem_learn)
                .expect("Failed to read lines");

            if medchem_learn.as_str() == "1" || medchem_learn.as_str() == "2" {
                break;
            } else {
                continue;
            }
        }

        

        // We'll have to convert the Strings from the user input
        // into boolean
        let mut medchem: bool = match medchem_learn.as_str() {
            "1" => true,
            "2" => false, 
            _ => false, // Just for the compiler not complaining
                        // the if-else handles it 
        };

        Self {
            learn_medchem: medchem
        }
    }
}

struct DoBoth {
    field1: DoCse,
    field2: DoPhar
}

impl Quality for DoBoth {
    fn action(&self) -> String {
        let cse_act: String = self.field1.action();
        let phar_act: String = self.field2.action();
        format!("This is what you get from cse and phar {}; {} respectively", cse_act, phar_act)
    }
}

impl DoBoth {
    fn build() -> Self {
        Self {
            field1: DoCse::build(),
            field2: DoPhar::build()
        }
    }
}

struct Nothing {
    field: String,
}

impl Quality for Nothing {
    fn action(&self) -> String {
        format!("Nothing here")
    }
}

impl Nothing {
    fn build() -> Self {
        Self {
            field: String::from("Nothing")
        }
    }
}


fn main() {
    // I'll need a variable to store user input
    let mut user_input: String = String::new();
    // Prompt the user for input
    println!("Clasify your action:");
    println!("Type 1 for Cse. Type 2 for Phar. Type 3 for both");

    std::io::stdout()
        .flush()
        .expect("Failed to flush stdout.");

    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input!");

    let cleaned_variable: &str = user_input.trim(); // I am not sure if this'd be a String or
                                                      // &str
    let you_chose: Box<dyn Quality> = match cleaned_variable {
        "1" => Box::new(DoCse::build()), 
        "2" => Box::new(DoPhar::build()), 
        "3" => Box::new(DoBoth::build()), 
        _ => Box::new(Nothing::build()),
    }; // I gotta play with these to make this program work.
    // println!("You chose {}", you_chose);





}
