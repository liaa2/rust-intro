#![allow(dead_code)]
use std::mem;

struct Point
{
  x: f64, //size of x is 8 bytes
  y: f64  //size of y is 8 bytes
}

fn origin() -> Point
{
  Point{x: 0.0, y: 0.0}
}

pub fn stack_and_heap()
{
  let p1 = origin();
  let p2 = Box::new(origin());//p2 as a pointer to some location where the point is actually stored

  println!("p1 takes up {} bytes", mem::size_of_val(&p1));
  println!("p2 takes up {} bytes", mem::size_of_val(&p2));

  let p3 = *p2; //* means follow the pointer
  println!("{}", p3.x);
}