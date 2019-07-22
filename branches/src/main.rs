fn main() {
    let number = 3;
    
    //NOTE: Rust will not automatically try to convert non-Boolean types to a Boolean -> cant simply use 'if number' for conditions
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //as if is an expression, can use it on the right ide of a let statement
    let condition = true;
    let number = if condition {
        5
    } else {
        -9
    };
    println!("the value of number is {}", number);
}
