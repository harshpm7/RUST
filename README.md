# RUST

The basics of RUST programming

    • Doesn’t have garbage collection and not needed to manage memory, checks on demand when needed
    • Package manager: cargo
    • Create a file and start with a main function as “fn main()” that acts as an entry point and compile using “rustc filename” and run the file using “./filename” and the file extension is .rs
    • Also can compile and run directly using the cargo package manager with the help of the command “cargo run” else find the executable in the target folder and provide the path after ./
    •  cargo build:  to just want to build and not run it
    • cargo build –release:  build for production
    • Cannot just directly prinln the integer and so can use {} as place holders
    • Also we can use positional arguments and use the numbers starting from 0 index into the curly braces
    • Apart from these there are named arguments where you can assign a value to a particular variable and mention the variable in the print statement
    • Placeholder traits used to get the binary, hexadecimal, octal values etc for given values
    • Placeholder for debug traits that becoomes handy when you want to print a complete array also can do basic maths
    • Rust is a statically typed language, which means that it must know the types of all variables at compile time, however the compiler can usually infer what type we want ot use based on the value and how we use it
