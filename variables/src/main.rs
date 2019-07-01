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
}