fn main() {
    let s1 = String::from("hello");

    //use function that has a reference to an object as a parameter instead of taking ownership of the value
    let len = calculate_length(&s1);

    //The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it. Because it does not own it, the value it points to will not be dropped when the reference goes out of scope.
    println!("The length of {} is {}", s1, len);


    //Same as variables, references are immutable by default, to make it mutable:
    let mut a = String::from("mutable");

    change(&mut a);
}

//These ampersands are references, and they allow you to refer to some value without taking ownership of it
fn calculate_length(s: &String) -> usize{ //s is a reference to a String
    s.len()
}// Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens.


fn change(some_string: &mut String) {
    some_string.push_str(", world");
}