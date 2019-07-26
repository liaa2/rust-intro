fn main() {
    //String literal is stored on the stack and popped off the stack when their scope is over,
    let a = "this string is stored on the stack";

    //Rust offers a second string type, 'String'. This type is allocated on the heap and as such is able to store an amount of text that is unknown tu us at compile time. We can create a String from a string literal using the 'from' function
    let s = String::from("hello");
    //The double colon (::) is an operator that allows us to namespace this particular from function under the String type rather than using some sort of name like string_from.


    // 1 - Shallow copy - copy data on stack only

    let s1 = String::from("test");
    //when s2 copy the pointer, length and capacity from s1(which stored in stack), Rust automatically invalids s1 -> s1 as moved into s2 instead of making a shallow copy -> prevents double free error when both s1 and s2 are out of scope

    // string literal "test" is stored in the heap
    let s2 = s1;

    //we CANT use s1 after it has been moved
    //println!("{}, world!", s1);


    // 2 - deep copy - copy data on stack AND heap
    let d1 = String::from("deep copy");
    let d2 = d1.clone();

    println!("s1 = {}, s2 = {}", d1, d2);


    //Special case  - copying from a integer
    let x = 5;
    let y = x;


    //types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. That means there’s no reason we would want to prevent x from being valid after we create the variable y.
    println!("x = {}, y = {}", x, y);


    // COPY TRAIT -  If a type has the Copy trait, an older variable is still usable after assignment. Here are some of the types that are Copy:
    // 1 - All the integer types, such as u32.
    // 2 - The Boolean type, bool, with values true and false.
    // 3 - All the floating point types, such as f64.
    // 4 - The character type, char.
    // 5 - Tuples, if they only contain types that are also Copy. For example, (i32, i32) is Copy, but (i32, String) is not.


    //when a variable goes out of scope - e.g. end of a block , Rust automatically calls the drop function and cleans up the heap memory for that variable

    let a = String::from("scope"); // a comes into scope

    takes_ownerships(a); // a's value moves into the function, and so is no longer valid here

    let x  = 10;

    makes_copy(x); // x would move into the function, but i32 is Copy, so it's okay to still use x afterward


    return_value_and_scope();


    //return multiple values using a tuple
    let t1 = String::from("multiple values");
    let (t2, len) = calculate_length(t1);

    println!("The length of '{}' is {}.", t2, len);
} // Here, x goes out of scope, then a. But because a's value was moved, nothing special happens.

fn takes_ownerships(some_string: String) { // some_string comes into scope
    println!("{}", some_string); 
} // Here, some_string goes out of scope and 'drop' is called. The backing memory is freed. 

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.



fn return_value_and_scope(){
    let s1 = gives_ownership();        
    // gives_ownership moves its return value into s1

    let s2 = String::from("hello");    
    // s2 comes into scope

    // let s3 = takes_and_gives_back(s2);  

    // s2 is moved into takes_and_gives_back, which also moves its return value into s3

} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was moved, so nothing happens. s1 goes out of scope and is dropped.


//give_ownership will move its return value into the function that calls it
fn gives_ownership() -> String {
    let some_string = String::from("random"); // some_string comes into scope

    some_string // some_string is returned and moves out to the calling function
}

//takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); //len() turns the length of a String

    (s, length)
}