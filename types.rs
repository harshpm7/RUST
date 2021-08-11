pub fn run() {

    //Default is i32
    let x = 1;

    //Default is f64
    let y = 2.5;

    //Add explicit type
    let z: i64 = 1234567890;

    //find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //Boolean
    let is_active: bool = true;         //let is_active = true; both works fine

    //Get boolean from expression
    let is_Greater: bool = 10 < 5;

    let a1 = 'a'; //Characters are described in a single quote
    let face = '\u{1F600}'; // \u stands for unicode

    println!("{:?}", (x,y,z,is_active, is_Greater, a1, face));
}