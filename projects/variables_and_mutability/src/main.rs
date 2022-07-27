use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    let y = 4;
    println!("The value of y is {}", y);

    //constant
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("3 hours = {} seconds", THREE_HOURS_IN_SECONDS);

    //shadowing
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }

    println!("The value of y is {}", y);

    let spaces = "    ";
    println!("{}", spaces);
    let spaces = spaces.len();
    println!("{}", spaces);
}
