fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); //the owner doesn't change
                                     //reference로 받아옴
                                     //수정 불가능

    println!("The length of '{}' is {}", s1, len);

    //Mutable references
    let mut s = String::from("hello");
    change(&mut s);

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    //variables r1 and r2 will not be used after this point
    let r3 = &mut s;
    println!("{}", r3);

    //Dangling References
    let references_to_nothing=dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle()->String{    // not '&s'
    let s=String::from("hello");

    s   // not '&s'
}