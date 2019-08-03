enum IpAddrKind {
    //V4 and V6 are known as the variants of the enum
    // V4(String),
    V4(u8, u8, u8, u8),
    V6(String),
}

/*
    Advantages of enums over structs:
    - attach data to each variant of the enum directly, so there is no need for an extra struct.

    - each variant can have different types and amounts of associated data.
*/


// This enum has four variants with different types:
enum Message {
    Quit, // Quit has no data associated with it at all
    Move { x: i32, y: i32 }, // Move includes an anonymous struct inside it
    Write(String), // Write includes a single String
    ChangeColor(i32, i32, i32), //ChangeColor includes three i32 values.
}

// which is the same as using structs
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct


// same as defining methods on structs using impl, we’re also able to define methods on enums.
impl Message {
  fn call(&self) {
      // method body would be defined here
  }  
}

fn main() {
    /*
        Note that the variants of the enum are namespaced under its identifier, and we use a double colon to separate the two. The reason this is useful is that now both values IpAddrKind::V4 and IpAddrKind::V6 are of the same type: IpAddrKind. We can then, for instance, define a function that takes any IpAddrKind.
    */
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);

    // let home = IpAddr::V4(String::from("127.0.0.1"));
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // no Option:: prefix needed for Some & None, no need to bring Option into scope explicitly
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
}

// fn route(ip_kind :IpAddrKind) {}


// Option Enum!
enum Option<T> {
    //Some(T) and None are variants of type Option<T>
    Some(T), // T is a generic type parameter, it means the Some variant of the Option enum can hold one piece of data of any type
    None,
}