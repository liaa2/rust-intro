fn how_many(x:i32) -> &'static str {
  match x {
    0 => "no",
    1 | 2 => "one or two",
    12 => "a dozen",
    //give the range a name, 9 to 11 is called z via @
    z @ 9...11 => "lots of",
    _ if (x % 2 == 0) => "some",
    _ => "a few"
  }
}

enum Color {
    Red,
    Green,
    Blue,
    //u8, u8, u8 is tuple format, thus can't put the name as parameters
    RgbColor(u8, u8, u8), //tuple 
    CmykColor{cyan: u8, magenta: u8, yellow: u8, black: u8}, //struct
}

pub fn pattern_matching(){
  for x in 0..13 {
    println!("{}: I have {} oranges", x, how_many(x));
  }

  let point = (1,1);

  match (point) {
    (0,0) => println!("origin"),
    (0, y) => println!("x axis, y={}", y),
    (x,0) => println!("y axis, x ={}", x),
    //if not care what x value is 
    (_,y) => println!("(?, {})", y)
  }

    let c:Color = Color::CmykColor{cyan: 0, magenta: 128, yellow: 0, black: 255};

  match c {
      Color::Red => println!("r"),
      Color::Green => println!("g"),
      // _ => println!("some other color"),
      Color::Blue => println!("b"),
      Color::RgbColor(0,0,0)
      //.. means rest of the values that we don't care about
      | Color::CmykColor{black: 255, ..} => println!("black"),
      Color::RgbColor(r, g, b) => println!("rgb {}, {}, {}",r,g,b),
      _ => ()
  }
}