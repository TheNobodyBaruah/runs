use std::fmt::{Display, Formatter};

// Define a trait using a generic
/* One of more of the functions may 
  take variables as parameters with 
  some generic types */
trait SomeTriat<T: Display> {
    fn some_fun(&self, var: T) -> ();
}



// Define the struct (type)
struct SomeStruct<'a, U: Display> {
    a: u32,
    b: i32,
    c: String,
    // d doesn't own the string, it's borrowing
    // the compiler doesn't know when it goes out of scope
    // Using 'a is like saying if the borrowed string goes 
    // out of scope then `SomeStruct` goes out of struct too!
    d: &'a str,
    e: U // Let's make the datatype U is a placeholder to
         // implement Display
}

// impl trait for struct
// generic constraits all of them,  related to the type should be described just after `impl`
// Concrete types for the traits generics should be decided 
// when implementing traits for a type
impl<'a, U: Display> SomeTriat<i32> for SomeStruct<'a, U> {
    // The compiler would know which implementation based on the argument type
    fn some_fun(&self, _var: i32) {
        println!("{} is a i32", &self.b);
    }
}

impl<'a, U: Display> SomeTriat<u32> for SomeStruct<'a, U> {
    fn some_fun(&self, _var: u32) {
        println!("{} is a u32", &self.a);
    }
}

impl<'a, U: Display> SomeTriat<Struct_U> for SomeStruct<'a, U> {
    fn some_fun(&self, _var: Struct_U) {
        println!("{} this is U", &self.e);

    }
}


impl<'a, U: Display> SomeTriat<String> for SomeStruct<'a, U> {
    fn some_fun(&self, _var: String) {
        println!("{} is a String", &self.c);
    }
}

impl<'a, U: Display> SomeTriat<&'a str> for SomeStruct<'a, U> {
    fn some_fun(&self, _var: &'a str) {
        println!("{} is a &str", &self.d);
    }
}

// Let's define the type to which U is a placeholder to
// Let's make it clonable
#[derive(Clone)]
struct Struct_U {
    x: i32,
    y: i32
}

// It implements Display trait
impl Display for Struct_U {
    // This trait requires `fmt` with this exact signature
    // std::fmt::Result is same as Result((), std::fmt::Error)
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Use the `write!` macro to write the formatted string into the 
        // formatter `f` passed as arguments
        write!(f, "({} {})", self.x, self.y) // expression not statement
    }
}

// One of the function with a generic variable use some concrete type
// That same function with some other concrete type
// That same function with some other concrete type


// General impl functions for struct





// Define trait with associated types
// Test multiple associated types on a single trait



// Define struct


// impl trait for struct
// Use associated type
// What is the reason for existence of associated types when generics are a thing



// our main()
fn main() {
    // Let's instantiate Struct_U
    let u_struct = Struct_U {
        x: 33, 
        y: 33
    };
    println!("{}", u_struct);
    // Let's instantiate the struct 
    let d_struct = SomeStruct {
        a: 32,
        b: -32, 
        c: String::from("Hello"), 
        d: "Hi", 
        e: u_struct.clone()
    };



    // To Remember: &self is implicitly handled by the dot notation
    d_struct.some_fun(-44); 
    d_struct.some_fun(33);
    d_struct.some_fun(u_struct.clone());
    d_struct.some_fun(String::from("Greeting")); 
    d_struct.some_fun("Greet"); 

}


