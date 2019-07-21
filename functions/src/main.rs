fn main() {
    println!("Hello, world!");

    another_function(5);
    expression_and_statement();

    let x = five();
    println!("the value of x is {}", x);

    let y = plus_one(4);
    println!("the value of y is {}", y);
}

// we could define another_function after or before the main function. Rust doesn’t care where you define your functions, only that they’re defined somewhere.
//In function signatures, must declare the type of each parameter
fn another_function(x: i32){
    println!("Another function.");
    println!("x = {}", x);
}

fn expression_and_statement(){
    let x = 5;

    //the block below evaluates to 4. That value gets bound to y as part of the let statement
    let y = {
        let x = 3;
        //NOTE: No semicolon for expression
        //Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value.
        x + 1
    };

    println!("x={}, y={}", x, y);
}

//declare return value's type after an arrow 
fn five() -> i32 {
    5 // no return keyword, Rust returns the last expression implicitly
}

fn plus_one(y: i32) -> i32 {
    y + 1
}
 