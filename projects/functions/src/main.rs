fn main() {
    println!("Hello, world!");
    another_function();
    func_with_params(23);
    print_labeled_measurement(5,'h');

    let y = {
        let x=3;
        println!("x: {}",x); //x: 3
        x+1 //세미콜론 없음
    };
    println!("y: {}",y); //y: 4

    let x = five();
    println!("The value of x is: {}",x);

    let x = plus_one(5);
    println!("The value of x is: {}",x);

}

fn another_function(){
    println!("Another function.");
}

fn func_with_params(x:i32){
    println!("The value of x is: {}",x);
}

fn print_labeled_measurement(value: i32, unit_label:char){
    println!("The measurement is: {}{}",value,unit_label);
}

fn five() -> i32{
    5 //returns 5
}

fn plus_one(x:i32)->i32{
    x+1 //리턴을 위해 세미콜론 사용 X
}