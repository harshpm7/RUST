
//Variables hold primitive data or reference to data
//Variables are immutable by default
//Rust is a block-scoped language

pub fn run() {
    let name = "Brad";
    let mut age = 25;   //to make the variable mutable

    println!("My name is {} and I am {}", name, age);
    age = 26;
    println!("My name is {} and I am {}", name, age);

    //Define const
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign multiple variables at once
    let (my_name, my_age) = ("Brad", 25);
    println!("My name is {} an I am {}", my_name, my_age);

}