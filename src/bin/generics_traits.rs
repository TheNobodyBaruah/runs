//! generics_traits.rs

// This is using Feynman's way of learning 
// The goal is to understand generics in Rust and 
// how it ties in with traits
// I also want to learn how associated types are used 
// what is the reason for existence of associated types

// Define trait
trait Vehicle {
    // Capable of moving 
    fn movement(&self) {
        println!("The vehicle is moving");
    }

    fn refuel(&self) {
        println!("Vehicle refueling!");
    }

    // Every type implementing Vehicle should have a 
    // name field, which should return the name 
    // as an &str
    // If any standalone function using generics
    // need to use the inner fields of the type 
    // which replaces the generics, it should be done 
    // using a method in the trait the type(struct) 
    // implements.
    fn name(&self) -> &str; // DEFAULT impl in the trait def
}




// Data type which implements the trait (struct)
struct Car {
    name: String, // Why can't we use &str here?
    year: String, // Why can't we use &str here?
}

// impl functions for the data type wrt to trait
impl Vehicle for Car  {
    fn movement(&self) {
        println!("The tires are rotating and {} is moving", self.name);
    }

    fn refuel(&self) {
        
    }

    fn name(&self) -> &str {
        &self.name.as_str()
    }
}


// general impl function of the data type
impl Car {
    fn new(name: String, year: String) -> Car {
        // we wish to return a Car struct with some name and year
        Car {
            name: name,
            year: year,
        }
    }
}


// Another Data type which implements the trait (struct)
struct Plane {
    name: String,
    year: String,
}

// impl functions for the data type wrt to trait
impl Vehicle for Plane {
    fn movement(&self) {
        println!("The {} is flying", Plane::name(&self));
    }
    
    fn refuel(&self) {
        
    }

    fn name(&self) -> &str {
        &self.name.as_str()
    }
}


// general impl function of the data type
impl Plane {
    fn new(name: String, year: String) -> Plane {
        Plane {
            name: name,
            year: year,
        }
    }
}






// A stand alone function using generics, 
// the concrete type replacing the generics should 
// implement trait
fn passengers_enter_vehicle<T: Vehicle>(vehicle: &T) -> String {
    format!("The {} is ready to go", vehicle.name())
}




// The main function
fn main() {
    let car1_name = String::from("Volvo");
    let car1_year = String::from("2005");
    let car1 = Car::new(car1_name, car1_year);


    // Let's call the standalone function
    let onboard = passengers_enter_vehicle(&car1);

    let plane1_name = String::from("Boeing");
    let plane1_year = String::from("1999");
    let plane1 = Plane::new(plane1_name, plane1_year);
    let onboard_plane = passengers_enter_vehicle(&plane1);

    println!("{}", onboard);
    println!("{}", onboard_plane);
    
}







