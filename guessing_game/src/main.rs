use std::io; // bring input/output library into scope, io library comes from the standard library
use std::cmp::Ordering;
use rand::Rng;

fn main () {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is : {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        //stdin is a type that represents a handle to the standard input for your terminal
        io::stdin().read_line(&mut guess) 
        /*The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. References are immutable by default, thus need to write &mut guess rather than &guess to make it mutable */
            .expect("Failed to read line");

        /*read_line puts what the user types into the string we’re passing it, but it also returns a value—in this case, an io::Result. Rust has a number of types named Result in its standard library: a generic Result as well as specific versions for submodules, such as io::Result.

        The Result types are enumerations, often referred to as enums. An enumeration is a type that can have a fixed set of values, and those values are called the enum’s variants.

        For Result, the variants are Ok or Err. The Ok variant indicates the operation was successful, and inside Ok is the successfully generated value. The Err variant means the operation failed, and Err contains information about how or why the operation failed. */  

        //The parse method on strings parses a string into some kind of number.
        //Because this method can parse a variety of number types, we need to tell Rust the exact number type we want by using let guess: u32.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // continue - tells the program to go to the next iteration of the loop
        };  //parse returns a Result type and Result is an enum that has the variants Ok or Err.
        
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}