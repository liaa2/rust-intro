#![allow(dead_code)]
#![allow(unused_variable)]
mod sh;
mod pm;
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
    let y = 2.0;
    
    // let result = x/y; //if y is 0 then the value of result is `inf`, we could use Option to avoid this situation

    //the new result is an Option of f64, Option can contain Some(z) or None
    //Some(z) means there is an answer and the value of the answer is z
    let result:Option<f64> = if y != 0.0 { Some(x/y) } else { None };
    println!("{:?}", result);

    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("cannot divide {} by {}", x, y)
    }

    // if let /while let - initiate variable inside condition
    //Get the result(either Some or None, then de-structured into z)
    //if the destructuring works(the if condition holds), then excute thing inside {} block
    if let Some(z) = result {println!("z={}", z);}
}

fn arrays (){
    //making a mutable variable called `a` which has 5 32 bits integers
    let mut a:[i32;5] = [1,2,3,4,5]; //same as a = [1,2,3,4,5];

    println!("a has {} elements, first is {}", a.len(), a[0]);
    a[0] = 321;
    println!("a[0] = {}", a[0]);

    println!("{:?}", a);

    if a!= [1,2,3,4,5] {
    println!("does not match");
    }

    let b = [1; 10];//10 elements all equal to 1
    for i in 0..b.len() {
        println!("{}",b[i]);
    }

    println!("b took up {} bytes", mem::size_of_val(&b));

    let mtx:[[f32;3];2] = [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0],
    ];
    println!("{:?}", mtx);

    //print diagonal
    for i in 0..mtx.len() {
       for j in 0..mtx[i].len() {
           if i == j {
               println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
           }
       } 
    }
}

fn vectors(){
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);

    a.push(44);                        

    println!("a[0] = {}", a[0]);

    //Option type
    match a.get(6) {
        Some(x) => println!("a[6] = {}", x),
        None => println!("No such element")
    }

    for x in &a { println!("{}", x); }

    a.push(44);
    println!("{:?}", a);

    //last_elem is not 44, it is Some(44)
    let last_elem = a.pop(); //pop() would also give you an Option
    println!("last element is {:?}, a={:?}", last_elem, a);

    //inside while condition add initiate variable, is a.pop() returns None, then the condition breaks, get out of the condition, thus won't break complier
    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}

//&[i32] means borrowing a part of i32
fn use_slice(slice: &mut[i32]){ //add mutable here to change original array
    println!("first elem ={}, len ={}", slice[0], slice.len());
    slice[0] = 2934;
}

fn slices(){
    let mut data = [1,2,3,4,5];

    use_slice(&mut data[1..4]); //also add mutable here
    println!("{:?}", data); // the value of data won't be the original data [1,2,3,4,5]
}

fn strings(){
    //s is string, which is also a vector of characters - utf8 format
    let s:&'static str = "hello there!"; // &str = string slice - a slice into string
    //static means the entire string 'hello there!' will be included inside our program, when we reference it we reference a particular location in our program.

    for c in s.chars().rev(){
        println!("{}", c);
    }

    //safe way to get first character of string
    if let Some(first_char) = s.chars().nth(0){
        println!("first letter is {}", first_char);
    }

    //another way to do it - String, heap allocated construct
    let mut letters = String::new();
    let mut new_letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8){
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }

    println!("{}", letters);

    // &str <> String conversion
    let u:&str = &letters; //& - deref conversion
    //concatentation
    //String + str
    let z = letters + &new_letters; // use deref here to concatenate

    //How to make a string? 2 ways:
    let mut abc = String::from("hello world");
    abc.remove(0);
    println!("abc is {}", abc);
    //or 
    let mut bce = "hello world".to_string();
    bce.push_str("!!!");
    println!("bce is {}", bce.replace("ello", "goodbye"));
}

//in a array, all the types have to be the same
//tuple is a collection of values of different types
//(i32, i32) makes a tuple
fn sum_and_product(x:i32, y:i32) -> (i32, i32) {
    (x+y, x*y)
}

