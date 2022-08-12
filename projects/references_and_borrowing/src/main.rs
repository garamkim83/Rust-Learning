fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); //the owner doesn't change
                                     //reference로 받아옴

    println!("The length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
