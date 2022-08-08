fn main() {
    /*
    [ Ownership rules ]
    1. Each values in Rust has a variables that's called its owner.
    2. There can only be one owner at a time.
    3. When the owner goes out of scope, the value will be dropped.
    */

    {
        //s is not valid here, it's not yet declared

        let s = "hello"; //s is valid from this point forward

        //do stuff with s
    } //this scope is now over, and s is no longer valid

    /*
    - When s comes into scope, it is valid
    - It remains valid until it goes out of scope
    */

    let mut s = String::from("hello");
    s.push_str(", world!"); //push_str() appends a literal to a string
    println!("{}", s); //This will print 'hello, world!'
}
