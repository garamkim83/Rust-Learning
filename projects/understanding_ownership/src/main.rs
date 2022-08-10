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

    //Ways variables and data interact: Move
    {
        let s1 = String::from("hello");
        let s2 = s1;
        //s1과 s2에 있는 값이 각각 stack에 쌓이지 않음
        //대신 s1에 있는 값이 s2로 이동
        //이를 통해 메모리 절약

        //println!("{}, world!", s1); -> 이동된 값이므로 에러 발생
    }

    //Ways variables and data interact: Clone
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1: {}, s2: {}", s1, s2);
    }

    //Stack only Data: Copy
    //integer, boolean, floating point, character, tuple only
    {
        let x = 5;
        let y = x; //x와 y의 5라는 값이 모두 stack에 쌓인다
    }
}
