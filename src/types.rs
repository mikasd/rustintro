pub fn run(){
    //by default without explicit declaration this is i32
    let x = 1;
    //defaults to f64 aka float 64bit
    let y=2.5;
    //explicit declaration
    let z: i64 = 234234234234;
    //find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    //boolean
    let is_active: bool = true;
    //get boolean from expression
    let is_greater: bool = 10 > 5;

    let a1 = 'a';
    let face = '\u{1F600}';
    println!("{:?}",(x,y,z, is_active, is_greater, a1, face));
}