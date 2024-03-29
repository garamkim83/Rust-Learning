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
    //spaces = spaces.len(); 는 자료형이 달라서 안 됨
    println!("{}", spaces);

    let z: f32 = 3.0; //float type
    let is_true: bool = false; //boolean
    let c = 'z'; //character

    //the compound type
    let tup: (i32, f64, u8) = (500, 6.4, 1); //the tuple type
    let (t1, t2, t3) = tup;
    println!("The value of t2 is: {}", t2);

    let five_hundred = tup.0; //val.index로 튜플의 각 요소에 접근 가능
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{}, {}, {}", five_hundred, six_point_four, one);

    //the array type
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "Fabruary",
        "March",
        "April",
        "May",
        "June",
        "july",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a: [i32; 5] = [1, 2, 3, 4, 5]; //type: i32 & 5 elements
    println!("The array a[1] is {}", a[1]);
    let a = [3; 5];
    println!("The array a[1] is {}", a[1]);

    //array example
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number.");

    let element = a[index];

    println!(
        "The value od the element at index {} is: {}",
        index, element
    );
}
