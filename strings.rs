// Primitive str = Immutable fixed length string somwhere in memory
// String = Growable, heap-allocated data structure ---- Use when you need to modify or own string data

pub fn run() {
    // let hello = "Hello"; //Immutable

    let mut hello = String::from("Hello ");

    //Get length
    println!("Length: {} ", hello.len());

    //two methods can be used to update the string push and push_str , push to add a character and push_str to add string
    hello.push('W');
    hello.push_str("orld");

    //Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    //Check if Empty
    println!("Is Empty: {}", hello.is_empty());

    // Contains a substring.?
    println!(" Contains 'World' {}" , hello.contains("World"));

    //Replace
    println!("Replace: {}", hello.replace("World", "There"));

    //Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}" , word);
    }


    //Create string with certain capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    //Assertion testing
    assert_eq!(2,s.len());
    assert_eq!(10,s.capacity());

    println!("{}", hello);
}