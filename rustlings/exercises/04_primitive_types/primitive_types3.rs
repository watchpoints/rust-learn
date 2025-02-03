    fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???
    //https://doc.rust-lang.org/book/ch03-02-data-types.html
    let a = [0;100];
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
