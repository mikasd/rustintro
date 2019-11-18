//variables hold primitive data or references to data
//variables are immutable by default
//rust is block scoped

pub fn run(){
    let name = "Mike";
    let age = 37;
    println!("My name is {} and I am {}", name, age);

    //define a constant

const ID: i32 = 001;
println!("ID: {}",ID); 
}