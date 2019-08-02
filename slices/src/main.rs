//Another data type that does not have ownership is the slice. Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.

fn main() {
    first_world("hello, world");

    let s = String::from("slice");

    let slice = &s[0..2];
    // start at the first index (zero), you can drop the value before the two periods
    let slice = &s[..2];

    let len = s.len();

    let slice = &s[3..len];
    // slice includes the last byte of the String, you can drop the trailing number
    let slice = &s[3..];


    let slice = &s[0..len];
    // You can also drop both values to take a slice of the entire string. So these are equal
    let slice = &s[..];

    
    //The type of a here is &str: it’s a slice pointing to that specific point of the binary. This is also why string literals are immutable; &str is an immutable reference.
    let a = "Hello, world";


    let my_string = String::from("String");

    //first_word works on slices of `String`s
    let word = first_world(&my_string[..]);

    let my_string_literal = "another string";

    //first_word works on slices of string literals
    let word = first_world(&my_string_literal[..]);

    //Because string literals ARE string slices already, this works too, without the slice syntax
    let word = first_world(my_string_literal);
}


// The type that signifies “string slice” is written as &str

// change type of s from &String to &str, it allows us to use the same function on both &String values and &str values

//If we have a string slice, we can pass that directly. If we have a String, we can pass a slice of the entire String
fn first_world(s: &str) -> &str {

    // Because we need to go through the String element by element and check whether a value is a space, we’ll convert our String to an array of bytes using the as_bytes method.
    let bytes = s.as_bytes();

    //create an iterator over the array of bytes using the iter method, it returns each element in a collection and that enumerate wraps the result of iter and returns each element as part of a tuple instead
    //enumerate returns a tuple, we can use patterns to destructure that tuple
    for (i, &item) in bytes.iter().enumerate() {
        //Inside the for loop, we search for the byte that represents the space by using the byte literal syntax
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
