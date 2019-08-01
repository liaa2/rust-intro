fn main() {
    let s1 = String::from("hello");

    //use function that has a reference to an object as a parameter instead of taking ownership of the value
    let len = calculate_length(&s1);

    //The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it. Because it does not own it, the value it points to will not be dropped when the reference goes out of scope.
    println!("The length of {} is {}", s1, len);


    //Same as variables, references are immutable by default, to make it mutable:
    let mut a = String::from("mutable");

    change(&mut a);

    let mut b = String::from("prevent data races");
    {
        let r1 = &mut b;
    } // r1 goes out of scope here, so we can make a new reference with not problems
    let r2 = &mut b;

    let mut c = String::from("mix of mutable and immutable");

    let c1 = &c; //no problem
    let c2 = &c; //no problem, multiple immutable references are okay because no one who is just reading the data has the ability to affect anyone else’s reading of the data

    println!("{} and {}", c1, c2); 
    //c1 and c2 are no longer used after this point, the reference's scope ends here

    let c3 = &mut c; //no problem, these scopes don't overlap, so this code is allowed.
    println!("{}", c3);

    //At any given time, you can have either ONE mutable reference or any number of immutable references.
}

//These ampersands are references, and they allow you to refer to some value without taking ownership of it
fn calculate_length(s: &String) -> usize{ //s is a reference to a String
    s.len()
}// Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens.


fn change(some_string: &mut String) {
    some_string.push_str(", world");
}