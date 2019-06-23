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

fn while_and_loop(){
    let mut x = 1;
    while x < 1000 {
        x *= 2;

        if x == 64 { continue; } // continue resume execution the loop without running code after this line
        println!("x= {}", x);
    }

    let mut y = 1;
    //while true
    loop {
        y *=2;
        println!("y={}", y);

        if y == 1 << 10 { break; }
    }
}

fn for_loop() {
    for x in 1..11  { // 11 in NOT included in .. operator
        if x == 3 { continue; }

        if x == 8 { break; }
        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate() {
        println!("{} : {}", pos, y);
    }
}

fn match_statement(){
    let country_code = 778; // 1 999
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1...999 => "unknown", // 999 is included in ... operator
        _ => "invalid", //if didn't match the above three numbers
    };

    println!("country with code is {}, {}", country, country_code );
}

struct Point {
    x: f64, 
    y: f64
}

struct Line {
    start: Point,
    end: Point,
}

fn structures(){
    let p = Point{
        x: 3.0,
        y: 4.0
    };
    println!("point p is at ({}, {})", p.x, p.y);

    let p2 = Point{ x:5.0, y: 10.0 };
    let myline = Line { start: p, end: p2};
}

enum Color {
    Red,
    Green,
    Blue,
    //u8, u8, u8 is tuple format, thus can't put the name as parameters
    RgbColor(u8, u8, u8), //tuple 
    CmykColor{cyan: u8, magenta: u8, yellow: u8, black: u8}, //struct
}

fn enums () {
    let c:Color = Color::CmykColor{cyan: 0, magenta: 128, yellow: 0, black: 255};

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        // _ => println!("some other color"),
        Color::Blue => println!("b"),
        Color::RgbColor(0,0,0)
        | Color::CmykColor{cyan: _, magenta: _, yellow: _, black: 255} => println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb {}, {}, {}",r,g,b),
        _ => ()
    }
}

//32 bits
union IntOrFloat {
    i: i32,
    f: f32
}

fn process_value(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat {i: 42} => {
                println!("meaning of life value");
            }

            IntOrFloat { f } => {
                println!("value={}", f);
            }
        }
    }
}

//option is the type which indicate the presence or absence of a particular value
fn option(){
    // Option<T>
    let x = 3.0;
    let y = 0.0;
    
    // let result = x/y; //if y is 0 then the value of result is `inf`, we could use Option to avoid this situation

    //the new result is an Option of f64, Option can contain Some(z) or None
    //Some(z) means there is an answer and the value of the answer is z
    let result:Option<f64> = if y != 0.0 { Some(x/y) } else { None };
    println!("{:?}", result);
}

fn main() {
    // sh::stack_and_heap();
    // let mut iof = IntOrFloat{i: 123 };
    // iof.i = 234;
    // process_value(IntOrFloat{i:123})
    option();
}
