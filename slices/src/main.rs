//Another data type that does not have ownership is the slice. Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.

fn main() {
    first_world("hello, world");
}

fn first_world(s: &String) -> usize {
    // Because we need to go through the String element by element and check whether a value is a space, we’ll convert our String to an array of bytes using the as_bytes method.
    let bytes = s.as_bytes();

    //create an iterator over the array of bytes using the iter method, it returns each element in a collection and that enumerate wraps the result of iter and returns each element as part of a tuple instead
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
