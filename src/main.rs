use std::fmt;
//This is returning any datatype that implements the Display trait:
//As you can see, you cannot return an unknown datatype, the compiler won't let you. Dynamic dispatch is what will allow a type to be known at runtime, which is not needed to be known at compile time.
fn get_diplayable(choice: bool) -> impl fmt::Display {
    if choice {
        return 13;
    } else {
        return "thirteen";
    }
}

fn main() {
    println!("output is {}", get_diplayable(true));
}
