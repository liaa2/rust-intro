#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/*
    Methods are similar to functions: they’re declared with the fn keyword and their name, they can have parameters and a return value, and they contain some code that is run when they’re called from somewhere else. However, methods are different from functions in that they’re defined within the context of a struct (or an enum or a trait object), and their first parameter is always self, which represents the instance of the struct the method is being called on.
*/

// To define the function within the context of Rectangle, we start an impl (implementation) block. 
impl Rectangle {
    /* 
        In the signature for area, we use &self instead of rectangle: &Rectangle because Rust knows the type of self is Rectangle due to this method’s being inside the impl Rectangle context

        Chosen &self here for the same reason we used &Rectangle in the function version: we don’t want to take ownership, and we just want to read the data in the struct, not write to it.

        If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use &mut self as the first parameter.
    */    
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    /*
        Another useful feature of impl blocks is that we’re allowed to define functions within impl blocks that don’t take self as a parameter. These are called associated functions because they’re associated with the struct. They’re still functions, not methods, because they don’t have an instance of the struct to work with. E.g. String::from is an associated function.

        Associated functions are often used for constructors that will return a new instance of the struct. 

        To call this associated function, we use the :: syntax with the struct name. This function is namespaced by the struct: the :: syntax is used for both associated functions and namespaces created by modules.
    */

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {

    // 1. basic solution
    //problem with this version - the parameters are related, but that’s not expressed anywhere in our program. It would be more readable and more manageable to group width and height together.
    // let width1 = 30;
    // let height1 = 50;

    // 2. improved solution - use tuple
    let rect1 = (30, 50); // problem with this version - tuples don’t name their elements, so our calculation has become more confusing because we have to index into the parts of the tuple


    // 3. better one - refactoring with Structs
    let rect2 = Rectangle { width: 30, height: 50 };

    println!("The SIMPLE area of the rectangle is {} square pixels", simple_area(&rect2));

    /* 
        The println! macro can do many kinds of formatting, and by default, the curly brackets tell println! to use formatting known as Display: output intended for direct end user consumption. 

        But with structs, the way println! should format the output is less clear because there are more display possibilities: Do you want commas or not? Do you want to print the curly brackets? Should all the fields be shown? Due to this ambiguity, Rust doesn’t try to guess what we want, and structs don’t have a provided implementation of Display.

        Putting the specifier :? inside the curly brackets tells println! we want to use an output format called Debug. The Debug trait enables us to print our struct in a way that is useful for developers so we can see its value while we’re debugging our code.

        Rust does include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct. To do that, we add the annotation #[derive(Debug)] just before the struct definition
    */
    println!("rect1 is {:?}", rect2); // output -> rect1 is Rectangle { width: 30, height: 50 }

    /*
        When we have larger structs, it’s useful to have output that’s a bit easier to read; in those cases, we can use {:#?} instead of {:?} in the println! string. When we use the {:#?} style in the example, the output will look like this:
            rect1 is Rectangle {
                width: 30,
                height: 50,
            }
    */
    println!("rect1 is {:#?}", rect2);

    println!("The area of the rectangle is {} square pixels.", rect2.area());

    let rect3 = Rectangle { width: 10, height: 40 };
    let rect4 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect2.can_hold(&rect3));
    println!("Can rect1 hold rect3? {}", rect2.can_hold(&rect4));

    let sq = Rectangle::square(3);
    println!("Associated function called -> {:?}", &sq);
}

//change parameter from "width: u32, height: u32" to dimensions(u32, u32)
//change parameter from dimensions(u32, u32) to rectangle: &Rectangle, we want to borrow the struct rather than take ownership of it. 
fn simple_area(rectangle: &Rectangle) -> u32 {
    // width * height
    // dimensions.0 * dimensions.1
    rectangle.width * rectangle.height
}