fn tuples(){
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);
    println!("sp={:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    //destructuring
    let (a, b) = sp;
    println!("a = {}, b ={}", a, b);

    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2);// a tuple of tuples
    println!("{:?}", combined);
    println!("last elem = {}", (combined.1).1);

    let ((c,d), (e,f)) = combined;

    let foo = (true, 42.0, -1i8);
    println!("foo = {:?}", foo);

    let meaning = (42,); //add , inside bracket to make single element tuple, otherwise will be treated a int
    println!("{:?}", meaning);
}

//instead of explicit listed x, y, <T> means there is a generic paramter <T>
struct AnotherPoint<T> {
    x: T,
    y: T
}

struct AnotherLine<T> {
    start: AnotherPoint<T>,
    end: AnotherPoint<T>
}

fn generics(){
    //we can now make any type
    let a:AnotherPoint<f64> = AnotherPoint {x: 0.0, y: 4f64};
    let b = AnotherPoint {x:1.2, y:3.4};
    // or just a:AnotherPoint, rust will guess them

    let myline = AnotherLine {start: a, end: b};
}

fn print_value(x:i32){
    println!("value ={}", x);
}

fn increase(x: &mut i32){
    //dereference the reference and increase the acual value by 1
    *x += 1;
}

fn product(x:i32, y:i32) -> i32 {
    x * y
}

fn functions(){
    print_value(33);

    //declared mutable variable
    let mut z = 1;
    //pass the mutable reference into the function
    increase(&mut z);
    //when execute the function, we changed the original value that we passed in, thus z is 2
    println!("z ={}", z);

    let a = 3;
    let b = 5;
    let p = product(a, b);
    println!("{} * {} = {}", a, b, p);
}

//add behaviour to the structure
impl Line {
    //self reference to the class that we are working with 
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx*dx+dy*dy).sqrt()
    }
}

fn methods(){
    let p = Point {x: 3.0, y:4.0};
    let p2 = Point {x:1.2, y:3.4};
    let myline = Line {start: p, end: p2};
    println!("length = {}", myline.len());
}

fn say_hello(){println!("hello");}

fn closures(){
    let sh = say_hello;
    sh();
    //closure is a function that defines inline
    // use vertical bars to delimit the actual parameters
    let plus_one = |x:i32| -> i32 {x+1};
    let a = 6;
    println!("{} + 1 ={}", a, plus_one(a));

    let mut two = 2;
    //add {} around closure, as we get out of scope, everything is declared locally, inner scope gets destroyed,
    //so we can borrow two after closure
    {
        let plus_two = |x| {
            let mut z = x;
            //two is borrowed by closure here
            z += two;
            z //returns z
        };
        println!("{} + 2 = {}", 3, plus_two(3));
    }
    
    //impossible to use two here if closure has the same scope level as this line
    let borrow_two = &mut two; //after two has been borrowed by closure, we can't use it at all

    //T: by value - make a copy of the value
    //T& - by reference
    // &mut - by mutable reference
    let plus_three = |x:&mut i32| *x += 3;
    let mut f = 12;
    plus_three(&mut f);
    println!("f = {}", f);
}

fn is_even(x:i32) -> bool {
    x % 2 == 0
}

//higher order function
fn hof(){
    let limit = 500;
    let mut sum = 0;
    for i in 0 .. {
        let isq = i * i;
        if isq > limit {break;}
        else if is_even(isq) {sum += isq;}
    }
    println!("loop sum = {}", sum);
    let sum2 = (0..).map(|x| x*x)
        .take_while(|&x| x <= limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum +x);
        println!("hof sum = {}", sum2);
}

fn main() {
    // sh::stack_and_heap();
    // let mut iof = IntOrFloat{i: 123 };
    // iof.i = 234;
    // process_value(IntOrFloat{i:123})
    // tuples();
    // pm::pattern_matching();
    hof();
}
