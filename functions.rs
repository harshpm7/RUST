// Functions - Used to store blocks of code for re-use


pub fn run() {
    greeting("Hello", "Jane");

    //Bind funtion values to variables
    let get_sum = add(5,5);
    println!("Sum: {}", get_sum);

    //Closure
    let num3: i32 = 10;
    let add_nums = |num1:i32, num2:i32| num1+num2+num3;
    println!("C sum {}", add_nums(3,3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {} , nice to meet you", greet, name);
}

fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2         //While returning no semi colon
}