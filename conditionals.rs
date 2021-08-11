// Conditionals - Used to check on the condition of something and act on the result

pub fn run() {

    let age: u8 = 25;
    let check_id: bool = true;

     

    //If/Else
    if age >= 21 && check_id{       // Also can use || the OR operator
        println!("Bartender: What would you like to drink?");
    } else if age <= 21 && check_id{
        println!("Bartender: Sorry, you have to leave");
    } else {
        println!("Bartender: I'll need to see your Id");
    }

    //Shorthand If
    let is_of_age = if age >=21 {true} else {false};
    println!("Is Of Age: {}", is_of_age);


}