//primitive string is an immutable fixed length str somewhere in memory
// String = growable, heap-allocated data structure - use when you need to modify or own string data

pub fn run(){
    let mut hello = String::from("Hello ");
    //get length
    println!("Length: {}", hello.len());

    hello.push('W');

    hello.push_str("orld!");

    println!("{}", hello);

}