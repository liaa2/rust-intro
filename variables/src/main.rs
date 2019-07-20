fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //declare constants using the const keyword instead of the let keyword, AND the type of the value must be annotated. 
    // constants may be set only to a constant expression, not the result of a function call or any other value that could only be computed at runtime
    const MAX_POINTS: u32 = 100_000;

    let y = 1;
    // the first variable is "shadowed" by the second
    let y = y + 1;
    //Shadowing is different from marking a variable as mut, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the "let" keyword

    //The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name. 
    println!("y is {}", y);

    //Rust has two primitive compound types: tuples and arrays.
    // A tuple is a general way of grouping together some number of other values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //To get the individual values out of a tuple, use pattern matching to destructure a tuple value
    let (x, y, z) = tup;
    println!("the value of x, y, z is {}, {}, {}", x, y, z);

    //In addition to destructuring through pattern matching, could also access a tuple element directly by using a period (.) followed by the index of the value we want to access.
    let first_value = tup.0;

    let second_value = tup.1;

    println!("first_value={}, second_value={}", first_value, second_value);

    //Another way to have a collection of multiple values is with an array.
    //Unlike a tuple, every element of an array must have the same type.
    // Arrays in Rust have a FIXED length, like tuples.
    let a = [1,2,3,4,5];

    // would write an array’s type by using square brackets, and within the brackets include the type of each element, a semicolon, and then the number of elements in the array
    let b: [i32; 4] = [1,2,3,4];

    //To initialize an array
    let c = [4; 6]; //c = [4,4,4,4,4,4];
}