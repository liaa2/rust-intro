#![allow(dead_code)]
#![allow(unused_variable)]
mod sh;
//import
use std::mem;

//define a constant, give it a type, then give it an initial value
const MEANING_OF_LIFE:u8 = 42; //it has no fixed address

static Z:i32 = 123;

fn fundamental_data_types() {
    // println!("Hello, world!");

    //u - stands for unsigned 0 or positive whole numbers
    //unsigned single bit - value goes from 0 to 255
    let a:u8 = 123; // 8 bits

    //i - signed number, -ve integers
    //value ranges from -127 to 128
    let b:i8 = -123;

    //in the output, the {} will be replaced by the value comes after
    println!("a = {}", a);
    println!("b = {}", b);

    //if try to re-assign the value, would get the following error when try to compile it:
    //cannot assign twice to immutable variable `a`
    // a = 456; 

    //In Rust, we created immutable variable by default
    //If want it to be changeable, have to be explicit about it by using the mutable keyword - mut
    let mut c:i8 = 0; // mutable
    println!("c = {}", c);
    c = -10;
    println!("new c = {}", c);

    //We could also let Rust to figure out how many bits it should allocated to the specific variable
    let mut c = 123456789; //32-bit signed variable i32
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

    c = -1;
    println!("c = {} after modification, size = {} bytes", c, mem::size_of_val(&c));

    // i8, u8, i16, u16, i32, u32, i64, u64
    //isize/usize - integral data types
    let z:isize = 123; //isize/usize
    let size_of_z = mem::size_of_val(&z);
    //size is bytes, multiply by 8 to get bit
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8);

    //:char is optional
    let d:char = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    let e = 2.5; //double-precision, 8 bytes or 64 bits, data type: f64 (floating points)
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    //true / false
    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
    let f = 4>0; //true
    println!("f = {}, size = {} bytes", f, mem::size_of_val(&f));
}

fn operators() {
  // arithmetic
  let mut a = 2 + 3*4; // +=*/
  println!("{}", a);
  a = a+1; // Rust does not support -- or ++
  a -= 2; // a = a-2
  a %= 2;
  println!("remainder of a is {}", a);

  let a_cubed = i32::pow(a, 3);
  println!("{} cubed is {}", a, a_cubed);

  let b = 2.5;
  let b_cubed = f64::powi(b, 3);
  let b_to_pi = f64::powf(b, std::f64::consts::PI); //f means floating point
  println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

  //bitwise - only available for integers
  let c = 1 | 2; // | OR & AND ^ XOR (exclusive or) !NOR (negation)
  // 01 OR 10 = 11 === 3_10 (11 is equal to 3 in the decimal system)

  println!("1|2 = {}", c);
  let two_to_10 = 1 << 10; // let variable equals to 1 and shifted 10 points to the left
  println!("2^10 = {}", two_to_10);

  //logical operators
  let pi_less_4 = std::f64::consts::PI < 4.0; //true
  // > <= >= == 
  let x = 5;
  let x_is_5 = x == 5; //returns boolean value
}

fn scope_and_shadowing(){
    let a = 123;
    {
        let b = 456;
        println!("inside, b={}", b);
        

        let a = 777;
        println!("inside, a={}", a);
    }
    println!("outside, a={}", a);
}

fn if_statement(){
    let temp = 35;

    if temp > 30 {
        println!("really hot outside");
    } else if temp < 10 {
        println!("really code");
    }
    else {
        println!("ok");
    }

    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("today is {}", day);

    println!("is it {}", 
        if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"ok"});
    
    println!("it is {}", 
        if temp > 20 {
            if temp > 30 {"very hot"} else {"hot"}
        } else if temp < 10 {"cold"} else {"Ok"}
    )
}

fn main() {
    // println!("{}", Z);
    // sh::stack_and_heap();
    if_statement();
}
