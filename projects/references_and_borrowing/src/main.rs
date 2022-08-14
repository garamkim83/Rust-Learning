fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); //the owner doesn't change
                                     //reference로 받아옴
                                     //수정 불가능

    println!("The length of '{}' is {}", s1, len);

    //Mutable references
    let mut s = String::from("hello");
    change(&mut s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
